use api_types::itunes::*;
use api_types::subscriptions::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::{html::*, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::components;

stylance::import_crate_style!(styles, "src/main.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name="invoke")]
    async fn invoke_one(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ItunesSearchArgs<'a> {
    query: &'a str,
}

#[derive(Serialize, Deserialize)]
struct SubscribeArgs {
    subscription: Subscription,
}

#[component]
pub fn app() -> impl IntoView {
    let query = create_rw_signal(String::new());
    let results = create_rw_signal::<Option<Vec<ItunesResult>>>(None);
    let subscriptions = create_resource(
        || (),
        |_| async move {
            let response = invoke_one("load_subscriptions").await;
            let subscriptions: Subscriptions = serde_wasm_bindgen::from_value(response).unwrap();
            subscriptions
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

    let update_value = move |ev| {
        let v = event_target_value(&ev);
        query.set(v);
    };

    let search = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let term = query.get_untracked();
            if term.is_empty() {
                return;
            }
            let response = invoke(
                "search_itunes",
                serde_wasm_bindgen::to_value(&ItunesSearchArgs {
                    query: term.as_str(),
                })
                .unwrap(),
            )
            .await;
            let response: ItunesResponse = serde_wasm_bindgen::from_value(response).unwrap();
            if response.result_count > 0 {
                results.set(Some(response.results.clone()));
            } else {
                results.set(None);
            }
        });
    };

    view! {
        <main class=styles::container>
            <div>
                <form on:submit=search>
                    <components::TextInput
                        id="search".to_string()
                        name="search".to_string()
                        placeholder="Search podcasts...".to_string()
                        on:input=update_value
                    />
                    <components::Button btn_type=components::ButtonType::Submit>
                        "Search"
                    </components::Button>
                </form>

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
                                            subscribe: subscribe.clone(),
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
                                <components::SubscriptionList subscriptions=subs.subscriptions />
                            }
                        })
                }}
            </Suspense>
        </main>
    }
}
