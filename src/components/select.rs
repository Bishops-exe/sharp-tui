use crate::{
    Block, Height, KeyEvent, NodeId, Text, Width, XYProps, measure_element, no, use_key_event,
};
use crossterm::event::KeyCode;
use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;

use crate::components::Scrolling;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Shared state a `Select` hands down to its `SelectOption` children: which index is currently
/// highlighted, how many options are mounted (for wraparound), and each option's node id (so
/// `Select` can measure an option to decide whether it needs to be scrolled into view).
#[derive(Clone)]
struct SelectState {
    selected: Signal<u32>,
    count: Signal<u32>,
    nodes: Rc<RefCell<HashMap<u32, NodeId>>>,
    clicking_disabled: bool,
}

/// A keyboard-navigable list. Wrap `SelectOption { index, .. }` children in it — `index` should
/// match the position you'd hand the option in your own list (e.g. a loop counter), since that's
/// what `on_enter` reports back. Everything is wrapped in a `Scrolling` viewport that auto-scrolls
/// whenever the highlighted option would otherwise fall outside it.
#[component]
pub fn Select(
    children: Element,
    on_enter: EventHandler<u32>,
    /// Whether this `Select` should react to `Up`/`Down`/`Enter`. Keep this in sync with whatever
    /// your app considers "focused" — e.g. only the active field in a form.
    #[props(default)]
    active: bool,
    #[props(default)] width: Width,
    #[props(default)] height: Height,
    #[props(default)] disable_clicking: bool,
) -> Element {
    let selected = use_signal(|| 0u32);
    let count = use_signal(|| 0u32);
    let nodes = use_hook(|| Rc::new(RefCell::new(HashMap::<u32, NodeId>::new())));

    use_context_provider(|| SelectState {
        selected,
        count,
        nodes: nodes.clone(),
        clicking_disabled: disable_clicking,
    });

    use_key_event(move |e: KeyEvent| {
        if !active || (!e.is_press() && !e.is_repeat()) {
            return;
        }

        let total = count();
        if total == 0 {
            return;
        }

        // `use_key_event` stores its handler as `Rc<dyn Fn(KeyEvent)>`, so it can't be `FnMut` —
        // `Signal::set` needs `&mut self`, which an `Fn` closure can't give its captures, so
        // writes here go through `write_unchecked` (`&self`) instead (same as `Input`).
        match e.code {
            KeyCode::Up => *selected.write_unchecked() = (selected() + total - 1) % total,
            KeyCode::Down => *selected.write_unchecked() = (selected() + 1) % total,
            KeyCode::Enter => on_enter.call(selected()),
            _ => {}
        }
    });

    let mut viewport = use_signal(|| None::<NodeId>);
    let mut scroll = use_signal(|| XYProps::<u16>::both(0));
    let mut rect = use_signal(|| no!());

    if let Some(w) = viewport().and_then(measure_element)
        && w != rect()
    {
        rect.set(w);
    };

    let viewport_rect = rect();
    let option_rect = nodes
        .borrow()
        .get(&selected())
        .copied()
        .and_then(measure_element);
    if let Some(option_rect) = option_rect {
        let mut offset = scroll();
        if option_rect.first.y < viewport_rect.first.y {
            offset.y = offset
                .y
                .saturating_sub((viewport_rect.first.y - option_rect.first.y) as u16);
        } else if option_rect.second.y > viewport_rect.second.y {
            offset.y = offset
                .y
                .saturating_add((option_rect.second.y - viewport_rect.second.y) as u16);
        }
        if offset != scroll() {
            scroll.set(offset);
        }
    }

    rsx! {
        Scrolling {
            width,
            height,
            scroll: scroll(),
            on_scroll: move |offset: XYProps<u16>| scroll.set(offset),
            on_mounted: move |id: NodeId| viewport.set(Some(id)),
            {children}
        }
    }
}

/// A single `Select` option. `index` should match the position this option occupies in the
/// caller's own list — it's both this option's identity (for highlighting/measurement) and the
/// value `Select::on_enter` reports back when it's chosen.
#[component]
pub fn SelectOption(
    index: u32,
    children: Element,
    #[props(default)] style: ContentStyle,
) -> Element {
    let state: SelectState = use_context();
    let mut selected = state.selected;
    let mut count = state.count;
    let nodes = state.nodes;

    use_hook(|| count.set(count() + 1));
    use_drop({
        let nodes = nodes.clone();
        move || {
            count.set(count().saturating_sub(1));
            nodes.borrow_mut().remove(&index);
        }
    });

    let is_selected = selected() == index;
    let applied_style = if is_selected { style.reverse() } else { style };

    rsx! {
        Block {
            on_mouse_event: move |e: crate::MouseEvent| {
                if state.clicking_disabled {
                    return;
                }
                if !e.kind.is_down() {
                    return
                }

                selected.set(index);
            },
            on_mounted: move |id: NodeId| {
                nodes.borrow_mut().insert(index, id);
            },
            Text {
                style: applied_style,
                {children}
            }
        }
    }
}
