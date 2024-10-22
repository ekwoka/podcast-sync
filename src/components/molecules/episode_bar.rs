use api_types::podcast_feed::Episode;
use leptos::*;
stylance::import_crate_style!(styles, "src/components/molecules/episode_bar.css");

#[component]
pub fn episode_bar(episode: Episode) -> impl IntoView {
    view! {
        <div class=styles::bar>
            <div>
                <h4>{episode.title}</h4>
            </div>
            <div class=styles::controls>
                <button>"play"</button>
                <button>"pause"</button>
            </div>
        </div>
    }
}
