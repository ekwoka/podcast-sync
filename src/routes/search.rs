use leptos::*;
use leptos_router::*;

#[component]
pub fn view() -> impl IntoView {
    let query = use_query::<SearchQueries>();
    let query_string = query.with(move |qs| {
        qs.as_ref()
            .ok()
            .and_then(|qs| qs.q.clone())
            .unwrap_or_default()
    });
    view! {
        <div>
            <h1>{format!("Hello, {}!", query_string)}</h1>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct SearchQueries {
    q: Option<String>,
}
