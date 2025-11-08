use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    h_flex,
    input::{Input, InputState},
    label::Label,
    tab::{Tab, TabBar},
    table::{Table, TableState},
    tag::Tag,
    v_flex,
};
use http::Response;
use http_client::{AsyncBody, ResponseExt};

use crate::headers::HeadersTableDelegate;

#[derive(Debug, Default, Clone, PartialEq)]
#[repr(usize)]
enum ResponseTab {
    #[default]
    Body,
    Headers,
}

impl ResponseTab {
    fn all() -> [Self; 2] {
        [ResponseTab::Body, ResponseTab::Headers]
    }
}

impl From<ResponseTab> for usize {
    fn from(value: ResponseTab) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for ResponseTab {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResponseTab::Body),
            1 => Ok(ResponseTab::Headers),
            _ => Err(()),
        }
    }
}

impl From<ResponseTab> for Tab {
    fn from(value: ResponseTab) -> Self {
        match value {
            ResponseTab::Body => Tab::new("Body"),
            ResponseTab::Headers => Tab::new("Headers"),
        }
    }
}

pub struct ResponsePanel {
    headers_table: Entity<TableState<HeadersTableDelegate>>,
    body: Entity<InputState>,
    selected_tab: ResponseTab,
    focus_handle: FocusHandle,
    response: Response<AsyncBody>,
}

impl Focusable for ResponsePanel {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl ResponsePanel {
    pub fn new(
        body: String,
        response: Response<AsyncBody>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let headers = cx.new(|cx| TableState::new(HeadersTableDelegate::new(), window, cx));
        let body = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("html")
                .indent_guides(false)
                .default_value(body)
        });

        Self {
            body,
            headers_table: headers,
            selected_tab: ResponseTab::default(),
            focus_handle,
            response,
        }
    }

    pub fn update_response(
        &mut self,
        body: String,
        response: Response<AsyncBody>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let headers = response.headers().clone();

        self.response = response;
        self.body.update(cx, |editor, cx| {
            editor.set_value(body, window, cx);
        });

        // self.headers_table.update(cx, |table, _cx| {
        //     table.delegate_mut().set_headers(headers);
        // });

        cx.notify();
    }

    fn activate_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_tab = ResponseTab::try_from(index).unwrap_or_default();

        cx.notify();
    }

    fn render_status_tag(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let code = self.response.status();

        if code.is_success() {
            Tag::success().child(code.to_string())
        } else if code.is_server_error() | code.is_client_error() {
            Tag::danger().child(code.to_string())
        } else {
            Tag::primary().child(code.to_string())
        }
    }

    fn render_status_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(self.render_status_tag(cx))
            .when_some(self.response.metrics(), |this, metrics| {
                let total_time = metrics.total_time();

                this.child(Label::new(
                    humantime::format_duration(total_time).to_string(),
                ))
            })
            .when_some(self.response.body().len(), |this, size| {
                this.child(Label::new(format!("{} bytes", size)))
            })
    }

    fn render_body_tab(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Input::new(&self.body)
            .h_full()
            .disabled(true)
            .focus_bordered(false)
            .into_any_element()
    }

    fn render_headers_tab(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Table::new(&self.headers_table)
    }
}

impl Render for ResponsePanel {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .gap_2()
            .child(self.render_status_bar(window, cx))
            .child(
                TabBar::new("response_tabs")
                    .w_full()
                    .underline()
                    .selected_index(self.selected_tab.clone().into())
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.activate_tab(*index, cx);
                    }))
                    .children(ResponseTab::all()),
            )
            .map(|parent| match self.selected_tab {
                ResponseTab::Body => parent.child(self.render_body_tab(window, cx)),
                ResponseTab::Headers => parent.child(self.render_headers_tab(window, cx)),
            })
    }
}
