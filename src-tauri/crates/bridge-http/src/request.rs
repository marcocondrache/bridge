use crate::{HeaderList, Method};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub url: Url,

    #[serde(default)]
    pub headers: HeaderList,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,

    #[serde(default)]
    pub options: RequestOptions,
}

impl Request {
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: HeaderList::new(),
            body: None,
            options: RequestOptions::default(),
        }
    }

    pub fn get(url: Url) -> Self {
        Self::new(Method::Get, url)
    }
    pub fn post(url: Url) -> Self {
        Self::new(Method::Post, url)
    }
    pub fn put(url: Url) -> Self {
        Self::new(Method::Put, url)
    }
    pub fn delete(url: Url) -> Self {
        Self::new(Method::Delete, url)
    }
    pub fn patch(url: Url) -> Self {
        Self::new(Method::Patch, url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum Body {
    Bytes(#[serde(with = "serde_bytes")] Vec<u8>),
    Text(String),
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
    Multipart(Vec<FormPart>),
    Stream(StreamBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamBody {
    pub source: StreamSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum StreamSource {
    File(PathBuf),
    Handle { handle_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPart {
    pub name: String,
    pub value: FormPartValue,
    #[serde(default, skip_serializing_if = "HeaderList::is_empty")]
    pub headers: HeaderList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum FormPartValue {
    Text(String),
    Bytes {
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },
    File {
        path: PathBuf,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOptions {
    #[serde(default)]
    pub timeout: Option<Duration>,
    #[serde(default)]
    pub connect_timeout: Option<Duration>,
    pub follow_redirects: RedirectPolicy,
    pub verify_tls: bool,
    pub accept_invalid_certs: bool,
    pub accept_invalid_hostnames: bool,
    pub http_version: HttpVersionPref,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    pub use_cookie_jar: bool,
    pub decompress_response: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_buffer: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
            connect_timeout: Some(Duration::from_secs(10)),
            follow_redirects: RedirectPolicy::Limited(10),
            verify_tls: true,
            accept_invalid_certs: false,
            accept_invalid_hostnames: false,
            http_version: HttpVersionPref::Auto,
            proxy: None,
            auth: None,
            use_cookie_jar: true,
            decompress_response: true,
            max_response_buffer: None,
            user_agent: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "max", rename_all = "snake_case")]
pub enum RedirectPolicy {
    None,
    Limited(u8),
    Unlimited,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HttpVersionPref {
    Auto,
    Http10Only,
    Http11Only,
    Http2Only,
    Http3Only,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub url: Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<ProxyAuth>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub no_proxy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Auth {
    Basic { username: String, password: String },
    Bearer { token: String },
    Digest { username: String, password: String },
}
