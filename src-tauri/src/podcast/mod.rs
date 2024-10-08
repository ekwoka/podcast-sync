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
/*     #[serde(rename = "link")]
    pub atom_link: AtomLink, */
    pub title: String,
/*     pub link: String, */
    pub language: String,
    pub copyright: String,
    pub description: String,
/*     pub image: Image, */
    #[serde(rename = "explicit")]
    pub itunes_explicit: String,
    #[serde(rename = "type")]
    pub itunes_type: String,
    #[serde(rename = "subtitle")]
    pub itunes_subtitle: String,
    #[serde(rename = "author")]
    pub itunes_author: String,
    #[serde(rename = "summary")]
    pub itunes_summary: String,
    #[serde(rename = "encoded")]
    pub content_encoded: String,
    #[serde(rename = "owner")]
    pub itunes_owner: ItunesOwner,
/*     #[serde(rename = "image")]
    pub itunes_image: ChannelItunesImage, */
    #[serde(rename = "category")]
    pub itunes_category: ItunesCategory,
    pub item: Vec<Item>,
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
    #[serde(rename = "name")]
    pub itunes_name: String,
    #[serde(rename = "email")]
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
pub struct Item {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "image")]
    pub itunes_image: Option<ItemItunesImage>,
    #[serde(rename = "episode")]
    pub itunes_episode: Option<String>,
    #[serde(rename = "season")]
    pub itunes_season: Option<String>,
/*     pub title: String, */
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
/*     #[serde(rename = "title")]
    pub itunes_title: String, */
    #[serde(rename = "episodeType")]
    pub itunes_episode_type: String,
    #[serde(rename = "author")]
    pub itunes_author: String,
    #[serde(rename = "subtitle")]
    pub itunes_subtitle: String,
    #[serde(rename = "summary")]
    pub itunes_summary: String,
    #[serde(rename = "encoded")]
    pub content_encoded: String,
    #[serde(rename = "duration")]
    pub itunes_duration: String,
    pub guid: Guid,
    pub enclosure: Enclosure,
    #[serde(rename = "explicit")]
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

#[test]
fn parses_xml() {
  use quick_xml::de::from_str;
    let xml = include_str!("../../test_data/feed.rss");
    let feed: Feed = from_str(xml).unwrap();
    assert_eq!(feed.podcast.title, "Regulation Podcast");
    assert_eq!(feed.podcast.description, "The Regulation Podcast is a show about friendship and embracing absurdity. Join Andrew, Nick, Eric, Geoff and Gavin in a never-ending pursuit to make each other laugh at almost any cost. Whether it's Podcasts, Let's Plays or Anal Waxings something incredibly stupid is always happening and we'd love for you to be along for the ride.\u{a0}");
}
