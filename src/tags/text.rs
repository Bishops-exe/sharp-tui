use super::dioxus_elements;
use crate::props::TextWrap;
use crossterm::style::ContentStyle;
use dioxus::core::{AttributeValue, Element};
use dioxus::prelude::*;

/// A styled run of text. `children` is normally bare text, but may also contain other `Text`
/// nodes: a nested `Text` acts as an inline style span rather than a separate layout box, so its
/// `style` merges with (and can override individual colors/attributes of) its ancestor's, letting
/// one `Text` mix multiple styles, e.g. `Text { "plain " Text { style: bold, "bold" } }`. A span's
/// own `wrap` is ignored — only the outermost `Text`'s `wrap` applies to the whole paragraph.
#[component]
pub fn Text(
    #[props(default)] style: ContentStyle,
    #[props(default)] wrap: TextWrap,
    children: Element,
) -> Element {
    rsx! {
        text {
            "style": AttributeValue::any_value(style),
            "wrap": AttributeValue::any_value(wrap),
            {children}
        }
    }
}
