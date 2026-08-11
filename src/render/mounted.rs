use super::arena::NodeKey;
use dioxus::core::{ScopeId, current_scope_id};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;
use yoga::Layout;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// A node's position and size, in terminal cells, as of the most recent paint. `first` is the
/// top-left cell; `second` is exclusive (`first + (width, height)`), matching the half-open
/// ranges used elsewhere for clipping and hit-testing.
#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct LayoutRect {
    pub first: Point,
    pub second: Point,
}

impl From<Layout> for LayoutRect {
    fn from(value: Layout) -> Self {
        let top_left = Point::new(value.left() as i32, value.top() as i32);
        Self {
            first: top_left,
            second: top_left + Point::new(value.width() as i32, value.height() as i32),
        }
    }
}

impl LayoutRect {
    pub fn width(&self) -> u16 {
        (self.second.x - self.first.x) as u16
    }

    pub fn height(&self) -> u16 {
        (self.second.y - self.first.y) as u16
    }

    pub fn translate(&self, rhs: Point) -> LayoutRect {
        Self {
            first: self.first + rhs,
            second: self.second + rhs,
        }
    }
    pub fn inset(&self, amount: i32) -> LayoutRect {
        Self {
            first: self.first + Point::new(amount, amount),
            second: self.second - Point::new(amount, amount),
        }
    }

    /// Whether `point` falls inside this rect, treating `second` as exclusive.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.first.x
            && point.x < self.second.x
            && point.y >= self.first.y
            && point.y < self.second.y
    }

    /// Narrows this rect to also stay inside `other`.
    pub fn intersect(&self, other: LayoutRect) -> LayoutRect {
        Self {
            first: Point::new(
                self.first.x.max(other.first.x),
                self.first.y.max(other.first.y),
            ),
            second: Point::new(
                self.second.x.min(other.second.x),
                self.second.y.min(other.second.y),
            ),
        }
    }
}

/// Opaque handle to a node, delivered by `onmounted`. Pass it to [`measure_element`] — there's
/// nothing platform-specific to call on the id itself, unlike a DOM ref.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NodeId(pub(super) NodeKey);

thread_local! {
    /// Mirrors the current [`TerminalRenderer`](super::renderer::TerminalRenderer)'s layout
    /// cache so [`measure_element`] can be called as a bare function from component code, which
    /// has no other way to reach the renderer. Only one renderer runs at a time in a given
    /// thread (one per `launch` call), so this doesn't need to disambiguate between renderers.
    #[allow(clippy::missing_const_for_thread_local)]
    #[allow(clippy::type_complexity)]
    static LAYOUTS: RefCell<Option<Rc<RefCell<HashMap<NodeKey, LayoutRect>>>>> = RefCell::new(None);
    /// Mirrors the renderer's record of which scope last measured which node, so the renderer
    /// can re-render that scope directly (`Runtime::needs_update`) once that node's layout
    /// actually changes, instead of requiring a separate resize event/listener.
    #[allow(clippy::missing_const_for_thread_local)]
    #[allow(clippy::type_complexity)]
    static RESIZE_SCOPES: RefCell<Option<Rc<RefCell<HashMap<NodeKey, ScopeId>>>>> = RefCell::new(None);
}

pub(super) fn set_layouts(layouts: Rc<RefCell<HashMap<NodeKey, LayoutRect>>>) {
    LAYOUTS.with(|cell| *cell.borrow_mut() = Some(layouts));
}

pub(super) fn set_resize_scopes(scopes: Rc<RefCell<HashMap<NodeKey, ScopeId>>>) {
    RESIZE_SCOPES.with(|cell| *cell.borrow_mut() = Some(scopes));
}

/// The node's position and size, in terminal cells, as of the most recently painted frame.
/// `None` until at least one frame has been painted since the node mounted: layout is only ever
/// computed inside `paint`, so there's nothing to report before the first one.
///
/// Also records the calling scope as an observer of this node — call it from wherever you want
/// re-rendered (typically the component's own render body, not just its `onmounted` handler) and
/// the renderer will re-render that scope on its own the next time this node's layout changes,
/// no separate resize listener needed.
pub fn measure_element(id: NodeId) -> Option<LayoutRect> {
    RESIZE_SCOPES.with(|cell| {
        if let Some(scopes) = cell.borrow().as_ref() {
            scopes.borrow_mut().insert(id.0, current_scope_id());
        }
    });
    LAYOUTS.with(|cell| cell.borrow().as_ref()?.borrow().get(&id.0).copied())
}
