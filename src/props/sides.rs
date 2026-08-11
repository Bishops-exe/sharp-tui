use yoga::Edge;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Sides<T> {
    pub(crate) left: T,
    pub(crate) right: T,
    pub(crate) top: T,
    pub(crate) bottom: T,
}

impl<T: Copy> Sides<T> {
    pub fn new(left: T, right: T, top: T, bottom: T) -> Sides<T> {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn top(mut self, top: T) -> Self {
        self.top = top;
        self
    }
    pub fn bottom(mut self, bottom: T) -> Self {
        self.bottom = bottom;
        self
    }
    pub fn left(mut self, left: T) -> Self {
        self.left = left;
        self
    }
    pub fn right(mut self, right: T) -> Self {
        self.right = right;
        self
    }

    pub fn all(all: T) -> Sides<T> {
        Self::new(all, all, all, all)
    }

    pub fn get_edge(&self, edge: Edge) -> Option<T> {
        match edge {
            Edge::Left => Some(self.left),
            Edge::Right => Some(self.right),
            Edge::Top => Some(self.top),
            Edge::Bottom => Some(self.bottom),
            _ => None,
        }
    }

    pub fn get_pair(&self, edge: Edge) -> Option<(Edge, T)> {
        let value = self.get_edge(edge)?;

        Some((edge, value))
    }

    pub fn iterate(&self) -> [(Edge, T); 4] {
        [
            self.get_pair(Edge::Left).unwrap(),
            self.get_pair(Edge::Right).unwrap(),
            self.get_pair(Edge::Top).unwrap(),
            self.get_pair(Edge::Bottom).unwrap(),
        ]
    }
    
    pub fn rotate_90(&self) -> Sides<T> {
        Self {
            left: self.bottom,
            top: self.left,
            right: self.top,
            bottom: self.right
        }
    }

    pub fn rotate_180(&self) -> Sides<T> {
        self.rotate_90().rotate_90()
    }
    
    pub fn rotate_270(&self) -> Sides<T> {
        self.rotate_180().rotate_90()
    }
    
    
    
}
