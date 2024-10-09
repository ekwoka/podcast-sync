use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Feed {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@xmlns:itunes")]
    pub xmlns_itunes: String,
    #[serde(rename = "@xmlns:googleplay")]
    pub xmlns_googleplay: String,
    #[serde(rename = "@xmlns:atom")]
    pub xmlns_atom: String,
    #[serde(rename = "@xmlns:media")]
    pub xmlns_media: String,
    #[serde(rename = "@xmlns:content")]
    pub xmlns_content: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "channel")]
    pub podcast: Podcast,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Podcast {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "atom-link")]
    pub atom_link: AtomLink,
    pub title: String,
    pub link: String,
    pub language: String,
    pub copyright: String,
    pub description: String,
    pub image: Image,
    #[serde(rename = "itunes-explicit")]
    pub itunes_explicit: String,
    #[serde(rename = "itunes-type")]
    pub itunes_type: String,
    #[serde(rename = "itunes-subtitle")]
    pub itunes_subtitle: String,
    #[serde(rename = "itunes-author")]
    pub itunes_author: String,
    #[serde(rename = "itunes-summary")]
    pub itunes_summary: String,
    #[serde(rename = "encoded")]
    pub content_encoded: String,
    #[serde(rename = "itunes-owner")]
    pub itunes_owner: ItunesOwner,
    /*     #[serde(rename = "image")]
    pub itunes_image: ChannelItunesImage, */
    #[serde(rename = "itunes-category")]
    pub itunes_category: ItunesCategory,
    #[serde(rename = "item")]
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AtomLink {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@rel")]
    pub rel: String,
    #[serde(rename = "@type")]
    pub atom_link_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Image {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub url: String,
    pub title: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ItunesOwner {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "itunes-name")]
    pub itunes_name: String,
    #[serde(rename = "itunes-email")]
    pub itunes_email: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChannelItunesImage {
    #[serde(rename = "@href")]
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ItunesCategory {
    #[serde(rename = "@text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Episode {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "itunes-image")]
    pub itunes_image: Option<ItemItunesImage>,
    #[serde(rename = "itunes-episode")]
    pub itunes_episode: Option<String>,
    #[serde(rename = "itunes-season")]
    pub itunes_season: Option<String>,
    pub title: String,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    #[serde(rename = "itunes-title")]
    pub itunes_title: String,
    #[serde(rename = "itunes-episodeType")]
    pub itunes_episode_type: String,
    #[serde(rename = "itunes-author")]
    pub itunes_author: String,
    #[serde(rename = "itunes-subtitle")]
    pub itunes_subtitle: String,
    #[serde(rename = "itunes-summary")]
    pub itunes_summary: String,
    #[serde(rename = "encoded")]
    pub content_encoded: String,
    #[serde(rename = "itunes-duration")]
    pub itunes_duration: String,
    pub guid: Guid,
    pub enclosure: Enclosure,
    #[serde(rename = "itunes-explicit")]
    pub itunes_explicit: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ItemItunesImage {
    #[serde(rename = "@href")]
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Guid {
    #[serde(rename = "@isPermaLink")]
    pub is_perma_link: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Enclosure {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "@length")]
    pub length: String,
    #[serde(rename = "@type")]
    pub enclosure_type: String,
}

pub fn parse_feed(xml: &str) -> Feed {
    from_str(&xml.replace("itunes:", "itunes-").replace("atom:", "atom-")).unwrap()
}

#[test]
fn parses_xml() {
    let xml = include_str!("../../test_data/feed.rss");
    let podcast = parse_feed(xml).podcast;
    assert_eq!(podcast.atom_link.href, "https://feeds.megaphone.fm/fface");
    assert_eq!(podcast.title, "Regulation Podcast");
    assert_eq!(podcast.description, "The Regulation Podcast is a show about friendship and embracing absurdity. Join Andrew, Nick, Eric, Geoff and Gavin in a never-ending pursuit to make each other laugh at almost any cost. Whether it's Podcasts, Let's Plays or Anal Waxings something incredibly stupid is always happening and we'd love for you to be along for the ride.\u{a0}");
    assert_eq!(podcast.image.url, "https://megaphone.imgix.net/podcasts/8f49d8ba-9de6-11ea-8184-d33047a704dc/image/b0198dcae0e6a8eddec040c27f20cded.jpg?ixlib=rails-4.3.1 &max-w=3000 &max-h=3000 &fit=crop &auto=format,compress");

    assert!(podcast
        .episodes
        .iter()
        .any(|episode| episode.title == "Why We Sync // Fastball vs Linebacker [21]"))
}
