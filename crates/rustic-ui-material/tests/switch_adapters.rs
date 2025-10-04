#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use std::sync::{Arc, Mutex};

use rustic_ui_headless::{interaction::ControlKey, switch::SwitchState};
use rustic_ui_material::{
    switch::{
        self, capture_switch_render_snapshot, SwitchChangeEvent, SwitchKeyEvent, SwitchProps,
        SwitchRenderSnapshot, SwitchTelemetryEvent,
    },
    TelemetryContext, TelemetryHooks,
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

fn capture_plan_snapshot(
    component: &'static str,
    props: &SwitchProps,
    state: &SwitchState,
) -> SwitchRenderSnapshot {
    capture_switch_render_snapshot(component, props, state)
}

fn assert_markup_matches_plan(markup: &str, snapshot: &SwitchRenderSnapshot) {
    assert!(
        markup.contains(snapshot.attributes.label()),
        "rendered markup should include the switch label"
    );
    if let Some((_, class)) = snapshot
        .themed_attributes
        .iter()
        .find(|(key, _)| key == "class")
    {
        assert!(
            markup.contains(&format!("class=\"{class}\"")),
            "framework adapters must emit the themed class",
        );
    }
    if let Some(role) = snapshot.attributes.extra_attributes().get("role") {
        assert!(
            markup.contains(&format!("role=\"{role}\"")),
            "role attribute should propagate from the render plan",
        );
    }
    if let Some(value) = snapshot.attributes.aria_map().get("aria-checked") {
        assert!(
            markup.contains(&format!("aria-checked=\"{value}\"")),
            "ARIA state must mirror the descriptor snapshot",
        );
    }
    for key in [
        "data-on",
        "data-focus-visible",
        "data-component",
        "data-variant",
    ] {
        if let Some(value) = snapshot.attributes.data_map().get(key) {
            assert!(
                markup.contains(&format!("{key}=\"{value}\"")),
                "{key} should match the descriptor payload",
            );
        }
    }
}

fn assert_render_context(
    recorded: &Arc<Mutex<Vec<TelemetryContext>>>,
    expected: &TelemetryContext,
) {
    let contexts = recorded.lock().expect("telemetry context mutex poisoned");
    assert_eq!(
        contexts.as_slice(),
        &[expected.clone()],
        "adapters must instrument render telemetry with the plan context",
    );
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_on_state() {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some("telemetry.switch.yew".into());
        hooks.on_render = Some({
            let store = Arc::clone(&contexts);
            Arc::new(move |ctx: TelemetryContext| {
                store.lock().unwrap().push(ctx);
            })
        });
        let props = SwitchProps::new("Notifications", hooks);
        let mut state = SwitchState::uncontrolled(false, false);
        state.toggle(|_| {});

        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::yew::renders_on_state",
            &props,
            &state,
        );
        let out = switch::yew::render(&props, &state);
        assert_markup_matches_plan(&out, &snapshot);
        assert_render_context(&contexts, &snapshot.context);
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
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some("telemetry.switch.leptos".into());
        hooks.on_render = Some({
            let store = Arc::clone(&contexts);
            Arc::new(move |ctx: TelemetryContext| {
                store.lock().unwrap().push(ctx);
            })
        });
        let props = SwitchProps::new("Notifications", hooks);
        let state = SwitchState::uncontrolled(false, false);

        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::leptos::renders_off_state",
            &props,
            &state,
        );
        let out = switch::leptos::render(&props, &state);
        assert_markup_matches_plan(&out, &snapshot);
        assert_render_context(&contexts, &snapshot.context);
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
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some("telemetry.switch.dioxus".into());
        hooks.on_render = Some({
            let store = Arc::clone(&contexts);
            Arc::new(move |ctx: TelemetryContext| {
                store.lock().unwrap().push(ctx);
            })
        });
        let mut state = SwitchState::uncontrolled(false, false);
        state.focus();
        let props = SwitchProps::new("Notifications", hooks);

        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::dioxus::includes_focus_attribute",
            &props,
            &state,
        );
        let out = switch::dioxus::render(&props, &state);
        assert_markup_matches_plan(&out, &snapshot);
        assert_render_context(&contexts, &snapshot.context);
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
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some("telemetry.switch.sycamore".into());
        hooks.on_render = Some({
            let store = Arc::clone(&contexts);
            Arc::new(move |ctx: TelemetryContext| {
                store.lock().unwrap().push(ctx);
            })
        });
        let props = SwitchProps::new("Notifications", hooks);
        let state = SwitchState::uncontrolled(false, false);

        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::sycamore::renders_basic_markup",
            &props,
            &state,
        );
        let out = switch::sycamore::render(&props, &state);
        assert_markup_matches_plan(&out, &snapshot);
        assert_render_context(&contexts, &snapshot.context);
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating() {
        assert_controlled_behavior(ControlledHarness::new_controlled());
    }
}
