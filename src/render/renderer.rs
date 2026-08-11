use super::arena::{Arena, NodeKey, RealNode};
use super::keys::{self, KeyListeners};
use super::mounted::{self, LayoutRect, NodeId, Point};
use super::paint;
use super::paint::Screen;
use super::style::{apply_block_attribute, apply_text_attribute};
use crate::event::KeyEvent;
use bidimap::BiHashMap;
use dioxus::core::{AttributeValue, ElementId, ScopeId, Template, TemplateNode, WriteMutations};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;
use yoga::Node as YogaNode;

/// Turns the `VirtualDom`'s mutation stream into a real tree of [`RealNode`]s, mirrored 1:1 into
/// a `yoga` layout tree. See the trait docs on `WriteMutations` for the exact stack/path
/// semantics this implements against.
pub struct TerminalRenderer {
    pub(super) arena: Arena,
    id_to_nodekey: BiHashMap<ElementId, NodeKey>,
    screen: Screen,
    stack: Vec<NodeKey>,
    pub(super) root: NodeKey,
    dirty: bool,
    layouts: Rc<RefCell<HashMap<NodeKey, LayoutRect>>>,
    /// Newly attached `onmounted` listeners, not yet fired. Drained (and never refilled for the
    /// same element) so `onmounted` fires exactly once per element.
    pending_mounted: Vec<ElementId>,
    /// Which scope last called `measure_element` for which node — populated from component code
    /// via the thread-local mirror in `mounted`, not by anything in this file. Lets `paint`'s
    /// caller re-render exactly the right scope directly (`Runtime::needs_update`) once that
    /// node's layout changes, with no `onresize` listener/event involved.
    resize_scopes: Rc<RefCell<HashMap<NodeKey, ScopeId>>>,
    /// Scopes subscribed via `use_key_event` — populated from component code via the
    /// thread-local mirror in `keys`, not by anything in this file. Key events aren't hit-tested
    /// to an element, so this is the only way anything gets notified of them.
    key_listeners: KeyListeners,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        let mut arena = Arena::default();
        let root = arena.insert(RealNode::block(YogaNode::new()));
        let mut id_to_nodekey = BiHashMap::new();
        id_to_nodekey.insert(ElementId(0), root);
        let layouts = Rc::new(RefCell::new(HashMap::new()));
        let resize_scopes = Rc::new(RefCell::new(HashMap::new()));
        let key_listeners: KeyListeners = Rc::new(RefCell::new(HashMap::new()));
        mounted::set_layouts(layouts.clone());
        mounted::set_resize_scopes(resize_scopes.clone());
        keys::set_key_listeners(key_listeners.clone());

        Self {
            arena,
            id_to_nodekey,
            screen: Screen::default(),
            stack: Vec::new(),
            root,
            dirty: true,
            layouts,
            pending_mounted: Vec::new(),
            resize_scopes,
            key_listeners,
        }
    }

    /// Whether anything changed since the last call. Used to skip redundant repaints.
    pub(super) fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    /// Renders the current tree and writes only the terminal cells that changed since the last
    /// call, using `screen` to remember what was actually painted last time. Returns every node
    /// whose position or size actually differs from what it was before this call.
    pub(super) fn paint(
        &mut self,
        out: &mut impl Write,
        width: u32,
        height: u32,
    ) -> io::Result<Vec<(NodeKey, LayoutRect)>> {
        paint::paint(
            &mut self.screen,
            &mut self.arena,
            self.root,
            out,
            Point::new(width as i32, height as i32),
            &self.layouts,
        )
    }

    /// Drains the elements that registered an `onmounted` listener since the last call, pairing
    /// each with its [`NodeId`] so the caller can fire the event into the `VirtualDom`.
    pub(super) fn take_pending_mounted(&mut self) -> Vec<(ElementId, NodeId)> {
        std::mem::take(&mut self.pending_mounted)
            .into_iter()
            .filter_map(|id| {
                let key = *self.id_to_nodekey.get_by_left(&id)?;
                Some((id, NodeId(key)))
            })
            .collect()
    }

    /// Of the nodes a `paint` reported as changed, the scopes that last measured them via
    /// `measure_element` — i.e. the scopes that need to be re-rendered to see the new layout.
    pub(super) fn scopes_for(&self, changed: &[(NodeKey, LayoutRect)]) -> Vec<ScopeId> {
        let resize_scopes = self.resize_scopes.borrow();
        changed
            .iter()
            .filter_map(|(key, _)| resize_scopes.get(key).copied())
            .collect()
    }

    /// Every currently-subscribed `use_key_event` handler, snapshotted so the caller can run them
    /// without holding a borrow of the registry (a handler could itself mount/unmount a
    /// subscriber and re-enter it).
    pub(super) fn key_listeners(&self) -> Vec<Rc<dyn Fn(KeyEvent)>> {
        self.key_listeners.borrow().values().cloned().collect()
    }

    /// Finds the topmost `Block` under the given terminal cell, and the `ElementId` mouse
    /// events on it should be dispatched to.
    pub(super) fn hit_test(&self, column: u16, row: u16) -> Option<ElementId> {
        let key = paint::hit_test(&self.arena, self.root, column, row)?;
        self.id_to_nodekey.get_by_right(&key).copied()
    }

    fn pop_n(&mut self, m: usize) -> Vec<NodeKey> {
        let start = self.stack.len() - m;
        self.stack.split_off(start)
    }

    /// `path` is relative to whatever is currently on top of the stack.
    fn resolve_path(&self, path: &[u8]) -> NodeKey {
        let mut current = *self
            .stack
            .last()
            .expect("no node on stack to resolve a path against");
        for &index in path {
            current = self.arena.get(current).children()[index as usize];
        }
        current
    }

    fn build_template_node(&mut self, node: &TemplateNode) -> NodeKey {
        match node {
            TemplateNode::Element { tag, children, .. } => {
                let key = self.arena.insert(match *tag {
                    "block" => RealNode::block(YogaNode::new()),
                    "text" => RealNode::text(YogaNode::new()),
                    _ => RealNode::placeholder(YogaNode::new()),
                });
                for (i, child) in children.iter().enumerate() {
                    let child_key = self.build_template_node(child);
                    self.arena.attach(key, child_key, i);
                }
                key
            }
            TemplateNode::Text { text } => {
                self.arena.insert(RealNode::raw_text((*text).to_string()))
            }
            TemplateNode::Dynamic { .. } => {
                self.arena.insert(RealNode::placeholder(YogaNode::new()))
            }
        }
    }

    /// Detach `target`, drop its whole subtree, and splice `new_nodes` in at its old position.
    fn splice(&mut self, target: NodeKey, new_nodes: Vec<NodeKey>) {
        match self.arena.get(target).parent() {
            Some(parent) => {
                let index = self
                    .arena
                    .get(parent)
                    .children()
                    .iter()
                    .position(|k| *k == target)
                    .expect("target not among its parent's children");
                self.arena.remove_subtree(target);
                for (offset, node) in new_nodes.into_iter().enumerate() {
                    self.arena.attach(parent, node, index + offset);
                }
            }
            None => self.arena.remove_subtree(target),
        }
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteMutations for TerminalRenderer {
    fn append_children(&mut self, id: ElementId, m: usize) {
        self.dirty = true;
        let parent = *self
            .id_to_nodekey
            .get_by_left(&id)
            .expect("parent not found");
        let nodes = self.pop_n(m);
        let start_index = self.arena.get(parent).children().len();
        for (offset, node) in nodes.into_iter().enumerate() {
            self.arena.attach(parent, node, start_index + offset);
        }
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        let key = self.resolve_path(path);
        self.id_to_nodekey.insert(id, key);
    }

    fn create_placeholder(&mut self, id: ElementId) {
        self.dirty = true;
        let key = self.arena.insert(RealNode::placeholder(YogaNode::new()));
        self.id_to_nodekey.insert(id, key);
        self.stack.push(key);
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        self.dirty = true;
        let key = self.arena.insert(RealNode::raw_text(value.to_string()));
        self.id_to_nodekey.insert(id, key);
        self.stack.push(key);
    }

    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        self.dirty = true;
        let key = self.build_template_node(&template.roots[index]);
        self.id_to_nodekey.insert(id, key);
        self.stack.push(key);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        self.dirty = true;
        let nodes = self.pop_n(m);
        let target = self
            .id_to_nodekey
            .remove_by_left(&id)
            .expect("target not found")
            .1;
        self.splice(target, nodes);
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        self.dirty = true;
        let nodes = self.pop_n(m);
        let target = self.resolve_path(path);
        self.splice(target, nodes);
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        self.dirty = true;
        let nodes = self.pop_n(m);
        let anchor = *self
            .id_to_nodekey
            .get_by_left(&id)
            .expect("anchor not found");
        let parent = self
            .arena
            .get(anchor)
            .parent()
            .expect("insert_nodes_after on a node with no parent");
        let index = self
            .arena
            .get(parent)
            .children()
            .iter()
            .position(|k| *k == anchor)
            .unwrap()
            + 1;
        for (offset, node) in nodes.into_iter().enumerate() {
            self.arena.attach(parent, node, index + offset);
        }
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        self.dirty = true;
        let nodes = self.pop_n(m);
        let anchor = *self
            .id_to_nodekey
            .get_by_left(&id)
            .expect("anchor not found");
        let parent = self
            .arena
            .get(anchor)
            .parent()
            .expect("insert_nodes_before on a node with no parent");
        let index = self
            .arena
            .get(parent)
            .children()
            .iter()
            .position(|k| *k == anchor)
            .unwrap();
        for (offset, node) in nodes.into_iter().enumerate() {
            self.arena.attach(parent, node, index + offset);
        }
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        self.dirty = true;
        let Some(&key) = self.id_to_nodekey.get_by_left(&id) else {
            return;
        };
        match self.arena.get_mut(key) {
            RealNode::Block {
                yoga,
                border,
                bg_color,
                overflow,
                ..
            } => apply_block_attribute(yoga, border, bg_color, overflow, name, value),
            RealNode::Text { style, wrap, .. } => apply_text_attribute(style, wrap, name, value),
            RealNode::RawText { .. } | RealNode::Placeholder { .. } => unreachable!(),
        }
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        self.dirty = true;
        let Some(&key) = self.id_to_nodekey.get_by_left(&id) else {
            return;
        };
        if let RealNode::RawText { content, .. } = self.arena.get_mut(key) {
            *content = value.to_string();
        }
    }

    // Other events aren't wired up to the terminal input loop yet.
    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        if name == "mounted" {
            self.pending_mounted.push(id);
        }
    }
    fn remove_event_listener(&mut self, _name: &'static str, _id: ElementId) {}

    fn remove_node(&mut self, id: ElementId) {
        self.dirty = true;
        if let Some((_, key)) = self.id_to_nodekey.remove_by_left(&id) {
            self.resize_scopes.borrow_mut().remove(&key);
            self.arena.remove_subtree(key);
        }
    }

    fn push_root(&mut self, id: ElementId) {
        let key = *self.id_to_nodekey.get_by_left(&id).expect("key not found");
        self.stack.push(key);
    }
}
