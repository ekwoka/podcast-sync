use crate::components;
use api_types::itunes::ItunesResult;
use leptos::{html::*, *};

stylance::import_crate_style!(styles, "src/components/molecules/itunes_result.css");

#[component]
pub fn itunes_result(show: ItunesResult) -> impl IntoView {
    view! {
        <div class=styles::show>
            <h2 class=styles::title>
                {match show.collection_name.clone() {
                    Some(title) => title,
                    None => "No Title".to_string(),
                }}
            </h2>
            {match show.artwork_url100.clone() {
                Some(url) => {
                    span()
                        .child(
                            components::ItunesThumb(components::ItunesThumbProps {
                                url: url,
                            }),
                        )
                }
                None => span().child("No Artwork"),
            }}
        </div>
    }
}
