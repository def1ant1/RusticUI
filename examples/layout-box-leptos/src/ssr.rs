use crate::components::{LayoutBoxApp, HYDRATION_CONTAINER_ID};
use leptos::{create_runtime, view, IntoView};
use rustic_ui_system::theme::Theme;

/// Renders the Leptos showcase into a deterministic HTML document so SSR and
/// CSR builds share the same markup structure and automation markers.
pub fn render_document() -> String {
    let theme = Theme::default();
    let runtime = create_runtime();
    let markup = view! { <LayoutBoxApp theme=theme.clone() /> }
        .into_view()
        .render_to_string();
    runtime.dispose();

    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>RusticUI Leptos Box</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head><body><div id=\"{root}\" data-rustic-layout-box-root>{markup}</div></body></html>",
        root = HYDRATION_CONTAINER_ID
    )
}

#[cfg(all(test, feature = "ssr", not(feature = "csr")))]
mod tests {
    use super::*;

    #[test]
    fn document_contains_hydration_container() {
        let document = render_document();
        assert!(document.contains(HYDRATION_CONTAINER_ID));
        assert!(document.contains("data-rustic-layout-box-phase"));
    }
}
