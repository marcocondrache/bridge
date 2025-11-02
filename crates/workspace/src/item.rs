use gpui::{AnyView, App, Entity, EntityId, Focusable, Render, SharedString};

pub trait Item: Focusable + Render + Sized {
    fn tab_title(&self, cx: &App) -> SharedString;
}

pub trait ItemHandle {
    fn to_any(&self) -> AnyView;
    fn tab_title(&self, cx: &App) -> SharedString;
    fn item_id(&self) -> EntityId;
}

impl<T: Item> ItemHandle for Entity<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn tab_title(&self, cx: &App) -> SharedString {
        self.read(cx).tab_title(cx)
    }

    fn item_id(&self) -> EntityId {
        self.entity_id()
    }
}
