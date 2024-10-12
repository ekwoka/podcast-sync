use leptos::*;
stylance::import_crate_style!(styles, "src/components/atoms/button.css");

#[derive(Default)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl std::fmt::Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ButtonType::Submit => "submit".to_string(),
                ButtonType::Reset => "reset".to_string(),
                ButtonType::Button => "button".to_string(),
            }
        )
    }
}

#[component]
pub fn button(btn_type: ButtonType, children: Children) -> impl IntoView {
    view! {
        <button type=btn_type.to_string() class=styles::button>
            {children()}
        </button>
    }
}
