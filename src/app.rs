use crate::routes::*;
use crate::utils::*;
use api_types::{podcast_feed::*, subscriptions::*};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(styles, "src/main.css");

#[derive(Serialize, Deserialize)]
struct SubscribeArgs {
    subscription: Subscription,
}

#[derive(Serialize, Deserialize)]
struct LoadPodcastArgs {
    podcast: Subscription,
}

#[component]
pub fn app() -> impl IntoView {
    let selected_podcast = create_rw_signal::<Option<Subscription>>(None);
    let subscriptions = create_resource(
        || (),
        |_| async move {
            let response = invoke_one("load_subscriptions").await;
            let subscriptions: Subscriptions = serde_wasm_bindgen::from_value(response).unwrap();
            subscriptions
        },
    );
    let podcast = create_resource(
        move || selected_podcast.get(),
        |selected_podcast: Option<Subscription>| async move {
            match selected_podcast {
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

    let subscribe = create_action(move |input: &Subscription| {
        let input = input.clone();
        async move {
            invoke(
                "subscribe",
                serde_wasm_bindgen::to_value(&SubscribeArgs {
                    subscription: input,
                })
                .unwrap(),
            )
            .await;
            subscriptions.refetch();
        }
    });

    view! {
        <Router>
            <main class=styles::container>
                <Routes>
                    <Route path="/" view=root::View />
                    <Route path="/search" view=search::View />
                </Routes>
            </main>
        </Router>
    }
}

/*
{move || match results.get() {
                        None => div().child("No Results"),
                        Some(results) => {
                            view! {
                                <div class=styles::results_grid>
                                    {results
                                        .clone()
                                        .iter()
                                        .map(|result| {
                                            components::ItunesResult(components::ItunesResultProps {
                                                show: result.clone(),
                                                subscribe,
                                            })
                                        })
                                        .collect_view()}
                                </div>
                            }
                        }
                    }}
                </div>
                <Suspense fallback=|| {
                    div().child("Loading...")
                }>
                    {move || {
                        subscriptions
                            .get()
                            .map(|subs| {
                                view! {
                                    <components::SubscriptionList
                                        subscriptions=subs.subscriptions
                                        selected_podcast=selected_podcast
                                    />
                                }
                            })
                    }}
                </Suspense>
                <Suspense fallback=|| {
                    div().child("Loading...")
                }>
                    {move || {
                        podcast
                            .get()
                            .map(|podcast| {
                                podcast
                                    .map(|podcast| {
                                        view! { <components::EpisodeList podcast=podcast /> }
                                    })
                            })
                    }}
                </Suspense>
*/
