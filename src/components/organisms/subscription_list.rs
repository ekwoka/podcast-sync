use crate::components;
use api_types::subscriptions::Subscription;
use leptos::{html::*, *};
use leptos_router::A;
stylance::import_crate_style!(styles, "src/components/organisms/subscription_list.css");

#[component]
pub fn subscription_list(subscriptions: Vec<Subscription>) -> impl IntoView {
    view! {
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
                                            <A href=format!(
                                                "/library/{}",
                                                subscription.id,
                                            )>{subscription_title}</A>
                                        },
                                    )
                            })
                            .collect_view(),
                    )
            }
        }}
    }
}
