use yoga::prelude::Point;
use crate::render::Apply;
use crate::Sides;
use yoga::Node as YogaNode;
use crate::wrap;

wrap!(pub Margin => Sides<i32>);

impl Apply for Margin {
    fn apply(&self, yoga: &mut YogaNode) {
        for (edge, size) in self.iterate() {
            yoga.set_margin(edge, size.point())
        }
    }
}
