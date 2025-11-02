use gpui::{
    App, AppContext, InteractiveElement, ParentElement, Render, Styled, div,
    prelude::FluentBuilder, px,
};
use gpui_component::ActiveTheme;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let item = cx.new(|cx| TitleBar::new(cx));

        workspace.set_titlebar_item(item.into(), window, cx);
    })
    .detach();
}

pub struct TitleBar {}

impl TitleBar {
    pub fn new(cx: &mut App) -> Self {
        Self {}
    }
}

impl Render for TitleBar {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();

        div()
            .id("titlebar")
            .flex()
            .w_full()
            .text_sm()
            .min_h(px(37.0))
            .max_h(px(37.0))
            .bg(theme.background)
            .window_control_area(gpui::WindowControlArea::Drag)
            .when(cfg!(target_os = "macos"), |this| {
                this.child(div().w(px(72.0)))
            })
    }
}
