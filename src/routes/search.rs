use crate::{
    components::{ItunesResult, ItunesResultProps},
    utils::*,
};
use api_types::{itunes::ItunesResponse, subscriptions::Subscription};
use html::div;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(styles, "src/routes/search.css");

#[derive(Serialize, Deserialize)]
struct ItunesSearchArgs<'a> {
    query: &'a str,
}
#[derive(Serialize, Deserialize)]
struct SubscribeArgs {
    subscription: Subscription,
}
#[component]
pub fn view() -> impl IntoView {
    let query = use_query::<SearchQueries>();
    let results = create_resource(
        move || query.with(|qs| qs.as_ref().ok().and_then(|qs| qs.q.clone())),
        |query: Option<String>| async move {
            match query {
                Some(query) => {
                    let response = invoke(
                        "search_itunes",
                        serde_wasm_bindgen::to_value(&ItunesSearchArgs {
                            query: query.as_str(),
                        })
                        .unwrap(),
                    )
                    .await;
                    let response: ItunesResponse =
                        serde_wasm_bindgen::from_value(response).unwrap();
                    response.results.clone()
                }
                None => Vec::new(),
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
        }
    });
    view! {
        <div>
            <h2>"Results"</h2>
            <div>
                {move || match results.get() {
                    None => div().child("Searching..."),
                    Some(results) => {
                        match results.len() {
                            0 => div().child("No results found"),
                            _ => {
                                view! {
                                    <div class=styles::results_grid>
                                        {results
                                            .clone()
                                            .iter()
                                            .map(|result| {
                                                ItunesResult(ItunesResultProps {
                                                    show: result.clone(),
                                                    subscribe,
                                                })
                                            })
                                            .collect_view()}
                                    </div>
                                }
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct SearchQueries {
    q: Option<String>,
}
