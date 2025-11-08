use gpui::{
    AppContext, Context, Entity, ParentElement, Render, SharedString, Styled, WeakEntity, Window,
    div,
};
use gpui_component::{
    ActiveTheme, IndexPath, h_flex,
    label::Label,
    select::{Select, SelectDelegate, SelectItem, SelectState},
    v_flex,
};

use crate::HttpEditor;

#[derive(Debug, Default, Clone)]
pub enum AuthorizationKind {
    #[default]
    None,
    Basic,
    Bearer,
}

impl From<AuthorizationKind> for SharedString {
    fn from(kind: AuthorizationKind) -> Self {
        match kind {
            AuthorizationKind::None => "None".into(),
            AuthorizationKind::Basic => "Basic".into(),
            AuthorizationKind::Bearer => "Bearer".into(),
        }
    }
}

impl AuthorizationKind {
    pub fn all() -> [Self; 3] {
        [
            AuthorizationKind::None,
            AuthorizationKind::Basic,
            AuthorizationKind::Bearer,
        ]
    }
}

impl SelectItem for AuthorizationKind {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        self.clone().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

pub struct AuthorizationDelegator {
    items: [AuthorizationKind; 3],
}

impl AuthorizationDelegator {
    pub fn new() -> Self {
        Self {
            items: AuthorizationKind::all(),
        }
    }
}

impl SelectDelegate for AuthorizationDelegator {
    type Item = AuthorizationKind;

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

pub struct AuthorizationSelector {
    state: Entity<SelectState<AuthorizationDelegator>>,
}

impl AuthorizationSelector {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| {
            SelectState::new(
                AuthorizationDelegator::new(),
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        Self { state }
    }
}

impl Render for AuthorizationSelector {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        h_flex()
            .justify_between()
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .p_2()
            .child(
                h_flex()
                    .gap_4()
                    .child(Label::new("Authorization type"))
                    .child(Select::new(&self.state)),
            )
    }
}

pub struct AuthorizationTab {
    selector: Entity<AuthorizationSelector>,
    editor: WeakEntity<HttpEditor>,
}

impl AuthorizationTab {
    pub fn new(
        editor: WeakEntity<HttpEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selector = cx.new(|cx| AuthorizationSelector::new(window, cx));

        Self { selector, editor }
    }
}

impl Render for AuthorizationTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex().child(self.selector.clone())
    }
}
