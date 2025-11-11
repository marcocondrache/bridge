use gpui::{AnyView, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{ActiveTheme, v_flex};

pub trait NavigationItemView: Render {}

trait NavigationItemViewHandle: Send {
    fn to_any(&self) -> AnyView;
}

pub struct NavigationBar {
    items: Vec<Box<dyn NavigationItemViewHandle>>,
}

impl NavigationBar {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item<I>(&mut self, item: Entity<I>, window: &mut Window, cx: &mut Context<Self>)
    where
        I: NavigationItemView + 'static,
    {
        self.items.push(Box::new(item));
        cx.notify();
    }

    fn render_items(&self) -> impl IntoElement {
        v_flex()
            .gap_1()
            .overflow_y_hidden()
            .children(self.items.iter().map(|item| item.to_any()))
    }
}

impl Render for NavigationBar {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .py_6()
            .px_6()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(self.render_items())
    }
}

impl<T: NavigationItemView> NavigationItemViewHandle for Entity<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }
}
