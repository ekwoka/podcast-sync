use api_types::subscriptions::Subscription;
use leptos::{html::*, *};

#[component]
pub fn subscription_list(subscriptions: Vec<Subscription>) -> impl IntoView {
    view! {
        <div>
            <h2>{"Subscriptions"}</h2>
            {match subscriptions.len() {
                0 => ul().child(li().child("No subscriptions")),
                _ => {
                    ul()
                        .child(
                            subscriptions
                                .iter()
                                .map(|subscription| { li().child(subscription.title.clone()) })
                                .collect_view(),
                        )
                }
            }}

        </div>
    }
}
