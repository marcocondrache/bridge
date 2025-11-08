use gpui::{ParentElement, Render, Styled, div, px};
use workspace::dock::Panel;

pub struct Collection {}

impl Panel for Collection {
    fn priority(&self) -> u32 {
        0
    }
}

impl Render for Collection {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().h_full().w(px(200.0)).child("test")
    }
}
