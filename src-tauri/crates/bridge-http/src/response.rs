//! Types describing an HTTP response produced by the executor.

use crate::{HeaderList, HttpVersion};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub head: ResponseHead,
    pub body: ResponseBody,
    pub timing: Timing,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redirects: Vec<RedirectHop>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsInfo>,
    pub final_url: Url,
    pub http_version: HttpVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHead {
    pub status: u16,
    pub status_text: String,
    pub headers: HeaderList,
}

impl ResponseHead {
    pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.status)
    }
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status)
    }
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .get("content-type")
            .map(|v| v.split(';').next().unwrap_or("").trim())
    }

    pub fn content_length(&self) -> Option<u64> {
        self.headers.get("content-length")?.parse().ok()
    }

    pub fn is_chunked(&self) -> bool {
        self.headers
            .get("transfer-encoding")
            .is_some_and(|v| v.to_ascii_lowercase().contains("chunked"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum ResponseBody {
    Buffered(#[serde(with = "serde_bytes")] Vec<u8>),
    FileBacked {
        path: PathBuf,
        size: u64,
    },
    Truncated {
        #[serde(with = "serde_bytes")]
        prefix: Vec<u8>,
        total_received: u64,
        limit: u64,
    },
    Streaming(StreamHandle),
    None,
}

impl ResponseBody {
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            ResponseBody::Buffered(b) => Some(b.as_slice()),
            ResponseBody::Truncated { prefix, .. } => Some(prefix.as_slice()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.as_bytes().map(String::from_utf8_lossy)
    }

    pub fn len(&self) -> Option<u64> {
        match self {
            ResponseBody::Buffered(b) => Some(b.len() as u64),
            ResponseBody::FileBacked { size, .. } => Some(*size),
            ResponseBody::Truncated { total_received, .. } => Some(*total_received),
            ResponseBody::None => Some(0),
            ResponseBody::Streaming(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, ResponseBody::None) || self.len() == Some(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamHandle {
    pub id: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    pub started_at: SystemTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttfb_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_ms: Option<u64>,
    pub total_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectHop {
    pub url: Url,
    pub status: u16,
    pub status_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Url>,
    pub headers: HeaderList,
    pub timing: Timing,
    pub http_version: HttpVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsInfo {
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_cert_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_cert_issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_cert_valid_from: Option<SystemTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_cert_expiry: Option<SystemTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peer_cert_sans: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_cert_sha256: Option<String>,
}
