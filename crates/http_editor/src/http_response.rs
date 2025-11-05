use gpui::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div,
    prelude::FluentBuilder,
};
use gpui_component::{
    h_flex,
    input::{InputState, TextInput},
    label::Label,
    tab::TabBar,
    v_flex,
};
use http_client::StatusCode;

pub struct HttpResponse {
    body: Entity<InputState>,
    status_code: Option<StatusCode>,
    metrics: Option<http_client::Metrics>,
}

impl HttpResponse {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let body = cx.new(|cx| InputState::new(window, cx).code_editor("").multi_line());
        let this = cx.new(|_cx| Self {
            body,
            status_code: None,
            metrics: None,
        });

        this
    }

    pub fn set_status_code(&mut self, status_code: Option<StatusCode>) {
        self.status_code = status_code;
    }

    pub fn set_body_content(&mut self, content: String, window: &mut Window, cx: &mut App) {
        self.body
            .update(cx, |state, cx| state.set_value(content, window, cx))
    }

    pub fn set_metrics(&mut self, metrics: Option<http_client::Metrics>) {
        self.metrics = metrics;
    }
}

impl Render for HttpResponse {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(TabBar::new(""))
                    .child(h_flex().when_some(self.status_code.clone(), |this, code| {
                        this.child(Label::new(code.to_string()))
                    })),
            )
            .child(TextInput::new(&self.body).h_full())
    }
}
