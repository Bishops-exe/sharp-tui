use crate::Sides;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;
use yoga::prelude::Point;

wrap!(pub Padding => Sides<usize>);

impl Apply for Padding {
    fn apply(&self, yoga: &mut YogaNode) {
        for (edge, size) in self.iterate() {
            yoga.set_padding(edge, (size as i32).point())
        }
    }
}
