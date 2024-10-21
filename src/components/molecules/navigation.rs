use leptos::*;
use leptos_router::A;

stylance::import_crate_style!(styles, "src/components/molecules/navigation.css");

#[component]
pub fn navigation() -> impl IntoView {
    view! {
        <nav class=styles::nav>
            <ul>
                <li>
                    <A href="/">"Home"</A>
                </li>
                <li>
                    <A href="/library">"Library"</A>
                </li>
            </ul>
        </nav>
    }
}
