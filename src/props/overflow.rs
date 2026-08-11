use yoga::Overflow as YGOverflow;
use crate::wrap;

wrap!(pub Overflow => YGOverflow; default YGOverflow::Visible );

impl Overflow {
    /// Whether content outside this box should be clipped at paint time rather than left
    /// visible (e.g. a scrolled child that's shifted past its container's edge).
    pub(crate) fn clips(&self) -> bool {
        **self != YGOverflow::Visible
    }
}