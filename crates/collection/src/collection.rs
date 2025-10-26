use gpui::{Render, div};
use workspace::dock::Panel;

pub struct Collection {}

impl Panel for Collection {
    fn priority(&self) -> u32 {
        0
    }

    fn placement(&self) -> ui::placement::Placement {
        ui::placement::Placement::Left
    }
}

impl Render for Collection {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
    }
}
