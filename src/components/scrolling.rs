use crate::event::MouseEvent;
use crate::render::{measure_element, NodeId};
use crate::{Block, Height, Margin, Overflow, Sides, Width, XYProps};
use crossterm::event::MouseEventKind;
use dioxus::prelude::*;


#[component]
pub fn Scrolling(
    children: Element,
    #[props(default)] width: Width,
    #[props(default)] height: Height,
    #[props(default = XYProps::both(false))] lock_scroll: XYProps<bool>,
    /// The current scroll offset. The parent owns this — `Scrolling` never tracks it itself,
    /// only ever reads it and reports the offset the mouse wheel asks for via `on_scroll`, same
    /// as `Input`'s `value`/`on_change`.
    #[props(default)] scroll: XYProps<u16>,
    #[props(default)] on_scroll: EventHandler<XYProps<u16>>,
    /// Fires once, when the viewport itself first has a layout to report. Forwarded from the
    /// inner `Block` so a parent that needs the viewport's own rect (to compute an auto-scroll
    /// offset, say) doesn't have to duplicate this component's layout structure to get it.
    #[props(default)] on_mounted: EventHandler<NodeId>,
) -> Element {
    let mut viewport = use_signal(|| None::<NodeId>);
    let mut content = use_signal(|| None::<NodeId>);

    // Both are `None` until their first paint; nothing scrolls until then, which is fine — there
    // can't be anything to scroll to yet either.
    let viewport_size = viewport().and_then(measure_element);
    let content_size = content().and_then(measure_element);

    // How far there is to scroll on each axis: zero (and so, in effect, disabled) whenever the
    // content already fits the viewport — the viewport's own size is exactly the `width`/`height`
    // this component was given, inset by the border it always draws.
    let (max_x, max_y) = match (viewport_size, content_size) {
        (Some(v), Some(c)) => (
            c.width().saturating_sub(v.width()),
            c.height().saturating_sub(v.height()),
        ),
        _ => (0, 0),
    };

    let x_offset = if lock_scroll.x { 0 } else { scroll.x.min(max_x) };
    let y_offset = if lock_scroll.y { 0 } else { scroll.y.min(max_y) };

    rsx! {
        Block {
            width,
            height,
            on_mounted: move |id: NodeId| {
                viewport.set(Some(id));
                on_mounted.call(id);
            },
            on_mouse_event: move |e: MouseEvent| {
                let mut offset = scroll;
                match e.kind {
                    MouseEventKind::ScrollDown => offset.y = offset.y.saturating_add(1),
                    MouseEventKind::ScrollUp => offset.y = offset.y.saturating_sub(1),
                    MouseEventKind::ScrollLeft => offset.x = offset.x.saturating_sub(1),
                    MouseEventKind::ScrollRight => offset.x = offset.x.saturating_add(1),
                    _ => return,
                };
                on_scroll.call(offset);
            },
            overflow: XYProps::both(Overflow::new(yoga::Overflow::Scroll)),

            Block {
                on_mounted: move |id: NodeId| content.set(Some(id)),
                margin: Margin::new(
                    Sides::all(0).left(-(x_offset as i32)).top(-(y_offset as i32))
                ),
                {children}
            }
        }
    }
}