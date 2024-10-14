use crate::components;
use api_types::{itunes::ItunesResult, subscriptions::Subscription};
use leptos::{html::*, *};

stylance::import_crate_style!(styles, "src/components/molecules/itunes_result.css");

#[component]
pub fn itunes_result(show: ItunesResult, subscribe: Action<Subscription, ()>) -> impl IntoView {
    view! {
        <div class=styles::show>
            <h2 class=styles::title>
                {match show.collection_name.clone() {
                    Some(title) => title,
                    None => "No Title".to_string(),
                }}
            </h2>
            {match show.artwork_url100.clone() {
                Some(url) => {
                    span()
                        .child(
                            components::ItunesThumb(components::ItunesThumbProps {
                                url,
                            }),
                        )
                }
                None => span().child("No Artwork"),
            }}
            <components::Button
                btn_type=components::ButtonType::Button
                on:click=move |_| {
                    subscribe
                        .dispatch(Subscription {
                            id: show.collection_id,
                            title: show.collection_name.clone().unwrap_or("No Title".to_string()),
                            feed_url: show.feed_url.clone().unwrap_or("".to_string()),
                            image_url: show.artwork_url100.clone(),
                            last_updated: chrono::Utc::now(),
                        })
                }
            >
                "Subscribe"
            </components::Button>
        </div>
    }
}
