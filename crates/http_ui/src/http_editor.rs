use gpui::{App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
};

use crate::{http_method_selector::HttpMethodSelector, http_target::HttpTarget};

pub struct HttpEditor {
    target: Entity<HttpTarget>,
    method_selector: Entity<HttpMethodSelector>,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let method_selector = cx.new(|cx| HttpMethodSelector::new());
        let target = cx.new(|cx| HttpTarget::new(window, cx));

        Self {
            method_selector,
            target,
        }
    }
}

impl Render for HttpEditor {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .v_flex()
            .gap_4()
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.method_selector.clone())
                    .child(self.target.clone())
                    .child(Button::new("Send").label("Send").primary()),
            )
            .child(
                TabBar::new("Tabs")
                    .selected_index(0)
                    .child(Tab::new("Parameters"))
                    .child(Tab::new("Body"))
                    .child(Tab::new("Headers"))
                    .child(Tab::new("Authorization")),
            )
    }
}
