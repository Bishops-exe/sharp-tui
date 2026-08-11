use crate::props::size_clamp::SizeClamp;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;

wrap!(pub Height => SizeClamp);

impl Apply for Height {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_height(*self.value);
        yoga.set_min_height(*self.min);
        yoga.set_max_height(*self.max);
    }
}