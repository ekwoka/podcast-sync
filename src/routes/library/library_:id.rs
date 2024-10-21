use crate::components::EpisodeList;
use crate::utils::*;
use api_types::{podcast_feed::Podcast, subscriptions::Subscription};
use leptos::{html::*, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(styles, "src/routes/library/library.css");

#[derive(Params, PartialEq, Debug)]
pub struct LibraryShowParams {
    pub id: Option<usize>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoadSubscriptionArgs {
    subscription_id: usize,
}

#[derive(Serialize, Deserialize)]
struct LoadPodcastArgs {
    podcast: Subscription,
}

#[component]
pub fn view() -> impl IntoView {
    let params = use_params::<LibraryShowParams>();

    let subscription = create_resource(
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default()),
        |podcast_id: Option<usize>| async move {
            match podcast_id {
                Some(subscription_id) => {
                    let response = invoke(
                        "load_subscription",
                        serde_wasm_bindgen::to_value(&LoadSubscriptionArgs { subscription_id })
                            .unwrap(),
                    )
                    .await;
                    let podcast: Subscription = serde_wasm_bindgen::from_value(response).unwrap();
                    Some(podcast)
                }
                None => None,
            }
        },
    );

    let podcast = create_resource(
        move || subscription.get(),
        |subscription: Option<Option<Subscription>>| async move {
            match subscription.flatten() {
                Some(subscription) => {
                    let response = invoke(
                        "load_podcast_feed",
                        serde_wasm_bindgen::to_value(&LoadPodcastArgs {
                            podcast: subscription,
                        })
                        .unwrap(),
                    )
                    .await;
                    let podcast: Podcast = serde_wasm_bindgen::from_value(response).unwrap();
                    Some(podcast)
                }
                None => None,
            }
        },
    );
    view! {
        <Suspense fallback=|| {
            div().child("Loading...")
        }>
            {move || {
                podcast
                    .get()
                    .map(|podcast| {
                        podcast
                            .map(|podcast| {
                                view! { <EpisodeList podcast=podcast /> }
                            })
                    })
            }}
        </Suspense>
    }
}
