use gpui::http_client::{Method, http};
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Window, actions, div,
};
use gpui_component::Sizable;
use gpui_component::button::Button;
use gpui_component::popup_menu::PopupMenuExt;

actions!(
    http_method_selector,
    [Get, Post, Put, Delete, Patch, Options]
);

pub struct MethodSelector {
    method: http::Method,
}

impl MethodSelector {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let this = cx.new(|_| Self {
            method: http::Method::GET,
        });

        this
    }

    pub fn method(&self) -> http::Method {
        self.method.clone()
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

impl Render for MethodSelector {
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
