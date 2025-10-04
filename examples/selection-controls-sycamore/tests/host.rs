#![cfg(not(target_arch = "wasm32"))]

use selection_controls_sycamore::{render_ssr, SelectionControlsProps, TelemetryRecorder};

fn index_of_phase<'a>(
    events: &'a [selection_controls_sycamore::RecordedEvent],
    phase: &str,
    channel: &str,
) -> usize {
    events
        .iter()
        .position(|event| event.channel == channel && event.phase == phase)
        .unwrap_or_else(|| {
            panic!(
                "expected {phase} event for channel {channel}",
                phase = phase,
                channel = channel
            )
        })
}

#[test]
fn ssr_markup_is_stable_and_events_are_ordered() {
    let recorder = TelemetryRecorder::new();
    let props = SelectionControlsProps::enterprise_defaults(&recorder);

    let first = render_ssr(props.clone());
    let second = render_ssr(props.clone());
    assert_eq!(
        first, second,
        "SSR rendering should be deterministic for hydration"
    );

    props.simulate_nominal_cycle();

    let events = recorder.events();
    assert!(
        events.len() >= 12,
        "expected telemetry events to be recorded"
    );

    // Ensure each control recorded render telemetry first.
    assert!(events.iter().take(4).all(|event| event.phase == "render"));

    // Telemetry delegates must precede change handlers to keep automation deterministic.
    let checkbox_telemetry = index_of_phase(&events, "telemetry", "checkbox");
    let checkbox_change = index_of_phase(&events, "change-handler", "checkbox");
    assert!(checkbox_telemetry < checkbox_change);

    let switch_telemetry = index_of_phase(&events, "telemetry", "switch");
    let switch_change = index_of_phase(&events, "change-handler", "switch");
    assert!(switch_telemetry < switch_change);

    let radio_telemetry = index_of_phase(&events, "telemetry", "radio.component");
    let radio_change = index_of_phase(&events, "change-handler", "radio.component");
    assert!(radio_telemetry < radio_change);

    // Focus transitions should include both focus and blur events per control.
    let focus_counts = |channel: &str| {
        let focus = events
            .iter()
            .filter(|event| event.channel == channel && event.phase == "focus")
            .count();
        let blur = events
            .iter()
            .filter(|event| event.channel == channel && event.phase == "blur")
            .count();
        (focus, blur)
    };
    let (checkbox_focus, checkbox_blur) = focus_counts("checkbox");
    assert_eq!(checkbox_focus, 1);
    assert_eq!(checkbox_blur, 1);
    let (switch_focus, switch_blur) = focus_counts("switch");
    assert_eq!(switch_focus, 1);
    assert_eq!(switch_blur, 1);
    let (radio_focus, radio_blur) = focus_counts("radio.component");
    assert_eq!(radio_focus, 1);
    assert_eq!(radio_blur, 1);
}
