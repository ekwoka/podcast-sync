use quick_xml::{de::from_str, DeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Feed {
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@xmlns:itunes")]
    pub xmlns_itunes: Option<String>,
    #[serde(rename = "@xmlns:googleplay")]
    pub xmlns_googleplay: Option<String>,
    #[serde(rename = "@xmlns:atom")]
    pub xmlns_atom: Option<String>,
    #[serde(rename = "@xmlns:media")]
    pub xmlns_media: Option<String>,
    #[serde(rename = "@xmlns:content")]
    pub xmlns_content: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "channel")]
    pub podcast: Podcast,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Podcast {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub title: String,
    pub link: String,
    pub language: String,
    pub copyright: String,
    pub description: String,
    pub image: Image,
    #[serde(rename = "itunes-explicit")]
    pub itunes_explicit: Option<String>,
    #[serde(rename = "itunes-type")]
    pub itunes_type: Option<String>,
    #[serde(rename = "itunes-subtitle")]
    pub itunes_subtitle: Option<String>,
    #[serde(rename = "itunes-author")]
    pub itunes_author: Option<String>,
    #[serde(rename = "itunes-summary")]
    pub itunes_summary: Option<String>,
    #[serde(rename = "encoded")]
    pub content_encoded: Option<String>,
    #[serde(rename = "itunes-owner")]
    pub itunes_owner: ItunesOwner,
    /*     #[serde(rename = "image")]
    pub itunes_image: ChannelItunesImage, */
    #[serde(rename = "itunes-category")]
    pub itunes_category: Vec<ItunesCategory>,
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
    pub itunes_title: Option<String>,
    #[serde(rename = "itunes-episodeType")]
    pub itunes_episode_type: Option<String>,
    #[serde(rename = "itunes-author")]
    pub itunes_author: Option<String>,
    #[serde(rename = "itunes-subtitle")]
    pub itunes_subtitle: Option<String>,
    #[serde(rename = "itunes-summary")]
    pub itunes_summary: Option<String>,
    #[serde(rename = "encoded")]
    pub content_encoded: Option<String>,
    #[serde(rename = "itunes-duration")]
    pub itunes_duration: Option<String>,
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

pub fn parse_feed(xml: &str) -> Result<Feed, DeError> {
    let feed: Feed = from_str(&xml.replace("itunes:", "itunes-").replace("atom:", "atom-"))?;
    Ok(feed)
}

#[test]
fn parses_xml_one() {
    let xml = include_str!("../test_data/test_feed_one.rss");
    let podcast = parse_feed(xml).unwrap().podcast;
    assert_eq!(podcast.title, "Regulation Podcast");
    assert_eq!(podcast.description, "The Regulation Podcast is a show about friendship and embracing absurdity. Join Andrew, Nick, Eric, Geoff and Gavin in a never-ending pursuit to make each other laugh at almost any cost. Whether it's Podcasts, Let's Plays or Anal Waxings something incredibly stupid is always happening and we'd love for you to be along for the ride.\u{a0}");
    assert_eq!(podcast.image.url, "https://megaphone.imgix.net/podcasts/8f49d8ba-9de6-11ea-8184-d33047a704dc/image/b0198dcae0e6a8eddec040c27f20cded.jpg?ixlib=rails-4.3.1 &max-w=3000 &max-h=3000 &fit=crop &auto=format,compress");

    assert!(podcast
        .episodes
        .iter()
        .any(|episode| episode.title == "Why We Sync // Fastball vs Linebacker [21]"));
}
#[test]
fn parses_xml_two() {
    let xml = include_str!("../test_data/test_feed_two.rss");
    let podcast = parse_feed(xml).unwrap().podcast;
    assert_eq!(podcast.title, "99% Invisible");
    assert_eq!(podcast.description, "Design is everywhere in our lives, perhaps most importantly in the places where we've just stopped noticing. 99% Invisible is a weekly exploration of the process and power of design and architecture. From award winning producer Roman Mars. Learn more at 99percentinvisible.org.");
    assert_eq!(podcast.image.url, "https://image.simplecastcdn.com/images/96792a27-13c3-40ce-b933-36bdb43a299e/54c9f8d8-80d8-425d-a3f3-751478db2e52/3000x3000/cover-99percentinvisible-3000x3000-r2021-final.jpg?aid=rss_feed");

    assert!(podcast
        .episodes
        .iter()
        .any(|episode| episode.title == "The Infernal Machine"));
}

#[test]
fn parses_xml_three() {
    let xml = include_str!("../test_data/test_feed_three.rss");
    let podcast = parse_feed(xml).unwrap().podcast;
    assert_eq!(podcast.title, "The Anthropocene Reviewed");
    assert_eq!(podcast.description, "The Anthropocene is the current geological age, in which human activity has profoundly shaped the planet and its biodiversity. On The Anthropocene Reviewed, #1 New York Times bestselling author John Green (The Fault in Our Stars, Turtles All the Way Down) reviews different facets of the human-centered planet on a five-star scale. WNYC Studios is a listener-supported producer of other leading podcasts including On the Media, Snap Judgment, Death, Sex & Money, Nancy and Here’s the Thing with Alec Baldwin. © WNYC Studios");
    assert_eq!(podcast.image.url, "https://image.simplecastcdn.com/images/d48cf57e-8709-499d-a9c6-1c6264aff730/d7543167-91d7-455a-9048-b1843e740206/3000x3000/tar-complexly.jpg?aid=rss_feed");

    assert!(podcast
        .episodes
        .iter()
        .any(|episode| episode.title == "Lascaux Paintings and the Taco Bell Breakfast Menu"));
}
