use api_types::podcast_feed::Podcast;
use leptos::*;
stylance::import_crate_style!(styles, "src/components/molecules/podcast_summary.css");

#[component]
pub fn podcast_summary(podcast: Podcast) -> impl IntoView {
    view! {
        <div class=styles::summary>
            <img class=styles::image src=podcast.image.url />
            <div class=styles::info>
                <h3>{podcast.title}</h3>
                <p>{podcast.description}</p>
            </div>
        </div>
    }
}
