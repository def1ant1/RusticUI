//! Enterprise-grade Sycamore showcase for Rustic UI selection controls.
//!
//! The module deliberately embraces verbose documentation and helper types so
//! that automation, design partners, and developers can all interact with the
//! telemetry contract from a single place.  The goal mirrors the JavaScript
//! monorepo: ship pre-production ready examples that double as living
//! reference implementations.

use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use rustic_ui_headless::checkbox::{CheckboxState, CheckboxValue};
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_material::checkbox::{
    sycamore::{SycamoreCheckbox, SycamoreCheckboxProps},
    CheckboxChangeEvent, CheckboxFocusEvent, CheckboxProps, CheckboxTelemetryEvent,
};
use rustic_ui_material::radio::{
    sycamore::{SycamoreRadioGroup, SycamoreRadioGroupProps},
    RadioChangeEvent, RadioFocusEvent, RadioGroupProps, RadioKeyEvent, RadioTelemetryEvent,
};
use rustic_ui_material::switch::{
    sycamore::{SycamoreSwitch, SycamoreSwitchProps},
    SwitchChangeEvent, SwitchFocusEvent, SwitchKeyEvent, SwitchProps, SwitchTelemetryEvent,
};
use rustic_ui_material::telemetry::{
    TelemetryAnalyticsPayload, TelemetryCommitPayload, TelemetryContext, TelemetryError,
    TelemetryFocusPayload, TelemetryHooks, TelemetryStateChangePayload,
};
use sycamore::prelude::*;

/// Human friendly label applied to the checkbox component.
const CHECKBOX_LABEL: &str = "Enable nightly metrics";
/// Label applied to the switch component.
const SWITCH_LABEL: &str = "Allow automation overrides";
/// Radio options shared between SSR and hydration builds.
const RADIO_OPTIONS: [&str; 3] = ["System", "Dark", "Custom"];

/// Structured capture of telemetry phases emitted while the demo executes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedEvent {
    /// Logical channel (e.g. `checkbox.controlled`).
    pub channel: String,
    /// Lifecycle phase or hook that triggered this record.
    pub phase: String,
    /// Human readable payload extracted from telemetry objects.
    pub detail: String,
}

impl fmt::Display for RecordedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{channel}] {phase}: {detail}",
            channel = self.channel,
            phase = self.phase,
            detail = self.detail
        )
    }
}

/// Thread-safe recorder shared between host smoke tests and WASM harnesses.
#[derive(Clone, Default)]
pub struct TelemetryRecorder {
    events: Arc<Mutex<Vec<RecordedEvent>>>,
}

impl TelemetryRecorder {
    /// Construct a new recorder ready to capture events.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a named channel used to tag analytics hooks.
    #[must_use]
    pub fn channel(&self, name: impl Into<String>) -> TelemetryChannel {
        TelemetryChannel::new(name.into(), self.clone())
    }

    /// Snapshot the recorded events in insertion order.
    #[must_use]
    pub fn events(&self) -> Vec<RecordedEvent> {
        self.events.lock().expect("recorder mutex poisoned").clone()
    }

    fn push(&self, event: RecordedEvent) {
        self.events
            .lock()
            .expect("recorder mutex poisoned")
            .push(event);
    }
}

#[inline]
fn console_trace(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&message.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{message}");
    }
}

/// Wrapper that centralises telemetry ID derivation and hook wiring.
#[derive(Clone)]
pub struct TelemetryChannel {
    name: String,
    recorder: TelemetryRecorder,
}

impl TelemetryChannel {
    fn new(name: String, recorder: TelemetryRecorder) -> Self {
        Self { name, recorder }
    }

    /// Fully-qualified analytics identifier used across SSR and hydration.
    #[must_use]
    pub fn analytics_id(&self) -> String {
        format!("selection-controls.sycamore.{}", self.name)
    }

    /// Automation identifier mirrored to data attributes for QA hooks.
    #[must_use]
    pub fn automation_id(&self) -> String {
        format!("automation.selection-controls.{}", self.name)
    }

    fn record(&self, phase: impl Into<String>, detail: impl Into<String>) {
        let phase = phase.into();
        let detail = detail.into();
        let event = RecordedEvent {
            channel: self.name.clone(),
            phase: phase.clone(),
            detail: detail.clone(),
        };
        console_trace(&format!("{event}"));
        self.recorder.push(event);
    }

    /// Build [`TelemetryHooks`] instrumented with comprehensive callbacks.
    #[must_use]
    pub fn hooks(&self) -> TelemetryHooks {
        let mut hooks = TelemetryHooks::default();
        hooks.analytics_id = Some(self.analytics_id());
        hooks.automation_id = Some(self.automation_id());

        let render_channel = self.clone();
        hooks.on_render = Some(Arc::new(move |context: TelemetryContext| {
            render_channel.record(
                "render",
                format!(
                    "component={} analytics={:?} automation={:?} attrs={:?}",
                    context.component,
                    context.analytics_id,
                    context.automation_id,
                    context
                        .descriptor
                        .as_ref()
                        .map(|descriptor| descriptor.attributes.clone()),
                ),
            );
        }));

        let analytics_channel = self.clone();
        hooks.on_analytics = Some(Arc::new(
            move |context: TelemetryContext, payload: TelemetryAnalyticsPayload| {
                analytics_channel.record(
                    "analytics",
                    format!(
                        "component={} channel={} props={:?}",
                        context.component, payload.channel, payload.properties
                    ),
                );
            },
        ));

        let focus_channel = self.clone();
        hooks.on_focus_transition = Some(Arc::new(
            move |context: TelemetryContext, payload: TelemetryFocusPayload| {
                focus_channel.record(
                    "focus-transition",
                    format!(
                        "component={} focused={} props={:?}",
                        context.component, payload.focused, payload.properties
                    ),
                );
            },
        ));

        let state_channel = self.clone();
        hooks.on_state_change = Some(Arc::new(
            move |context: TelemetryContext, payload: TelemetryStateChangePayload| {
                state_channel.record(
                    "state-change",
                    format!(
                        "component={} previous={} next={} props={:?}",
                        context.component, payload.previous, payload.next, payload.properties
                    ),
                );
            },
        ));

        let commit_channel = self.clone();
        hooks.on_commit_ack = Some(Arc::new(
            move |context: TelemetryContext, payload: TelemetryCommitPayload| {
                commit_channel.record(
                    "commit",
                    format!(
                        "component={} correlation={:?} props={:?}",
                        context.component, payload.correlation_id, payload.properties
                    ),
                );
            },
        ));

        let error_channel = self.clone();
        hooks.on_error = Some(Arc::new(
            move |context: TelemetryContext, error: TelemetryError| {
                error_channel.record(
                    "error",
                    format!("component={} message={}", context.component, error.message),
                );
            },
        ));

        hooks
    }

    /// Delegate that mirrors checkbox telemetry payloads into the recorder.
    #[must_use]
    pub fn checkbox_delegate(&self) -> Rc<dyn Fn(CheckboxTelemetryEvent)> {
        let channel = self.clone();
        Rc::new(move |event: CheckboxTelemetryEvent| {
            channel.record("telemetry", format!("checkbox::{event:?}"));
        })
    }

    /// Delegate that mirrors switch telemetry payloads into the recorder.
    #[must_use]
    pub fn switch_delegate(&self) -> Rc<dyn Fn(SwitchTelemetryEvent)> {
        let channel = self.clone();
        Rc::new(move |event: SwitchTelemetryEvent| {
            channel.record("telemetry", format!("switch::{event:?}"));
        })
    }

    /// Delegate that mirrors radio telemetry payloads into the recorder.
    #[must_use]
    pub fn radio_delegate(&self) -> Rc<dyn Fn(RadioTelemetryEvent)> {
        let channel = self.clone();
        Rc::new(move |event: RadioTelemetryEvent| {
            channel.record("telemetry", format!("radio::{event:?}"));
        })
    }
}

/// Record a checkbox change event in the shared telemetry channel.
pub fn record_checkbox_change(channel: &TelemetryChannel, event: &CheckboxChangeEvent) {
    channel.record(
        "change-handler",
        format!(
            "previous={:?} next={:?} disabled={} label={}",
            event.previous, event.next, event.disabled, event.label
        ),
    );
}

/// Record a checkbox focus transition.
pub fn record_checkbox_focus(channel: &TelemetryChannel, event: &CheckboxFocusEvent) {
    let phase = if event.focused { "focus" } else { "blur" };
    channel.record(
        phase,
        format!(
            "checked={:?} disabled={} label={}",
            event.checked, event.disabled, event.label
        ),
    );
}

/// Record a switch change event in the shared telemetry channel.
pub fn record_switch_change(channel: &TelemetryChannel, event: &SwitchChangeEvent) {
    channel.record(
        "change-handler",
        format!(
            "previous={} next={} disabled={} label={}",
            event.previous, event.next, event.disabled, event.label
        ),
    );
}

/// Record a switch focus transition.
pub fn record_switch_focus(channel: &TelemetryChannel, event: &SwitchFocusEvent) {
    let phase = if event.focused { "focus" } else { "blur" };
    channel.record(
        phase,
        format!(
            "on={} disabled={} label={}",
            event.on, event.disabled, event.label
        ),
    );
}

/// Record a switch keyboard interaction.
pub fn record_switch_key(channel: &TelemetryChannel, event: &SwitchKeyEvent) {
    channel.record(
        "key",
        format!(
            "key={:?} previous={} next={} disabled={}",
            event.key, event.previous, event.next, event.disabled
        ),
    );
}

/// Record a radio telemetry change event.
pub fn record_radio_change(channel: &TelemetryChannel, event: &RadioChangeEvent) {
    channel.record(
        "change-handler",
        format!(
            "previous={:?} next={} disabled={} label={}",
            event.previous, event.next, event.disabled, event.label
        ),
    );
}

/// Record a radio focus transition.
pub fn record_radio_focus(channel: &TelemetryChannel, event: &RadioFocusEvent) {
    let phase = if event.focused { "focus" } else { "blur" };
    channel.record(
        phase,
        format!(
            "index={} disabled={} label={}",
            event.index, event.disabled, event.label
        ),
    );
}

/// Record a radio keyboard interaction.
pub fn record_radio_key(channel: &TelemetryChannel, event: &RadioKeyEvent) {
    channel.record(
        "key",
        format!(
            "key={:?} previous={:?} next={:?} disabled={}",
            event.key, event.previous, event.next, event.disabled
        ),
    );
}

/// Bundle of telemetry channels so builders can configure props without
/// repeating boilerplate.
#[derive(Clone)]
pub struct SelectionControlsTelemetry {
    pub checkbox: TelemetryChannel,
    pub switch: TelemetryChannel,
    pub radio_group: TelemetryChannel,
    pub radio_component: TelemetryChannel,
}

impl SelectionControlsTelemetry {
    /// Construct telemetry channels with enterprise-friendly names.
    #[must_use]
    pub fn new(recorder: &TelemetryRecorder) -> Self {
        Self {
            checkbox: recorder.channel("checkbox"),
            switch: recorder.channel("switch"),
            radio_group: recorder.channel("radio.group"),
            radio_component: recorder.channel("radio.component"),
        }
    }
}

/// Props shared between SSR renders, hydration, and manual smoke tests.
#[derive(Clone)]
pub struct SelectionControlsProps {
    pub telemetry: SelectionControlsTelemetry,
    pub checkbox_state: CheckboxState,
    pub switch_state: SwitchState,
    pub radio_state: RadioGroupState,
}

impl SelectionControlsProps {
    /// Construct the demo state using the same defaults across environments.
    #[must_use]
    pub fn enterprise_defaults(recorder: &TelemetryRecorder) -> Self {
        let telemetry = SelectionControlsTelemetry::new(recorder);
        let checkbox_state = CheckboxState::uncontrolled(CheckboxValue::Off, true);
        let switch_state = SwitchState::uncontrolled(false, false);
        let radio_state = RadioGroupState::uncontrolled(
            RADIO_OPTIONS
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>(),
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        Self {
            telemetry,
            checkbox_state,
            switch_state,
            radio_state,
        }
    }

    /// Simulate a representative interaction cycle to validate telemetry order.
    pub fn simulate_nominal_cycle(&self) {
        // Checkbox change and focus lifecycle.
        let checkbox_channel = self.telemetry.checkbox.clone();
        let checkbox_previous = self.checkbox_state.checked();
        let checkbox_next = match checkbox_previous {
            CheckboxValue::On => CheckboxValue::Off,
            CheckboxValue::Off => CheckboxValue::On,
            CheckboxValue::Indeterminate => CheckboxValue::On,
        };
        let checkbox_change = CheckboxChangeEvent {
            previous: checkbox_previous,
            next: checkbox_next,
            disabled: self.checkbox_state.disabled(),
            analytics_id: Some(checkbox_channel.analytics_id()),
            automation_id: Some(checkbox_channel.automation_id()),
            label: CHECKBOX_LABEL.to_string(),
        };
        (checkbox_channel.checkbox_delegate())(CheckboxTelemetryEvent::Change(
            checkbox_change.clone(),
        ));
        record_checkbox_change(&checkbox_channel, &checkbox_change);

        let checkbox_focus = CheckboxFocusEvent {
            focused: true,
            checked: checkbox_previous,
            disabled: self.checkbox_state.disabled(),
            analytics_id: Some(checkbox_channel.analytics_id()),
            automation_id: Some(checkbox_channel.automation_id()),
            label: CHECKBOX_LABEL.to_string(),
        };
        (checkbox_channel.checkbox_delegate())(CheckboxTelemetryEvent::Focus(
            checkbox_focus.clone(),
        ));
        record_checkbox_focus(&checkbox_channel, &checkbox_focus);

        let checkbox_blur = CheckboxFocusEvent {
            focused: false,
            ..checkbox_focus
        };
        (checkbox_channel.checkbox_delegate())(CheckboxTelemetryEvent::Blur(checkbox_blur.clone()));
        record_checkbox_focus(&checkbox_channel, &checkbox_blur);

        // Switch interactions mirror the checkbox flow but include keyboard data.
        let switch_channel = self.telemetry.switch.clone();
        let switch_change = SwitchChangeEvent {
            previous: self.switch_state.on(),
            next: !self.switch_state.on(),
            disabled: self.switch_state.disabled(),
            analytics_id: Some(switch_channel.analytics_id()),
            automation_id: Some(switch_channel.automation_id()),
            label: SWITCH_LABEL.to_string(),
        };
        (switch_channel.switch_delegate())(SwitchTelemetryEvent::Change(switch_change.clone()));
        record_switch_change(&switch_channel, &switch_change);

        let switch_focus = SwitchFocusEvent {
            focused: true,
            on: self.switch_state.on(),
            disabled: self.switch_state.disabled(),
            analytics_id: Some(switch_channel.analytics_id()),
            automation_id: Some(switch_channel.automation_id()),
            label: SWITCH_LABEL.to_string(),
        };
        (switch_channel.switch_delegate())(SwitchTelemetryEvent::Focus(switch_focus.clone()));
        record_switch_focus(&switch_channel, &switch_focus);

        let switch_blur = SwitchFocusEvent {
            focused: false,
            ..switch_focus
        };
        (switch_channel.switch_delegate())(SwitchTelemetryEvent::Blur(switch_blur.clone()));
        record_switch_focus(&switch_channel, &switch_blur);

        let switch_key = SwitchKeyEvent {
            key: ControlKey::Enter,
            previous: self.switch_state.on(),
            next: !self.switch_state.on(),
            disabled: self.switch_state.disabled(),
            analytics_id: Some(switch_channel.analytics_id()),
            automation_id: Some(switch_channel.automation_id()),
            label: SWITCH_LABEL.to_string(),
        };
        (switch_channel.switch_delegate())(SwitchTelemetryEvent::Key(switch_key.clone()));
        record_switch_key(&switch_channel, &switch_key);

        // Radio flow captures keyboard + change ordering for hydration parity.
        let radio_channel = self.telemetry.radio_component.clone();
        let previous = self.radio_state.selected_index();
        let next = ((previous.unwrap_or(0) + 1) % RADIO_OPTIONS.len()) as usize;
        let radio_change = RadioChangeEvent {
            previous,
            next,
            disabled: self.radio_state.disabled(),
            analytics_id: Some(radio_channel.analytics_id()),
            automation_id: Some(radio_channel.automation_id()),
            label: RADIO_OPTIONS[next].to_string(),
        };
        (radio_channel.radio_delegate())(RadioTelemetryEvent::Change(radio_change.clone()));
        record_radio_change(&radio_channel, &radio_change);

        let radio_focus = RadioFocusEvent {
            index: next,
            focused: true,
            disabled: self.radio_state.disabled(),
            analytics_id: Some(radio_channel.analytics_id()),
            automation_id: Some(radio_channel.automation_id()),
            label: RADIO_OPTIONS[next].to_string(),
        };
        (radio_channel.radio_delegate())(RadioTelemetryEvent::Focus(radio_focus.clone()));
        record_radio_focus(&radio_channel, &radio_focus);

        let radio_blur = RadioFocusEvent {
            focused: false,
            ..radio_focus
        };
        (radio_channel.radio_delegate())(RadioTelemetryEvent::Blur(radio_blur.clone()));
        record_radio_focus(&radio_channel, &radio_blur);

        let radio_key = RadioKeyEvent {
            key: match self.radio_state.orientation() {
                RadioOrientation::Horizontal => ControlKey::ArrowRight,
                RadioOrientation::Vertical => ControlKey::ArrowDown,
            },
            previous,
            next: Some(next),
            disabled: self.radio_state.disabled(),
            analytics_id: Some(radio_channel.analytics_id()),
            automation_id: Some(radio_channel.automation_id()),
            label: RADIO_OPTIONS[next].to_string(),
        };
        (radio_channel.radio_delegate())(RadioTelemetryEvent::Key(radio_key.clone()));
        record_radio_key(&radio_channel, &radio_key);
    }
}

/// Render the selection controls using Sycamore adapters with exhaustive
/// telemetry wiring.
#[component]
pub fn SelectionControls<G: Html>(cx: Scope, props: SelectionControlsProps) -> View<G> {
    // Checkbox wiring demonstrates the render -> telemetry -> change ordering that
    // enterprise analytics expect.
    let checkbox_channel = props.telemetry.checkbox.clone();
    let checkbox_props = CheckboxProps::new(CHECKBOX_LABEL, checkbox_channel.hooks());
    let checkbox_component = SycamoreCheckboxProps {
        checkbox: checkbox_props.clone(),
        state: props.checkbox_state.clone(),
        on_change: Some({
            let channel = checkbox_channel.clone();
            Rc::new(move |event: CheckboxChangeEvent| {
                record_checkbox_change(&channel, &event);
            })
        }),
        on_focus: Some({
            let channel = checkbox_channel.clone();
            Rc::new(move |event: CheckboxFocusEvent| {
                record_checkbox_focus(&channel, &event);
            })
        }),
        on_blur: Some({
            let channel = checkbox_channel.clone();
            Rc::new(move |event: CheckboxFocusEvent| {
                record_checkbox_focus(&channel, &event);
            })
        }),
        on_key: None,
        telemetry_delegate: Some(checkbox_channel.checkbox_delegate()),
    };

    // Switch wiring mirrors the checkbox and adds keyboard analytics.
    let switch_channel = props.telemetry.switch.clone();
    let switch_props = SwitchProps::new(SWITCH_LABEL, switch_channel.hooks());
    let switch_component = SycamoreSwitchProps {
        switch: switch_props.clone(),
        state: props.switch_state.clone(),
        on_change: Some({
            let channel = switch_channel.clone();
            Rc::new(move |event: SwitchChangeEvent| {
                record_switch_change(&channel, &event);
            })
        }),
        on_focus: Some({
            let channel = switch_channel.clone();
            Rc::new(move |event: SwitchFocusEvent| {
                record_switch_focus(&channel, &event);
            })
        }),
        on_blur: Some({
            let channel = switch_channel.clone();
            Rc::new(move |event: SwitchFocusEvent| {
                record_switch_focus(&channel, &event);
            })
        }),
        on_key: Some({
            let channel = switch_channel.clone();
            Rc::new(move |event: SwitchKeyEvent| {
                record_switch_key(&channel, &event);
            })
        }),
        telemetry_delegate: Some(switch_channel.switch_delegate()),
    };

    // Radio group includes both container telemetry and nested option telemetry.
    let radio_group_channel = props.telemetry.radio_group.clone();
    let radio_group_props =
        RadioGroupProps::from_state(&props.radio_state, radio_group_channel.hooks());
    let mut radio_component =
        SycamoreRadioGroupProps::new(radio_group_props, props.radio_state.clone());
    radio_component.telemetry = props.telemetry.radio_component.hooks();
    radio_component.on_change = Some({
        let channel = props.telemetry.radio_component.clone();
        Rc::new(move |event: RadioChangeEvent| {
            record_radio_change(&channel, &event);
        })
    });
    radio_component.on_focus = Some({
        let channel = props.telemetry.radio_component.clone();
        Rc::new(move |event: RadioFocusEvent| {
            record_radio_focus(&channel, &event);
        })
    });
    radio_component.on_blur = Some({
        let channel = props.telemetry.radio_component.clone();
        Rc::new(move |event: RadioFocusEvent| {
            record_radio_focus(&channel, &event);
        })
    });
    radio_component.on_key = Some({
        let channel = props.telemetry.radio_component.clone();
        Rc::new(move |event: RadioKeyEvent| {
            record_radio_key(&channel, &event);
        })
    });
    radio_component.telemetry_delegate = Some(props.telemetry.radio_component.radio_delegate());

    view! { cx,
        div(class="selection-controls-stack") {
            h2 { "Selection controls" }
            SycamoreCheckbox(checkbox_component)
            SycamoreSwitch(switch_component)
            SycamoreRadioGroup(radio_component)
        }
    }
}

/// Render the view to HTML so SSR smoke tests can assert deterministic output.
#[must_use]
pub fn render_ssr(props: SelectionControlsProps) -> String {
    sycamore::render_to_string(|cx| view! { cx, SelectionControls(props.clone()) })
}

/// Launch a CSR render in native binaries for quick local demos.
pub fn render_cli_preview() {
    let recorder = TelemetryRecorder::new();
    let props = SelectionControlsProps::enterprise_defaults(&recorder);
    let markup = render_ssr(props.clone());
    println!("SSR snapshot:\n{markup}");
    props.simulate_nominal_cycle();
    println!("--- telemetry log ---");
    for event in recorder.events() {
        println!("{event}");
    }
}

/// Hydrate the Sycamore app in the browser using the existing DOM snapshot.
#[cfg(target_arch = "wasm32")]
pub fn hydrate_web_app() {
    let recorder = TelemetryRecorder::new();
    let props = SelectionControlsProps::enterprise_defaults(&recorder);
    let render_props = props.clone();
    sycamore::render(move |cx| view! { cx, SelectionControls(render_props.clone()) });
    // Emit a baseline telemetry cycle immediately so automation can assert order.
    props.simulate_nominal_cycle();
}
