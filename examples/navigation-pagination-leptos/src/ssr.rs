use crate::app::{configure_state, HYDRATION_CONTAINER_ID, PAGE_LABELS};
use crate::telemetry::{sample_log, TelemetryLog};
use rustic_ui_material::pagination::{
    render_pagination_html, PaginationAdapterProps, PaginationItemDescriptor,
};

pub fn render_document() -> String {
    let state = configure_state(Some(0));
    let labels = PAGE_LABELS
        .iter()
        .map(|label| (*label).to_string())
        .collect::<Vec<_>>();
    let descriptors: Vec<PaginationItemDescriptor<'_>> = super::app::build_descriptors(&labels);
    let markup = render_pagination_html(PaginationAdapterProps {
        state: &state,
        root_attributes: state
            .root_attributes()
            .id(HYDRATION_CONTAINER_ID)
            .labelled_by("navigation-pagination-label"),
        list_attributes: state.list_attributes(),
        items: &descriptors,
        on_select_event: Some("navigation.pagination.select"),
    });
    let telemetry: TelemetryLog = sample_log();
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>RusticUI Leptos Pagination</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head><body><div id=\"{root}\" data-rustic-pagination=\"navigation.pagination\">{markup}</div><pre data-rustic-analytics-log>{log}</pre></body></html>",
        root = HYDRATION_CONTAINER_ID,
        log = htmlescape::encode_minimal(&telemetry.as_json_lines())
    )
}
