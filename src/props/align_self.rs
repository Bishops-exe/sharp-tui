use crate::props::align_item::AlignItem;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;

wrap!(pub AlignSelf => AlignItem);

impl Apply for AlignSelf {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_align_self(self.to_yoga());
    }
}