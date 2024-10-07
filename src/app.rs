use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
pub mod fs;
use fs::{File, Config, BaseDirectory};



#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
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
          let name = name.get_untracked();
          if name.is_empty() {
            return;
          }
          let config = Config { directory: BaseDirectory::AppConfig, ..Config::default() }
            .create_if_missing().await.unwrap().update(name.as_str()).save().await.unwrap().load().await.unwrap();
            set_greet_msg.set(serde_json::to_string_pretty(&config).unwrap());
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

            <p style="text-align: left; max-width: fit-content; margin: 0 auto;">
            <pre>{ move || greet_msg.get() }</pre>
            </p>
        </main>
    }
}
