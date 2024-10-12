use leptos::*;
stylance::import_crate_style!(styles, "src/components/atoms/button.css");

#[derive(strum_macros::Display, Default)]
#[strum(serialize_all = "lowercase")]
pub enum ButtonType {
    Submit,
    #[default]
    Button,
}

#[component]
pub fn button(btn_type: ButtonType, children: Children) -> impl IntoView {
    html::button()
        .attr("type", btn_type.to_string())
        .attr("class", styles::button)
        .child(children())
}
