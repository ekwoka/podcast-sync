use api_types::{podcast::*, subscriptions::*};

#[tauri::command]
pub async fn load_podcast_feed(podcast: Subscription) -> Result<Podcast, String> {
    dbg!(&podcast);
    let response = tauri_plugin_http::reqwest::get(&podcast.feed_url)
        .await
        .map_err(|e| e.to_string())
        .inspect_err(|err: &String| {
            dbg!(err);
        })?;
    let feed = response.text().await.map_err(|e| e.to_string())?;
    let feed: Feed = parse_feed(&feed)
        .map_err(|e| e.to_string())
        .inspect_err(|err| {
            dbg!(&err);
        })?;
    Ok(feed.podcast)
}
