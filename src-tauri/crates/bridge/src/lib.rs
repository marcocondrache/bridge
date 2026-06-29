use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Serialize)]
pub struct HttpResponse {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
    elapsed_ms: u128,
}

#[tauri::command]
async fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let method = reqwest::Method::from_bytes(request.method.as_bytes())
        .map_err(|e| e.to_string())?;

    let client = reqwest::Client::new();
    let mut builder = client.request(method, &request.url);
    for (k, v) in request.headers {
        if !k.is_empty() {
            builder = builder.header(k, v);
        }
    }
    if let Some(body) = request.body {
        builder = builder.body(body);
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
        headers,
        body,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
