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

/// A single character together with the (already-merged) style it should paint with. Building
/// a flat run of these out of a `Text` node's whole subtree is what lets a `Text` nest other
/// `Text` spans for multi-style content while still wrapping/measuring as one paragraph.
#[derive(Clone, Copy)]
struct StyledChar {
    ch: char,
    style: ContentStyle,
}

struct TextContext {
    chars: Vec<StyledChar>,
    wrap: TextWrap,
    /// The node's own `style`, used for measurement (where styling is irrelevant to size) and as
    /// the style of any `...` an ellipsis-style `TextWrap` inserts, since that punctuation isn't
    /// part of any particular nested span.
    fallback_style: ContentStyle,
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
    let wrapped = wrap_styled(ctx.wrap, &ctx.chars, available, ctx.fallback_style);
    let measured_width = wrapped.iter().map(Vec::len).max().unwrap_or(0) as f32;
    let measured_height = wrapped.len() as f32;
    Size {
        width: measured_width,
        height: measured_height,
    }
}

/// Merges a nested span's own `style` onto its ancestor's: colors set on the span win, unset
/// ones fall back to the ancestor's; attributes (bold, italic, ...) from both apply together.
fn merge_style(parent: ContentStyle, child: ContentStyle) -> ContentStyle {
    ContentStyle {
        foreground_color: child.foreground_color.or(parent.foreground_color),
        background_color: child.background_color.or(parent.background_color),
        underline_color: child.underline_color.or(parent.underline_color),
        attributes: parent.attributes | child.attributes,
    }
}

/// Flattens a `Text` node's subtree into a single run of styled characters: bare text
/// (`RawText`) contributes its content at `style`, and a nested `Text` span contributes its own
/// subtree recursively, at `style` merged with that span's own `style`. This is what lets a
/// `Text` node contain other `Text` nodes purely as inline style spans, rather than as
/// independently laid-out boxes.
fn collect_styled(arena: &Arena, key: NodeKey, style: ContentStyle, out: &mut Vec<StyledChar>) {
    for &child in arena.get(key).children() {
        match arena.get(child) {
            RealNode::RawText { content, .. } => {
                out.extend(content.chars().map(|ch| StyledChar { ch, style }));
            }
            RealNode::Text { style: span, .. } => {
                collect_styled(arena, child, merge_style(style, *span), out);
            }
            RealNode::Block { .. } | RealNode::Placeholder { .. } => panic!(
                "Text node has a non-RawText/Text child; Text can only contain bare text or nested Text spans, not Block/etc."
            ),
        }
    }
}

/// Splits `chars` on literal `\n`s, mirroring `str::lines`: a blank line in the middle survives
/// as an empty slice, a trailing newline does not produce a trailing empty one.
fn split_on_newline(chars: &[StyledChar]) -> Vec<&[StyledChar]> {
    let mut paragraphs = Vec::new();
    let mut start = 0;
    for (i, sc) in chars.iter().enumerate() {
        if sc.ch == '\n' {
            paragraphs.push(&chars[start..i]);
            start = i + 1;
        }
    }
    if start < chars.len() {
        paragraphs.push(&chars[start..]);
    }
    paragraphs
}

/// Greedily packs whitespace-separated words from `paragraph` onto lines no wider than `width`,
/// normalizing runs of whitespace between words to a single space of `space_style` — the styled
/// analogue of `textwrap::fill`.
fn wrap_words(
    paragraph: &[StyledChar],
    width: usize,
    space_style: ContentStyle,
) -> Vec<Vec<StyledChar>> {
    let mut lines = Vec::new();
    let mut current: Vec<StyledChar> = Vec::new();
    let mut i = 0;
    while i < paragraph.len() {
        while i < paragraph.len() && paragraph[i].ch.is_whitespace() {
            i += 1;
        }
        let word_start = i;
        while i < paragraph.len() && !paragraph[i].ch.is_whitespace() {
            i += 1;
        }
        let word = &paragraph[word_start..i];
        if word.is_empty() {
            continue;
        }
        if current.is_empty() {
            current.extend_from_slice(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(StyledChar {
                ch: ' ',
                style: space_style,
            });
            current.extend_from_slice(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.extend_from_slice(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(Vec::new());
    }
    lines
}

/// Wraps a single paragraph (no embedded newlines) per `mode`, the styled analogue of
/// [`TextWrap::process`](crate::props::TextWrap::process). `fallback_style` styles any `...` an
/// ellipsis mode inserts.
fn wrap_paragraph(
    mode: TextWrap,
    paragraph: &[StyledChar],
    width: usize,
    fallback_style: ContentStyle,
) -> Vec<Vec<StyledChar>> {
    if paragraph.is_empty() {
        return vec![Vec::new()];
    }
    if paragraph.len() <= width {
        return vec![paragraph.to_vec()];
    }

    let dot = || StyledChar {
        ch: '.',
        style: fallback_style,
    };

    match mode {
        TextWrap::Wrap => wrap_words(paragraph, width, fallback_style),
        TextWrap::Hard => paragraph
            .chunks(width)
            .map(<[StyledChar]>::to_vec)
            .collect(),
        TextWrap::TruncateStart => {
            if width <= 3 {
                return vec![paragraph[paragraph.len() - width..].to_vec()];
            }
            let keep = width - 3;
            let mut row = vec![dot(); 3];
            row.extend_from_slice(&paragraph[paragraph.len() - keep..]);
            vec![row]
        }
        TextWrap::TruncateMiddle => {
            if width <= 3 {
                return vec![paragraph[..width].to_vec()];
            }
            let keep = width - 3;
            let half = keep / 2;
            let extra = keep % 2;
            let mut row = paragraph[..half + extra].to_vec();
            row.extend(std::iter::repeat_with(dot).take(3));
            row.extend_from_slice(&paragraph[paragraph.len() - half..]);
            vec![row]
        }
        TextWrap::Truncate => {
            if width <= 3 {
                return vec![paragraph[..width].to_vec()];
            }
            let keep = width - 3;
            let mut row = paragraph[..keep].to_vec();
            row.extend(std::iter::repeat_with(dot).take(3));
            vec![row]
        }
        TextWrap::Cut => vec![paragraph[..width].to_vec()],
    }
}

/// The styled analogue of `TextWrap::process`: wraps/truncates `chars` per `mode`, returning one
/// row of styled characters per resulting line. `measure_text` and `draw_text` both call this
/// (never the plain-`str` `TextWrap::process`), so a `Text` node's measured size and its painted
/// content are always derived from the exact same wrapping — they can't drift apart.
fn wrap_styled(
    mode: TextWrap,
    chars: &[StyledChar],
    width: usize,
    fallback_style: ContentStyle,
) -> Vec<Vec<StyledChar>> {
    if width == 0 {
        return split_on_newline(chars)
            .into_iter()
            .map(<[StyledChar]>::to_vec)
            .collect();
    }
    split_on_newline(chars)
        .into_iter()
        .flat_map(|paragraph| wrap_paragraph(mode, paragraph, width, fallback_style))
        .collect()
}

fn sync_measure_contexts(arena: &mut Arena, key: NodeKey) {
    // A `Text` node's children are inline spans folded into its own content by `collect_styled`
    // below, not independent layout boxes, so (unlike a `Block`'s or the root's) they're never
    // walked generically here.
    if !matches!(arena.get(key), RealNode::Text { .. }) {
        let children: Vec<NodeKey> = arena.get(key).children().to_vec();
        for child in children {
            sync_measure_contexts(arena, child);
        }
    }

    if let RealNode::Text { style, wrap, .. } = arena.get(key) {
        let fallback_style = *style;
        let wrap = *wrap;
        let mut chars = Vec::new();
        collect_styled(arena, key, fallback_style, &mut chars);

        let context = TextContext {
            chars,
            wrap,
            fallback_style,
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

    let cell = Cell::new(
        ContentStyle {
            background_color: Some(bg_color),
            ..Default::default()
        }
        .apply(' '),
    );

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
    chars: &[StyledChar],
    wrap: TextWrap,
    fallback_style: ContentStyle,
) {
    let wrapped = wrap_styled(
        wrap,
        chars,
        (rect.second.x - rect.first.x) as usize,
        fallback_style,
    );
    for (row, line) in wrapped.iter().enumerate() {
        for (col, sc) in line.iter().enumerate() {
            set_clipped_unsure(
                buffer,
                clip,
                Point::new(rect.first.x + col as i32, rect.first.y + row as i32),
                Cell::new(sc.style.apply(sc.ch)),
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
    // A `Text` node nested inside another `Text` is an inline span, not an independent layout
    // box (see `Arena::attach`): its content was already painted as part of its ancestor's own
    // `draw_text` call, and it has no yoga-tree position of its own to paint at here.
    if matches!(node, RealNode::Text { .. })
        && node
            .parent()
            .is_some_and(|parent| matches!(arena.get(parent), RealNode::Text { .. }))
    {
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
            let mut chars = Vec::new();
            collect_styled(arena, key, *style, &mut chars);
            draw_text(buffer, clip, &rect, &chars, *wrap, *style);
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
/// earlier ones (see `paint_node`), so they're checked first. `clip`, when present, mirrors the
/// clip rect `paint_node` would have applied when painting this node — a point outside it landed
/// on content that an ancestor's `overflow: hidden`/`scroll` clipped away, so it can't be a hit.
fn hit_test_node(
    arena: &Arena,
    key: NodeKey,
    origin: Point,
    point: Point,
    clip: Option<LayoutRect>,
) -> Option<NodeKey> {
    let node = arena.get(key);
    let layout = node.yoga()?.get_layout();
    let rect = LayoutRect::from(layout).translate(origin);

    let child_clip = if let RealNode::Block {
        border, overflow, ..
    } = node
        && overflow.clips()
    {
        let inner = rect.inset(border.get_inset());
        Some(clip.map(|outer| outer.intersect(inner)).unwrap_or(inner))
    } else {
        clip
    };

    for &child in node.children().iter() {
        if let Some(hit) = hit_test_node(arena, child, rect.first, point, child_clip) {
            return Some(hit);
        }
    }

    let inside = rect.contains(point) && clip.is_none_or(|c| c.contains(point));
    (inside && matches!(node, RealNode::Block { .. })).then_some(key)
}

/// Finds the topmost `Block` under the terminal cell `(column, row)`, using each node's layout
/// as of the last `paint` call.
pub(super) fn hit_test(arena: &Arena, root: NodeKey, point: Point) -> Option<NodeKey> {
    let layout = arena.get(root).yoga()?.get_layout();
    let origin = LayoutRect::from(layout).first;
    hit_test_node(arena, root, origin, point, None)
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
