use selection_controls_dioxus::{
    build_virtual_dom, simulate_telemetry_cycle, TelemetryRecorder, TelemetrySignal,
};

#[test]
fn hydration_order_matches_render_sequence() {
    let recorder = TelemetryRecorder::default();
    let mut dom = build_virtual_dom(recorder.clone());
    let _ = dom.rebuild();
    let signals = recorder.drain();
    let render_channels: Vec<&'static str> = signals
        .iter()
        .filter_map(|signal| match signal {
            TelemetrySignal::Render { channel, .. } => Some(*channel),
            _ => None,
        })
        .collect();
    assert_eq!(
        render_channels,
        vec!["checkbox", "switch", "radio", "radio.component"],
        "render telemetry should follow the README component order",
    );
}

#[test]
fn telemetry_cycle_emits_structured_events() {
    let recorder = TelemetryRecorder::default();
    let signals = simulate_telemetry_cycle(recorder);
    let mut console_channels = Vec::new();
    let mut has_checkbox = false;
    let mut has_switch = false;
    let mut has_radio = false;

    for signal in &signals {
        match signal {
            TelemetrySignal::Console { channel, .. } => console_channels.push(*channel),
            TelemetrySignal::Checkbox(event) => {
                has_checkbox = true;
                assert!(matches!(
                    event,
                    rustic_ui_material::checkbox::CheckboxTelemetryEvent::Change(_)
                ));
            }
            TelemetrySignal::Switch(event) => {
                has_switch = true;
                assert!(matches!(
                    event,
                    rustic_ui_material::switch::SwitchTelemetryEvent::Change(_)
                ));
            }
            TelemetrySignal::Radio(event) => {
                has_radio = true;
                assert!(matches!(
                    event,
                    rustic_ui_material::radio::RadioTelemetryEvent::Change(_)
                ));
            }
            _ => {}
        }
    }

    assert_eq!(
        console_channels,
        vec!["checkbox", "switch", "radio"],
        "console telemetry should be emitted for each control",
    );
    assert!(has_checkbox && has_switch && has_radio);
}
