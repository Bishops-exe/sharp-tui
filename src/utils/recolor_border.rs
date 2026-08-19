use crate::{BorderCharset, Cell};
use crossterm::style::{ContentStyle, StyledContent};

pub fn recolor_border(charset: BorderCharset, style: Option<ContentStyle>) -> BorderCharset {
    let Some(style) = style else {
        return charset;
    };

    let mut charset = charset.clone();
    
    for i in [
        &mut charset.top,
        &mut charset.bottom,
        &mut charset.left,
        &mut charset.right,
        &mut charset.top_left,
        &mut charset.top_right,
        &mut charset.bottom_right,
        &mut charset.bottom_left,
    ] {
        *i = Cell::new(StyledContent::new(style, *i.content()))
    };
    
    charset
}