#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use std::sync::{Arc, Mutex};

use rustic_ui_headless::checkbox::{CheckboxState, CheckboxValue};
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_material::checkbox::{
    self, CheckboxChangeEvent, CheckboxFocusEvent, CheckboxKeyEvent, CheckboxProps,
    CheckboxTelemetryEvent,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};

/// Toggle helper mirroring the internal adapter logic so tests can reason about
/// the expected `next` value without duplicating the full render pipeline.
fn toggled_value(current: CheckboxValue) -> CheckboxValue {
    match current {
        CheckboxValue::Off => CheckboxValue::On,
        CheckboxValue::On => CheckboxValue::Off,
        CheckboxValue::Indeterminate => CheckboxValue::On,
    }
}

/// Assemble the deterministic change payload emitted by adapters.
fn expected_change(props: &CheckboxProps, state: &CheckboxState) -> CheckboxChangeEvent {
    let previous = state.checked();
    let next = if state.disabled() {
        previous
    } else {
        toggled_value(previous)
    };
    CheckboxChangeEvent {
        previous,
        next,
        disabled: state.disabled(),
        analytics_id: props.telemetry.analytics_id.clone(),
        automation_id: props.telemetry.automation_id.clone(),
        label: props.label.clone(),
    }
}

/// Build the focus payload for either focus or blur transitions.
fn expected_focus(
    props: &CheckboxProps,
    state: &CheckboxState,
    focused: bool,
) -> CheckboxFocusEvent {
    CheckboxFocusEvent {
        focused,
        checked: state.checked(),
        disabled: state.disabled(),
        analytics_id: props.telemetry.analytics_id.clone(),
        automation_id: props.telemetry.automation_id.clone(),
        label: props.label.clone(),
    }
}

/// Build the key payload emitted when the checkbox handles keyboard input.
fn expected_key(props: &CheckboxProps, state: &CheckboxState, key: ControlKey) -> CheckboxKeyEvent {
    let previous = state.checked();
    let next = if state.disabled() {
        previous
    } else {
        toggled_value(previous)
    };
    CheckboxKeyEvent {
        key,
        previous,
        next,
        disabled: state.disabled(),
        analytics_id: props.telemetry.analytics_id.clone(),
        automation_id: props.telemetry.automation_id.clone(),
        label: props.label.clone(),
    }
}

/// Configure telemetry hooks that record render lifecycles into the supplied
/// buffer.  The helper keeps individual framework tests concise while ensuring
/// analytics/automation identifiers are propagated consistently.
fn instrumented_hooks(
    analytics: &str,
    automation: &str,
    storage: &Arc<Mutex<Vec<TelemetryContext>>>,
) -> TelemetryHooks {
    let mut telemetry = TelemetryHooks::default();
    telemetry.analytics_id = Some(analytics.to_string());
    telemetry.automation_id = Some(automation.to_string());
    telemetry.on_render = Some({
        let store = Arc::clone(storage);
        Arc::new(move |ctx: TelemetryContext| {
            store.lock().unwrap().push(ctx);
        })
    });
    telemetry
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    struct YewHarness {
        props: CheckboxProps,
        state: CheckboxState,
        contexts: Arc<Mutex<Vec<TelemetryContext>>>,
        telemetry_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>>,
        change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>>,
        focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>>,
        telemetry_delegate: yew::Callback<CheckboxTelemetryEvent>,
        on_change: yew::Callback<CheckboxChangeEvent>,
        on_focus: yew::Callback<CheckboxFocusEvent>,
        on_blur: yew::Callback<CheckboxFocusEvent>,
        on_key: yew::Callback<CheckboxKeyEvent>,
    }

    impl YewHarness {
        fn new() -> Self {
            Self::new_with_state(CheckboxState::uncontrolled(false, false))
        }

        fn new_controlled() -> Self {
            Self::new_with_state(CheckboxState::controlled(false, false))
        }

        fn new_with_state(state: CheckboxState) -> Self {
            let contexts = Arc::new(Mutex::new(Vec::new()));
            let mut props = CheckboxProps::new("Marketing opt-in");
            props.telemetry = instrumented_hooks(
                "analytics::checkbox::yew",
                "automation::checkbox::yew",
                &contexts,
            );
            let telemetry_events = Rc::new(RefCell::new(Vec::new()));
            let telemetry_delegate = {
                let events = Rc::clone(&telemetry_events);
                yew::Callback::from(move |event: CheckboxTelemetryEvent| {
                    events.borrow_mut().push(event);
                })
            };

            let change_events = Rc::new(RefCell::new(Vec::new()));
            let on_change = {
                let events = Rc::clone(&change_events);
                yew::Callback::from(move |event: CheckboxChangeEvent| {
                    events.borrow_mut().push(event);
                })
            };

            let focus_events = Rc::new(RefCell::new(Vec::new()));
            let on_focus = {
                let events = Rc::clone(&focus_events);
                yew::Callback::from(move |event: CheckboxFocusEvent| {
                    events.borrow_mut().push(event);
                })
            };

            let blur_events = Rc::new(RefCell::new(Vec::new()));
            let on_blur = {
                let events = Rc::clone(&blur_events);
                yew::Callback::from(move |event: CheckboxFocusEvent| {
                    events.borrow_mut().push(event);
                })
            };

            let key_events = Rc::new(RefCell::new(Vec::new()));
            let on_key = {
                let events = Rc::clone(&key_events);
                yew::Callback::from(move |event: CheckboxKeyEvent| {
                    events.borrow_mut().push(event);
                })
            };

            let component_props = checkbox::yew::YewCheckboxProps {
                checkbox: props.clone(),
                state: state.clone(),
                on_change: Some(on_change.clone()),
                on_focus: Some(on_focus.clone()),
                on_blur: Some(on_blur.clone()),
                on_key: Some(on_key.clone()),
                telemetry_delegate: Some(telemetry_delegate.clone()),
            };

            let rendered = checkbox::yew::yew_checkbox(&component_props);
            let markup = rendered.to_string();
            assert!(markup.contains("role=\"checkbox\""));
            assert!(markup.contains("aria-checked"));
            assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::yew\""));
            assert!(markup.contains("data-automation-id=\"automation::checkbox::yew\""));

            Self {
                props,
                state,
                contexts,
                telemetry_events,
                change_events,
                focus_events,
                blur_events,
                key_events,
                telemetry_delegate,
                on_change,
                on_focus,
                on_blur,
                on_key,
            }
        }

        fn simulate_change(&mut self) -> CheckboxChangeEvent {
            let payload = expected_change(&self.props, &self.state);
            self.telemetry_delegate
                .emit(CheckboxTelemetryEvent::Change(payload.clone()));
            self.on_change.emit(payload.clone());
            if !self.state.is_controlled() {
                self.state.toggle(|_| {});
            }
            payload
        }

        fn simulate_focus(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, true);
            self.telemetry_delegate
                .emit(CheckboxTelemetryEvent::Focus(payload.clone()));
            self.on_focus.emit(payload.clone());
            self.state.focus();
            payload
        }

        fn simulate_blur(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, false);
            self.telemetry_delegate
                .emit(CheckboxTelemetryEvent::Blur(payload.clone()));
            self.on_blur.emit(payload.clone());
            self.state.blur();
            payload
        }

        fn simulate_key(&mut self, key: ControlKey) -> (CheckboxKeyEvent, CheckboxChangeEvent) {
            let key_payload = expected_key(&self.props, &self.state, key);
            let change_payload = expected_change(&self.props, &self.state);
            self.telemetry_delegate
                .emit(CheckboxTelemetryEvent::Key(key_payload.clone()));
            self.telemetry_delegate
                .emit(CheckboxTelemetryEvent::Change(change_payload.clone()));
            self.on_key.emit(key_payload.clone());
            self.on_change.emit(change_payload.clone());
            if !self.state.is_controlled() {
                self.state.on_key(key, |_| {});
            }
            (key_payload, change_payload)
        }
    }

    #[test]
    fn telemetry_and_state_transitions_follow_adapter_contract() {
        let mut harness = YewHarness::new();

        let recorded = harness.contexts.lock().unwrap();
        assert_eq!(recorded.len(), 1);
        let context = &recorded[0];
        assert!(context.component.contains("checkbox"));
        assert_eq!(
            context.analytics_id.as_deref(),
            Some("analytics::checkbox::yew")
        );
        assert_eq!(
            context.automation_id.as_deref(),
            Some("automation::checkbox::yew")
        );
        drop(recorded);

        let expected_focus_gain = harness.simulate_focus();
        assert!(harness.state.focus_visible());

        let expected_change = harness.simulate_change();
        assert_eq!(harness.state.checked(), CheckboxValue::On);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), CheckboxValue::Off);

        let expected_focus_loss = harness.simulate_blur();
        assert!(!harness.state.focus_visible());

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 5);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Focus(expected_focus_gain.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[3],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        assert_eq!(
            telemetry[4],
            CheckboxTelemetryEvent::Blur(expected_focus_loss.clone())
        );

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
        drop(changes);

        let focus_events = harness.focus_events.borrow();
        assert_eq!(focus_events.len(), 1);
        assert_eq!(focus_events[0], expected_focus_gain);
        drop(focus_events);

        let blur_events = harness.blur_events.borrow();
        assert_eq!(blur_events.len(), 1);
        assert_eq!(blur_events[0], expected_focus_loss);
        drop(blur_events);

        let key_events = harness.key_events.borrow();
        assert_eq!(key_events.len(), 1);
        assert_eq!(key_events[0], expected_key);
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating_local_snapshot() {
        let mut harness = YewHarness::new_controlled();
        let initial_value = harness.state.checked();

        let expected_change = expected_change(&harness.props, &harness.state);
        let change = harness.simulate_change();
        assert_eq!(change, expected_change);
        assert_eq!(harness.state.checked(), initial_value);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), initial_value);

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 3);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        drop(telemetry);

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    struct LeptosHarness {
        props: CheckboxProps,
        state: CheckboxState,
        contexts: Arc<Mutex<Vec<TelemetryContext>>>,
        telemetry_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>>,
        change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>>,
        focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>>,
        telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)>,
        on_change: Rc<dyn Fn(CheckboxChangeEvent)>,
        on_focus: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_blur: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_key: Rc<dyn Fn(CheckboxKeyEvent)>,
    }

    impl LeptosHarness {
        fn new() -> Self {
            Self::new_with_state(CheckboxState::uncontrolled(false, false))
        }

        fn new_controlled() -> Self {
            Self::new_with_state(CheckboxState::controlled(false, false))
        }

        fn new_with_state(state: CheckboxState) -> Self {
            let contexts = Arc::new(Mutex::new(Vec::new()));
            let mut props = CheckboxProps::new("Release automation");
            props.telemetry = instrumented_hooks(
                "analytics::checkbox::leptos",
                "automation::checkbox::leptos",
                &contexts,
            );
            let telemetry_events = Rc::new(RefCell::new(Vec::new()));
            let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
                let events = Rc::clone(&telemetry_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let change_events = Rc::new(RefCell::new(Vec::new()));
            let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
                let events = Rc::clone(&change_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let focus_events = Rc::new(RefCell::new(Vec::new()));
            let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&focus_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let blur_events = Rc::new(RefCell::new(Vec::new()));
            let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&blur_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let key_events = Rc::new(RefCell::new(Vec::new()));
            let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
                let events = Rc::clone(&key_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let leptos_props = checkbox::leptos::LeptosCheckboxProps {
                checkbox: props.clone(),
                state: state.clone(),
                on_change: Some(on_change.clone()),
                on_focus: Some(on_focus.clone()),
                on_blur: Some(on_blur.clone()),
                on_key: Some(on_key.clone()),
                telemetry_delegate: Some(telemetry_delegate.clone()),
            };

            let html = leptos::ssr::render_to_string({
                let props = leptos_props.clone();
                move || checkbox::leptos::LeptosCheckbox(props.clone())
            });
            let markup = html.to_string();
            assert!(markup.contains("role=\"checkbox\""));
            assert!(markup.contains("aria-checked"));
            assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::leptos\""));
            assert!(markup.contains("data-automation-id=\"automation::checkbox::leptos\""));

            Self {
                props,
                state,
                contexts,
                telemetry_events,
                change_events,
                focus_events,
                blur_events,
                key_events,
                telemetry_delegate,
                on_change,
                on_focus,
                on_blur,
                on_key,
            }
        }

        fn simulate_change(&mut self) -> CheckboxChangeEvent {
            let payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(payload.clone()));
            (self.on_change)(payload.clone());
            if !self.state.is_controlled() {
                self.state.toggle(|_| {});
            }
            payload
        }

        fn simulate_focus(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, true);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Focus(payload.clone()));
            (self.on_focus)(payload.clone());
            self.state.focus();
            payload
        }

        fn simulate_blur(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, false);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Blur(payload.clone()));
            (self.on_blur)(payload.clone());
            self.state.blur();
            payload
        }

        fn simulate_key(&mut self, key: ControlKey) -> (CheckboxKeyEvent, CheckboxChangeEvent) {
            let key_payload = expected_key(&self.props, &self.state, key);
            let change_payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Key(key_payload.clone()));
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(change_payload.clone()));
            (self.on_key)(key_payload.clone());
            (self.on_change)(change_payload.clone());
            if !self.state.is_controlled() {
                self.state.on_key(key, |_| {});
            }
            (key_payload, change_payload)
        }
    }

    #[test]
    fn telemetry_and_state_transitions_follow_adapter_contract() {
        let mut harness = LeptosHarness::new();

        let recorded = harness.contexts.lock().unwrap();
        assert_eq!(recorded.len(), 1);
        let context = &recorded[0];
        assert!(context.component.contains("checkbox"));
        assert_eq!(
            context.analytics_id.as_deref(),
            Some("analytics::checkbox::leptos")
        );
        assert_eq!(
            context.automation_id.as_deref(),
            Some("automation::checkbox::leptos")
        );
        drop(recorded);

        let expected_focus_gain = harness.simulate_focus();
        assert!(harness.state.focus_visible());

        let expected_change = harness.simulate_change();
        assert_eq!(harness.state.checked(), CheckboxValue::On);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Enter);
        assert_eq!(harness.state.checked(), CheckboxValue::Off);

        let expected_focus_loss = harness.simulate_blur();
        assert!(!harness.state.focus_visible());

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 5);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Focus(expected_focus_gain.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[3],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        assert_eq!(
            telemetry[4],
            CheckboxTelemetryEvent::Blur(expected_focus_loss.clone())
        );

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
        drop(changes);

        let focus_events = harness.focus_events.borrow();
        assert_eq!(focus_events.len(), 1);
        assert_eq!(focus_events[0], expected_focus_gain);
        drop(focus_events);

        let blur_events = harness.blur_events.borrow();
        assert_eq!(blur_events.len(), 1);
        assert_eq!(blur_events[0], expected_focus_loss);
        drop(blur_events);

        let key_events = harness.key_events.borrow();
        assert_eq!(key_events.len(), 1);
        assert_eq!(key_events[0], expected_key);
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating_local_snapshot() {
        let mut harness = LeptosHarness::new_controlled();
        let initial_value = harness.state.checked();

        let expected_change = expected_change(&harness.props, &harness.state);
        let change = harness.simulate_change();
        assert_eq!(change, expected_change);
        assert_eq!(harness.state.checked(), initial_value);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), initial_value);

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 3);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        drop(telemetry);

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use dioxus::prelude::VirtualDom;
    use std::{cell::RefCell, rc::Rc};

    struct DioxusHarness {
        props: CheckboxProps,
        state: CheckboxState,
        contexts: Arc<Mutex<Vec<TelemetryContext>>>,
        telemetry_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>>,
        change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>>,
        focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>>,
        telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)>,
        on_change: Rc<dyn Fn(CheckboxChangeEvent)>,
        on_focus: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_blur: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_key: Rc<dyn Fn(CheckboxKeyEvent)>,
    }

    impl DioxusHarness {
        fn new() -> Self {
            Self::new_with_state(CheckboxState::uncontrolled(false, false))
        }

        fn new_controlled() -> Self {
            Self::new_with_state(CheckboxState::controlled(false, false))
        }

        fn new_with_state(state: CheckboxState) -> Self {
            let contexts = Arc::new(Mutex::new(Vec::new()));
            let mut props = CheckboxProps::new("Nightly deployments");
            props.telemetry = instrumented_hooks(
                "analytics::checkbox::dioxus",
                "automation::checkbox::dioxus",
                &contexts,
            );

            let telemetry_events = Rc::new(RefCell::new(Vec::new()));
            let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
                let events = Rc::clone(&telemetry_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let change_events = Rc::new(RefCell::new(Vec::new()));
            let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
                let events = Rc::clone(&change_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let focus_events = Rc::new(RefCell::new(Vec::new()));
            let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&focus_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let blur_events = Rc::new(RefCell::new(Vec::new()));
            let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&blur_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let key_events = Rc::new(RefCell::new(Vec::new()));
            let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
                let events = Rc::clone(&key_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let dioxus_props = checkbox::dioxus::DioxusCheckboxProps {
                checkbox: props.clone(),
                state: state.clone(),
                on_change: Some(on_change.clone()),
                on_focus: Some(on_focus.clone()),
                on_blur: Some(on_blur.clone()),
                on_key: Some(on_key.clone()),
                telemetry_delegate: Some(telemetry_delegate.clone()),
            };

            let mut dom =
                VirtualDom::new_with_props(checkbox::dioxus::DioxusCheckbox, dioxus_props);
            dom.rebuild();

            Self {
                props,
                state,
                contexts,
                telemetry_events,
                change_events,
                focus_events,
                blur_events,
                key_events,
                telemetry_delegate,
                on_change,
                on_focus,
                on_blur,
                on_key,
            }
        }

        fn simulate_change(&mut self) -> CheckboxChangeEvent {
            let payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(payload.clone()));
            (self.on_change)(payload.clone());
            if !self.state.is_controlled() {
                self.state.toggle(|_| {});
            }
            payload
        }

        fn simulate_focus(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, true);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Focus(payload.clone()));
            (self.on_focus)(payload.clone());
            self.state.focus();
            payload
        }

        fn simulate_blur(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, false);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Blur(payload.clone()));
            (self.on_blur)(payload.clone());
            self.state.blur();
            payload
        }

        fn simulate_key(&mut self, key: ControlKey) -> (CheckboxKeyEvent, CheckboxChangeEvent) {
            let key_payload = expected_key(&self.props, &self.state, key);
            let change_payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Key(key_payload.clone()));
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(change_payload.clone()));
            (self.on_key)(key_payload.clone());
            (self.on_change)(change_payload.clone());
            if !self.state.is_controlled() {
                self.state.on_key(key, |_| {});
            }
            (key_payload, change_payload)
        }
    }

    #[test]
    fn telemetry_and_state_transitions_follow_adapter_contract() {
        let mut harness = DioxusHarness::new();

        let recorded = harness.contexts.lock().unwrap();
        assert_eq!(recorded.len(), 1);
        let context = &recorded[0];
        assert!(context.component.contains("checkbox"));
        assert_eq!(
            context.analytics_id.as_deref(),
            Some("analytics::checkbox::dioxus")
        );
        assert_eq!(
            context.automation_id.as_deref(),
            Some("automation::checkbox::dioxus")
        );
        drop(recorded);

        let expected_focus_gain = harness.simulate_focus();
        assert!(harness.state.focus_visible());

        let expected_change = harness.simulate_change();
        assert_eq!(harness.state.checked(), CheckboxValue::On);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), CheckboxValue::Off);

        let expected_focus_loss = harness.simulate_blur();
        assert!(!harness.state.focus_visible());

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 5);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Focus(expected_focus_gain.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[3],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        assert_eq!(
            telemetry[4],
            CheckboxTelemetryEvent::Blur(expected_focus_loss.clone())
        );

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
        drop(changes);

        let focus_events = harness.focus_events.borrow();
        assert_eq!(focus_events.len(), 1);
        assert_eq!(focus_events[0], expected_focus_gain);
        drop(focus_events);

        let blur_events = harness.blur_events.borrow();
        assert_eq!(blur_events.len(), 1);
        assert_eq!(blur_events[0], expected_focus_loss);
        drop(blur_events);

        let key_events = harness.key_events.borrow();
        assert_eq!(key_events.len(), 1);
        assert_eq!(key_events[0], expected_key);
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating_local_snapshot() {
        let mut harness = DioxusHarness::new_controlled();
        let initial_value = harness.state.checked();

        let expected_change = expected_change(&harness.props, &harness.state);
        let change = harness.simulate_change();
        assert_eq!(change, expected_change);
        assert_eq!(harness.state.checked(), initial_value);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), initial_value);

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 3);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        drop(telemetry);

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    use sycamore::prelude::*;

    struct SycamoreHarness {
        props: CheckboxProps,
        state: CheckboxState,
        contexts: Arc<Mutex<Vec<TelemetryContext>>>,
        telemetry_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>>,
        change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>>,
        focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>>,
        key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>>,
        telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)>,
        on_change: Rc<dyn Fn(CheckboxChangeEvent)>,
        on_focus: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_blur: Rc<dyn Fn(CheckboxFocusEvent)>,
        on_key: Rc<dyn Fn(CheckboxKeyEvent)>,
    }

    impl SycamoreHarness {
        fn new() -> Self {
            Self::new_with_state(CheckboxState::uncontrolled(false, false))
        }

        fn new_controlled() -> Self {
            Self::new_with_state(CheckboxState::controlled(false, false))
        }

        fn new_with_state(state: CheckboxState) -> Self {
            let contexts = Arc::new(Mutex::new(Vec::new()));
            let mut props = CheckboxProps::new("Finance approvals");
            props.telemetry = instrumented_hooks(
                "analytics::checkbox::sycamore",
                "automation::checkbox::sycamore",
                &contexts,
            );

            let telemetry_events = Rc::new(RefCell::new(Vec::new()));
            let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
                let events = Rc::clone(&telemetry_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let change_events = Rc::new(RefCell::new(Vec::new()));
            let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
                let events = Rc::clone(&change_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let focus_events = Rc::new(RefCell::new(Vec::new()));
            let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&focus_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let blur_events = Rc::new(RefCell::new(Vec::new()));
            let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
                let events = Rc::clone(&blur_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let key_events = Rc::new(RefCell::new(Vec::new()));
            let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
                let events = Rc::clone(&key_events);
                Rc::new(move |event| {
                    events.borrow_mut().push(event);
                })
            };

            let sycamore_props = checkbox::sycamore::SycamoreCheckboxProps {
                checkbox: props.clone(),
                state: state.clone(),
                on_change: Some(on_change.clone()),
                on_focus: Some(on_focus.clone()),
                on_blur: Some(on_blur.clone()),
                on_key: Some(on_key.clone()),
                telemetry_delegate: Some(telemetry_delegate.clone()),
            };

            let html = sycamore::render_to_string(|cx| {
                checkbox::sycamore::SycamoreCheckbox(cx, sycamore_props.clone())
            });
            assert!(html.contains("role=\"checkbox\""));
            assert!(html.contains("aria-checked"));
            assert!(html.contains("data-rustic-analytics-id=\"analytics::checkbox::sycamore\""));
            assert!(html.contains("data-automation-id=\"automation::checkbox::sycamore\""));

            Self {
                props,
                state,
                contexts,
                telemetry_events,
                change_events,
                focus_events,
                blur_events,
                key_events,
                telemetry_delegate,
                on_change,
                on_focus,
                on_blur,
                on_key,
            }
        }

        fn simulate_change(&mut self) -> CheckboxChangeEvent {
            let payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(payload.clone()));
            (self.on_change)(payload.clone());
            if !self.state.is_controlled() {
                self.state.toggle(|_| {});
            }
            payload
        }

        fn simulate_focus(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, true);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Focus(payload.clone()));
            (self.on_focus)(payload.clone());
            self.state.focus();
            payload
        }

        fn simulate_blur(&mut self) -> CheckboxFocusEvent {
            let payload = expected_focus(&self.props, &self.state, false);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Blur(payload.clone()));
            (self.on_blur)(payload.clone());
            self.state.blur();
            payload
        }

        fn simulate_key(&mut self, key: ControlKey) -> (CheckboxKeyEvent, CheckboxChangeEvent) {
            let key_payload = expected_key(&self.props, &self.state, key);
            let change_payload = expected_change(&self.props, &self.state);
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Key(key_payload.clone()));
            (self.telemetry_delegate)(CheckboxTelemetryEvent::Change(change_payload.clone()));
            (self.on_key)(key_payload.clone());
            (self.on_change)(change_payload.clone());
            if !self.state.is_controlled() {
                self.state.on_key(key, |_| {});
            }
            (key_payload, change_payload)
        }
    }

    #[test]
    fn telemetry_and_state_transitions_follow_adapter_contract() {
        let mut harness = SycamoreHarness::new();

        let recorded = harness.contexts.lock().unwrap();
        assert_eq!(recorded.len(), 1);
        let context = &recorded[0];
        assert!(context.component.contains("checkbox"));
        assert_eq!(
            context.analytics_id.as_deref(),
            Some("analytics::checkbox::sycamore")
        );
        assert_eq!(
            context.automation_id.as_deref(),
            Some("automation::checkbox::sycamore")
        );
        drop(recorded);

        let expected_focus_gain = harness.simulate_focus();
        assert!(harness.state.focus_visible());

        let expected_change = harness.simulate_change();
        assert_eq!(harness.state.checked(), CheckboxValue::On);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Enter);
        assert_eq!(harness.state.checked(), CheckboxValue::Off);

        let expected_focus_loss = harness.simulate_blur();
        assert!(!harness.state.focus_visible());

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 5);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Focus(expected_focus_gain.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[3],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        assert_eq!(
            telemetry[4],
            CheckboxTelemetryEvent::Blur(expected_focus_loss.clone())
        );

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
        drop(changes);

        let focus_events = harness.focus_events.borrow();
        assert_eq!(focus_events.len(), 1);
        assert_eq!(focus_events[0], expected_focus_gain);
        drop(focus_events);

        let blur_events = harness.blur_events.borrow();
        assert_eq!(blur_events.len(), 1);
        assert_eq!(blur_events[0], expected_focus_loss);
        drop(blur_events);

        let key_events = harness.key_events.borrow();
        assert_eq!(key_events.len(), 1);
        assert_eq!(key_events[0], expected_key);
    }

    #[test]
    fn controlled_state_still_emits_events_without_mutating_local_snapshot() {
        let mut harness = SycamoreHarness::new_controlled();
        let initial_value = harness.state.checked();

        let expected_change = expected_change(&harness.props, &harness.state);
        let change = harness.simulate_change();
        assert_eq!(change, expected_change);
        assert_eq!(harness.state.checked(), initial_value);

        let (expected_key, expected_key_change) = harness.simulate_key(ControlKey::Space);
        assert_eq!(harness.state.checked(), initial_value);

        let telemetry = harness.telemetry_events.borrow();
        assert_eq!(telemetry.len(), 3);
        assert_eq!(
            telemetry[0],
            CheckboxTelemetryEvent::Change(expected_change.clone())
        );
        assert_eq!(
            telemetry[1],
            CheckboxTelemetryEvent::Key(expected_key.clone())
        );
        assert_eq!(
            telemetry[2],
            CheckboxTelemetryEvent::Change(expected_key_change.clone())
        );
        drop(telemetry);

        let changes = harness.change_events.borrow();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], expected_change);
        assert_eq!(changes[1], expected_key_change);
    }
}
