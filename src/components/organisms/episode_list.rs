use api_types::podcast_feed::Podcast;
use leptos::{html::*, *};

#[component]
pub fn episode_list(podcast: Podcast) -> impl IntoView {
    let episodes = podcast
        .episodes
        .iter()
        .map(|episode| {
            li().child(episode.title.clone())
                .child(" - ")
                .child(episode.description.clone())
        })
        .collect_view();

    ul().child(episodes)
}
