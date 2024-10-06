mod app;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(async move {
      app::config::Config::setup().await
    });
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
