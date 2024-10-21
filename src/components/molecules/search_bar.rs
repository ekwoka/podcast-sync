use crate::components::{Button, ButtonType, TextInput};
use leptos::*;
use leptos_router::Form;

#[component]
pub fn search_bar(initial: String) -> impl IntoView {
    view! {
        <Form method="GET" action="/search">
            <TextInput
                id="search".to_string()
                name="q".to_string()
                placeholder="Search podcasts...".to_string()
                value=initial
            />
            <Button btn_type=ButtonType::Submit>"Search"</Button>
        </Form>
    }
}
