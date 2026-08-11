use crate::props::*;
use crossterm::style::ContentStyle;
use dioxus::core::AttributeValue;
use yoga::Node as YogaNode;

pub trait Apply {
    fn apply(&self, yoga: &mut YogaNode);
}

/// Pull a typed prop back out of an `AttributeValue::Any` (or `Float`, for plain numbers).
pub(super) fn any_prop<T: Clone + 'static>(value: &AttributeValue) -> Option<T> {
    match value {
        AttributeValue::Any(rc) => rc.as_any().downcast_ref::<T>().cloned(),
        _ => None,
    }
}

/// Downcasts `value` to `T` and, if present, applies it to `yoga`.
fn apply_prop<T: Apply + Clone + 'static>(yoga: &mut YogaNode, value: &AttributeValue) {
    if let Some(v) = any_prop::<T>(value) {
        v.apply(yoga);
    }
}

/// Dispatches a `set_attribute` mutation for a `block {}` element to the right style setter,
/// pulling the typed prop back out of the `AttributeValue` it was wrapped in.
pub(super) fn apply_block_attribute(
    yoga: &mut YogaNode,
    border: &mut Border,
    bg_color: &mut Color,
    overflow: &mut Overflow,
    name: &str,
    value: &AttributeValue,
) {
    match name {
        "margin" => apply_prop::<Margin>(yoga, value),
        "padding" => apply_prop::<Padding>(yoga, value),
        "width" => apply_prop::<Width>(yoga, value),
        "height" => apply_prop::<Height>(yoga, value),
        "gap" => apply_prop::<XYProps<usize>>(yoga, value),
        "flex" => apply_prop::<Flex>(yoga, value),
        "align_items" => apply_prop::<AlignItems>(yoga, value),
        "align_self" => apply_prop::<AlignSelf>(yoga, value),
        "align_content" => apply_prop::<AlignContent>(yoga, value),
        "justify_content" => apply_prop::<JustifyContent>(yoga, value),
        "position" => apply_prop::<Position>(yoga, value),
        "overflow" => {
            if let Some(v) = any_prop::<XYProps<Overflow>>(value) {
                v.apply(yoga);
                *overflow = v.effective();
            }
        }
        "display" => apply_prop::<Display>(yoga, value),
        "aspect_ratio" => apply_prop::<AspectRatio>(yoga, value),
        "border" => {
            if let Some(v) = any_prop::<Border>(value) {
                v.apply(yoga);
                *border = v;
            }
        }
        "bg_color" => {
            if let Some(v) = any_prop::<Color>(value) {
                *bg_color = v;
            }
        }
        _ => {}
    }
}

/// Dispatches a `set_attribute` mutation for a `text {}` element.
pub(super) fn apply_text_attribute(
    style: &mut ContentStyle,
    wrap: &mut TextWrap,
    name: &str,
    value: &AttributeValue,
) {
    match name {
        "style" => {
            if let Some(v) = any_prop::<ContentStyle>(value) {
                *style = v;
            }
        }
        "wrap" => {
            if let Some(v) = any_prop::<TextWrap>(value) {
                *wrap = v;
            }
        }
        _ => {}
    }
}
