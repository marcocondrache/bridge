use gpui::{Context, Window};
use gpui_component::{
    IndexPath,
    select::{SelectDelegate, SelectItem, SelectState},
};

use crate::body_tab::BodyType;

pub type BodyTypeSelector = SelectState<BodyTypeDelegate>;

pub fn body_type_selector(
    window: &mut Window,
    cx: &mut Context<BodyTypeSelector>,
) -> BodyTypeSelector {
    SelectState::new(
        BodyTypeDelegate::new(),
        Some(IndexPath::default()),
        window,
        cx,
    )
}

pub struct BodyTypeDelegate {
    items: [BodyType; 4],
}

impl BodyTypeDelegate {
    pub fn new() -> Self {
        Self {
            items: BodyType::all(),
        }
    }
}

impl SelectDelegate for BodyTypeDelegate {
    type Item = BodyType;

    fn items_count(&self, _section: usize) -> usize {
        self.items.len()
    }

    fn item(&self, ix: gpui_component::IndexPath) -> Option<&Self::Item> {
        self.items.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<gpui_component::IndexPath>
    where
        Self::Item: gpui_component::select::SelectItem<Value = V>,
        V: PartialEq,
    {
        self.items
            .iter()
            .position(|item| item.value() == value)
            .map(|row| gpui_component::IndexPath::new(row))
    }
}
