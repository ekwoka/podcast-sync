use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItunesResponse {
    pub result_count: i32,
    pub results: Vec<ItunesResult>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItunesResult {
    pub wrapper_type: String,
    pub kind: String,
    pub collection_id: i32,
    pub track_id: i32,
    pub artist_name: Option<String>,
    pub collection_name: Option<String>,
    pub track_name: Option<String>,
    pub feed_url: Option<String>,
    pub collection_censored_name: Option<String>,
    pub track_censored_name: Option<String>,
    pub artist_view_url: Option<Option<String>>,
    pub collection_view_url: Option<String>,
    pub track_view_url: Option<String>,
    pub artwork_url30: Option<String>,
    pub artwork_url60: Option<String>,
    pub artwork_url100: Option<String>,
    pub collection_price: f32,
    pub track_price: f32,
    pub release_date: Option<String>,
    pub collection_explicitness: Option<String>,
    pub track_explicitness: Option<String>,
    pub track_count: i32,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub primary_genre_name: Option<String>,
    pub content_advisory_rating: Option<String>,
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
