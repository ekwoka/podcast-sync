use crate::components::{SearchBar, SearchBarProps};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ItunesSearchArgs<'a> {
    query: &'a str,
}

#[component]
pub fn view() -> impl IntoView {
    SearchBar(SearchBarProps {
        initial: "".to_string(),
    })
}
