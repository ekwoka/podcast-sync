use api_types::itunes::*;

#[tauri::command]
pub async fn search_itunes(query: String) -> Result<ItunesResponse, String> {
    let url = format!(
        "https://itunes.apple.com/search?term={}&media=podcast&entity=podcast&limit=2",
        query
    );
    let response = tauri_plugin_http::reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}
