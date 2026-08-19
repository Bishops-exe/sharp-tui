use crate::{AlignItem, AlignItems, Block, Border, BorderCharset, MouseEvent, Text};
use crossterm::event::{MouseButton, MouseEventKind};
use crossterm::style::{ContentStyle, Stylize};
use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(default)] children: Element,
    #[props(default)] on_click: EventHandler<()>,
    #[props(default)] disabled: bool,
    border: Option<Border>,
) -> Element {
    let style = disabled
        .then(|| ContentStyle::new().dark_grey())
        .unwrap_or_default();

    let border = border.unwrap_or_else(|| Border::new(Some(BorderCharset::double())));

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
            border: border,
            align_items: AlignItems::new(AlignItem::Center),
            Text {
                style,
                {children}
            }

        }
    }
}
