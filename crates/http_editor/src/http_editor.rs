mod http_method_selector;
mod http_response;
mod http_target;

use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, ParentElement,
    Render, Styled, Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
};
use http_client::{HttpClient, Request};
use workspace::{AppState, NewHttpEditor, Workspace, item::Item};

use crate::{
    http_method_selector::HttpMethodSelector, http_response::HttpResponse, http_target::HttpTarget,
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

pub struct HttpEditor {
    focus_handle: FocusHandle,
    target_uri: Entity<HttpTarget>,
    method_selector: Entity<HttpMethodSelector>,
    response: Entity<HttpResponse>,
}

impl HttpEditor {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let method_selector = cx.new(|_cx| HttpMethodSelector::new());
        let target_uri = cx.new(|cx| HttpTarget::new(window, cx));
        let response = cx.new(|cx| HttpResponse::new(window, cx));

        Self {
            target_uri,
            method_selector,
            response,
            focus_handle: cx.focus_handle(),
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

    fn send(&self, cx: &mut Context<Self>) {
        let client = HttpClient::global(cx);
        let method = self.method_selector.read(cx).method();
        let uri = self.target_uri.read_with(cx, |this, cx| this.url(cx));

        cx.background_spawn(async move {
            let request = Request::builder().method(method).uri(uri).body(()).unwrap();
            let response = client.send(request).await;

            println!("{:?}", response);
        })
        .detach();
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
                    .child(
                        Button::new("Send")
                            .label("Send")
                            .primary()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.send(cx);
                            })),
                    ),
            )
            .child(
                TabBar::new("Tabs")
                    .selected_index(0)
                    .child(Tab::new("Parameters"))
                    .child(Tab::new("Body"))
                    .child(Tab::new("Headers"))
                    .child(Tab::new("Authorization")),
            )
            .child(self.response.clone())
            .track_focus(&self.focus_handle)
    }
}
