use crate::app::{build_state, render_markup, TELEMETRY_CHANNEL};
use crate::telemetry::{sample_log, TelemetryLog};

pub fn render_document() -> String {
    let state = build_state(false, Some(0));
    let markup = render_markup(&state);
    let telemetry: TelemetryLog = sample_log();
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>RusticUI Dioxus Speed Dial</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head><body><div id=\"speed-dial-root\" data-rustic-speed-dial=\"{channel}\">{markup}</div><pre data-rustic-analytics-log>{log}</pre></body></html>",
        channel = TELEMETRY_CHANNEL,
        markup = markup,
        log = htmlescape::encode_minimal(&telemetry.as_json_lines())
    )
}
