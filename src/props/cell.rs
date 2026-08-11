use std::ops::Deref;
use crate::no;
use crossterm::style::StyledContent;

/// One terminal cell's intended contents. Compared frame-to-frame so `paint` only writes cells
/// that actually changed, instead of clearing and redrawing the whole screen every time.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Cell(StyledContent<char>);

impl Default for Cell {
    fn default() -> Self {
        Cell::unstyled(' ')
    }
}

impl Deref for Cell {
    type Target = StyledContent<char>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Cell {
    pub fn new(content: StyledContent<char>) -> Self {
        Self(content)
    }

    pub fn unstyled(ch: char) -> Self {
        Self::new(StyledContent::new(no!(), ch))
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        Self::unstyled(ch)
    }
}
