#[path = "./library_:id.rs"]
pub mod id;
use crate::components::SubscriptionList;
use crate::utils::*;
use api_types::subscriptions::Subscriptions;
use leptos::{html::*, *};

stylance::import_crate_style!(styles, "src/routes/library/library.css");
#[component]
pub fn view() -> impl IntoView {
    let subscriptions = create_resource(
        || (),
        |_| async move {
            let response = invoke_one("load_subscriptions").await;
            let subscriptions: Subscriptions = serde_wasm_bindgen::from_value(response).unwrap();
            subscriptions
        },
    );
    view! {
        <div>
            <h2>"Library"</h2>
            <Suspense fallback=|| {
                div().child("Loading...")
            }>
                {move || {
                    subscriptions
                        .get()
                        .map(|subs| {
                            view! { <SubscriptionList subscriptions=subs.subscriptions /> }
                        })
                }}
            </Suspense>
        </div>
    }
}
