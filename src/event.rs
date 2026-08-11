use crate::wrap;
use crossterm::event::{self, KeyModifiers};

wrap!(pub MouseEvent => event::MouseEvent; default event::MouseEvent {
    kind: event::MouseEventKind::Moved,
    column: 0,
    row: 0,
    modifiers: KeyModifiers::NONE,
});

// Unlike `MouseEvent`, `crossterm::event::KeyEvent` doesn't derive `Eq`/`Hash`, so it can't go
// through the `wrap!` macro (which derives both on the wrapper).
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct KeyEvent {
    inner: event::KeyEvent,
}

impl From<event::KeyEvent> for KeyEvent {
    fn from(value: event::KeyEvent) -> Self {
        Self { inner: value }
    }
}

impl core::ops::Deref for KeyEvent {
    type Target = event::KeyEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
