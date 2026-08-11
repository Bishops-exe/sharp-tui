use crossterm::style;
use crate::wrap;

wrap!(pub Color => style::Color; default style::Color::Reset);