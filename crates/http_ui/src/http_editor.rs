use gpui::{Render, Styled, Window, div, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            url: String::new(),
            headers: Vec::new(),
            body: String::new(),
        }
    }
}

pub struct HttpEditor {
    request: HttpRequest,
}

impl HttpEditor {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            request: HttpRequest::default(),
        }
    }

    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    pub fn set_method(&mut self, method: HttpMethod, cx: &mut Context<Self>) {
        self.request.method = method;
        cx.notify();
    }

    pub fn set_url(&mut self, url: String, cx: &mut Context<Self>) {
        self.request.url = url;
        cx.notify();
    }

    pub fn add_header(&mut self, key: String, value: String, cx: &mut Context<Self>) {
        self.request.headers.push(HttpHeader {
            key,
            value,
            enabled: true,
        });
        cx.notify();
    }

    pub fn remove_header(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.request.headers.len() {
            self.request.headers.remove(index);
            cx.notify();
        }
    }

    pub fn set_body(&mut self, body: String, cx: &mut Context<Self>) {
        self.request.body = body;
        cx.notify();
    }

    fn render_method_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(div().child(self.request.method.as_str().to_string()))
    }

    fn render_url_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .child(div().child(self.request.url.clone()))
    }

    fn render_headers(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().child("Headers"))
            .children(self.request.headers.iter().enumerate().map(|(i, header)| {
                div()
                    .flex()
                    .gap_2()
                    .child(div().child(header.key.clone()))
                    .child(div().child(header.value.clone()))
            }))
    }

    fn render_body(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().child("Body"))
            .child(div().child(self.request.body.clone()))
    }
}

impl Render for HttpEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .p_4()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.render_method_selector(cx))
                    .child(self.render_url_input(cx)),
            )
            .child(self.render_headers(cx))
            .child(self.render_body(cx))
    }
}
