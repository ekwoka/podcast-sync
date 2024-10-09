use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
pub mod fs;
use fs::{BaseDirectory, Config, File};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
    let (config_data, set_config_data) = create_signal::<Option<Config>>(None);
    let current_config = create_resource(
        || (),
        |_| async move {
            Config {
                directory: BaseDirectory::AppConfig,
                ..Config::default()
            }
            .create_if_missing()
            .await
            .unwrap()
            .load()
            .await
            .unwrap()
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
            if let Some(config) = config_data.get() {
                let config = config.update(name.as_str()).save().await.unwrap();
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
