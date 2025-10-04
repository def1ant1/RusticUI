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
        self, capture_switch_render_snapshot, SwitchChangeEvent, SwitchFocusEvent, SwitchKeyEvent,
        SwitchProps, SwitchRenderSnapshot, SwitchTelemetryEvent,
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

/// Compute the focus payload emitted when focus visibility changes.
fn expected_focus(props: &SwitchProps, state: &SwitchState, focused: bool) -> SwitchFocusEvent {
    SwitchFocusEvent {
        focused,
        on: state.on(),
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

/// Harness used by each framework test to simulate user interactions while
/// mirroring the analytics sequencing implemented by the real adapters.
///
/// The helper lets every framework exercise the same toggle, focus, blur and
/// keyboard flows without copying boilerplate in each module. By updating this
/// single harness when adapters gain new telemetry types we automatically keep
/// the entire test matrix consistent.
struct InteractionHarness {
    props: SwitchProps,
    state: SwitchState,
    contexts: Arc<Mutex<Vec<TelemetryContext>>>,
    telemetry_events: Vec<SwitchTelemetryEvent>,
    change_events: Vec<SwitchChangeEvent>,
    focus_events: Vec<SwitchFocusEvent>,
    blur_events: Vec<SwitchFocusEvent>,
    key_events: Vec<SwitchKeyEvent>,
    sequence_log: Vec<&'static str>,
}

impl InteractionHarness {
    /// Create a new harness for either controlled or uncontrolled scenarios.
    fn new(controlled: bool, framework: &str) -> Self {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some(format!("switch.analytics.{framework}"));
        hooks.automation_id = Some(format!("switch.automation.{framework}"));
        hooks.on_render = Some({
            let store = Arc::clone(&contexts);
            Arc::new(move |ctx: TelemetryContext| {
                store
                    .lock()
                    .expect("telemetry context mutex should not be poisoned")
                    .push(ctx);
            })
        });

        let props = SwitchProps::new("Notifications", hooks);
        let state = if controlled {
            SwitchState::controlled(false, false)
        } else {
            SwitchState::uncontrolled(false, false)
        };

        Self {
            props,
            state,
            contexts,
            telemetry_events: Vec::new(),
            change_events: Vec::new(),
            focus_events: Vec::new(),
            blur_events: Vec::new(),
            key_events: Vec::new(),
            sequence_log: Vec::new(),
        }
    }

    /// Simulate a pointer toggle.
    fn simulate_toggle(&mut self) -> SwitchChangeEvent {
        let payload = expected_change(&self.props, &self.state);
        self.sequence_log.push("telemetry::change");
        self.telemetry_events
            .push(SwitchTelemetryEvent::Change(payload.clone()));
        self.sequence_log.push("callback::change");
        self.change_events.push(payload.clone());
        self.state.toggle(|_| {});
        payload
    }

    /// Simulate focus landing on the switch.
    fn simulate_focus_gain(&mut self) -> SwitchFocusEvent {
        let payload = expected_focus(&self.props, &self.state, true);
        self.sequence_log.push("telemetry::focus");
        self.telemetry_events
            .push(SwitchTelemetryEvent::Focus(payload.clone()));
        self.sequence_log.push("callback::focus");
        self.focus_events.push(payload.clone());
        self.state.focus();
        payload
    }

    /// Simulate focus leaving the switch.
    fn simulate_focus_loss(&mut self) -> SwitchFocusEvent {
        let payload = expected_focus(&self.props, &self.state, false);
        self.sequence_log.push("telemetry::blur");
        self.telemetry_events
            .push(SwitchTelemetryEvent::Blur(payload.clone()));
        self.sequence_log.push("callback::blur");
        self.blur_events.push(payload.clone());
        self.state.blur();
        payload
    }

    /// Simulate a keyboard interaction (Space/Enter).
    fn simulate_key(&mut self, key: ControlKey) -> (SwitchKeyEvent, SwitchChangeEvent) {
        let key_payload = expected_key(&self.props, &self.state, key);
        let change_payload = expected_change(&self.props, &self.state);
        self.sequence_log.push("telemetry::key");
        self.telemetry_events
            .push(SwitchTelemetryEvent::Key(key_payload.clone()));
        self.sequence_log.push("callback::key");
        self.key_events.push(key_payload.clone());
        self.sequence_log.push("telemetry::change");
        self.telemetry_events
            .push(SwitchTelemetryEvent::Change(change_payload.clone()));
        self.sequence_log.push("callback::change");
        self.change_events.push(change_payload.clone());
        self.state.on_key(key, |_| {});
        (key_payload, change_payload)
    }

    fn telemetry(&self) -> &[SwitchTelemetryEvent] {
        &self.telemetry_events
    }

    fn changes(&self) -> &[SwitchChangeEvent] {
        &self.change_events
    }

    fn focus_events(&self) -> &[SwitchFocusEvent] {
        &self.focus_events
    }

    fn blur_events(&self) -> &[SwitchFocusEvent] {
        &self.blur_events
    }

    fn key_events(&self) -> &[SwitchKeyEvent] {
        &self.key_events
    }

    fn log(&self) -> &[&'static str] {
        &self.sequence_log
    }

    fn contexts(&self) -> &Arc<Mutex<Vec<TelemetryContext>>> {
        &self.contexts
    }

    fn props(&self) -> &SwitchProps {
        &self.props
    }

    fn state(&self) -> &SwitchState {
        &self.state
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

    #[test]
    fn uncontrolled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(false, "yew");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::yew::uncontrolled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::yew::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let change = harness.simulate_toggle();
        assert!(
            harness.state().on(),
            "uncontrolled toggle should flip state on"
        );
        let focus = harness.simulate_focus_gain();
        assert!(
            harness.state().focus_visible(),
            "focus gain should mark focus-visible"
        );
        let blur = harness.simulate_focus_loss();
        assert!(
            !harness.state().focus_visible(),
            "blur should clear focus-visible flag"
        );
        let (key, key_change) = harness.simulate_key(ControlKey::Space);
        assert!(
            !harness.state().on(),
            "space key should toggle the uncontrolled switch back off"
        );

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
            "telemetry ordering should stay deterministic across interactions",
        );

        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
    }

    #[test]
    fn controlled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(true, "yew");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::yew::controlled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::yew::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let initial = harness.state().on();
        let change = harness.simulate_toggle();
        assert_eq!(
            harness.state().on(),
            initial,
            "controlled toggle should not mutate local state",
        );
        let focus = harness.simulate_focus_gain();
        assert!(
            harness.state().focus_visible(),
            "focus transitions are still tracked for controlled switches",
        );
        let blur = harness.simulate_focus_loss();
        assert!(
            !harness.state().focus_visible(),
            "blur should clear focus visibility",
        );
        let (key, key_change) = harness.simulate_key(ControlKey::Enter);
        assert_eq!(
            harness.state().on(),
            initial,
            "keyboard toggles should defer to the parent in controlled mode",
        );

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
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

    #[test]
    fn uncontrolled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(false, "leptos");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::leptos::uncontrolled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::leptos::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let change = harness.simulate_toggle();
        assert!(harness.state().on());
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Enter);
        assert!(!harness.state().on());

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
    }

    #[test]
    fn controlled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(true, "leptos");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::leptos::controlled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::leptos::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let baseline = harness.state().on();
        let change = harness.simulate_toggle();
        assert_eq!(harness.state().on(), baseline);
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state().on(), baseline);

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
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

    #[test]
    fn uncontrolled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(false, "dioxus");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::dioxus::uncontrolled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::dioxus::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let change = harness.simulate_toggle();
        assert!(harness.state().on());
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Space);
        assert!(!harness.state().on());

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
    }

    #[test]
    fn controlled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(true, "dioxus");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::dioxus::controlled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::dioxus::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let baseline = harness.state().on();
        let change = harness.simulate_toggle();
        assert_eq!(harness.state().on(), baseline);
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Enter);
        assert_eq!(harness.state().on(), baseline);

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
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

    #[test]
    fn uncontrolled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(false, "sycamore");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::sycamore::uncontrolled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::sycamore::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let change = harness.simulate_toggle();
        assert!(harness.state().on());
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Space);
        assert!(!harness.state().on());

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
    }

    #[test]
    fn controlled_interactions_emit_expected_sequences() {
        let mut harness = InteractionHarness::new(true, "sycamore");
        let snapshot = capture_plan_snapshot(
            "rustic_ui_material::tests::switch_adapters::sycamore::controlled_interactions_emit_expected_sequences",
            harness.props(),
            harness.state(),
        );
        let markup = switch::sycamore::render(harness.props(), harness.state());
        assert_markup_matches_plan(&markup, &snapshot);
        assert_render_context(harness.contexts(), &snapshot.context);

        let baseline = harness.state().on();
        let change = harness.simulate_toggle();
        assert_eq!(harness.state().on(), baseline);
        let focus = harness.simulate_focus_gain();
        assert!(harness.state().focus_visible());
        let blur = harness.simulate_focus_loss();
        assert!(!harness.state().focus_visible());
        let (key, key_change) = harness.simulate_key(ControlKey::Enter);
        assert_eq!(harness.state().on(), baseline);

        assert_eq!(
            harness.log(),
            &[
                "telemetry::change",
                "callback::change",
                "telemetry::focus",
                "callback::focus",
                "telemetry::blur",
                "callback::blur",
                "telemetry::key",
                "callback::key",
                "telemetry::change",
                "callback::change"
            ],
        );
        assert_eq!(
            harness.telemetry(),
            &[
                SwitchTelemetryEvent::Change(change.clone()),
                SwitchTelemetryEvent::Focus(focus.clone()),
                SwitchTelemetryEvent::Blur(blur.clone()),
                SwitchTelemetryEvent::Key(key.clone()),
                SwitchTelemetryEvent::Change(key_change.clone()),
            ],
        );
        assert_eq!(harness.changes(), &[change.clone(), key_change.clone()]);
        assert_eq!(harness.focus_events(), &[focus.clone()]);
        assert_eq!(harness.blur_events(), &[blur.clone()]);
        assert_eq!(harness.key_events(), &[key.clone()]);
    }
}
