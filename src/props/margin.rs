use crate::Sides;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;
use yoga::prelude::Point;

wrap!(pub Margin => Sides<i32>);

impl Apply for Margin {
    fn apply(&self, yoga: &mut YogaNode) {
        for (edge, size) in self.iterate() {
            yoga.set_margin(edge, size.point())
        }
    }
}
