use crate::Cell;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct BorderCharset {
    pub top_left: Cell,
    pub top: Cell,
    pub top_right: Cell,
    pub left: Cell,
    pub right: Cell,
    pub bottom_left: Cell,
    pub bottom: Cell,
    pub bottom_right: Cell,
}

impl BorderCharset {
    pub fn single() -> BorderCharset {
        BorderCharset {
            top_left: '┌'.into(),
            top: '─'.into(),
            top_right: '┐'.into(),
            right: '│'.into(),
            bottom_right: '┘'.into(),
            bottom: '─'.into(),
            bottom_left: '└'.into(),
            left: '│'.into(),
        }
    }

    pub fn double() -> BorderCharset {
        BorderCharset {
            top_left: '╔'.into(),
            top: '═'.into(),
            top_right: '╗'.into(),
            right: '║'.into(),
            bottom_right: '╝'.into(),
            bottom: '═'.into(),
            bottom_left: '╚'.into(),
            left: '║'.into(),
        }
    }
    pub fn round() -> BorderCharset {
        BorderCharset {
            top_left: '╭'.into(),
            top: '─'.into(),
            top_right: '╮'.into(),
            right: '│'.into(),
            bottom_right: '╯'.into(),
            bottom: '─'.into(),
            bottom_left: '╰'.into(),
            left: '│'.into(),
        }
    }
    pub fn bold() -> BorderCharset {
        BorderCharset {
            top_left: '┏'.into(),
            top: '━'.into(),
            top_right: '┓'.into(),
            right: '┃'.into(),
            bottom_right: '┛'.into(),
            bottom: '━'.into(),
            bottom_left: '┗'.into(),
            left: '┃'.into(),
        }
    }

    pub fn single_double() -> BorderCharset {
        BorderCharset {
            top_left: '╓'.into(),
            top: '─'.into(),
            top_right: '╖'.into(),
            right: '║'.into(),
            bottom_right: '╜'.into(),
            bottom: '─'.into(),
            bottom_left: '╙'.into(),
            left: '║'.into(),
        }
    }
    pub fn double_single() -> BorderCharset {
        BorderCharset {
            top_left: '╒'.into(),
            top: '═'.into(),
            top_right: '╕'.into(),
            right: '│'.into(),
            bottom_right: '╛'.into(),
            bottom: '═'.into(),
            bottom_left: '╘'.into(),
            left: '│'.into(),
        }
    }
    pub fn classic() -> BorderCharset {
        BorderCharset {
            top_left: '+'.into(),
            top: '-'.into(),
            top_right: '+'.into(),
            right: '|'.into(),
            bottom_right: '+'.into(),
            bottom: '-'.into(),
            bottom_left: '+'.into(),
            left: '|'.into(),
        }
    }
}
