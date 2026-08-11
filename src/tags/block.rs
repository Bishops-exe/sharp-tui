use super::dioxus_elements;
use crate::event::MouseEvent;
use crate::props::*;
use crate::render::NodeId;
use dioxus::core::{AttributeValue, Element, Event};
use dioxus::prelude::*;

#[component]
pub fn Block(
    #[props(default)] margin: Margin,
    #[props(default)] padding: Padding,
    #[props(default)] width: Width,
    #[props(default)] height: Height,
    #[props(default)] aspect_ratio: AspectRatio,
    #[props(default)] gap: XYProps<usize>,
    #[props(default)] flex: Flex,
    #[props(default)] align_items: AlignItems,
    #[props(default)] align_self: AlignSelf,
    #[props(default)] align_content: AlignContent,
    #[props(default)] justify_content: JustifyContent,
    #[props(default)] position: Position,
    #[props(default)] overflow: XYProps<Overflow>,
    #[props(default)] display: Display,
    #[props(default)] border: Border,
    #[props(default)] bg_color: Color,
    #[props(default)] on_mouse_event: EventHandler<MouseEvent>,
    /// Fires once, when this element first has a layout to report — never again afterward. Call
    /// [`crate::measure_element`] with the id, from wherever you want re-rendered when this
    /// element's layout changes (typically the component's own render body, not just here) —
    /// the renderer re-renders whichever scope last measured a node whenever that node's
    /// position or size actually changes, no separate resize listener needed.
    #[props(default)] on_mounted: EventHandler<NodeId>,
    children: Element,
) -> Element {
    rsx! {
        block {
            "margin": AttributeValue::any_value(margin),
            "padding": AttributeValue::any_value(padding),
            "width": AttributeValue::any_value(width),
            "height": AttributeValue::any_value(height),
            "aspect_ratio": AttributeValue::any_value(aspect_ratio),
            "gap": AttributeValue::any_value(gap),
            "flex": AttributeValue::any_value(flex),
            "align_items": AttributeValue::any_value(align_items),
            "align_self": AttributeValue::any_value(align_self),
            "align_content": AttributeValue::any_value(align_content),
            "justify_content": AttributeValue::any_value(justify_content),
            "position": AttributeValue::any_value(position),
            "overflow": AttributeValue::any_value(overflow),
            "display": AttributeValue::any_value(display),
            "border": AttributeValue::any_value(border),
            "bg_color": AttributeValue::any_value(bg_color),
            "onmouseevent": AttributeValue::listener(move |evt: Event<MouseEvent>| {
                on_mouse_event.call(*evt.data());
            }),
            "onmounted": AttributeValue::listener(move |evt: Event<NodeId>| {
                on_mounted.call(*evt.data());
            }),
            {children}
        }
    }
}
