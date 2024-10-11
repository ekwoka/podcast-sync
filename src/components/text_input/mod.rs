use leptos::*;
stylance::import_crate_style!(styles, "src/components/text_input/mod.css");

#[component]
pub fn TextInput(id: String, name: String, placeholder: String) -> impl IntoView {
    view! {
      <input class=styles::input
                        id={id}
                        name={name}
                        placeholder={placeholder}
                    />
    }
}
