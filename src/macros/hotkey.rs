use crate::KeyEvent;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn contains_modifier(event: KeyEvent, modifier: KeyModifiers) -> bool {
    event.modifiers.contains(modifier)
}

pub fn is_key(event: KeyEvent, key: KeyCode) -> bool {
    event.code == key
}

#[macro_export]
macro_rules! match_key {
    ($e:expr, Shift) => {
        $crate::macros::hotkey::contains_modifier($e, $crate::crossterm::event::KeyModifiers::SHIFT)
    };
    ($e:expr, Ctrl) => {
        $crate::macros::hotkey::contains_modifier(
            $e,
            $crate::crossterm::event::KeyModifiers::CONTROL,
        )
    };
    ($e:expr, Control) => {
        $crate::macros::hotkey::contains_modifier(
            $e,
            $crate::crossterm::event::KeyModifiers::CONTROL,
        )
    };
    ($e:expr, Alt) => {
        $crate::macros::hotkey::contains_modifier($e, $crate::crossterm::event::KeyModifiers::ALT)
    };
    ($e:expr, Meta) => {
        $crate::macros::hotkey::contains_modifier($e, $crate::crossterm::event::KeyModifiers::META)
    };
    ($e:expr, Super) => {
        $crate::macros::hotkey::contains_modifier($e, $crate::crossterm::event::KeyModifiers::SUPER)
    };
    ($e:expr, Hyper) => {
        $crate::macros::hotkey::contains_modifier($e, $crate::crossterm::event::KeyModifiers::HYPER)
    };

    ($e:expr, Backspace) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Backspace)
    };
    ($e:expr, Tab) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Tab)
    };
    ($e:expr, Delete) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Delete)
    };
    ($e:expr, Enter) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Enter)
    };
    ($e:expr, Insert) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Insert)
    };
    ($e:expr, DownArrow) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Down)
    };
    ($e:expr, UpArrow) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Up)
    };
    ($e:expr, LeftArrow) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Left)
    };
    ($e:expr, RightArrow) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Right)
    };
    ($e:expr, CapsLock) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::CapsLock)
    };
    ($e:expr, BackTab) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::BackTab)
    };
    ($e:expr, F$num:tt) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::F($num))
    };
    ($e:expr, End) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::End)
    };
    ($e:expr, PrintScreen) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::PrintScreen)
    };
    ($e:expr, Home) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Home)
    };
    ($e:expr, KeypadBegin) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::KeypadBegin)
    };
    ($e:expr, Media$key:ident) => {
        $crate::macros::hotkey::is_key(
            $e,
            $crate::crossterm::event::KeyCode::Media($crate::crossterm::event::MediaKeyCode::$key),
        )
    };
    ($e:expr, ScrollLock) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::ScrollLock)
    };
    ($e:expr, Esc) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Esc)
    };
    ($e:expr, Null) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Null)
    };
    ($e:expr, NumLock) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::NumLock)
    };
    ($e:expr, PageUp) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::PageUp)
    };
    ($e:expr, PageDown) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::PageDown)
    };
    ($e:expr, Menu) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Menu)
    };
    ($e:expr, Pause) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Pause)
    };
    ($e:expr, $char:literal) => {
        $crate::macros::hotkey::is_key($e, $crate::crossterm::event::KeyCode::Char($char))
    };
}

#[macro_export]
macro_rules! match_key_event {
    ( $event:expr, $( $tail:tt ),+ ) => {
        [$($crate::match_key_event!(@eval $event, $tail)),+].iter().all(|&result| result)
    };

    (@eval $event:expr, ! $key:tt) => {
        !$crate::match_key!($event, $key)
    };

    (@eval $event:expr, $key:tt) => {
        $crate::match_key!($event, $key)
    };
}
#[allow(unused_imports)]
pub use match_key;
#[allow(unused_imports)]
pub use match_key_event;
