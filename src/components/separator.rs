use crate::{Block, Cell, Flex, Margin, NodeId, Text, measure_element, no};
use crossterm::style::ContentStyle;
use dioxus::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SeparatorCharset {
    horizontal: Cell,
    vertical: Cell,
}

impl SeparatorCharset {
    pub fn get(&self, dir: SeparatorDirection) -> Cell {
        if dir == SeparatorDirection::Horizontal {
            self.horizontal
        } else {
            self.vertical
        }
    }

    pub fn classic() -> SeparatorCharset {
        SeparatorCharset {
            horizontal: '-'.into(),
            vertical: '|'.into(),
        }
    }
    pub fn box_char() -> SeparatorCharset {
        SeparatorCharset {
            horizontal: '─'.into(),
            vertical: '│'.into(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SeparatorDirection {
    Horizontal,
    Vertical,
}

#[component]
pub fn Separator(
    dir: SeparatorDirection,
    charset: SeparatorCharset,
    #[props(default)] style: ContentStyle,
    #[props(default)] margin: Margin,
) -> Element {
    let mut track = use_signal(|| None::<NodeId>);
    let mut text_state: Signal<(u16, Cell)> = use_signal(|| (0, ' '.into()));

    // `measure_element` also registers this render as an observer of the track, so the renderer
    // re-renders this component directly whenever the track's actual size changes (a terminal
    // resize, a sibling's content changing how much room is left, etc.) — no separate resize
    // listener needed. Before the track's first paint there's nothing to measure yet, so the bar
    // starts out empty and fills in on the very next re-render.
    if let Some(rect) = track().and_then(measure_element) {
        let text_width = if dir == SeparatorDirection::Horizontal {
            rect.width()
        } else {
            rect.height()
        };

        let state = (text_width, charset.get(dir));

        if text_state() != state {
            text_state.set(state);
        }
    };

    let (text_width, character) = text_state();
    let text = character.content().to_string().repeat(text_width as usize);

    rsx! {
        Block {
            margin,
            on_mounted: move |id: NodeId| track.set(Some(id)),
            flex: Flex::new(1, no!(), no!(), no!(), no!()),
            Text {
                style,
                "{text}"
            }
        }
    }
}
