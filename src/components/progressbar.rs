use crate::Cell;
use crate::no;
use crate::render::{NodeId, measure_element};
use crate::{Block, Flex, Text};
use dioxus::prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ProgressBarCharset {
    start_filled: Cell,
    center_filled: Cell,
    end_filled: Cell,
    remaining: Cell,
}

impl ProgressBarCharset {
    pub fn classic() -> ProgressBarCharset {
        ProgressBarCharset {
            start_filled: '#'.into(),
            center_filled: '#'.into(),
            end_filled: '#'.into(),
            remaining: '.'.into(),
        }
    }
    pub fn diamond() -> ProgressBarCharset {
        ProgressBarCharset {
            start_filled: '<'.into(),
            center_filled: '#'.into(),
            end_filled: '>'.into(),
            remaining: '-'.into(),
        }
    }
}

impl Default for ProgressBarCharset {
    fn default() -> Self {
        Self::classic()
    }
}

/// Renders exactly `width` characters: `filled` glyphs (start/center/end), then `remaining`
/// glyphs for the rest. `width` is only known once the track `Block` has been measured via
/// `onmounted` (see [`ProgressBar`]), so this has to happen at render time, not build time.
pub fn render_bar(charset: ProgressBarCharset, width: u16, filled: u16) -> String {
    let filled = filled.min(width);
    let mut bar = String::with_capacity(width as usize);
    for i in 0..filled {
        let cell = if i == 0 {
            charset.start_filled
        } else if i == filled - 1 {
            charset.end_filled
        } else {
            charset.center_filled
        };
        bar.push(*cell.content());
    }
    for _ in filled..width {
        bar.push(*charset.remaining.content());
    }
    bar
}

#[component]
pub fn ProgressBar(
    percent: u8,
    #[props(default = true)] show_percentage_text: bool,
    #[props(default)] charset: ProgressBarCharset,
) -> Element {
    assert!(percent <= 100, "Percent cannot be greater than 100");

    let mut track = use_signal(|| None::<NodeId>);
    let mut width = use_signal(|| 0u16);

    // `measure_element` also registers this render as an observer of the track, so the renderer
    // re-renders this component directly whenever the track's actual size changes (a terminal
    // resize, a sibling's content changing how much room is left, etc.) — no separate resize
    // listener needed. Before the track's first paint there's nothing to measure yet, so the bar
    // starts out empty and fills in on the very next re-render.
    if let Some(w) = track().and_then(measure_element).map(|rect| rect.width())
        && w != width()
    {
        width.set(w);
    };

    let percent_text = if show_percentage_text {
        format!(" {}%", percent)
    } else {
        "".into()
    };

    let bar_width = width().saturating_sub(percent_text.len() as u16);
    let filled = bar_width * percent as u16 / 100;
    let bar = render_bar(charset, bar_width, filled);

    rsx! {
        Block {
            flex: Flex::new(1, no!(), no!(), no!(), no!()),
            on_mounted: move |id: NodeId| track.set(Some(id)),
            Text {
                "{bar}{percent_text}"
            }
        }
    }
}
