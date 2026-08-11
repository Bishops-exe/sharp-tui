use crate::props::sides::Sides;
use crate::props::size_unit::SizeUnit;
use crate::render::Apply;
use yoga::{Node as YogaNode, PositionType};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Position {
    Relative(Sides<SizeUnit>),
    Absolute(Sides<SizeUnit>),
    Static,
}

impl Default for Position {
    fn default() -> Self {
        Position::Relative(Sides::default())
    }
}

impl Apply for Position {
    fn apply(&self, yoga: &mut YogaNode) {
        match self {
            Position::Static => yoga.set_position_type(PositionType::Static),
            Position::Relative(sides) => {
                yoga.set_position_type(PositionType::Relative);
                apply_position_sides(yoga, sides);
            }
            Position::Absolute(sides) => {
                yoga.set_position_type(PositionType::Absolute);
                apply_position_sides(yoga, sides);
            }
        }
    }
}

fn apply_position_sides(yoga: &mut YogaNode, sides: &Sides<SizeUnit>) {
    for (edge, size) in sides.iterate() {
        yoga.set_position(edge, *size);
    }
}
