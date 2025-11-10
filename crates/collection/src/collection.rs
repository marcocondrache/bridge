use gpui::{ParentElement, Pixels, Render, Styled, div, px};
use gpui_component::IconName;
use ui::utils::placement::Placement;
use workspace::dock::Panel;

pub struct Collection {}

impl Collection {
    pub fn new() -> Self {
        Self {}
    }
}

impl Panel for Collection {
    fn priority(&self) -> u32 {
        0
    }

    fn placement(&self, window: &gpui::Window, cx: &gpui::App) -> Placement {
        Placement::Left
    }

    fn icon(&self, window: &gpui::Window, cx: &gpui::App) -> Option<gpui_component::IconName> {
        Some(IconName::Folder)
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
