mod app;

use app::*;
use fs::{BaseDirectory, File};
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(async move {
        app::fs::Config {
            directory: BaseDirectory::AppConfig,
            ..app::fs::Config::default()
        }
        .create_if_missing()
        .await
        .unwrap();
    });
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
