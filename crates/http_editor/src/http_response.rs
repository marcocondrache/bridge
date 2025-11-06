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
    table::Table,
    v_flex,
};
use http_client::{HeaderMap, StatusCode};

use crate::http_headers::HttpHeaders;

pub struct HttpResponse {
    headers: Entity<Table<HttpHeaders>>,
    body: Entity<InputState>,
    status_code: Option<StatusCode>,
    size: Option<u64>,
    metrics: Option<http_client::Metrics>,
    current_tab: usize,
}

impl HttpResponse {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let body = cx.new(|cx| InputState::new(window, cx).code_editor("").multi_line());
        let headers = cx.new(|cx| Table::new(HttpHeaders::new(), window, cx));
        let this = cx.new(|_cx| Self {
            body,
            headers,
            status_code: None,
            size: None,
            metrics: None,
            current_tab: 0,
        });

        this
    }

    pub fn set_headers(&mut self, headers: HeaderMap, cx: &mut App) {
        self.headers
            .update(cx, |table, _cx| table.delegate_mut().set_headers(headers));
    }

    pub fn set_status_code(&mut self, status_code: Option<StatusCode>) {
        self.status_code = status_code;
    }

    pub fn set_size(&mut self, size: Option<u64>) {
        self.size = size;
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
            1 => self.headers.clone().into_any_element(),
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
                            .child(Tab::new("Headers"))
                            .child(Tab::new("Cookies")),
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
                            })
                            .when_some(self.size.clone(), |this, size| {
                                this.child(Label::new(format!("{} bytes", size)))
                            }),
                    ),
            )
            .child(self.render_tab(cx))
    }
}
