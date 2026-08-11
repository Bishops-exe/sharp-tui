use crate::render::Apply;
use crate::wrap;
use yoga::Align;
use yoga::Node as YogaNode;

wrap!(pub AlignContent => Align; default Align::FlexStart);

impl Apply for AlignContent {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_align_content(**self);
    }
}