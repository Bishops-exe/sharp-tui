use super::arena::NodeKey;
use super::mounted::LayoutRect;
use super::renderer::TerminalRenderer;
use crate::event::KeyEvent as SharpKeyEvent;
use crate::event::MouseEvent as SharpMouseEvent;
use crate::match_key_event;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};
use dioxus::core::{Element, Event as DioxusEvent};
use dioxus::prelude::VirtualDom;
use std::io::{self, Stdout, Write};
use std::rc::Rc;
use std::time::Duration;

#[inline]
fn is_ctrl_c(key: &SharpKeyEvent) -> bool {
    match_key_event!(*key, Ctrl, 'c')
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Props {
    pub event_poll_speed: Duration,
    pub ctrl_c: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            event_poll_speed: Duration::from_millis(50),
            ctrl_c: false,
        }
    }
}

#[inline]
fn prelaunch(stdout: &mut Stdout) -> Result<(), io::Error> {
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;

    Ok(())
}

#[inline]
fn postlaunch(stdout: &mut Stdout) -> Result<(), io::Error> {
    execute!(stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;

    disable_raw_mode()?;

    Ok(())
}

/// Fires a `mounted` event for every element that registered an `onmounted` listener since the
/// last call. Must run after every `rebuild`/`render_immediate`, since that's when the renderer
/// actually learns about new listeners (via `create_event_listener`). Each element only ever
/// appears here once — `onmounted` is a one-shot; see `rerender_resized` for what keeps a
/// component reactive to that element's layout afterward.
fn dispatch_pending_mounted(vdom: &mut VirtualDom, renderer: &mut TerminalRenderer) {
    for (id, node_id) in renderer.take_pending_mounted() {
        vdom.runtime().handle_event(
            "mounted",
            DioxusEvent::new(Rc::new(node_id), false).into_any(),
            id,
        );
    }
}

/// Re-renders every scope that called `measure_element` on a node whose position or size
/// actually changed in the frame that just painted — directly, via `Runtime::needs_update`,
/// rather than through a separate resize event/listener. Must run after `paint`, since that's
/// when the new layout — and thus whether anything actually changed — becomes known.
///
/// Unlike `handle_event` (which sets up the runtime context itself while it runs listeners),
/// `needs_update` expects a runtime to already be active on the current thread — hence the
/// explicit `RuntimeGuard` here.
fn rerender_resized(
    vdom: &VirtualDom,
    renderer: &TerminalRenderer,
    changed: &[(NodeKey, LayoutRect)],
) {
    let scopes = renderer.scopes_for(changed);
    if scopes.is_empty() {
        return;
    }
    let _guard = dioxus::core::RuntimeGuard::new(vdom.runtime().clone());
    for scope in scopes {
        vdom.runtime().needs_update(scope);
    }
}

/// Runs every `use_key_event` handler for `key`. Unlike mouse events, key events have no
/// `ElementId` to hit-test against, so this goes straight to the renderer's subscriber registry
/// instead of `Runtime::handle_event`. Handlers may write to signals, so they need the same
/// `RuntimeGuard` `rerender_resized` sets up for the same reason.
fn dispatch_key_event(vdom: &VirtualDom, renderer: &TerminalRenderer, key: SharpKeyEvent) {
    let handlers = renderer.key_listeners();
    if handlers.is_empty() {
        return;
    }
    let _guard = dioxus::core::RuntimeGuard::new(vdom.runtime().clone());
    for handler in handlers {
        handler(key);
    }
}

/// Builds and runs a `VirtualDom` for `app`, painting every frame to the alternate screen via
/// this crate's [`TerminalRenderer`].
pub fn launch(app: fn() -> Element, props: Props) -> io::Result<()> {
    // `render_immediate` drives dioxus tasks synchronously, but hooks like
    // `dioxus_sdk_time::use_interval` register their timers with Tokio. Entering a runtime here
    // (and holding it for the whole run) gives those timers a reactor to register with, even
    // though nothing ever calls `block_on`.
    let tokio_runtime = tokio::runtime::Runtime::new()?;
    let _tokio_guard = tokio_runtime.enter();

    let mut vdom = VirtualDom::new(app);
    let mut renderer = TerminalRenderer::new();
    vdom.rebuild(&mut renderer);
    dispatch_pending_mounted(&mut vdom, &mut renderer);

    let mut stdout = io::stdout();

    prelaunch(&mut stdout)?;
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = postlaunch(&mut io::stdout());
        prev_hook(info);
    }));

    let result = run_loop(&props, &mut vdom, &mut renderer, &mut stdout);
    postlaunch(&mut stdout)?;
    drop(std::panic::take_hook());

    result
}

fn run_loop(
    props: &Props,
    vdom: &mut VirtualDom,
    renderer: &mut TerminalRenderer,
    out: &mut impl Write,
) -> io::Result<()> {
    let mut redraw = true;
    let (mut width, mut height) = size()?;

    loop {
        if event::poll(props.event_poll_speed)? {
            match event::read()? {
                Event::Key(key) if props.ctrl_c && is_ctrl_c(&key.into()) => {
                    return Ok(());
                }
                Event::Key(key) => {
                    dispatch_key_event(vdom, renderer, SharpKeyEvent::from(key));
                }
                Event::Mouse(mouse_event) => {
                    if let Some(id) = renderer.hit_test(mouse_event.into()) {
                        let data = Rc::new(SharpMouseEvent::from(mouse_event));
                        vdom.runtime().handle_event(
                            "mouseevent",
                            DioxusEvent::new(data, true).into_any(),
                            id,
                        );
                    }
                }

                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                    redraw = true;
                }
                _ => {}
            }
        }

        vdom.render_immediate(renderer);
        dispatch_pending_mounted(vdom, renderer);
        if redraw || renderer.take_dirty() {
            let changed = renderer.paint(out, width as u32, height as u32)?;
            rerender_resized(vdom, renderer, &changed);
            redraw = false;
        }
    }
}
