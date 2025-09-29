use crate::components::{LayoutGridApp, LayoutGridAppProps, HYDRATION_CONTAINER_ID};
use rustic_ui_system::theme::Theme;
use yew::ServerRenderer;

/// Renders the Yew showcase into a standalone HTML document suitable for
/// pre-rendering pipelines or HTTP handlers. The markup mirrors what the CSR
/// bundle hydrates so automation can diff snapshots across render modes.
pub async fn render_document() -> String {
    let theme = Theme::default();
    let markup = ServerRenderer::<LayoutGridApp>::with_props(move || LayoutGridAppProps {
        theme: Some(theme.clone()),
    })
    .render()
    .await;

    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>RusticUI Yew Grid</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head><body><div id=\"{root}\" data-rustic-layout-grid-root>{markup}</div></body></html>",
        root = HYDRATION_CONTAINER_ID
    )
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn document_contains_hydration_container() {
        let document = render_document().await;
        assert!(document.contains(HYDRATION_CONTAINER_ID));
        assert!(document.contains("data-rustic-layout-grid-phase"));
    }
}
