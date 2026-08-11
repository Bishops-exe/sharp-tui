use crate::props::flex_wrap::FlexWrap;
use crate::props::size_unit::SizeUnit;
use crate::render::Apply;
use yoga::Node as YogaNode;
use crate::props::flex_direction::FlexDirection;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Flex {
    pub grow: usize,
    pub shrink: usize,
    pub basis: SizeUnit,
    pub wrap: FlexWrap,
    pub direction: FlexDirection,
}

impl Flex {
    pub fn new(grow: usize, shrink: usize, basis: SizeUnit, wrap: FlexWrap, direction: FlexDirection) -> Flex {
        Self {
            grow,
            shrink,
            basis,
            wrap,
            direction
        }
    }
}

impl Apply for Flex {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_flex_grow(self.grow as f32);
        yoga.set_flex_shrink(self.shrink as f32);
        yoga.set_flex_basis(*self.basis);
        yoga.set_flex_wrap(*self.wrap);
        yoga.set_flex_direction(*self.direction);
    }
}