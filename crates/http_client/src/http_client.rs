#[macro_use]
mod macros;

pub mod cookies;

mod agent;
mod body;
mod client;
mod default_headers;
mod handler;
mod headers;
mod info;
mod metrics;
mod parsing;
mod redirect;
mod request;
mod response;
mod task;
mod text;
mod trailer;

pub mod auth;
pub mod config;
pub mod error;

pub use http::*;

pub mod interceptor;

use gpui::{App, Global};

pub use crate::{
    body::{AsyncBody, Body},
    client::{HttpClient, HttpClientBuilder, ResponseFuture},
    error::Error,
    info::*,
    metrics::Metrics,
    request::RequestExt,
    response::{AsyncReadResponseExt, ReadResponseExt, ResponseExt},
    trailer::Trailer,
};

/// A "prelude" for importing commonly used HTTP client types and traits.
///
/// The prelude re-exports most commonly used traits and macros from this crate.
///
/// # Example
///
/// Import the prelude with:
///
/// ```
/// use http::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        AsyncReadResponseExt, ReadResponseExt, RequestExt, ResponseExt, config::Configurable,
    };
}

pub fn init(cx: &mut App) {
    let client = HttpClientBuilder::default().build().unwrap();

    cx.set_global(GlobalHttpClient(client));
}

pub struct GlobalHttpClient(HttpClient);

impl Global for GlobalHttpClient {}

impl HttpClient {
    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalHttpClient>().0.clone()
    }
}
