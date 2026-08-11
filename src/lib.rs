#![forbid(unsafe_code)]

mod event;
mod macros;
pub mod props;
mod render;
mod tags;
pub mod components;

pub use yoga;
pub use crossterm;
pub use event::{KeyEvent, MouseEvent};
pub use render::launch;
pub use render::Props;
pub use render::{measure_element, use_key_event, LayoutRect, NodeId};
pub use tags::{Block, Text};
pub use props::*;

pub use macros::no;
pub use macros::hotkey::*;
