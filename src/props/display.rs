use crate::render::Apply;
use crate::wrap;
use yoga::Display as YGDisplay;
use yoga::Node as YogaNode;

wrap!(pub Display => YGDisplay; default YGDisplay::Flex);

impl Apply for Display {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_display(**self);
    }
}
