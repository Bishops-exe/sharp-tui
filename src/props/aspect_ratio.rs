use crate::render::Apply;
use crate::wrap;
use ordered_float::OrderedFloat;
use yoga::Node as YogaNode;

wrap!(pub AspectRatio => OrderedFloat<f32>; default OrderedFloat(0.0));

impl Apply for AspectRatio {
    fn apply(&self, yoga: &mut YogaNode) {
        let ratio = f32::from(**self);
        if ratio > 0.0 {
            yoga.set_aspect_ratio(ratio);
        }
    }
}
