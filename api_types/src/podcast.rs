use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Podcast {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image: Image,
    pub link: String,
    pub episodes: Vec<Episode>,
}

impl Podcast {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }
}

impl From<crate::podcast_feed::Podcast> for Podcast {
    fn from(feed_podcast: crate::podcast_feed::Podcast) -> Self {
        Self {
            id: 0,
            title: feed_podcast.title,
            description: feed_podcast.description,
            image: feed_podcast.image.into(),
            link: feed_podcast.link,
            episodes: feed_podcast
                .episodes
                .into_iter()
                .map(|episode| episode.into())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Image {
    pub url: String,
    pub title: String,
}

impl From<crate::podcast_feed::Image> for Image {
    fn from(feed_image: crate::podcast_feed::Image) -> Self {
        Self {
            url: feed_image.url,
            title: feed_image.title,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Episode {
    pub title: String,
    pub description: String,
    pub published: chrono::NaiveDateTime,
    pub url: String,
    pub length: i32,
    pub mime_type: String,
    pub duration: Option<String>,
    pub progress: i32,
    pub listened: bool,
}

impl From<crate::podcast_feed::Episode> for Episode {
    fn from(feed_episode: crate::podcast_feed::Episode) -> Self {
        Self {
            title: feed_episode.title,
            description: feed_episode.description,
            published: parse_date(&feed_episode.pub_date),
            url: feed_episode.enclosure.url,
            length: feed_episode.enclosure.length.parse::<i32>().unwrap_or(0),
            mime_type: feed_episode.enclosure.enclosure_type,
            duration: feed_episode.itunes_duration,
            progress: 0,
            listened: false,
        }
    }
}

pub fn parse_date(date: &str) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(date, "%a, %d %b %Y %H:%M:%S %z")
        .unwrap_or(chrono::NaiveDateTime::default())
}

#[test]
fn can_parse_podcast_date() {
    let date = "Wed, 12 May 2021 07:00:00 -0000";
    let parsed = parse_date(date);
    assert_eq!(
        parsed,
        chrono::NaiveDate::from_ymd_opt(2021, 5, 12)
            .and_then(|d| d.and_hms_opt(7, 0, 0))
            .unwrap()
    );
}

#[test]
fn can_convert_from_feed() {
    let xml = include_str!("../test_data/test_feed_one.rss");
    let podcast = crate::podcast_feed::parse_feed(xml).unwrap().podcast;
    let podcast: Podcast = podcast.into();
    assert_eq!(podcast.title, "Regulation Podcast");
    assert_eq!(podcast.description, "The Regulation Podcast is a show about friendship and embracing absurdity. Join Andrew, Nick, Eric, Geoff and Gavin in a never-ending pursuit to make each other laugh at almost any cost. Whether it's Podcasts, Let's Plays or Anal Waxings something incredibly stupid is always happening and we'd love for you to be along for the ride.\u{a0}");
    assert_eq!(podcast.episodes[0], Episode {
      title: "Why We Sync // Fastball vs Linebacker [21]".to_string(),
      description: "Geoff, Gavin and Andrew talk about Andrew's recordings, audio syncing, getting distracted, Jolly Green Giant, hot dog enthusiasm, Matt Strahm, baseball mound, hit by a pitch vs hit by a linebacker, terminal velocity of a grape, balloon update, songs of the summer, duel of the fates, falcon tier football pick em loser, kicker picker, open faced mashed hashed cashed sandwich, puzzle times, Geoff's f**kface, and Crime Capsule.
Sponsored by Cosmic Crisp. Visit http://www.cosmiccrisp.com for merch and recipe ideas. Follow TheCosmicCrisp on Instagram and other social channels. They love to hear from Regulation Podcast fans!
Support us directly at http://patreon.com/theregulationpod
Stay up to date, get exclusive supplemental, and connect with other Regulation Listeners.
Learn more about your ad choices. Visit megaphone.fm/adchoices".to_string(),
      published: chrono::NaiveDate::from_ymd_opt(2024, 10, 2)
          .and_then(|d| d.and_hms_opt(7, 0, 0))
          .unwrap(),
      url: "https://www.podtrac.com/pts/redirect.mp3/pdst.fm/e/mgln.ai/e/94/claritaspod.com/measure/verifi.podscribe.com/rss/p/pfx.vpixl.com/j0JIg/traffic.megaphone.fm/REGULATIONCOMPANYLLC7453036557.mp3?updated=1727834300".to_string(),
      length: 0,
      mime_type: "audio/mpeg".to_string(),
      duration: Some("4559".to_string()),
      progress: 0,
      listened: false,
    })
}
