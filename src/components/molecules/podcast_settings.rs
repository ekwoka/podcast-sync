use crate::components::{Button, ButtonType};
use leptos::*;

stylance::import_crate_style!(styles, "src/components/molecules/podcast_settings.css");

#[component]
pub fn podcast_settings(podcast_id: usize) -> impl IntoView {
    view! {
        <div class=styles::list_box>
            <Button btn_type=ButtonType::Button>unsubscribe from {format!("{podcast_id}")}</Button>
        </div>
    }
}
