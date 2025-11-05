use gpui::{
    AnyElement, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled,
    Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    badge::Badge,
    h_flex,
    input::{InputState, TextInput},
    label::Label,
    tab::{Tab, TabBar},
    v_flex,
};
use http_client::StatusCode;

pub struct HttpResponse {
    body: Entity<InputState>,
    status_code: Option<StatusCode>,
    metrics: Option<http_client::Metrics>,
    current_tab: usize,
}

impl HttpResponse {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let body = cx.new(|cx| InputState::new(window, cx).code_editor("").multi_line());
        let this = cx.new(|_cx| Self {
            body,
            status_code: None,
            metrics: None,
            current_tab: 0,
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

    pub fn activate_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    fn render_tab(&self, _cx: &mut Context<Self>) -> AnyElement {
        match self.current_tab {
            0 => TextInput::new(&self.body)
                .h_full()
                .disabled(true)
                .focus_bordered(false)
                .into_any_element(),
            _ => div().into_any_element(),
        }
    }
}

impl Render for HttpResponse {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .gap_2()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        TabBar::new("")
                            .segmented()
                            .selected_index(self.current_tab)
                            .on_click(cx.listener(|view, index, _, cx| {
                                view.activate_tab(*index);
                                cx.notify();
                            }))
                            .child(Tab::new("Body"))
                            .child(Tab::new("Cookies"))
                            .child(Tab::new("Headers")),
                    )
                    .child(
                        h_flex()
                            .p_2()
                            .gap_2()
                            .when_some(self.status_code.clone(), |this, code| {
                                this.child(Badge::new().child(code.to_string()))
                            })
                            .when_some(self.metrics.clone(), |this, metrics| {
                                let total_time = metrics.total_time();

                                this.child(Label::new(
                                    humantime::format_duration(total_time).to_string(),
                                ))
                            }),
                    ),
            )
            .child(self.render_tab(cx))
    }
}
