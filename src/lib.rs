#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod components;
mod event;
mod macros;
pub mod props;
mod render;
mod tags;
pub mod utils;

pub use crossterm;
pub use event::{KeyEvent, MouseEvent};
pub use props::*;
pub use render::Props;
pub use render::launch;
pub use render::{LayoutRect, NodeId, measure_element, use_key_event};
pub use tags::{Block, Text};
pub use yoga;

pub use macros::hotkey::*;
pub use macros::no;
