use super::dioxus_elements;
use crate::props::TextWrap;
use crossterm::style::ContentStyle;
use dioxus::core::{AttributeValue, Element};
use dioxus::prelude::*;

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
