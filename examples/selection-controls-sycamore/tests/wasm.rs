#![cfg(target_arch = "wasm32")]

use rustic_ui_headless::checkbox::CheckboxValue;
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_material::checkbox::{CheckboxChangeEvent, CheckboxTelemetryEvent};
use rustic_ui_material::radio::{RadioKeyEvent, RadioTelemetryEvent};
use rustic_ui_material::telemetry::{instrument_render, TelemetryContext};
use selection_controls_sycamore::{
    record_checkbox_change, record_radio_key, SelectionControlsTelemetry, TelemetryRecorder,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn checkbox_events_emit_in_browser_order() {
    let recorder = TelemetryRecorder::new();
    let telemetry = SelectionControlsTelemetry::new(&recorder);
    let hooks = telemetry.checkbox.hooks();
    instrument_render(&hooks, TelemetryContext::new("wasm.checkbox"), || {});

    let change_event = CheckboxChangeEvent {
        previous: CheckboxValue::Off,
        next: CheckboxValue::On,
        disabled: false,
        analytics_id: Some(telemetry.checkbox.analytics_id()),
        automation_id: Some(telemetry.checkbox.automation_id()),
        label: "Browser checkbox".into(),
    };

    telemetry.checkbox.checkbox_delegate()(CheckboxTelemetryEvent::Change(change_event.clone()));
    record_checkbox_change(&telemetry.checkbox, &change_event);

    let events = recorder.events();
    assert_eq!(events[0].phase, "render");
    assert_eq!(events[1].phase, "telemetry");
    assert_eq!(events[2].phase, "change-handler");
}

#[wasm_bindgen_test]
fn radio_keyboard_events_are_sequenced() {
    let recorder = TelemetryRecorder::new();
    let telemetry = SelectionControlsTelemetry::new(&recorder);
    let hooks = telemetry.radio_component.hooks();
    instrument_render(&hooks, TelemetryContext::new("wasm.radio"), || {});

    let key_event = RadioKeyEvent {
        key: ControlKey::ArrowRight,
        previous: Some(0),
        next: Some(1),
        disabled: false,
        analytics_id: Some(telemetry.radio_component.analytics_id()),
        automation_id: Some(telemetry.radio_component.automation_id()),
        label: "Browser radio".into(),
    };

    telemetry.radio_component.radio_delegate()(RadioTelemetryEvent::Key(key_event.clone()));
    record_radio_key(&telemetry.radio_component, &key_event);

    let events = recorder.events();
    assert_eq!(events[0].phase, "render");
    assert_eq!(events[1].phase, "telemetry");
    assert_eq!(events[2].phase, "key");
}
