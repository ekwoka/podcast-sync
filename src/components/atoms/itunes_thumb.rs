use leptos::{html::*, *};
stylance::import_crate_style!(styles, "src/components/atoms/itunes_thumb.css");

#[component]
pub fn itunes_thumb(url: String) -> impl IntoView {
    img()
        .attr("src", url.clone())
        .attr("srcset", generate_srcset(&url))
        .attr("width", 500)
        .attr("height", 500)
        .attr("sizes", "40vw")
        .attr("alt", "")
        .attr("class", styles::thumb)
}

fn generate_srcset(url: &str) -> String {
    vec![128, 256, 512, 1024]
        .iter()
        .map(|size| {
            format!(
                "{} {}w",
                url.replace("100x100", &format!("{}x{}", size, size)),
                size
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}
