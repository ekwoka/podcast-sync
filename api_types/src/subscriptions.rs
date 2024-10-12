use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Subscriptions {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: i32,
    pub title: String,
    pub feed_url: String,
    pub image_url: Option<String>,
    pub last_updated: DateTime<Utc>,
}

impl Subscriptions {
    pub fn to_string(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::new().struct_names(true).to_owned(),
        )
    }
    pub fn to_writer(&self, writer: &mut impl std::io::Write) -> Result<(), ron::Error> {
        ron::ser::to_writer_pretty(
            writer,
            self,
            ron::ser::PrettyConfig::new().struct_names(true).to_owned(),
        )
    }
}

#[test]
fn can_parse_subscriptions() {
    let json = r#"{"subscriptions":[{"id": 0, "title":"Regulation Podcast","feedUrl":"https://feed.com","lastUpdated":"2014-11-28T12:45:59.324310806Z"}]}"#;
    let subs: Subscriptions = serde_json::from_str(json).expect("Test data is validated");
    assert_eq!(
        subs,
        Subscriptions {
            subscriptions: vec![Subscription {
                id: 0,
                title: "Regulation Podcast".to_owned(),
                feed_url: "https://feed.com".to_owned(),
                image_url: None,
                last_updated: DateTime::parse_from_rfc3339("2014-11-28T12:45:59.324310806Z")
                    .unwrap()
                    .with_timezone(&Utc),
            }]
        }
    )
}
