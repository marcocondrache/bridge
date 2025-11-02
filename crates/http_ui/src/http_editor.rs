use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, ParentElement,
    Render, Styled, Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
};
use workspace::item::Item;

use crate::{http_method_selector::HttpMethodSelector, http_target::HttpTarget};

pub struct HttpEditor {
    focus_handle: FocusHandle,
    target_uri: Entity<HttpTarget>,
    method_selector: Entity<HttpMethodSelector>,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let method_selector = cx.new(|_cx| HttpMethodSelector::new());
        let target_uri = cx.new(|cx| HttpTarget::new(window, cx));

        Self {
            target_uri,
            method_selector,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for HttpEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for HttpEditor {
    fn tab_title(&self, cx: &App) -> gpui::SharedString {
        "HTTP Editor".into()
    }
}

impl Render for HttpEditor {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .key_context("HttpEditor")
            .v_flex()
            .size_full()
            .gap_4()
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.method_selector.clone())
                    .child(self.target_uri.clone())
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
            .track_focus(&self.focus_handle)
    }
}
