use crate::{
    AlignItem, AlignItems, Block, Border, BorderCharset, MouseEvent, Text, match_key_event, no,
    use_key_event,
};
use crossterm::event::{MouseButton, MouseEventKind};
use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(default)] children: Element,
    #[props(default)] on_click: EventHandler<()>,
    #[props(default)] disabled: bool,
    active: bool,
    border: Option<BorderCharset>,
    #[props(default)] border_style: ContentStyle,
    pressed_override: Option<bool>,
) -> Element {
    let enter = use_signal(|| false);
    let space = use_signal(|| false);
    let click = use_signal(|| false);

    let style = if disabled {
        ContentStyle::new().dark_grey()
    } else {
        no!()
    };

    let border = Border::new(Some(border.unwrap_or_else(|| {
        if active && !disabled {
            if pressed_override.unwrap_or_else(|| enter() || space() || click()) {
                return BorderCharset::double();
            }

            if active {
                return BorderCharset::single_double();
            }
        }

        BorderCharset::single()
    })));

    use_key_event(move |ev| {
        let value = !ev.kind.is_release();

        if match_key_event!(ev, ' ') {
            *space.write_unchecked() = value;
        } else if match_key_event!(ev, Enter) {
            *enter.write_unchecked() = value
        }
    });

    rsx! {
        Block {
            on_mouse_event: move |e: MouseEvent| {
                if !matches!(e.kind, MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Up(MouseButton::Left)) {
                    return
                }

                let value = e.kind.is_down();

                *click.write_unchecked() = value;
            },
            border: border,
            align_items: AlignItems::new(AlignItem::Center),
            Text {
                style,
                {children}
            }

        }
    }
}
