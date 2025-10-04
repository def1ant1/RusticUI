#![cfg(target_arch = "wasm32")]

use selection_controls_yew::{record_checkbox_change, record_radio_key, TelemetryRecorder};
use wasm_bindgen_test::*;

use rustic_ui_headless::checkbox::CheckboxValue;
use rustic_ui_material::checkbox::{CheckboxChangeEvent, CheckboxTelemetryEvent};
use rustic_ui_material::radio::{RadioKeyEvent, RadioTelemetryEvent};
use rustic_ui_material::telemetry::{instrument_render, TelemetryContext};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn checkbox_events_order_matches_browser_expectations() {
    let recorder = TelemetryRecorder::new();
    let channel = recorder.channel("checkbox.wasm");
    let hooks = channel.hooks();
    instrument_render(&hooks, TelemetryContext::new("wasm.checkbox"), || ());

    let change_event = CheckboxChangeEvent {
        previous: CheckboxValue::Off,
        next: CheckboxValue::On,
        disabled: false,
        analytics_id: Some(channel.analytics_id()),
        automation_id: Some(channel.automation_id()),
        label: "Browser checkbox".into(),
    };

    channel
        .checkbox_delegate()
        .emit(CheckboxTelemetryEvent::Change(change_event.clone()));
    record_checkbox_change(&channel, &change_event);

    let events = recorder.events();
    assert_eq!(events[0].phase, "render");
    assert_eq!(events[1].phase, "telemetry");
    assert_eq!(events[2].phase, "change-handler");
}

#[wasm_bindgen_test]
fn radio_keyboard_events_are_chronological() {
    let recorder = TelemetryRecorder::new();
    let channel = recorder.channel("radio.wasm");
    let hooks = channel.hooks();
    instrument_render(&hooks, TelemetryContext::new("wasm.radio"), || ());

    let key_event = RadioKeyEvent {
        key: rustic_ui_headless::interaction::ControlKey::ArrowDown,
        previous: Some(1),
        next: Some(2),
        disabled: false,
        analytics_id: Some(channel.analytics_id()),
        automation_id: Some(channel.automation_id()),
        label: "Browser radio".into(),
    };

    channel
        .radio_delegate()
        .emit(RadioTelemetryEvent::Key(key_event.clone()));
    record_radio_key(&channel, &key_event);

    let events = recorder.events();
    assert_eq!(events[0].phase, "render");
    assert_eq!(events[1].phase, "telemetry");
    assert_eq!(events[2].phase, "key");
}
