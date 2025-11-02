use crate::config::{
    Configurable,
    request::{RequestConfig, WithRequestConfig},
};
use http::Request;

/// Extension methods on an HTTP request.
pub trait RequestExt<T> {
    /// Create a new request builder with the method, URI, and headers cloned
    /// from this request.
    ///
    /// Note that third-party extensions are not cloned.
    fn to_builder(&self) -> http::request::Builder;
}

impl<T> RequestExt<T> for Request<T> {
    fn to_builder(&self) -> http::request::Builder {
        let mut builder = Request::builder()
            .method(self.method().clone())
            .uri(self.uri().clone())
            .version(self.version());

        *builder.headers_mut().unwrap() = self.headers().clone();

        if let Some(config) = self.extensions().get::<RequestConfig>() {
            builder = builder.extension(config.clone());
        }

        if let Some(cookie_jar) = self.extensions().get::<crate::cookies::CookieJar>() {
            builder = builder.extension(cookie_jar.clone());
        }

        builder
    }
}

impl Configurable for http::request::Builder {
    fn cookie_jar(self, cookie_jar: crate::cookies::CookieJar) -> Self {
        self.extension(cookie_jar)
    }
}

impl WithRequestConfig for http::request::Builder {
    #[inline]
    fn with_config(mut self, f: impl FnOnce(&mut RequestConfig)) -> Self {
        if let Some(extensions) = self.extensions_mut() {
            if let Some(config) = extensions.get_mut() {
                f(config);
            } else {
                extensions.insert(RequestConfig::default());
                f(extensions.get_mut().unwrap());
            }
        }

        self
    }
}
