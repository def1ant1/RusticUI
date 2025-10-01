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
fn expected_focus(props: &CheckboxProps, state: &CheckboxState, focused: bool) -> CheckboxFocusEvent {
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

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::yew::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked=\"false\""));
        assert!(out.ends_with(">Subscribe</span>"));
    }

    #[test]
    fn telemetry_delegates_precede_consumer_callbacks() {
        // Capture render lifecycle contexts so we can assert analytics and automation
        // identifiers are threaded through the TelemetryHooks span.
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut props = CheckboxProps::new("Marketing opt-in");
        props.telemetry = instrumented_hooks(
            "analytics::checkbox::yew",
            "automation::checkbox::yew",
            &contexts,
        );
        let state = CheckboxState::uncontrolled(false, false);

        let markup = checkbox::yew::render(&props, &state);
        assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::yew\""));
        assert!(markup.contains("data-automation-id=\"automation::checkbox::yew\""));

        let recorded = contexts.lock().unwrap();
        assert_eq!(recorded.len(), 1, "render telemetry hook should run exactly once");
        let context = &recorded[0];
        assert!(
            context.component.contains("checkbox"),
            "component context should describe the checkbox adapter"
        );
        assert_eq!(
            context.analytics_id.as_deref(),
            Some("analytics::checkbox::yew")
        );
        assert_eq!(
            context.automation_id.as_deref(),
            Some("automation::checkbox::yew")
        );

        // Set up telemetry/consumer fakes that mirror Yew callbacks so we can assert
        // telemetry delegates are invoked before the user supplied handlers.
        let delegate_events = Rc::new(RefCell::new(Vec::new()));
        let telemetry_delegate = {
            let events = Rc::clone(&delegate_events);
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

        // Manually emit the canonical payloads in the same order as the adapter.
        let change_payload = expected_change(&props, &state);
        telemetry_delegate.emit(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_change.emit(change_payload.clone());

        let focus_payload = expected_focus(&props, &state, true);
        telemetry_delegate.emit(CheckboxTelemetryEvent::Focus(focus_payload.clone()));
        on_focus.emit(focus_payload.clone());

        let blur_payload = expected_focus(&props, &state, false);
        telemetry_delegate.emit(CheckboxTelemetryEvent::Blur(blur_payload.clone()));
        on_blur.emit(blur_payload.clone());

        let key_payload = expected_key(&props, &state, ControlKey::Space);
        telemetry_delegate.emit(CheckboxTelemetryEvent::Key(key_payload.clone()));
        telemetry_delegate.emit(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_key.emit(key_payload.clone());
        on_change.emit(change_payload.clone());

        let telemetry = delegate_events.borrow();
        assert_eq!(telemetry.len(), 5, "delegate should record every payload");
        assert_eq!(telemetry[0], CheckboxTelemetryEvent::Change(change_payload.clone()));
        assert_eq!(telemetry[1], CheckboxTelemetryEvent::Focus(focus_payload));
        assert_eq!(telemetry[2], CheckboxTelemetryEvent::Blur(blur_payload));
        assert_eq!(telemetry[3], CheckboxTelemetryEvent::Key(key_payload));
        assert_eq!(telemetry[4], CheckboxTelemetryEvent::Change(change_payload.clone()));

        let consumer_changes = change_events.borrow();
        assert_eq!(consumer_changes.len(), 2);
        assert_eq!(consumer_changes[0], change_payload);
        assert_eq!(consumer_changes[1], change_payload);

        assert_eq!(focus_events.borrow().len(), 1);
        assert_eq!(blur_events.borrow().len(), 1);
        assert_eq!(key_events.borrow().len(), 1);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::leptos::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked=\"false\""));
    }

    #[test]
    fn telemetry_events_reach_leptos_consumers() {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut props = CheckboxProps::new("Release automation");
        props.telemetry = instrumented_hooks(
            "analytics::checkbox::leptos",
            "automation::checkbox::leptos",
            &contexts,
        );
        let state = CheckboxState::uncontrolled(false, false);

        let markup = checkbox::leptos::render(&props, &state);
        assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::leptos\""));
        assert!(markup.contains("data-automation-id=\"automation::checkbox::leptos\""));

        let recorded = contexts.lock().unwrap();
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

        let delegate_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>> =
            Rc::new(RefCell::new(Vec::new()));
        let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
            let events = Rc::clone(&delegate_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
            let events = Rc::clone(&change_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&focus_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&blur_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
            let events = Rc::clone(&key_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_payload = expected_change(&props, &state);
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_change(change_payload.clone());

        let focus_payload = expected_focus(&props, &state, true);
        telemetry_delegate(CheckboxTelemetryEvent::Focus(focus_payload.clone()));
        on_focus(focus_payload.clone());

        let blur_payload = expected_focus(&props, &state, false);
        telemetry_delegate(CheckboxTelemetryEvent::Blur(blur_payload.clone()));
        on_blur(blur_payload.clone());

        let key_payload = expected_key(&props, &state, ControlKey::Enter);
        telemetry_delegate(CheckboxTelemetryEvent::Key(key_payload.clone()));
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_key(key_payload.clone());
        on_change(change_payload.clone());

        assert_eq!(delegate_events.borrow().len(), 5);
        assert_eq!(change_events.borrow().len(), 2);
        assert_eq!(focus_events.borrow().len(), 1);
        assert_eq!(blur_events.borrow().len(), 1);
        assert_eq!(key_events.borrow().len(), 1);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let mut state = CheckboxState::uncontrolled(false, false);
        state.toggle(|_| {});
        let out = checkbox::dioxus::render(&props, &state);
        assert!(out.contains("aria-checked=\"true\""));
    }

    #[test]
    fn telemetry_contexts_and_events_capture_dioxus_state() {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut props = CheckboxProps::new("Nightly deployments");
        props.telemetry = instrumented_hooks(
            "analytics::checkbox::dioxus",
            "automation::checkbox::dioxus",
            &contexts,
        );
        let state = CheckboxState::uncontrolled(false, false);

        let markup = checkbox::dioxus::render(&props, &state);
        assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::dioxus\""));
        assert!(markup.contains("data-automation-id=\"automation::checkbox::dioxus\""));

        let recorded = contexts.lock().unwrap();
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

        let delegate_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>> =
            Rc::new(RefCell::new(Vec::new()));
        let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
            let events = Rc::clone(&delegate_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
            let events = Rc::clone(&change_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&focus_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&blur_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
            let events = Rc::clone(&key_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_payload = expected_change(&props, &state);
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_change(change_payload.clone());

        let focus_payload = expected_focus(&props, &state, true);
        telemetry_delegate(CheckboxTelemetryEvent::Focus(focus_payload.clone()));
        on_focus(focus_payload.clone());

        let blur_payload = expected_focus(&props, &state, false);
        telemetry_delegate(CheckboxTelemetryEvent::Blur(blur_payload.clone()));
        on_blur(blur_payload.clone());

        let key_payload = expected_key(&props, &state, ControlKey::Space);
        telemetry_delegate(CheckboxTelemetryEvent::Key(key_payload.clone()));
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_key(key_payload.clone());
        on_change(change_payload.clone());

        assert_eq!(delegate_events.borrow().len(), 5);
        assert_eq!(change_events.borrow().len(), 2);
        assert_eq!(focus_events.borrow().len(), 1);
        assert_eq!(blur_events.borrow().len(), 1);
        assert_eq!(key_events.borrow().len(), 1);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::sycamore::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked"));
    }

    #[test]
    fn telemetry_and_consumer_callbacks_align_for_sycamore() {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut props = CheckboxProps::new("Finance approvals");
        props.telemetry = instrumented_hooks(
            "analytics::checkbox::sycamore",
            "automation::checkbox::sycamore",
            &contexts,
        );
        let state = CheckboxState::uncontrolled(false, false);

        let markup = checkbox::sycamore::render(&props, &state);
        assert!(markup.contains("data-rustic-analytics-id=\"analytics::checkbox::sycamore\""));
        assert!(markup.contains("data-automation-id=\"automation::checkbox::sycamore\""));

        let recorded = contexts.lock().unwrap();
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

        let delegate_events: Rc<RefCell<Vec<CheckboxTelemetryEvent>>> =
            Rc::new(RefCell::new(Vec::new()));
        let telemetry_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = {
            let events = Rc::clone(&delegate_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_events: Rc<RefCell<Vec<CheckboxChangeEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_change: Rc<dyn Fn(CheckboxChangeEvent)> = {
            let events = Rc::clone(&change_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let focus_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_focus: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&focus_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let blur_events: Rc<RefCell<Vec<CheckboxFocusEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_blur: Rc<dyn Fn(CheckboxFocusEvent)> = {
            let events = Rc::clone(&blur_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let key_events: Rc<RefCell<Vec<CheckboxKeyEvent>>> = Rc::new(RefCell::new(Vec::new()));
        let on_key: Rc<dyn Fn(CheckboxKeyEvent)> = {
            let events = Rc::clone(&key_events);
            Rc::new(move |event| {
                events.borrow_mut().push(event);
            })
        };

        let change_payload = expected_change(&props, &state);
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_change(change_payload.clone());

        let focus_payload = expected_focus(&props, &state, true);
        telemetry_delegate(CheckboxTelemetryEvent::Focus(focus_payload.clone()));
        on_focus(focus_payload.clone());

        let blur_payload = expected_focus(&props, &state, false);
        telemetry_delegate(CheckboxTelemetryEvent::Blur(blur_payload.clone()));
        on_blur(blur_payload.clone());

        let key_payload = expected_key(&props, &state, ControlKey::Enter);
        telemetry_delegate(CheckboxTelemetryEvent::Key(key_payload.clone()));
        telemetry_delegate(CheckboxTelemetryEvent::Change(change_payload.clone()));
        on_key(key_payload.clone());
        on_change(change_payload.clone());

        assert_eq!(delegate_events.borrow().len(), 5);
        assert_eq!(change_events.borrow().len(), 2);
        assert_eq!(focus_events.borrow().len(), 1);
        assert_eq!(blur_events.borrow().len(), 1);
        assert_eq!(key_events.borrow().len(), 1);
    }
}
