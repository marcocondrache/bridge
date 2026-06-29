use bridge_history::{History, HistoryEntry, HistoryQuery};
use bridge_http::{HttpRequest, HttpResponse};
use tauri::Manager;

#[tauri::command]
async fn send_request(
    request: HttpRequest,
    history: tauri::State<'_, History>,
) -> Result<HttpResponse, String> {
    let response = bridge_http::execute(&request).await?;
    // Every request is saved automatically (README contract).
    history
        .insert(&request, &response)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}

#[tauri::command]
async fn query_history(
    query: HistoryQuery,
    history: tauri::State<'_, History>,
) -> Result<Vec<HistoryEntry>, String> {
    history.query(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_history_entry(
    id: i64,
    history: tauri::State<'_, History>,
) -> Result<Option<HistoryEntry>, String> {
    history.get(id).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let history = tauri::async_runtime::block_on(History::open(&dir.join("history.db")))?;
            app.manage(history);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_request,
            query_history,
            get_history_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
