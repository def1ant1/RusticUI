#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;

use leptos::prelude::*;
use selection_controls_leptos::{SelectionControlsDemo, TelemetryRecorder};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn initial_state_events_precede_render_on_hydration() {
    console_error_panic_hook::set_once();

    let recorder = TelemetryRecorder::new();
    let recorder_clone = recorder.clone();

    leptos::mount_to_body(
        move || view! { <SelectionControlsDemo recorder=Some(recorder_clone.clone())/> },
    );

    let mut first_phase: HashMap<String, String> = HashMap::new();
    for event in recorder.events() {
        first_phase
            .entry(event.channel.clone())
            .or_insert(event.phase.clone());
        if event.phase == "render" {
            assert_eq!(
                first_phase.get(&event.channel),
                Some(&"initial-state".to_string())
            );
        }
    }
}

#[wasm_bindgen_test]
fn automation_attributes_exist_in_dom() {
    console_error_panic_hook::set_once();

    let recorder = TelemetryRecorder::new();
    let recorder_clone = recorder.clone();

    leptos::mount_to_body(
        move || view! { <SelectionControlsDemo recorder=Some(recorder_clone.clone())/> },
    );

    let document = web_sys::window().unwrap().document().unwrap();
    let selector = "[data-automation-id^=automation.selection-controls]";
    let matches = document.query_selector_all(selector).unwrap();
    assert!(
        matches.length() > 0,
        "expected automation attributes to render"
    );

    // Ensure telemetry events were emitted in order for at least one channel.
    let events = recorder.events();
    let checkbox_events: Vec<_> = events
        .iter()
        .filter(|event| event.channel == "checkbox.controlled")
        .map(|event| event.phase.clone())
        .collect();
    assert!(checkbox_events
        .windows(2)
        .any(|window| window == ["initial-state", "render"]));
}
