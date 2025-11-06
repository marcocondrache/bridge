mod http_headers;
mod method_selector;
mod response_viewer;

use anyhow::Result;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Task, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
    tab::{Tab, TabBar},
};
use http_client::{AsyncReadResponseExt, HttpClient, Request, config::Configurable};
use workspace::{AppState, NewHttpEditor, Workspace, item::Item};

use crate::{method_selector::MethodSelector, response_viewer::ResponseViewer};

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
    Parameters,
    Headers,
    Body,
    Authorization,
}

impl HttpEditorTab {
    pub fn all() -> [Self; 4] {
        [
            HttpEditorTab::Parameters,
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
            0 => Ok(HttpEditorTab::Parameters),
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
            HttpEditorTab::Parameters => Tab::new("Parameters"),
            HttpEditorTab::Authorization => Tab::new("Authorization"),
        }
    }
}

pub struct HttpEditor {
    url_input: Entity<InputState>,
    method_selector: Entity<MethodSelector>,
    response_viewer: Option<Entity<ResponseViewer>>,
    executing_task: Option<Task<Result<()>>>,
    active_tab: HttpEditorTab,
    focus_handle: FocusHandle,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let method_selector = MethodSelector::new(cx);
        let target_uri = cx.new(|cx| InputState::new(window, cx));

        Self {
            response_viewer: None,
            url_input: target_uri,
            method_selector,
            focus_handle: cx.focus_handle(),
            executing_task: None,
            active_tab: HttpEditorTab::default(),
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

    fn render_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Tab")
    }

    fn activate_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_tab = HttpEditorTab::try_from(index).unwrap_or_default();

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
        let method = self.method_selector.read(cx).method();
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
                    let viewer = cx.new(|cx| ResponseViewer::new(body, response, window, cx));

                    this.response_viewer = Some(viewer);
                }

                this.executing_task = None;
            })?;

            Ok(())
        }));

        cx.notify();

        Ok(())
    }

    fn render_request_section(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .w_full()
            .gap_4()
            .child(self.method_selector.clone())
            .child(TextInput::new(&self.url_input))
            .child(
                Button::new("execute")
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
        _window: &mut gpui::Window,
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
                    .segmented()
                    .selected_index(self.active_tab.into())
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.activate_tab(*index, cx);
                    }))
                    .children(HttpEditorTab::all()),
            )
            .child(self.render_tab(cx))
            .when_some(self.response_viewer.as_ref(), |this, viewer| {
                this.child(viewer.clone())
            })
            .track_focus(&self.focus_handle)
    }
}
