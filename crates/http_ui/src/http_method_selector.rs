use gpui::http_client::{Method, http};
use gpui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Window, actions, div};
use gpui_component::Sizable;
use gpui_component::button::Button;
use gpui_component::popup_menu::PopupMenuExt;

actions!(
    http_method_selector,
    [Get, Post, Put, Delete, Patch, Options]
);

pub struct HttpMethodSelector {
    method: http::Method,
}

impl HttpMethodSelector {
    pub fn new() -> Self {
        Self {
            method: http::Method::GET,
        }
    }

    fn get(&mut self, _: &Get, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::GET;
        cx.notify();
    }

    fn post(&mut self, _: &Post, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::POST;
        cx.notify();
    }

    fn put(&mut self, _: &Put, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::PUT;
        cx.notify();
    }

    fn delete(&mut self, _: &Delete, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::DELETE;
        cx.notify();
    }

    fn patch(&mut self, _: &Patch, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::PATCH;
        cx.notify();
    }

    fn options(&mut self, _: &Options, _window: &mut Window, cx: &mut Context<Self>) {
        self.method = Method::OPTIONS;
        cx.notify();
    }
}

impl Render for HttpMethodSelector {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(cx.listener(Self::get))
            .on_action(cx.listener(Self::post))
            .on_action(cx.listener(Self::put))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::patch))
            .on_action(cx.listener(Self::options))
            .child(
                Button::new("Method Selector")
                    .label(self.method.to_string())
                    .large()
                    .popup_menu(|menu, _window, _cx| {
                        menu.menu("GET", Box::new(Get))
                            .menu("POST", Box::new(Post))
                            .menu("PUT", Box::new(Put))
                            .menu("DELETE", Box::new(Delete))
                            .menu("PATCH", Box::new(Patch))
                            .menu("OPTIONS", Box::new(Options))
                    }),
            )
    }
}
