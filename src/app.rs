use crate::components::Navigation;
use crate::routes::*;
use api_types::subscriptions::*;
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
    /*
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
    */

    view! {
        <Router>
            <div class=styles::layout>
                <Navigation />
                <main class=styles::main>
                    <Routes>
                        <Route path="/" view=root::View />
                        <Route path="/search" view=search::View />
                        <Route path="/library" view=library::View />
                        <Route path="/library/:id" view=library::id::View />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

/*
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
