use crate::props::size_clamp::SizeClamp;
use crate::render::Apply;
use crate::wrap;
use yoga::Node as YogaNode;

wrap!(pub Width => SizeClamp);

impl Apply for Width {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_width(*self.value);
        yoga.set_min_width(*self.min);
        yoga.set_max_width(*self.max);
    }
}
