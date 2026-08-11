use crate::props::align_item::AlignItem;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;

wrap!(pub AlignItems => AlignItem);

impl Apply for AlignItems {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_align_items(self.to_yoga());
    }
}
