use crate::event::KeyEvent;
use dioxus::core::{ScopeId, current_scope_id, use_drop};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) type KeyListeners = Rc<RefCell<HashMap<ScopeId, Rc<dyn Fn(KeyEvent)>>>>;

thread_local! {
    /// Mirrors the current `TerminalRenderer`'s key-listener registry so `use_key_event` can be
    /// called as a bare hook from component code, which has no other way to reach the renderer.
    static LISTENERS: RefCell<Option<KeyListeners>> = RefCell::new(None);
}

pub(super) fn set_key_listeners(listeners: KeyListeners) {
    LISTENERS.with(|cell| *cell.borrow_mut() = Some(listeners));
}

/// Subscribes the calling component to every key press for as long as it stays mounted. Key
/// events, unlike mouse events, aren't hit-tested to a particular element — there's no element to
/// attach an `on_key_event`-style prop to — so this hook is the only way to observe them.
///
/// `handler` is re-registered on every render, so — like an `EventHandler` prop — it always runs
/// whatever it most recently closed over.
pub fn use_key_event(handler: impl Fn(KeyEvent) + 'static) {
    let scope = current_scope_id();
    LISTENERS.with(|cell| {
        if let Some(listeners) = cell.borrow().as_ref() {
            listeners.borrow_mut().insert(scope, Rc::new(handler));
        }
    });
    use_drop(move || {
        LISTENERS.with(|cell| {
            if let Some(listeners) = cell.borrow().as_ref() {
                listeners.borrow_mut().remove(&scope);
            }
        });
    });
}