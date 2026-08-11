use crate::props::size_unit::SizeUnit;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SizeClamp {
    pub(crate) min: SizeUnit,
    pub(crate) value: SizeUnit,
    pub(crate) max: SizeUnit,
}

impl SizeClamp {
    pub fn new(min: SizeUnit, value: SizeUnit, max: SizeUnit) -> Self {
        Self {
            min,
            value,
            max
        }
    }
}