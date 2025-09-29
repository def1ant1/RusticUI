use crate::app::{configure_state, HYDRATION_CONTAINER_ID};
use crate::telemetry::{sample_log, TelemetryLog};
use rustic_ui_material::bottom_navigation::{
    render_bottom_navigation_html, BottomNavigationAdapterProps, BottomNavigationItemDescriptor,
};

/// Renders the navigation showcase into a deterministic HTML document used for SSR snapshots.
pub fn render_document() -> String {
    let state = configure_state(Some(0), Some(0));
    let labels = crate::app::DESTINATIONS
        .iter()
        .map(|(_, label)| (*label).to_string())
        .collect::<Vec<_>>();
    let descriptors: Vec<BottomNavigationItemDescriptor<'_>> = labels
        .iter()
        .map(|label| BottomNavigationItemDescriptor {
            id: None,
            controls: None,
            content: label.as_str(),
        })
        .collect();
    let markup = render_bottom_navigation_html(BottomNavigationAdapterProps {
        state: &state,
        attributes: state
            .root_attributes()
            .id(HYDRATION_CONTAINER_ID)
            .labelled_by("navigation-bottom-label"),
        items: &descriptors,
        on_select_event: Some("navigation.bottom.select"),
    });
    let telemetry: TelemetryLog = sample_log();
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>RusticUI Yew Bottom Navigation</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head><body><div id=\"{root}\" data-rustic-bottom-nav=\"navigation.bottom\">{markup}</div><pre data-rustic-analytics-log>{log}</pre></body></html>",
        root = HYDRATION_CONTAINER_ID,
        log = htmlescape::encode_minimal(&telemetry.as_json_lines())
    )
}
