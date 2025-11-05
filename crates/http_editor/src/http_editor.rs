mod http_method_selector;
mod http_response;

use anyhow::{Ok, Result};
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, ParentElement,
    Render, Styled, Task, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
    tab::{Tab, TabBar},
};
use http_client::{
    AsyncBody, AsyncReadResponseExt, HttpClient, Request, Response, ResponseExt,
    config::Configurable,
};
use workspace::{AppState, NewHttpEditor, Workspace, item::Item};

use crate::{http_method_selector::HttpMethodSelector, http_response::HttpResponse};

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

pub struct HttpEditor {
    focus_handle: FocusHandle,
    target_uri: Entity<InputState>,
    method_selector: Entity<HttpMethodSelector>,
    response: Entity<HttpResponse>,
    executing_task: Option<Task<Result<()>>>,
    current_tab: usize,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let response = HttpResponse::new(window, cx);
        let method_selector = HttpMethodSelector::new(cx);
        let target_uri = cx.new(|cx| InputState::new(window, cx));

        Self {
            response,
            target_uri,
            method_selector,
            focus_handle: cx.focus_handle(),
            executing_task: None,
            current_tab: 0,
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

    fn activate_tab(&mut self, index: usize) {
        self.current_tab = index;
    }

    fn build_request<T>(&self, cx: &mut Context<Self>, body: T) -> Result<Request<T>> {
        let method = self.method_selector.read(cx).method();
        let uri = self
            .target_uri
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
        let output = self.response.clone();

        self.executing_task = Some(cx.spawn_in(window, async move |this, cx| {
            let mut response = cx
                .update(|_window, cx| {
                    cx.background_spawn(async move { client.send(request).await })
                })?
                .await?;

            let body = response.text().await?;

            output.update_in(cx, |state, window, cx| {
                state.set_body_content(body, window, cx);
                state.set_status_code(Some(response.status()));
                state.set_metrics(response.metrics().cloned());
            })?;

            this.update(cx, |state, _cx| {
                state.executing_task = None;
            })?;

            Ok(())
        }));

        cx.notify();

        Ok(())
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
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .gap_4()
                    .p_4()
                    .child(self.method_selector.clone())
                    .child(TextInput::new(&self.target_uri))
                    .child(
                        Button::new("execute")
                            .map(|this| {
                                if self.executing_task.is_none() {
                                    this.label("Send").primary()
                                } else {
                                    this.label("Cancel")
                                }
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                if this.executing_task.is_none() {
                                    let _ = this.send_request(window, cx);
                                } else {
                                    this.cancel_request(cx);
                                }
                            })),
                    ),
            )
            .child(
                TabBar::new("Tabs")
                    .segmented()
                    .selected_index(self.current_tab)
                    .on_click(cx.listener(|view, index, _, cx| {
                        view.activate_tab(*index);
                        cx.notify();
                    }))
                    .child(Tab::new("Parameters"))
                    .child(Tab::new("Body"))
                    .child(Tab::new("Headers"))
                    .child(Tab::new("Authorization")),
            )
            .child(self.response.clone())
            .track_focus(&self.focus_handle)
    }
}
