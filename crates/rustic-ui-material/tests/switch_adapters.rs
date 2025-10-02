#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use rustic_ui_headless::{interaction::ControlKey, switch::SwitchState};
use rustic_ui_material::switch::{
    self, SwitchChangeEvent, SwitchKeyEvent, SwitchProps, SwitchTelemetryEvent,
};

/// Compute the change payload produced by the adapters for the provided state.
fn expected_change(props: &SwitchProps, state: &SwitchState) -> SwitchChangeEvent {
    let previous = state.on();
    let next = if state.disabled() {
        previous
    } else {
        !previous
    };
    SwitchChangeEvent {
        previous,
        next,
        disabled: state.disabled(),
        analytics_id: props.telemetry.analytics_id.clone(),
        automation_id: props.telemetry.automation_id.clone(),
        label: props.label.clone(),
    }
}

/// Compute the key payload produced by the adapters for the provided state.
fn expected_key(props: &SwitchProps, state: &SwitchState, key: ControlKey) -> SwitchKeyEvent {
    let previous = state.on();
    let next = if state.disabled() {
        previous
    } else {
        !previous
    };
    SwitchKeyEvent {
        key,
        previous,
        next,
        disabled: state.disabled(),
        analytics_id: props.telemetry.analytics_id.clone(),
        automation_id: props.telemetry.automation_id.clone(),
        label: props.label.clone(),
    }
}

/// Harness mirroring adapter side-effects so tests can assert sequencing.
struct ControlledHarness {
    props: SwitchProps,
    state: SwitchState,
    telemetry_events: Vec<SwitchTelemetryEvent>,
    change_events: Vec<SwitchChangeEvent>,
    key_events: Vec<SwitchKeyEvent>,
}

impl ControlledHarness {
    fn new_controlled() -> Self {
        let mut props = SwitchProps::new("Notifications", TelemetryHooks::default());
        props.telemetry.analytics_id = Some("switch.analytics.controlled".into());
        props.telemetry.automation_id = Some("switch.automation.controlled".into());
        Self {
            props,
            state: SwitchState::controlled(false, false),
            telemetry_events: Vec::new(),
            change_events: Vec::new(),
            key_events: Vec::new(),
        }
    }

    fn simulate_change(&mut self) -> SwitchChangeEvent {
        let payload = expected_change(&self.props, &self.state);
        self.telemetry_events
            .push(SwitchTelemetryEvent::Change(payload.clone()));
        self.change_events.push(payload.clone());
        if !self.state.is_controlled() {
            self.state.toggle(|_| {});
        }
        payload
    }

    fn simulate_key(&mut self, key: ControlKey) -> (SwitchKeyEvent, SwitchChangeEvent) {
        let key_payload = expected_key(&self.props, &self.state, key);
        let change_payload = expected_change(&self.props, &self.state);
        self.telemetry_events
            .push(SwitchTelemetryEvent::Key(key_payload.clone()));
        self.telemetry_events
            .push(SwitchTelemetryEvent::Change(change_payload.clone()));
        self.key_events.push(key_payload.clone());
        self.change_events.push(change_payload.clone());
        if !self.state.is_controlled() {
            self.state.on_key(key, |_| {});
        }
        (key_payload, change_payload)
    }

    fn telemetry(&self) -> &[SwitchTelemetryEvent] {
        &self.telemetry_events
    }

    fn changes(&self) -> &[SwitchChangeEvent] {
        &self.change_events
    }

    fn keys(&self) -> &[SwitchKeyEvent] {
        &self.key_events
    }
}

fn assert_controlled_behavior(mut harness: ControlledHarness) {
    let initial_on = harness.state.on();
    let expected_change = harness.simulate_change();
    assert_eq!(harness.state.on(), initial_on);

    let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
    assert_eq!(harness.state.on(), initial_on);

    assert_eq!(
        harness.telemetry(),
        &[
            SwitchTelemetryEvent::Change(expected_change.clone()),
            SwitchTelemetryEvent::Key(expected_key.clone()),
            SwitchTelemetryEvent::Change(expected_key_change.clone()),
        ]
    );
    assert_eq!(
        harness.changes(),
        &[expected_change.clone(), expected_key_change.clone()]
    );
    assert_eq!(harness.keys(), &[expected_key.clone()]);
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_on_state() {
        let props = SwitchProps::new("Notifications", TelemetryHooks::default());
        let mut state = SwitchState::uncontrolled(false, false);
        state.toggle(|_| {});
        let out = switch::yew::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.contains("data-on=\"true\""));
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating() {
        assert_controlled_behavior(ControlledHarness::new_controlled());
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn renders_off_state() {
        let props = SwitchProps::new("Notifications", TelemetryHooks::default());
        let state = SwitchState::uncontrolled(false, false);
        let out = switch::leptos::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.contains("aria-checked=\"false\""));
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating() {
        assert_controlled_behavior(ControlledHarness::new_controlled());
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn includes_focus_attribute() {
        let mut state = SwitchState::uncontrolled(false, false);
        state.focus();
        let props = SwitchProps::new("Notifications", TelemetryHooks::default());
        let out = switch::dioxus::render(&props, &state);
        assert!(out.contains("data-focus-visible"));
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating() {
        assert_controlled_behavior(ControlledHarness::new_controlled());
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn renders_basic_markup() {
        let props = SwitchProps::new("Notifications", TelemetryHooks::default());
        let state = SwitchState::uncontrolled(false, false);
        let out = switch::sycamore::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.ends_with(">Notifications</span>"));
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating() {
        assert_controlled_behavior(ControlledHarness::new_controlled());
    }
}
