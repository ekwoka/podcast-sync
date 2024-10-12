use leptos::{html::*, *};
stylance::import_crate_style!(styles, "src/components/atoms/text_input.css");

#[component]
pub fn text_input(id: String, name: String, placeholder: String) -> impl IntoView {
    input()
        .id(id)
        .attr("class", styles::input)
        .attr("name", name)
        .attr("placeholder", placeholder)
}
