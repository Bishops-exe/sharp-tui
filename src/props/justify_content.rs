use crate::render::Apply;
use crate::wrap;
use yoga::Justify;
use yoga::Node as YogaNode;

wrap!(pub JustifyContent => Justify; default Justify::FlexStart);

impl Apply for JustifyContent {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_justify_content(**self);
    }
}
