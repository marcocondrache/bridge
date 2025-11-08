mod authorization_tab;
mod authorization_type_selector;
mod body_tab;
mod headers;
mod method_selector;
mod query_table;
mod response_panel;

use anyhow::Result;
use gpui::{
    AnyElement, App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, Task, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, StyledExt,
    button::{Button, ButtonVariants},
    divider::Divider,
    h_flex,
    input::{Input, InputState},
    select::Select,
    tab::{Tab, TabBar},
    table::{Table, TableState},
};
use http::Request;
use http_client::{AsyncReadResponseExt, HttpClient, config::Configurable};
use workspace::{AppState, NewHttpEditor, Workspace, area::Item};

use crate::{
    authorization_tab::AuthorizationTab,
    body_tab::BodyTab,
    headers::HeadersTableDelegate,
    method_selector::{MethodSelector, method_selector},
    query_table::QueryTableDelegate,
    response_panel::ResponsePanel,
};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        // workspace.register_action(|workspace, _: &NewHttpEditor, window, cx| {
        //     println!("Action called");

        //     HttpEditor::new_in_workspace(workspace, window, cx);
        // });

        HttpEditor::new_in_workspace(workspace, window.unwrap(), cx);
    })
    .detach();

    cx.on_action(|&NewHttpEditor, cx| {
        if let Some(app_state) = AppState::global(cx).upgrade() {
            workspace::open_new(app_state, cx);
        }
    });
}

#[derive(Clone, Copy, Default)]
#[repr(usize)]
pub enum HttpEditorTab {
    #[default]
    Query,
    Headers,
    Body,
    Authorization,
}

impl HttpEditorTab {
    pub fn all() -> [Self; 4] {
        [
            HttpEditorTab::Query,
            HttpEditorTab::Headers,
            HttpEditorTab::Body,
            HttpEditorTab::Authorization,
        ]
    }
}

impl From<HttpEditorTab> for usize {
    fn from(value: HttpEditorTab) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for HttpEditorTab {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HttpEditorTab::Query),
            1 => Ok(HttpEditorTab::Headers),
            2 => Ok(HttpEditorTab::Body),
            3 => Ok(HttpEditorTab::Authorization),
            _ => Err(()),
        }
    }
}

impl From<HttpEditorTab> for Tab {
    fn from(value: HttpEditorTab) -> Self {
        match value {
            HttpEditorTab::Body => Tab::new("Body"),
            HttpEditorTab::Headers => Tab::new("Headers"),
            HttpEditorTab::Query => Tab::new("Query"),
            HttpEditorTab::Authorization => Tab::new("Authorization"),
        }
    }
}

pub struct HttpEditor {
    body_tab: Entity<BodyTab>,
    url_input: Entity<InputState>,
    method_selector: Entity<MethodSelector>,
    response_viewer: Option<Entity<ResponsePanel>>,
    executing_task: Option<Task<Result<()>>>,
    query_table: Entity<TableState<QueryTableDelegate>>,
    headers_table: Entity<TableState<HeadersTableDelegate>>,
    authorization_tab: Entity<AuthorizationTab>,
    current_tab: HttpEditorTab,
    focus_handle: FocusHandle,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let this = cx.entity().downgrade();

        let method_selector = cx.new(|cx| method_selector(window, cx));

        let body = cx.new(|cx| BodyTab::new(window, cx));
        let target_uri = cx.new(|cx| InputState::new(window, cx).placeholder("Enter URL"));
        let authorization_tab = cx.new(|cx| AuthorizationTab::new(this, window, cx));
        let query_table = cx.new(|cx| TableState::new(QueryTableDelegate::new(), window, cx));
        let headers_table =
            cx.new(|cx| TableState::new(HeadersTableDelegate::new_editable(), window, cx));

        Self {
            response_viewer: None,
            url_input: target_uri,
            method_selector,
            body_tab: body,
            query_table,
            headers_table,
            authorization_tab,
            focus_handle: cx.focus_handle(),
            executing_task: None,
            current_tab: HttpEditorTab::default(),
        }
    }

    pub fn new_in_workspace(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let item = Box::new(cx.new(|cx| Self::new(window, cx)));

        workspace.add_item(item, window, cx);
    }

    pub fn is_executing(&self) -> bool {
        self.executing_task.is_some()
    }

    fn activate_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.current_tab = HttpEditorTab::try_from(index).unwrap_or_default();

        cx.notify();
    }

    fn handle_request(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.executing_task.is_none() {
            let _ = self.send_request(window, cx);
        } else {
            self.cancel_request(cx);
        }
    }

    fn build_request<T>(&self, cx: &mut Context<Self>, body: T) -> Result<Request<T>> {
        let method = self.method_selector.read(cx).selected_value().unwrap();
        let uri = self
            .url_input
            .read_with(cx, |this, _cx| this.value())
            .to_string();

        Request::builder()
            .method(method)
            .uri(uri)
            .automatic_decompression(true)
            .redirect_policy(http_client::config::RedirectPolicy::Follow)
            .metrics(true)
            .body(body)
            .map_err(|e| e.into())
    }

    fn cancel_request(&mut self, cx: &mut Context<Self>) {
        if let Some(_) = self.executing_task.take() {
            cx.notify();
        }
    }

    fn send_request(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Result<()> {
        let client = HttpClient::global(cx);
        let request = self.build_request(cx, ())?;

        self.executing_task = Some(cx.spawn_in(window, async move |this, cx| {
            let (response, body) = cx
                .update(|_window, cx| {
                    cx.background_spawn(async move {
                        let mut response = client.send(request).await?;
                        let body = response.text().await?;

                        Ok::<_, anyhow::Error>((response, body))
                    })
                })?
                .await?;

            this.update_in(cx, |this, window, cx| {
                if let Some(viewer) = &this.response_viewer {
                    viewer.update(cx, |viewer, cx| {
                        viewer.update_response(body, response, window, cx);
                    })
                } else {
                    this.response_viewer =
                        Some(cx.new(|cx| ResponsePanel::new(body, response, window, cx)));
                }

                this.executing_task = None;
            })?;

            Ok(())
        }));

        cx.notify();

        Ok(())
    }

    fn render_query_tab(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        Table::new(&self.query_table)
    }

    fn render_headers_tab(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        Table::new(&self.headers_table)
    }

    fn render_body_tab(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        self.body_tab.clone()
    }

    fn render_authorization_tab(
        &self,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        self.authorization_tab.clone()
    }

    fn render_request_section(&self, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .border_1()
            .border_color(cx.theme().input)
            .rounded(cx.theme().radius)
            .w_full()
            .gap_1()
            .child(
                div().w(px(140.)).child(
                    Select::new(&self.method_selector)
                        .flex_1()
                        .appearance(false)
                        .py_2()
                        .pl_3(),
                ),
            )
            .child(Divider::vertical())
            .child(
                div()
                    .flex_1()
                    .child(Input::new(&self.url_input).appearance(false).pr_3().py_2()),
            )
            .child(
                Button::new("execute")
                    .ml_2()
                    .when_else(
                        self.is_executing(),
                        |this| this.label("Cancel"),
                        |this| this.label("Send").primary(),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.handle_request(window, cx);
                    })),
            )
    }
}

impl Focusable for HttpEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for HttpEditor {
    fn tab_title(&self, _cx: &App) -> gpui::SharedString {
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
            .p_4()
            .gap_4()
            .child(self.render_request_section(cx))
            .child(
                TabBar::new("Tabs")
                    .underline()
                    .selected_index(self.current_tab.into())
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.activate_tab(*index, cx);
                    }))
                    .children(HttpEditorTab::all()),
            )
            .map(|parent| match &self.current_tab {
                HttpEditorTab::Query => parent.child(self.render_query_tab(window, cx)),
                HttpEditorTab::Headers => parent.child(self.render_headers_tab(window, cx)),
                HttpEditorTab::Body => parent.child(self.render_body_tab(window, cx)),
                HttpEditorTab::Authorization => {
                    parent.child(self.render_authorization_tab(window, cx))
                }
            })
            .when_some(self.response_viewer.as_ref(), |parent, viewer| {
                parent.child(viewer.clone())
            })
            .track_focus(&self.focus_handle)
    }
}
