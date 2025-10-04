//! Enterprise grade selection control showcase rendered via Dioxus.
//!
//! The example converts the README snippet into an executable crate with
//! extensive inline documentation, telemetry recording, and automation-
//! friendly smoke tests. Instead of depending on the unfinished
//! `rustic-ui-material` Dioxus adapters, the crate renders lightweight
//! controls directly with Dioxus primitives while reusing the shared
//! headless state machines and telemetry schemas from Rustic UI.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use rustic_ui_headless::checkbox::{CheckboxState, CheckboxValue};
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_material::checkbox::{CheckboxChangeEvent, CheckboxTelemetryEvent};
use rustic_ui_material::radio::{RadioChangeEvent, RadioTelemetryEvent};
use rustic_ui_material::switch::{SwitchChangeEvent, SwitchTelemetryEvent};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};

/// Human friendly telemetry channels reused across logging sinks.
const CHECKBOX_CHANNEL: &str = "checkbox";
const SWITCH_CHANNEL: &str = "switch";
const RADIO_CHANNEL: &str = "radio";
const RADIO_COMPONENT_CHANNEL: &str = "radio.component";

/// Structured telemetry emitted by the selection control showcase.
#[derive(Clone, Debug, PartialEq)]
pub enum TelemetrySignal {
    /// Lifecycle event captured by [`TelemetryHooks::on_render`].
    Render {
        channel: &'static str,
        component: &'static str,
        analytics_id: Option<String>,
        automation_id: Option<String>,
    },
    /// Plain text message mirrored to stdout/stderr for manual tracing.
    Console {
        channel: &'static str,
        message: String,
    },
    /// Structured payload raised by the checkbox telemetry delegate.
    Checkbox(CheckboxTelemetryEvent),
    /// Structured payload raised by the switch telemetry delegate.
    Switch(SwitchTelemetryEvent),
    /// Structured payload raised by the radio telemetry delegate.
    Radio(RadioTelemetryEvent),
}

impl fmt::Display for TelemetrySignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Render {
                channel,
                component,
                analytics_id,
                automation_id,
            } => write!(
                f,
                "render channel={channel} component={component} analytics={analytics_id:?} automation={automation_id:?}",
            ),
            Self::Console { channel, message } => {
                write!(f, "console channel={channel} message={message}")
            }
            Self::Checkbox(event) => write!(f, "checkbox telemetry::{event:?}"),
            Self::Switch(event) => write!(f, "switch telemetry::{event:?}"),
            Self::Radio(event) => write!(f, "radio telemetry::{event:?}"),
        }
    }
}

/// Thread-safe collector shared between the runtime component and the
/// test harness.
#[derive(Clone, Default)]
pub struct TelemetryRecorder {
    inner: Arc<Mutex<Vec<TelemetrySignal>>>,
}

impl PartialEq for TelemetryRecorder {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl TelemetryRecorder {
    /// Record a telemetry signal and mirror it to stdout for developers.
    pub fn record(&self, signal: TelemetrySignal) {
        println!("telemetry::{}", signal);
        if let Ok(mut guard) = self.inner.lock() {
            guard.push(signal);
        }
    }

    /// Convenience helper for console-only signals.
    pub fn record_console(&self, channel: &'static str, message: String) {
        self.record(TelemetrySignal::Console { channel, message });
    }

    /// Drain the collected signals for assertion driven tests.
    #[must_use]
    pub fn drain(&self) -> Vec<TelemetrySignal> {
        if let Ok(mut guard) = self.inner.lock() {
            let drained = guard.clone();
            guard.clear();
            drained
        } else {
            Vec::new()
        }
    }
}

/// Dioxus properties passed to [`selection_controls_app`].
#[derive(Clone, PartialEq, Props)]
pub struct SelectionControlsProps {
    /// Shared recorder so hydration, desktop, and web runners observe the
    /// same telemetry sequence during smoke tests.
    pub recorder: TelemetryRecorder,
}

/// Helper that fabricates [`TelemetryHooks`] for the supplied channel.
fn build_telemetry_hooks(channel: &'static str, recorder: TelemetryRecorder) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection-controls.dioxus.{channel}"));
    hooks.automation_id = Some(format!("automation.selection-controls.{channel}"));
    hooks.on_render = Some(Arc::new(move |context: TelemetryContext| {
        recorder.record(TelemetrySignal::Render {
            channel,
            component: context.component,
            analytics_id: context.analytics_id.clone(),
            automation_id: context.automation_id.clone(),
        });
    }));
    hooks
}

/// Internal harness holding the headless state machines and telemetry
/// configuration. The harness keeps `Rc<RefCell<...>>` handles so event
/// closures can mutate state without rebuilding the entire component tree.
struct SelectionControlHarness {
    recorder: TelemetryRecorder,
    checkbox_state: Rc<RefCell<CheckboxState>>,
    switch_state: Rc<RefCell<SwitchState>>,
    radio_state: Rc<RefCell<RadioGroupState>>,
    checkbox_label: String,
    switch_label: String,
    radio_telemetry: TelemetryHooks,
    checkbox_telemetry: TelemetryHooks,
    switch_telemetry: TelemetryHooks,
    radio_component_hooks: TelemetryHooks,
}

impl SelectionControlHarness {
    fn new(recorder: TelemetryRecorder) -> Self {
        let checkbox_state = Rc::new(RefCell::new(CheckboxState::uncontrolled(false, false)));
        let switch_state = Rc::new(RefCell::new(SwitchState::uncontrolled(false, true)));
        let radio_state = Rc::new(RefCell::new(RadioGroupState::uncontrolled(
            vec!["Cash".into(), "Card".into(), "Invoice".into()],
            false,
            RadioOrientation::Horizontal,
            Some(2),
        )));

        let checkbox_telemetry = build_telemetry_hooks(CHECKBOX_CHANNEL, recorder.clone());
        let switch_telemetry = build_telemetry_hooks(SWITCH_CHANNEL, recorder.clone());
        let radio_telemetry = build_telemetry_hooks(RADIO_CHANNEL, recorder.clone());
        let radio_component_hooks =
            build_telemetry_hooks(RADIO_COMPONENT_CHANNEL, recorder.clone());

        Self {
            recorder,
            checkbox_state,
            switch_state,
            radio_state,
            checkbox_label: "Accept terms".into(),
            switch_label: "Enable quick checkout".into(),
            radio_telemetry,
            checkbox_telemetry,
            switch_telemetry,
            radio_component_hooks,
        }
    }

    /// Replay a representative interaction cycle for tests.
    fn emit_smoke_events(&self) {
        self.emit_checkbox_change();
        self.emit_switch_change();
        self.emit_radio_change();
    }

    fn emit_checkbox_change(&self) {
        let state = self.checkbox_state.borrow();
        let previous = state.checked();
        let next = if state.disabled() {
            previous
        } else {
            match previous {
                CheckboxValue::On => CheckboxValue::Off,
                CheckboxValue::Off => CheckboxValue::On,
                CheckboxValue::Indeterminate => CheckboxValue::On,
            }
        };
        let change = CheckboxChangeEvent {
            previous,
            next,
            disabled: state.disabled(),
            analytics_id: self.checkbox_telemetry.analytics_id.clone(),
            automation_id: self.checkbox_telemetry.automation_id.clone(),
            label: self.checkbox_label.clone(),
        };
        drop(state);
        self.recorder.record_console(
            CHECKBOX_CHANNEL,
            format!(
                "checkbox::change next={:?} disabled={}",
                change.next, change.disabled
            ),
        );
        self.recorder
            .record(TelemetrySignal::Checkbox(CheckboxTelemetryEvent::Change(
                change,
            )));
    }

    fn emit_switch_change(&self) {
        let state = self.switch_state.borrow();
        let change = SwitchChangeEvent {
            previous: state.on(),
            next: if state.disabled() {
                state.on()
            } else {
                !state.on()
            },
            disabled: state.disabled(),
            analytics_id: self.switch_telemetry.analytics_id.clone(),
            automation_id: self.switch_telemetry.automation_id.clone(),
            label: self.switch_label.clone(),
        };
        drop(state);
        self.recorder.record_console(
            SWITCH_CHANNEL,
            format!(
                "switch::change next={} disabled={}",
                change.next, change.disabled
            ),
        );
        self.recorder
            .record(TelemetrySignal::Switch(SwitchTelemetryEvent::Change(
                change,
            )));
    }

    fn emit_radio_change(&self) {
        let state = self.radio_state.borrow();
        let selected = state.selected_index().unwrap_or(0);
        let label = state
            .options()
            .get(selected)
            .cloned()
            .unwrap_or_else(|| "Cash".into());
        let change = RadioChangeEvent {
            previous: state.selected_index(),
            next: selected,
            disabled: state.disabled(),
            analytics_id: self.radio_telemetry.analytics_id.clone(),
            automation_id: self.radio_telemetry.automation_id.clone(),
            label,
        };
        drop(state);
        self.recorder.record_console(
            RADIO_CHANNEL,
            format!(
                "radio::change previous={:?} next={} label={}",
                change.previous, change.next, change.label
            ),
        );
        self.recorder
            .record(TelemetrySignal::Radio(RadioTelemetryEvent::Change(change)));
    }
}

fn record_render(hooks: &TelemetryHooks, component: &'static str) {
    if let Some(callback) = hooks.on_render.as_ref() {
        let context = TelemetryContext::new(component)
            .with_analytics(hooks.analytics_id.clone())
            .with_automation(hooks.automation_id.clone());
        callback(context);
    }
}

fn checkbox_markup<'a>(
    _cx: Scope<'a, SelectionControlsProps>,
    harness: &SelectionControlHarness,
) -> LazyNodes<'a, 'a> {
    record_render(
        &harness.checkbox_telemetry,
        "selection_controls_dioxus::CheckboxControl",
    );
    let recorder = harness.recorder.clone();
    let state_handle = harness.checkbox_state.clone();
    let label = harness.checkbox_label.clone();
    let analytics = harness.checkbox_telemetry.analytics_id.clone();
    let automation = harness.checkbox_telemetry.automation_id.clone();
    let telemetry_recorder = harness.recorder.clone();
    let (checked, disabled, indeterminate) = {
        let state = state_handle.borrow();
        (
            state.is_checked(),
            state.disabled(),
            state.is_indeterminate(),
        )
    };
    let aria_checked = if indeterminate {
        "mixed"
    } else if checked {
        "true"
    } else {
        "false"
    };
    let tabindex = if disabled { "-1" } else { "0" };

    rsx! {
        label { class: "control checkbox", style: "display:flex;align-items:center;gap:12px;",
            input {
                r#type: "checkbox",
                checked: checked,
                disabled: disabled,
                aria_checked: aria_checked,
                tabindex: tabindex,
                oninput: move |_| {
                    let mut state = state_handle.borrow_mut();
                    let previous = state.checked();
                    let disabled = state.disabled();
                    let next = if disabled {
                        previous
                    } else {
                        match previous {
                            CheckboxValue::On => CheckboxValue::Off,
                            CheckboxValue::Off => CheckboxValue::On,
                            CheckboxValue::Indeterminate => CheckboxValue::On,
                        }
                    };
                    let change = CheckboxChangeEvent {
                        previous,
                        next,
                        disabled,
                        analytics_id: analytics.clone(),
                        automation_id: automation.clone(),
                        label: label.clone(),
                    };
                    recorder.record_console(
                        CHECKBOX_CHANNEL,
                        format!("checkbox::change next={:?} disabled={}", change.next, change.disabled),
                    );
                    telemetry_recorder.record(TelemetrySignal::Checkbox(
                        CheckboxTelemetryEvent::Change(change.clone()),
                    ));
                    if !disabled {
                        state.sync_checked(next);
                    }
                }
            }
            span { label.clone() }
        }
    }
}

fn switch_markup<'a>(
    _cx: Scope<'a, SelectionControlsProps>,
    harness: &SelectionControlHarness,
) -> LazyNodes<'a, 'a> {
    record_render(
        &harness.switch_telemetry,
        "selection_controls_dioxus::SwitchControl",
    );
    let state_handle = harness.switch_state.clone();
    let label = harness.switch_label.clone();
    let analytics = harness.switch_telemetry.analytics_id.clone();
    let automation = harness.switch_telemetry.automation_id.clone();
    let recorder = harness.recorder.clone();
    let (checked, disabled) = {
        let state = state_handle.borrow();
        (state.on(), state.disabled())
    };

    rsx! {
        label { class: "control switch", style: "display:flex;align-items:center;gap:12px;",
            input {
                r#type: "checkbox",
                role: "switch",
                checked: checked,
                disabled: disabled,
                oninput: move |_| {
                    let mut state = state_handle.borrow_mut();
                    let previous = state.on();
                    let disabled = state.disabled();
                    let next = if disabled { previous } else { !previous };
                    let change = SwitchChangeEvent {
                        previous,
                        next,
                        disabled,
                        analytics_id: analytics.clone(),
                        automation_id: automation.clone(),
                        label: label.clone(),
                    };
                    recorder.record_console(
                        SWITCH_CHANNEL,
                        format!("switch::change next={} disabled={}", change.next, change.disabled),
                    );
                    recorder.record(TelemetrySignal::Switch(
                        SwitchTelemetryEvent::Change(change.clone()),
                    ));
                    if !disabled {
                        state.sync_on(next);
                    }
                }
            }
            span { label.clone() }
        }
    }
}

fn radio_markup<'a>(
    _cx: Scope<'a, SelectionControlsProps>,
    harness: &SelectionControlHarness,
) -> LazyNodes<'a, 'a> {
    record_render(
        &harness.radio_telemetry,
        "selection_controls_dioxus::RadioGroupControl",
    );
    record_render(
        &harness.radio_component_hooks,
        "selection_controls_dioxus::RadioGroupShell",
    );
    let state_handle = harness.radio_state.clone();
    let options = harness.radio_state.borrow().options().to_vec();
    let analytics = harness.radio_telemetry.analytics_id.clone();
    let automation = harness.radio_telemetry.automation_id.clone();
    let recorder = harness.recorder.clone();

    rsx! {
        fieldset { class: "control radio-group", style: "display:flex;gap:16px;align-items:center;",
            legend { "Payment method" }
            {options.iter().enumerate().map(|(index, option)| {
                let state_handle = state_handle.clone();
                let label = option.clone();
                let analytics = analytics.clone();
                let automation = automation.clone();
                let recorder = recorder.clone();
                let (is_selected, disabled) = {
                    let state = state_handle.borrow();
                    (state.selected_index() == Some(index), state.disabled())
                };
                rsx! {
                    label { style: "display:flex;align-items:center;gap:8px;",
                        input {
                            r#type: "radio",
                            name: "checkout-method",
                            value: "{label}",
                            checked: is_selected,
                            disabled: disabled,
                            oninput: move |_| {
                                let mut state = state_handle.borrow_mut();
                                let disabled = state.disabled();
                                let previous = state.selected_index();
                                if disabled {
                                    return;
                                }
                                state.select(index, |_| {});
                                let change = RadioChangeEvent {
                                    previous,
                                    next: index,
                                    disabled,
                                    analytics_id: analytics.clone(),
                                    automation_id: automation.clone(),
                                    label: label.clone(),
                                };
                                recorder.record_console(
                                    RADIO_CHANNEL,
                                    format!(
                                        "radio::change previous={:?} next={} label={}",
                                        change.previous, change.next, change.label
                                    ),
                                );
                                recorder.record(TelemetrySignal::Radio(
                                    RadioTelemetryEvent::Change(change),
                                ));
                            }
                        }
                        span { label.clone() }
                    }
                }
            })}
        }
    }
}

/// Top-level Dioxus component that renders the checkbox, switch, and radio
/// group in the README order with comprehensive inline documentation.
#[allow(clippy::too_many_lines)]
pub fn selection_controls_app(cx: Scope<SelectionControlsProps>) -> Element {
    let recorder = cx.props.recorder.clone();
    let harness = use_ref(cx, || SelectionControlHarness::new(recorder));
    let harness_snapshot = harness.read();
    let checkbox = checkbox_markup(cx, &harness_snapshot);
    let switch = switch_markup(cx, &harness_snapshot);
    let radio = radio_markup(cx, &harness_snapshot);

    cx.render(rsx! {
        main {
            class: "selection-controls-shell",
            style: "display:flex;flex-direction:column;gap:24px;padding:32px;font-family:Inter,system-ui,sans-serif;",
            h1 { "RusticUI Selection Controls – Dioxus" }
            p {
                style: "max-width:72ch;color:#94a3b8;",
                "Telemetry hooks mirror the Rust headless state machines so analytics, automation, and hydration stay deterministic."
            }
            {checkbox}
            {switch}
            {radio}
        }
    })
}

/// Build a [`VirtualDom`] configured with telemetry recording. Desktop
/// smoke tests rely on this helper to assert hydration ordering.
#[must_use]
pub fn build_virtual_dom(recorder: TelemetryRecorder) -> VirtualDom {
    VirtualDom::new_with_props(selection_controls_app, SelectionControlsProps { recorder })
}

/// Generate a fresh harness and emit the telemetry smoke cycle for tests.
#[must_use]
pub fn simulate_telemetry_cycle(recorder: TelemetryRecorder) -> Vec<TelemetrySignal> {
    let harness = SelectionControlHarness::new(recorder.clone());
    harness.emit_smoke_events();
    recorder.drain()
}

/// Launch the desktop runner when the `desktop` feature is enabled.
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub fn run_desktop() {
    use dioxus_desktop::{launch::launch_virtual_dom, Config};

    let recorder = TelemetryRecorder::default();
    let dom = build_virtual_dom(recorder);
    let config = Config::default().with_window(|builder| {
        builder
            .with_title("RusticUI – Selection Controls (Dioxus)")
            .with_resizable(true)
    });
    launch_virtual_dom(dom, config);
}

/// Launch the web runner when compiled for WebAssembly.
#[cfg(all(feature = "web", target_arch = "wasm32"))]
pub fn run_web() {
    use dioxus_web::Config;

    let recorder = TelemetryRecorder::default();
    dioxus_web::launch_with_props(
        selection_controls_app,
        SelectionControlsProps { recorder },
        Config::new().hydrate(true),
    );
}

#[cfg(test)]
pub(crate) fn test_harness(recorder: TelemetryRecorder) -> SelectionControlHarness {
    SelectionControlHarness::new(recorder)
}
