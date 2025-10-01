#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use std::sync::{Arc, Mutex};

use rustic_ui_headless::{
    interaction::ControlKey,
    radio::{RadioGroupState, RadioOrientation},
};
use rustic_ui_material::radio::{
    self, RadioAnalyticsEvent, RadioChangeEvent, RadioCommitEvent, RadioFocusEvent,
    RadioGroupProps, RadioKeyEvent, RadioTelemetryEvent,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};

/// Provide a deterministic uncontrolled state used by every adapter harness.
fn sample_state() -> RadioGroupState {
    RadioGroupState::uncontrolled(
        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    )
}

/// Mirror the checkbox adapter tests by wiring telemetry spies so render
/// instrumentation can be asserted without replicating adapter internals in the
/// suites below.
fn instrumented_hooks(
    analytics: &str,
    automation: &str,
    storage: &Arc<Mutex<Vec<TelemetryContext>>>,
) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(analytics.to_string());
    hooks.automation_id = Some(automation.to_string());
    hooks.on_render = Some({
        let store = Arc::clone(storage);
        Arc::new(move |ctx: TelemetryContext| {
            store.lock().unwrap().push(ctx);
        })
    });
    hooks
}

/// State container shared by every framework specific test so telemetry,
/// keyboard handling and hydration attributes can be exercised consistently.
struct RadioHarness {
    props: RadioGroupProps,
    state: RadioGroupState,
    analytics_id: String,
    automation_id: String,
    contexts: Arc<Mutex<Vec<TelemetryContext>>>,
    telemetry_events: Vec<RadioTelemetryEvent>,
    change_events: Vec<RadioChangeEvent>,
    focus_events: Vec<RadioFocusEvent>,
    blur_events: Vec<RadioFocusEvent>,
    key_events: Vec<RadioKeyEvent>,
}

impl RadioHarness {
    fn new(analytics_id: String, automation_id: String) -> Self {
        let contexts = Arc::new(Mutex::new(Vec::new()));
        let mut props = RadioGroupProps::from_state(&sample_state());
        props.telemetry = instrumented_hooks(&analytics_id, &automation_id, &contexts);
        props
            .additional_group_attributes
            .push(("style".into(), "position: relative;".into()));
        props
            .additional_group_attributes
            .push(("data-e2e".into(), "radio-group-under-test".into()));
        props.additional_option_attributes = vec![
            vec![
                ("style".into(), "color: teal;".into()),
                ("data-option-flag".into(), "alpha".into()),
            ],
            vec![
                ("style".into(), "color: purple;".into()),
                ("data-option-flag".into(), "beta".into()),
            ],
            vec![
                ("style".into(), "color: orange;".into()),
                ("data-option-flag".into(), "gamma".into()),
            ],
        ];

        Self {
            props,
            state: sample_state(),
            analytics_id,
            automation_id,
            contexts,
            telemetry_events: Vec::new(),
            change_events: Vec::new(),
            focus_events: Vec::new(),
            blur_events: Vec::new(),
            key_events: Vec::new(),
        }
    }

    fn label(&self, index: usize) -> String {
        self.props
            .option_labels
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("option-{index}"))
    }

    fn analytics_event(&self, index: usize, selected: Option<usize>) -> RadioAnalyticsEvent {
        RadioAnalyticsEvent {
            index,
            selected,
            disabled: self.state.disabled(),
            analytics_id: Some(self.analytics_id.clone()),
            automation_id: Some(self.automation_id.clone()),
            label: self.label(index),
        }
    }

    fn focus_payload(&self, index: usize, focused: bool) -> RadioFocusEvent {
        RadioFocusEvent {
            index,
            focused,
            disabled: self.state.disabled(),
            analytics_id: Some(self.analytics_id.clone()),
            automation_id: Some(self.automation_id.clone()),
            label: self.label(index),
        }
    }

    fn change_payload(&self, previous: Option<usize>, next: usize) -> RadioChangeEvent {
        RadioChangeEvent {
            previous,
            next,
            disabled: self.state.disabled(),
            analytics_id: Some(self.analytics_id.clone()),
            automation_id: Some(self.automation_id.clone()),
            label: self.label(next),
        }
    }

    fn commit_payload(&self, selected: Option<usize>, index: usize) -> RadioCommitEvent {
        RadioCommitEvent {
            selected,
            controlled: self.state.is_controlled(),
            analytics_id: Some(self.analytics_id.clone()),
            automation_id: Some(self.automation_id.clone()),
            label: self.label(index),
        }
    }

    fn key_payload(
        &self,
        origin: usize,
        key: ControlKey,
        previous: Option<usize>,
        next: Option<usize>,
    ) -> RadioKeyEvent {
        RadioKeyEvent {
            key,
            previous,
            next,
            disabled: self.state.disabled(),
            analytics_id: Some(self.analytics_id.clone()),
            automation_id: Some(self.automation_id.clone()),
            label: self.label(origin),
        }
    }

    fn focus_option(&mut self, index: usize) -> RadioFocusEvent {
        let analytics = self.analytics_event(index, self.state.selected_index());
        let payload = self.focus_payload(index, true);
        self.telemetry_events
            .push(RadioTelemetryEvent::Analytics(analytics));
        self.telemetry_events
            .push(RadioTelemetryEvent::Focus(payload.clone()));
        self.focus_events.push(payload.clone());
        self.state.focus(index);
        payload
    }

    fn blur_option(&mut self, index: usize) -> RadioFocusEvent {
        let analytics = self.analytics_event(index, self.state.selected_index());
        let payload = self.focus_payload(index, false);
        self.telemetry_events
            .push(RadioTelemetryEvent::Analytics(analytics));
        self.telemetry_events
            .push(RadioTelemetryEvent::Blur(payload.clone()));
        self.blur_events.push(payload.clone());
        self.state.blur();
        payload
    }

    fn pointer_select(&mut self, index: usize) -> RadioChangeEvent {
        let previous = self.state.selected_index();
        let analytics = self.analytics_event(index, previous);
        let change = self.change_payload(previous, index);
        self.telemetry_events
            .push(RadioTelemetryEvent::Analytics(analytics));
        self.telemetry_events
            .push(RadioTelemetryEvent::Change(change.clone()));
        self.state.select(index, |_| {});
        self.state.focus(index);
        let commit = self.commit_payload(self.state.selected_index(), index);
        self.telemetry_events
            .push(RadioTelemetryEvent::Commit(commit));
        self.change_events.push(change.clone());
        change
    }

    fn key_from(&mut self, origin: usize, key: ControlKey) -> RadioKeyEvent {
        let previous = self.state.selected_index();
        let analytics = self.analytics_event(origin, previous);
        self.telemetry_events
            .push(RadioTelemetryEvent::Analytics(analytics));

        let mut selected_after = None;
        self.state.on_key(key, |selected| {
            selected_after = Some(selected);
        });

        let payload = self.key_payload(origin, key, previous, selected_after);
        self.key_events.push(payload.clone());
        self.telemetry_events
            .push(RadioTelemetryEvent::Key(payload.clone()));

        if let Some(next_index) = selected_after {
            let focus_event = self.focus_payload(next_index, true);
            self.telemetry_events
                .push(RadioTelemetryEvent::Focus(focus_event));

            if next_index != origin {
                let blur_event = self.focus_payload(origin, false);
                self.telemetry_events
                    .push(RadioTelemetryEvent::Blur(blur_event));
            }

            let change = self.change_payload(previous, next_index);
            self.telemetry_events
                .push(RadioTelemetryEvent::Change(change.clone()));
            let commit = self.commit_payload(self.state.selected_index(), next_index);
            self.telemetry_events
                .push(RadioTelemetryEvent::Commit(commit));
            self.change_events.push(change);
        }

        payload
    }
}

fn exercise_adapter<F>(framework: &str, render_fn: F)
where
    F: Fn(&RadioGroupProps, &RadioGroupState) -> String,
{
    let analytics_id = format!("analytics::radio::{framework}");
    let automation_id = format!("automation::radio::{framework}");
    let mut harness = RadioHarness::new(analytics_id.clone(), automation_id.clone());

    let initial_markup = render_fn(&harness.props, &harness.state);
    assert!(initial_markup.contains("role=\"radiogroup\""));
    assert!(initial_markup.contains("aria-orientation=\"horizontal\""));
    assert!(initial_markup.contains("data-orientation=\"horizontal\""));
    assert!(initial_markup.contains(&format!("data-rustic-analytics-id=\"{analytics_id}\"")));
    assert!(initial_markup.contains(&format!("data-automation-id=\"{automation_id}\"")));
    assert!(initial_markup.contains("data-e2e=\"radio-group-under-test\""));
    assert!(initial_markup.contains("data-option-flag=\"alpha\""));
    assert!(initial_markup.contains("style=\"position: relative;\""));
    assert!(initial_markup.contains("style=\"color: teal;\""));
    assert!(initial_markup.contains("aria-checked=\"true\""));

    {
        let contexts = harness.contexts.lock().unwrap();
        assert_eq!(contexts.len(), 1);
        let context = &contexts[0];
        assert!(context.component.contains("radio"));
        assert_eq!(context.analytics_id.as_deref(), Some(&analytics_id));
        assert_eq!(context.automation_id.as_deref(), Some(&automation_id));
        assert!(context
            .descriptor
            .as_ref()
            .map(|descriptor| descriptor.label.starts_with("radio-group"))
            .unwrap_or(false));
    }

    let focus_event = harness.focus_option(1);
    assert!(focus_event.focused);
    assert_eq!(harness.state.focus_visible_index(), Some(1));
    let focused_markup = render_fn(&harness.props, &harness.state);
    assert!(focused_markup.contains("data-option-flag=\"beta\""));
    assert!(focused_markup.contains("data-focus-visible=\"true\""));

    let blur_event = harness.blur_option(1);
    assert!(!blur_event.focused);
    assert_eq!(harness.state.focus_visible_index(), None);
    let blurred_markup = render_fn(&harness.props, &harness.state);
    assert!(blurred_markup.contains("data-option-flag=\"beta\""));
    assert!(!blurred_markup.contains("data-focus-visible=\"true\""));

    let change_event = harness.pointer_select(1);
    assert_eq!(change_event.previous, Some(0));
    assert_eq!(change_event.next, 1);
    assert_eq!(harness.state.selected_index(), Some(1));
    assert_eq!(harness.state.focus_visible_index(), Some(1));
    let selected_markup = render_fn(&harness.props, &harness.state);
    assert!(selected_markup.contains("data-option-flag=\"beta\""));
    assert!(selected_markup.contains("aria-checked=\"true\""));
    assert!(selected_markup.contains("data-focus-visible=\"true\""));
    assert!(selected_markup.contains("style=\"color: purple;\""));

    let key_event = harness.key_from(1, ControlKey::ArrowRight);
    assert_eq!(key_event.key, ControlKey::ArrowRight);
    assert_eq!(key_event.previous, Some(1));
    assert_eq!(key_event.next, Some(2));
    assert_eq!(harness.state.selected_index(), Some(2));
    assert_eq!(harness.state.focus_visible_index(), Some(2));
    let keyboard_markup = render_fn(&harness.props, &harness.state);
    assert!(keyboard_markup.contains("data-option-flag=\"gamma\""));
    assert!(keyboard_markup.contains("aria-checked=\"true\""));
    assert!(keyboard_markup.contains("data-focus-visible=\"true\""));
    assert!(keyboard_markup.contains("style=\"color: orange;\""));
    assert!(keyboard_markup.contains("class=\""));

    let contexts = harness.contexts.lock().unwrap();
    assert_eq!(contexts.len(), 5);
    drop(contexts);

    assert_eq!(harness.telemetry_events.len(), 13);
    let telemetry = &harness.telemetry_events;
    assert!(matches!(
        telemetry[0],
        RadioTelemetryEvent::Analytics(ref evt)
            if evt.index == 1
                && evt.selected == Some(0)
                && evt.analytics_id.as_deref() == Some(analytics_id.as_str())
    ));
    assert!(matches!(
        telemetry[1],
        RadioTelemetryEvent::Focus(ref evt)
            if evt.focused && evt.index == 1 && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[2],
        RadioTelemetryEvent::Analytics(ref evt)
            if evt.index == 1 && evt.selected == Some(0)
    ));
    assert!(matches!(
        telemetry[3],
        RadioTelemetryEvent::Blur(ref evt)
            if !evt.focused && evt.index == 1 && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[4],
        RadioTelemetryEvent::Analytics(ref evt)
            if evt.index == 1 && evt.selected == Some(0)
    ));
    assert!(matches!(
        telemetry[5],
        RadioTelemetryEvent::Change(ref evt)
            if evt.previous == Some(0) && evt.next == 1 && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[6],
        RadioTelemetryEvent::Commit(ref evt)
            if evt.selected == Some(1) && !evt.controlled && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[7],
        RadioTelemetryEvent::Analytics(ref evt)
            if evt.index == 1 && evt.selected == Some(1)
    ));
    assert!(matches!(
        telemetry[8],
        RadioTelemetryEvent::Key(ref evt)
            if evt.key == ControlKey::ArrowRight
                && evt.previous == Some(1)
                && evt.next == Some(2)
                && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[9],
        RadioTelemetryEvent::Focus(ref evt)
            if evt.focused && evt.index == 2 && evt.label == "Gamma"
    ));
    assert!(matches!(
        telemetry[10],
        RadioTelemetryEvent::Blur(ref evt)
            if !evt.focused && evt.index == 1 && evt.label == "Beta"
    ));
    assert!(matches!(
        telemetry[11],
        RadioTelemetryEvent::Change(ref evt)
            if evt.previous == Some(1) && evt.next == 2 && evt.label == "Gamma"
    ));
    assert!(matches!(
        telemetry[12],
        RadioTelemetryEvent::Commit(ref evt)
            if evt.selected == Some(2) && evt.label == "Gamma"
    ));

    assert_eq!(harness.change_events.len(), 2);
    assert_eq!(harness.change_events[0].next, 1);
    assert_eq!(harness.change_events[1].next, 2);

    assert_eq!(harness.focus_events.len(), 1);
    assert!(harness.focus_events[0].focused);
    assert_eq!(harness.focus_events[0].index, 1);

    assert_eq!(harness.blur_events.len(), 1);
    assert!(!harness.blur_events[0].focused);
    assert_eq!(harness.blur_events[0].index, 1);

    assert_eq!(harness.key_events.len(), 1);
    assert_eq!(harness.key_events[0].next, Some(2));
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn telemetry_and_markup_follow_state_transitions() {
        exercise_adapter("yew", radio::yew::render);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn telemetry_and_markup_follow_state_transitions() {
        exercise_adapter("leptos", radio::leptos::render);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn telemetry_and_markup_follow_state_transitions() {
        exercise_adapter("dioxus", radio::dioxus::render);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn telemetry_and_markup_follow_state_transitions() {
        exercise_adapter("sycamore", radio::sycamore::render);
    }
}
