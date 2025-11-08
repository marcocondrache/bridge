use gpui::{Context, SharedString, Window, actions};
use gpui_component::{
    IndexPath,
    select::{SelectDelegate, SelectItem, SelectState},
};

actions!(
    http_method_selector,
    [Get, Post, Put, Delete, Patch, Options]
);

pub type MethodSelector = SelectState<MethodDelegate>;

pub fn method_selector(window: &mut Window, cx: &mut Context<MethodSelector>) -> MethodSelector {
    SelectState::new(
        MethodDelegate::new(),
        Some(IndexPath::default()),
        window,
        cx,
    )
}

pub(crate) struct MethodDelegate {
    items: [Method; 6],
}

impl MethodDelegate {
    pub fn new() -> Self {
        Self {
            items: Method::all(),
        }
    }
}

impl SelectDelegate for MethodDelegate {
    type Item = Method;

    fn items_count(&self, _section: usize) -> usize {
        self.items.len()
    }

    fn item(&self, ix: gpui_component::IndexPath) -> Option<&Self::Item> {
        self.items.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<gpui_component::IndexPath>
    where
        Self::Item: SelectItem<Value = V>,
        V: PartialEq,
    {
        self.items
            .iter()
            .position(|item| item.value() == value)
            .map(|row| gpui_component::IndexPath::new(row))
    }
}

#[derive(Clone)]
pub struct Method(http::Method);

impl From<Method> for SharedString {
    fn from(value: Method) -> Self {
        value.0.to_string().into()
    }
}

impl Method {
    pub fn all() -> [Self; 6] {
        [
            Method(http::Method::GET),
            Method(http::Method::POST),
            Method(http::Method::PUT),
            Method(http::Method::DELETE),
            Method(http::Method::PATCH),
            Method(http::Method::OPTIONS),
        ]
    }
}

impl SelectItem for Method {
    type Value = http::Method;

    fn title(&self) -> SharedString {
        self.clone().into()
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}
