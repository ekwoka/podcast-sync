use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItunesResponse {
    result_count: i32,
    results: Vec<ItunesResult>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItunesResult {
    wrapper_type: String,
    kind: String,
    collection_id: i32,
    track_id: i32,
    artist_name: Option<String>,
    collection_name: Option<String>,
    track_name: Option<String>,
    feed_url: Option<String>,
    collection_censored_name: Option<String>,
    track_censored_name: Option<String>,
    artist_view_url: Option<Option<String>>,
    collection_view_url: Option<String>,
    track_view_url: Option<String>,
    artwork_url30: Option<String>,
    artwork_url60: Option<String>,
    artwork_url100: Option<String>,
    collection_price: f32,
    track_price: f32,
    release_date: Option<String>,
    collection_explicitness: Option<String>,
    track_explicitness: Option<String>,
    track_count: i32,
    country: Option<String>,
    currency: Option<String>,
    primary_genre_name: Option<String>,
    content_advisory_rating: Option<String>,
}

pub fn parse_itunes_response(json: &str) -> Result<ItunesResponse, serde_json::Error> {
    serde_json::from_str(json)
}

#[test]
fn test_parse_itunes_response() {
    let json = include_str!("../../test_data/itunes.json");
    let itunes_response = parse_itunes_response(json).expect("Test data is validated");
    assert_eq!(itunes_response.result_count, 60);
    assert_eq!(itunes_response.results.len(), 60);
    let itunes_result = &itunes_response.results[0];
    assert_eq!(itunes_result.wrapper_type, "track");
    assert_eq!(itunes_result.kind, "podcast");
    assert_eq!(
        itunes_result.track_name,
        Some("Regulation Podcast".to_owned())
    );
}
