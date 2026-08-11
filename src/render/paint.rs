use super::arena::{Arena, NodeKey, RealNode};
use super::mounted::{LayoutRect, Point};
use crate::Cell;
use crate::props::{Border, TextWrap};
use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color as CtColor, ContentStyle, PrintStyledContent};
use crossterm::terminal::{Clear, ClearType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use yoga::{Direction, MeasureMode, NodeRef, Size};

pub(super) type Layouts = RefCell<HashMap<NodeKey, LayoutRect>>;

struct TextContext {
    content: String,
    wrap: TextWrap,
}

extern "C" fn measure_text(
    node_ref: NodeRef,
    width: f32,
    width_mode: MeasureMode,
    _height: f32,
    _height_mode: MeasureMode,
) -> Size {
    let Some(ctx) =
        yoga::get_node_ref_context(&node_ref).and_then(|b| b.downcast_ref::<TextContext>())
    else {
        return Size {
            width: 0.0,
            height: 0.0,
        };
    };

    let available = match width_mode {
        MeasureMode::Undefined => usize::MAX,
        _ => width.max(0.0).round() as usize,
    };
    let wrapped = ctx.wrap.process(&ctx.content, available);
    let measured_width = wrapped
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as f32;
    let measured_height = wrapped.lines().count() as f32;
    Size {
        width: measured_width,
        height: measured_height,
    }
}

/// Concatenates the bare-text children of a `Text` node (the literal content of e.g.
/// `Text { "hi" }`); nested `Block`/`Text` children are laid out independently and skipped.
fn aggregate_content(arena: &Arena, key: NodeKey) -> String {
    let mut content = String::new();
    for &child in arena.get(key).children() {
        if let RealNode::RawText { content: c, .. } = arena.get(child) {
            content.push_str(c);
        }
    }
    content
}

fn sync_measure_contexts(arena: &mut Arena, key: NodeKey) {
    let children: Vec<NodeKey> = arena.get(key).children().to_vec();
    for child in children {
        sync_measure_contexts(arena, child);
    }

    if let RealNode::Text { wrap, .. } = arena.get(key) {
        for &child in arena.get(key).children() {
            assert!(
                matches!(arena.get(child), RealNode::RawText { .. }),
                "Text node has a non-RawText child; Text can only contain bare text, not nested Block/Text/etc."
            );
        }

        let context = TextContext {
            content: aggregate_content(arena, key),
            wrap: *wrap,
        };
        if let RealNode::Text { yoga, .. } = arena.get_mut(key) {
            yoga.set_context(Some(yoga::Context::new(context)));
            yoga.set_measure_func(Some(measure_text));
            // yoga caches the last measurement and only re-invokes `measure_text` once the node
            // is marked dirty; without this, a text node whose content changes (but whose
            // measure func/context pointers get reassigned to equivalent-looking values every
            // paint) keeps its stale size forever.
            yoga.mark_dirty();
        }
    }
}

/// An in-memory grid mirroring the terminal's contents. `paint` renders the whole tree into a
/// fresh buffer each frame, then diffs it against the previous frame's to find what changed.
pub(crate) struct Buffer {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Buffer {
    fn blank(width: u32, height: u32) -> Self {
        Buffer {
            width,
            height,
            cells: vec![vec![Cell::default(); width as usize]; height as usize],
        }
    }

    /// No-op if out of bounds: layout can legally overflow the terminal (e.g. content too big
    /// for the window), and unlike writing straight to the terminal, an in-memory buffer has no
    /// wraparound to fall back on.
    fn set(&mut self, x: u32, y: u32, cell: Cell) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.cells[y as usize][x as usize] = cell;
    }
}

/// Holds the previous frame's buffer across calls to `paint`, so it has something to diff
/// against.
#[derive(Default)]
pub(super) struct Screen {
    buffer: Option<Buffer>,
}

fn set_clipped(buffer: &mut Buffer, clip: Option<LayoutRect>, x: u32, y: u32, cell: Cell) {
    if clip.is_none_or(|c| c.contains(Point::new(x as i32, y as i32))) {
        buffer.set(x, y, cell);
    }
}

fn set_clipped_unsure(buffer: &mut Buffer, clip: Option<LayoutRect>, point: Point, cell: Cell) {
    if let Ok(x) = u32::try_from(point.x)
        && let Ok(y) = u32::try_from(point.y)
    {
        set_clipped(buffer, clip, x, y, cell);
    }
}
fn draw_block(
    buffer: &mut Buffer,
    clip: Option<LayoutRect>,
    rect: LayoutRect,
    border: &Border,
    bg_color: CtColor,
) {
    if rect.first == rect.second {
        return;
    }

    let cell = Cell::new(ContentStyle {
        background_color: Some(bg_color),
        ..Default::default()
    }.apply(' '));

    for x in rect.first.x..rect.second.x {
        for y in rect.first.y..rect.second.y {
            set_clipped_unsure(buffer, clip, Point::new(x, y), cell);
        }
    }

    let Some(charset) = border.style else {
        return;
    };

    let left_x = rect.first.x;
    let top_y = rect.first.y;
    let right_x = rect.second.x - 1;
    let bottom_y = rect.second.y - 1;

    for x in rect.first.x..rect.second.x {
        set_clipped_unsure(buffer, clip, Point::new(x, top_y), charset.top);
        set_clipped_unsure(buffer, clip, Point::new(x, bottom_y), charset.bottom);
    }

    for y in rect.first.y..rect.second.y {
        set_clipped_unsure(buffer, clip, Point::new(left_x, y), charset.left);
        set_clipped_unsure(buffer, clip, Point::new(right_x, y), charset.right);
    }

    set_clipped_unsure(buffer, clip, rect.first, charset.top_left);
    set_clipped_unsure(
        buffer,
        clip,
        Point::new(right_x, rect.first.y),
        charset.top_right,
    );
    set_clipped_unsure(
        buffer,
        clip,
        Point::new(rect.first.x, bottom_y),
        charset.bottom_left,
    );
    set_clipped_unsure(
        buffer,
        clip,
        Point::new(right_x, bottom_y),
        charset.bottom_right,
    );
}

fn draw_text(
    buffer: &mut Buffer,
    clip: Option<LayoutRect>,
    rect: &LayoutRect,
    content: &str,
    style: &ContentStyle,
    wrap: &TextWrap,
) {
    let wrapped = wrap.process(content, (rect.second.x - rect.first.x) as usize);
    for (row, line) in wrapped.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            set_clipped_unsure(
                buffer,
                clip,
                Point::new(rect.first.x + col as i32, rect.first.y + row as i32),
                Cell::new(style.apply(ch)),
            );
        }
    }
}

/// Updates `key`'s cached layout and, if it actually differs from what was cached before (or
/// nothing was cached yet), records the new value in `changed` so callers can fire `onresize`.
fn record_layout(
    layouts: &Layouts,
    changed: &mut Vec<(NodeKey, LayoutRect)>,
    key: NodeKey,
    rect: LayoutRect,
) {
    let prev = layouts.borrow_mut().insert(key, rect);
    if prev != Some(rect) {
        changed.push((key, rect));
    }
}

fn paint_node(
    arena: &Arena,
    key: NodeKey,
    origin: Point,
    buffer: &mut Buffer,
    layouts: &Layouts,
    changed: &mut Vec<(NodeKey, LayoutRect)>,
    clip: Option<LayoutRect>,
) {
    let node = arena.get(key);
    if matches!(node, RealNode::RawText { .. }) {
        return;
    }

    let (yoga, children) = (node.yoga().unwrap(), node.children());
    let layout: LayoutRect = yoga.get_layout().into();
    let rect = layout.translate(origin);
    record_layout(layouts, changed, key, rect);

    let child_clip = match node {
        RealNode::Block {
            border,
            bg_color,
            overflow,
            ..
        } => {
            draw_block(buffer, clip, rect, border, **bg_color);
            if overflow.clips() {
                // Matches the reservation `Border::apply` makes in yoga: children are already
                // positioned/sized to stay inside it, so the clip should too, or a scrolled
                // child could still paint over the border itself.
                let inner = rect.inset(border.get_inset());
                Some(clip.map(|outer| outer.intersect(inner)).unwrap_or(inner))
            } else {
                clip
            }
        }
        RealNode::Text { style, wrap, .. } => {
            let content = aggregate_content(arena, key);
            draw_text(buffer, clip, &rect, &content, style, wrap);
            clip
        }
        _ => clip,
    };
    for &child in children {
        paint_node(
            arena, child, rect.first, buffer, layouts, changed, child_clip,
        );
    }
}

/// Writes only the cells that differ between `old` and `new`, grouping consecutive same-style
/// changed cells in a row into a single write.
fn flush_diff(out: &mut impl Write, old: &Buffer, new: &Buffer) -> io::Result<()> {
    for y in 0..new.height as usize {
        let (old_row, new_row) = (&old.cells[y], &new.cells[y]);
        let mut x = 0;
        while x < new_row.len() {
            if new_row[x] == old_row[x] {
                x += 1;
                continue;
            }

            let style = new_row[x].style();
            let start = x;
            let mut run = String::new();
            while x < new_row.len() && new_row[x] != old_row[x] && new_row[x].style() == style {
                run.push(*new_row[x].content());
                x += 1;
            }

            out.queue(MoveTo(start as u16, y as u16))?;
            out.queue(PrintStyledContent(style.apply(run)))?;
        }
    }
    Ok(())
}

/// Depth-first search for the topmost `Block` under `(col, row)`. Later siblings paint over
/// earlier ones (see `paint_node`), so they're checked first.
fn hit_test_node(
    arena: &Arena,
    key: NodeKey,
    origin_x: f32,
    origin_y: f32,
    col: f32,
    row: f32,
) -> Option<NodeKey> {
    let node = arena.get(key);
    let layout = node.yoga()?.get_layout();
    let x = origin_x + layout.left();
    let y = origin_y + layout.top();
    let (w, h) = (layout.width(), layout.height());

    for &child in node.children().iter().rev() {
        if let Some(hit) = hit_test_node(arena, child, x, y, col, row) {
            return Some(hit);
        }
    }

    let inside = col >= x && col < x + w && row >= y && row < y + h;
    (inside && matches!(node, RealNode::Block { .. })).then_some(key)
}

/// Finds the topmost `Block` under the terminal cell `(column, row)`, using each node's layout
/// as of the last `paint` call.
pub(super) fn hit_test(arena: &Arena, root: NodeKey, column: u16, row: u16) -> Option<NodeKey> {
    let root_node = arena.get(root);
    let layout = root_node.yoga()?.get_layout();
    let (x, y) = (layout.left(), layout.top());
    let (col, row) = (column as f32, row as f32);
    root_node
        .children()
        .iter()
        .rev()
        .find_map(|&child| hit_test_node(arena, child, x, y, col, row))
}

pub(super) fn paint(
    screen: &mut Screen,
    arena: &mut Arena,
    root: NodeKey,
    out: &mut impl Write,
    max: Point,
    layouts: &Layouts,
) -> io::Result<Vec<(NodeKey, LayoutRect)>> {
    sync_measure_contexts(arena, root);
    arena
        .get_mut(root)
        .yoga_mut()
        .expect("root always has a yoga node")
        .calculate_layout(max.x as f32, max.y as f32, Direction::LTR);

    let mut frame = Buffer::blank(max.x as u32, max.y as u32);
    let mut changed = Vec::new();

    // The root node is an internal mounting point (nothing ever sets attributes on it, since
    // it's created directly by the renderer, not by the `Block` component) so it should stay
    // invisible: paint its children, but never a border/background box for the root itself.
    let root_node = arena.get(root);
    let layout = root_node
        .yoga()
        .expect("root always has a yoga node")
        .get_layout();
    let origin = LayoutRect::from(layout).first;
    paint_node(arena, root, origin, &mut frame, layouts, &mut changed, None);

    // A resized (or first-ever) frame has nothing valid to diff against: clear the terminal for
    // real and diff against a fresh blank buffer, so every non-blank cell gets (re)written.
    let prev = match screen.buffer.take() {
        Some(prev) if prev.width as i32 == max.x && prev.height as i32 == max.y => prev,
        _ => {
            out.queue(Clear(ClearType::All))?;
            Buffer::blank(max.x as u32, max.y as u32)
        }
    };

    flush_diff(out, &prev, &frame)?;
    screen.buffer = Some(frame);

    out.queue(crossterm::style::ResetColor)?;
    out.flush()?;
    Ok(changed)
}
