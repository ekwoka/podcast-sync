use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch)]
    async fn exists(token: &str, args: JsValue) -> Result<JsValue,JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch)]
    async fn create(token: &str, args: JsValue) -> Result<JsValue,JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch, js_name="writeTextFile")]
    async fn write_text_file(token: &str, content: &str, args: JsValue) -> Result<JsValue,JsValue>;
}



#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ExistsArgs {
    baseDir: u16,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_value = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let save = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if exists("config.json", serde_wasm_bindgen::to_value(&ExistsArgs {baseDir: 15 }).unwrap()).await.expect("should have permissions").is_falsy() && create("config.json", serde_wasm_bindgen::to_value(&ExistsArgs {baseDir: 15 }).unwrap()).await.expect("should have permissions").is_truthy() {
              write_text_file("config.json",
              "{ \"name\": \"This is a file\" }", serde_wasm_bindgen::to_value(&ExistsArgs {baseDir: 15 }).unwrap()).await.unwrap();
            };
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);

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

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
