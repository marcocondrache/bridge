use gpui::{SharedString, actions};
use gpui_component::select::{SelectDelegate, SelectItem};

actions!(
    http_method_selector,
    [Get, Post, Put, Delete, Patch, Options]
);

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

pub struct MethodDelegator {
    items: [Method; 6],
}

impl MethodDelegator {
    pub fn new() -> Self {
        Self {
            items: Method::all(),
        }
    }
}

impl SelectDelegate for MethodDelegator {
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
