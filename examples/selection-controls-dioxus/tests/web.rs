#![cfg(target_arch = "wasm32")]

use selection_controls_dioxus::{build_virtual_dom, TelemetryRecorder, TelemetrySignal};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn web_hydration_collects_render_hooks() {
    let recorder = TelemetryRecorder::default();
    let mut dom = build_virtual_dom(recorder.clone());
    let _ = dom.rebuild();
    let signals = recorder.drain();
    let has_radio_component = signals.iter().any(|signal| {
        matches!(signal, TelemetrySignal::Render { channel, .. } if *channel == "radio.component")
    });
    assert!(
        has_radio_component,
        "radio component telemetry should hydrate in the browser"
    );
}
