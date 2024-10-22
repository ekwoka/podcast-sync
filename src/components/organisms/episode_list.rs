use api_types::podcast_feed::Podcast;
use leptos::{html::*, *};

use crate::components::{EpisodeBar, EpisodeBarProps};

stylance::import_crate_style!(styles, "src/components/organisms/episode_list.css");

#[component]
pub fn episode_list(podcast: Podcast) -> impl IntoView {
    let episodes = podcast
        .episodes
        .iter()
        .map(|episode| {
            li().child(EpisodeBar(EpisodeBarProps {
                episode: episode.clone(),
            }))
        })
        .collect_view();

    ul().attr("class", styles::list).child(episodes)
}
