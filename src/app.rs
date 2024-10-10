use api_types::config::Config;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name="invoke")]
    async fn invoke_one(cmd: &str) -> JsValue;
}

#[derive(Serialize)]
struct ConfigArgs<'a> {
    config: &'a Config,
}

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
    let (config_data, set_config_data) = create_signal::<Option<Config>>(None);
    let current_config = create_resource(
        || (),
        |_| async move {
            let configData = invoke_one("load_config").await;
            let config: Config = serde_wasm_bindgen::from_value(configData).unwrap();
            config
        },
    );
    create_effect(move |_| match current_config.get() {
        None => (),
        Some(config) => {
            set_config_data.set(Some(config));
        }
    });

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
            if let Some(mut config) = config_data.get() {
                config.latest_message = name.to_string();
                invoke(
                    "save_config",
                    serde_wasm_bindgen::to_value(&ConfigArgs { config: &config }).unwrap(),
                )
                .await;
                set_config_data.set(Some(config));
            }
        });
    };

    view! {
        <main class="container">
            <div class="row">
                "Podcasts"
            </div>

            <p>"Play some podcasts"</p>

            <form class="row" on:submit=save>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_value
                />
                <button type="submit">"Greet"</button>
            </form>


            {move || match config_data.get() {
                None => {
                    view! {
                        <p>"Loading..."</p>
                    }
                }
                Some(config) => {
                    view! {
                        <p style="text-align: left; max-width: fit-content; margin: 0 auto;">
                            <pre>{ move || config.to_string().unwrap() }</pre>
                        </p>
                    }
                }
            }}
        </main>
    }
}
