use crate::components::{Button, ButtonType, TextInput};
use crate::utils::*;
use api_types::itunes::*;
use leptos::leptos_dom::ev;
use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ItunesSearchArgs<'a> {
    query: &'a str,
}

#[component]
pub fn view() -> impl IntoView {
    let query = create_rw_signal(String::new());
    let results = create_rw_signal::<Option<Vec<ItunesResult>>>(None);

    let search = move |ev: ev::SubmitEvent| {
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
            let navigate = use_navigate();
            navigate(
                &format!("/search?q={}", query.get_untracked()),
                Default::default(),
            );
        });
    };
    let update_value = move |ev| {
        let v = event_target_value(&ev);
        query.set(v);
    };

    view! {
        <form on:submit=search>
            <TextInput
                id="search".to_string()
                name="search".to_string()
                placeholder="Search podcasts...".to_string()
                on:input=update_value
            />
            <Button btn_type=ButtonType::Submit>"Search"</Button>
        </form>
    }
}
