use api_types::config::Config;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::Serialize;
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

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
    let (message, set_message) = create_signal::<Option<String>>(None);

    let update_value = move |ev| {
        let v = event_target_value(&ev);
        set_value.set(v);
    };

    let save = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = value.get_untracked();
            if name.is_empty() {
                return;
            }
            set_message.set(Some(name));
        });
    };

    view! {
        <main class="container">
            <form class="row" on:submit=save>
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


            {move || match message.get() {
                None => {
                    view! {
                        <p>"Ready"</p>
                    }
                }
                Some(message) => {
                    view! {
                        <p style="text-align: left; max-width: fit-content; margin: 0 auto;">
                            <pre>{ move || message.clone() }</pre>
                        </p>
                    }
                }
            }}
        </main>
    }
}
