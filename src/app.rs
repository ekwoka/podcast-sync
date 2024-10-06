use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
pub mod config;
use config::Config;
use std::ops::Not;



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
            if Config::exists().await.not() {
              Config::create().await.unwrap();
            };
            let name = name.get_untracked();
            if name.is_empty() {
              return;
            }
            Config::update(name.as_str()).await.unwrap();
            let config = Config::load().await.unwrap();
            set_greet_msg.set(config.as_string().unwrap())
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
