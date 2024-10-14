use crate::components;
use api_types::subscriptions::Subscription;
use leptos::{html::*, *};
stylance::import_crate_style!(styles, "src/components/organisms/subscription_list.css");

#[component]
pub fn subscription_list(
    subscriptions: Vec<Subscription>,
    selected_podcast: RwSignal<Option<Subscription>>,
) -> impl IntoView {
    view! {
        <div>
            <h2>{"Subscriptions"}</h2>
            {match subscriptions.len() {
                0 => ul().child(li().child("No subscriptions")),
                _ => {
                    ul()
                        .attr("class", styles::subscriptions)
                        .child(
                            subscriptions
                                .iter()
                                .map(move |subscription| {
                                    let subscription = subscription.clone();
                                    let subscription_title = subscription.title.clone();
                                    li()
                                        .child(
                                            view! {
                                                <components::Button
                                                    btn_type=components::ButtonType::Button
                                                    on:click=move |_| {
                                                        selected_podcast.set(Some(subscription.clone()))
                                                    }
                                                >
                                                    {subscription_title}
                                                </components::Button>
                                            },
                                        )
                                })
                                .collect_view(),
                        )
                }
            }}

        </div>
    }
}
