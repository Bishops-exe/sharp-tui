use crate::wrap;
use crossterm::style;

wrap!(pub Color => style::Color; default style::Color::Reset);
