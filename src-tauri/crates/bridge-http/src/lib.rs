use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub elapsed_ms: u64,
}

pub fn status_text(status: u16) -> String {
    reqwest::StatusCode::from_u16(status)
        .ok()
        .and_then(|s| s.canonical_reason())
        .unwrap_or_default()
        .to_string()
}

pub async fn execute(request: &HttpRequest) -> Result<HttpResponse, String> {
    let method =
        reqwest::Method::from_bytes(request.method.as_bytes()).map_err(|e| e.to_string())?;

    let client = reqwest::Client::new();
    let mut builder = client.request(method, &request.url);
    for (k, v) in &request.headers {
        if !k.is_empty() {
            builder = builder.header(k, v);
        }
    }
    if let Some(body) = &request.body {
        builder = builder.body(body.clone());
    }

    let start = std::time::Instant::now();
    let response = builder.send().await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(HttpResponse {
        status,
        status_text: status_text(status),
        headers,
        body,
        elapsed_ms: start.elapsed().as_millis() as u64,
    })
}
