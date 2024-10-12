use api_types::itunes::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
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

#[component]
pub fn App() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (message, set_message) = create_signal::<Option<String>>(None);
    let (results, set_results) = create_signal::<Option<Vec<ItunesResult>>>(None);

    let update_value = move |ev| {
        let v = event_target_value(&ev);
        set_query.set(v);
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
            let results: ItunesResponse = serde_wasm_bindgen::from_value(response).unwrap();
            if results.result_count > 0 {
                set_results.set(Some(results.results.clone()));
            } else {
                set_results.set(None);
            }
        });
    };

    view! {
        <main class="container">
            <form class="row" on:submit=search>
                <components::TextInput
                    id="search".to_string()
                    name="search".to_string()
                    placeholder="Search podcasts...".to_string()
                    on:input=update_value
                />
                <components::Button btn_type={components::ButtonType::Submit}>
                  "Search"
                </components::Button>
            </form>


            {move || match results.get() {
                None => {
                    view! {
                        <div>"No Results"</div>
                    }
                }
                Some(results) => {
                    view! {
                        <div style="display: grid; gap: 8px; grid-template-columns: repeat(2, 1fr);">
                            {results.clone().iter().map(|result| view!{
                              <div style="display: flex; flex-direction: column; gap: 4px;">
                                <h2>{match result.collection_name.clone() {
                                  Some(title) => title,
                                  None => "No Title".to_string()
                                }
                                }
                                </h2>
                                {match result.artwork_url100.clone() {
                                  Some(url) => view!{
                                    <span><img src={url} alt="Artwork" /></span>
                                  },
                                  None => view!{
                                    <span>"No Artwork"</span>
                                  }
                                }}
                                </div>
                            }).collect_view()}
                        </div>
                    }
                }
            }}
        </main>
    }
}
