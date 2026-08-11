#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AlignItem {
    FlexStart,
    Center,
    FlexEnd,
    #[default]
    Stretch,
    Baseline,
}

impl AlignItem {
    pub(crate) fn to_yoga(self) -> yoga::Align {
        match self {
            AlignItem::FlexStart => yoga::Align::FlexStart,
            AlignItem::Center => yoga::Align::Center,
            AlignItem::FlexEnd => yoga::Align::FlexEnd,
            AlignItem::Stretch => yoga::Align::Stretch,
            AlignItem::Baseline => yoga::Align::Baseline,
        }
    }
}