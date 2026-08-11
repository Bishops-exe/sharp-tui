use crate::props::overflow::Overflow;
use crate::render::Apply;
use yoga::Node as YogaNode;
use yoga::prelude::Point;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct XYProps<T>
where
    T: Default + Clone,
{
    pub x: T,
    pub y: T,
}

impl<T: Default + Clone> XYProps<T> {
    pub fn new(row: T, column: T) -> XYProps<T> {
        Self { x: row, y: column }
    }

    pub fn x(row: T) -> XYProps<T> {
        Self::new(row, T::default())
    }
    pub fn y(col: T) -> XYProps<T> {
        Self::new(T::default(), col)
    }

    pub fn both(both: T) -> XYProps<T> {
        Self::new(both.clone(), both)
    }
}

impl Apply for XYProps<usize> {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_row_gap((self.x as i32).point());
        yoga.set_column_gap((self.y as i32).point());
    }
}

impl XYProps<Overflow> {
    /// Yoga only has a single overflow axis; the `x` value wins unless it's left at the default.
    pub(crate) fn effective(&self) -> Overflow {
        if self.x.clips() { self.x } else { self.y }
    }
}

impl Apply for XYProps<Overflow> {
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_overflow(*self.effective());
    }
}
