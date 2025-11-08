use gpui::{Context, Window};
use gpui_component::{
    IndexPath,
    select::{SelectDelegate, SelectItem, SelectState},
};

use crate::authorization_tab::AuthorizationType;

pub type AuthorizationTypeSelector = SelectState<AuthorizationTypeDelegate>;

pub fn authorization_type_selector(
    window: &mut Window,
    cx: &mut Context<AuthorizationTypeSelector>,
) -> AuthorizationTypeSelector {
    SelectState::new(
        AuthorizationTypeDelegate::new(),
        Some(IndexPath::default()),
        window,
        cx,
    )
}

pub struct AuthorizationTypeDelegate {
    items: [AuthorizationType; 3],
}

impl AuthorizationTypeDelegate {
    pub fn new() -> Self {
        Self {
            items: AuthorizationType::all(),
        }
    }
}

impl SelectDelegate for AuthorizationTypeDelegate {
    type Item = AuthorizationType;

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
