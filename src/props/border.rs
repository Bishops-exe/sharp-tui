use crate::props::border_charset::BorderCharset;
use crate::render::Apply;
use yoga::{Edge, Node as YogaNode};

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Border {
    pub style: Option<BorderCharset>,
}

impl Border {
    pub fn new(style: Option<BorderCharset>) -> Border {
        Border { style }
    }
}

impl Apply for Border {
    /// Only reserves the yoga edge space for the border; the glyphs/style themselves are
    /// stored separately by the caller since they aren't part of the layout tree.
    fn apply(&self, yoga: &mut YogaNode) {
        yoga.set_border(Edge::All, self.get_inset() as f32);
    }
}

impl Border {
    pub fn get_inset(&self) -> i32 {
        if self.style.is_none() { 0 } else { 1 }
    }
}
