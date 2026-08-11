use crate::{AlignItem, AlignItems, Block, Border, BorderCharset, MouseEvent, Text, no};
use crossterm::event::{MouseButton, MouseEventKind};
use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(default)] children: Element,
    #[props(default)] on_click: EventHandler<()>,
    #[props(default)] disabled: bool,
) -> Element {
    let style = if disabled {
        ContentStyle::new().dark_grey()
    } else {
        no!()
    };

    rsx! {
        Block {
            on_mouse_event: move |e: MouseEvent| {
                if disabled {
                    return;
                }
                if let MouseEventKind::Down(MouseButton::Left) = e.kind {
                     on_click.call(());
                };
            },
            border: Border::new(Some(BorderCharset::double())),
            align_items: AlignItems::new(AlignItem::Center),
            Text {
                style,
                {children}
            }

        }
    }
}
