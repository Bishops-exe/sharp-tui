use crate::props::{Border, Color, Overflow, TextWrap};
use crossterm::style::ContentStyle;
use yoga::Node as YogaNode;

pub(super) type NodeKey = usize;

pub(super) enum RealNode {
    Block {
        yoga: YogaNode,
        parent: Option<NodeKey>,
        children: Vec<NodeKey>,
        border: Border,
        bg_color: Color,
        /// The effective (merged) overflow, mirrored from the yoga node so `paint` can decide
        /// whether this block should clip its children without re-deriving it from attributes.
        overflow: Overflow,
    },
    Text {
        yoga: YogaNode,
        parent: Option<NodeKey>,
        children: Vec<NodeKey>,
        style: ContentStyle,
        wrap: TextWrap,
    },
    /// Bare text content (e.g. the literal `"hi"` in `Text { "hi" }`). Has no yoga node of its
    /// own; its content is folded into the nearest ancestor `Text` node at paint time.
    RawText {
        parent: Option<NodeKey>,
        content: String,
    },
    /// An empty dynamic slot (e.g. a `{children}` fragment that rendered nothing).
    Placeholder {
        yoga: YogaNode,
        parent: Option<NodeKey>,
        children: Vec<NodeKey>,
    },
}

impl RealNode {
    pub(super) fn block(yoga: YogaNode) -> Self {
        RealNode::Block {
            yoga,
            parent: None,
            children: Vec::new(),
            border: Border::default(),
            bg_color: Color::default(),
            overflow: Overflow::default(),
        }
    }

    pub(super) fn text(yoga: YogaNode) -> Self {
        RealNode::Text {
            yoga,
            parent: None,
            children: Vec::new(),
            style: ContentStyle::default(),
            wrap: TextWrap::default(),
        }
    }

    pub(super) fn placeholder(yoga: YogaNode) -> Self {
        RealNode::Placeholder {
            yoga,
            parent: None,
            children: Vec::new(),
        }
    }

    pub(super) fn raw_text(content: String) -> Self {
        RealNode::RawText {
            parent: None,
            content,
        }
    }

    pub(super) fn parent(&self) -> Option<NodeKey> {
        match self {
            RealNode::Block { parent, .. }
            | RealNode::Text { parent, .. }
            | RealNode::RawText { parent, .. }
            | RealNode::Placeholder { parent, .. } => *parent,
        }
    }

    pub(super) fn set_parent(&mut self, new_parent: Option<NodeKey>) {
        match self {
            RealNode::Block { parent, .. }
            | RealNode::Text { parent, .. }
            | RealNode::RawText { parent, .. }
            | RealNode::Placeholder { parent, .. } => *parent = new_parent,
        }
    }

    pub(super) fn children(&self) -> &[NodeKey] {
        match self {
            RealNode::Block { children, .. }
            | RealNode::Text { children, .. }
            | RealNode::Placeholder { children, .. } => children,
            RealNode::RawText { .. } => &[],
        }
    }

    pub(super) fn children_mut(&mut self) -> Option<&mut Vec<NodeKey>> {
        match self {
            RealNode::Block { children, .. }
            | RealNode::Text { children, .. }
            | RealNode::Placeholder { children, .. } => Some(children),
            RealNode::RawText { .. } => None,
        }
    }

    pub(super) fn yoga(&self) -> Option<&YogaNode> {
        match self {
            RealNode::Block { yoga, .. }
            | RealNode::Text { yoga, .. }
            | RealNode::Placeholder { yoga, .. } => Some(yoga),
            RealNode::RawText { .. } => None,
        }
    }

    pub(super) fn yoga_mut(&mut self) -> Option<&mut YogaNode> {
        match self {
            RealNode::Block { yoga, .. }
            | RealNode::Text { yoga, .. }
            | RealNode::Placeholder { yoga, .. } => Some(yoga),
            RealNode::RawText { .. } => None,
        }
    }
}

#[derive(Default)]
pub(super) struct Arena {
    slots: Vec<Option<RealNode>>,
    free: Vec<NodeKey>,
}

impl Arena {
    pub(super) fn insert(&mut self, node: RealNode) -> NodeKey {
        if let Some(key) = self.free.pop() {
            self.slots[key] = Some(node);
            key
        } else {
            self.slots.push(Some(node));
            self.slots.len() - 1
        }
    }

    /// Detach `key` from its parent (if any) and drop it, along with every descendant.
    pub(super) fn remove_subtree(&mut self, key: NodeKey) {
        self.detach(key);
        self.remove_subtree_inner(key);
    }

    fn remove_subtree_inner(&mut self, key: NodeKey) {
        let node = self.slots[key].take().expect("dangling node key");
        for child in node.children() {
            self.remove_subtree_inner(*child);
        }

        self.free.push(key);
    }

    pub(super) fn get(&self, key: NodeKey) -> &RealNode {
        self.slots[key].as_ref().expect("dangling node key")
    }

    pub(super) fn get_mut(&mut self, key: NodeKey) -> &mut RealNode {
        self.slots[key].as_mut().expect("dangling node key")
    }

    fn take(&mut self, key: NodeKey) -> RealNode {
        self.slots[key].take().expect("dangling node key")
    }

    fn put_back(&mut self, key: NodeKey, node: RealNode) {
        self.slots[key] = Some(node);
    }

    /// Detach `child_key` from its current parent, in both our tree and the yoga tree. No-op if
    /// it has no parent.
    pub(super) fn detach(&mut self, child_key: NodeKey) {
        let Some(parent_key) = self.get(child_key).parent() else {
            return;
        };
        let mut child = self.take(child_key);

        // A `Text` node's children (bare text or nested `Text` spans) are folded into its own
        // content at paint time rather than laid out independently, so they're never wired into
        // the yoga tree in the first place — see the matching check in `attach` below.
        let parent_is_text = matches!(self.get(parent_key), RealNode::Text { .. });

        {
            let parent = self.get_mut(parent_key);
            if let Some(children) = parent.children_mut() {
                children.retain(|k| *k != child_key);
            }
            if !parent_is_text
                && let (Some(parent_yoga), Some(child_yoga)) = (parent.yoga_mut(), child.yoga_mut())
            {
                parent_yoga.remove_child(child_yoga);
            }
        }

        child.set_parent(None);
        self.put_back(child_key, child);
    }

    /// Attach `child_key` as the `index`-th child of `parent_key`, wiring the yoga tree so that
    /// yoga-less siblings (bare text) don't throw off the layout child order. If `parent_key` is
    /// itself a `Text` node, `child_key` is kept out of the yoga tree entirely: `Text` children
    /// (bare text or nested `Text` spans, used for multi-style content) are inline content
    /// folded into their ancestor's own paint, not independent layout boxes.
    pub(super) fn attach(&mut self, parent_key: NodeKey, child_key: NodeKey, index: usize) {
        self.detach(child_key);
        let mut child = self.take(child_key);
        child.set_parent(Some(parent_key));

        let parent_is_text = matches!(self.get(parent_key), RealNode::Text { .. });

        let yoga_index = if parent_is_text {
            0
        } else {
            let parent = self.get(parent_key);
            parent
                .children()
                .iter()
                .take(index)
                .filter(|k| self.get(**k).yoga().is_some())
                .count()
        };

        {
            let parent = self.get_mut(parent_key);
            if let Some(children) = parent.children_mut() {
                let at = index.min(children.len());
                children.insert(at, child_key);
            }
            if !parent_is_text
                && let (Some(parent_yoga), Some(child_yoga)) = (parent.yoga_mut(), child.yoga_mut())
            {
                parent_yoga.insert_child(child_yoga, yoga_index);
            }
        }

        self.put_back(child_key, child);
    }
}
