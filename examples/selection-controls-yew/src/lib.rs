use std::fmt;
use std::sync::{Arc, Mutex};

use rustic_ui_headless::checkbox::{CheckboxState, CheckboxValue};
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_material::checkbox::{
    yew::YewCheckbox, CheckboxChangeEvent, CheckboxFocusEvent, CheckboxProps,
    CheckboxTelemetryEvent,
};
use rustic_ui_material::radio::{
    yew::YewRadioGroup, RadioChangeEvent, RadioFocusEvent, RadioGroupProps, RadioKeyEvent,
    RadioTelemetryEvent,
};
use rustic_ui_material::selection_control::{
    RadioGroupAttributes, RadioOptionAttributes, SelectionControlDescriptor,
    SelectionControlTelemetry, SelectionControlThemeTokens,
};
use rustic_ui_material::switch::{
    yew::YewSwitch, SwitchChangeEvent, SwitchFocusEvent, SwitchProps, SwitchTelemetryEvent,
};
use rustic_ui_material::telemetry::{
    instrument_render, TelemetryAnalyticsPayload, TelemetryCommitPayload, TelemetryContext,
    TelemetryError, TelemetryFocusPayload, TelemetryHooks, TelemetryStateChangePayload,
};
use rustic_ui_styled_engine::{css, Style};
use yew::prelude::*;

/// Structured capture of instrumentation emitted while the demos execute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedEvent {
    /// Logical control channel (e.g. `checkbox.controlled`).
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

/// Thread-safe recorder shared between the host smoke tests and WASM harnesses.
#[derive(Clone, Default)]
pub struct TelemetryRecorder {
    events: Arc<Mutex<Vec<RecordedEvent>>>,
}

impl TelemetryRecorder {
    /// Construct a new recorder.
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
        gloo_console::log!(message);
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
        format!("selection-controls.yew.{}", self.name)
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
    pub fn checkbox_delegate(&self) -> Callback<CheckboxTelemetryEvent> {
        let channel = self.clone();
        Callback::from(move |event: CheckboxTelemetryEvent| {
            channel.record("telemetry", format!("checkbox::{event:?}"));
        })
    }

    /// Delegate that mirrors switch telemetry payloads into the recorder.
    #[must_use]
    pub fn switch_delegate(&self) -> Callback<SwitchTelemetryEvent> {
        let channel = self.clone();
        Callback::from(move |event: SwitchTelemetryEvent| {
            channel.record("telemetry", format!("switch::{event:?}"));
        })
    }

    /// Delegate that mirrors radio telemetry payloads into the recorder.
    #[must_use]
    pub fn radio_delegate(&self) -> Callback<RadioTelemetryEvent> {
        let channel = self.clone();
        Callback::from(move |event: RadioTelemetryEvent| {
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

fn checkbox_props(label: &str, channel: &TelemetryChannel) -> CheckboxProps {
    CheckboxProps::new(label, channel.hooks())
}

fn switch_props(label: &str, channel: &TelemetryChannel) -> SwitchProps {
    SwitchProps::new(label, channel.hooks())
}

fn radio_props(state: &RadioGroupState, channel: &TelemetryChannel) -> RadioGroupProps {
    RadioGroupProps::from_state(state, channel.hooks())
}

fn checkbox_state_summary(state: &CheckboxState) -> String {
    format!(
        "checked={:?} disabled={} focus_visible={}",
        state.checked(),
        state.disabled(),
        state.focus_visible()
    )
}

fn switch_state_summary(state: &SwitchState) -> String {
    format!(
        "on={} disabled={} focus_visible={}",
        state.on(),
        state.disabled(),
        state.focus_visible()
    )
}

fn radio_state_summary(state: &RadioGroupState) -> String {
    format!(
        "selected={:?} focus_visible={:?} disabled={}",
        state.selected_index(),
        state.focus_visible_index(),
        state.disabled()
    )
}

/// Yew component showcasing the selection controls with full telemetry wiring.
#[function_component(SelectionControlsDemo)]
pub fn selection_controls_demo() -> Html {
    let recorder = use_memo(|_| TelemetryRecorder::new(), ());

    // Dedicated telemetry channels per control + ownership mode. We only care about the
    // recorder identity, not its interior value, so memoise against a unit dependency to
    // avoid requiring the recorder itself to implement `PartialEq`.
    let checkbox_controlled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("checkbox.controlled"), ())
    };
    let checkbox_uncontrolled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("checkbox.uncontrolled"), ())
    };
    let switch_controlled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("switch.controlled"), ())
    };
    let switch_uncontrolled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("switch.uncontrolled"), ())
    };
    let radio_controlled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("radio.controlled"), ())
    };
    let radio_uncontrolled_channel = {
        let recorder = recorder.clone();
        use_memo(move |_| recorder.channel("radio.uncontrolled"), ())
    };

    let checkbox_controlled = use_state(|| CheckboxState::controlled(false, CheckboxValue::Off));
    let checkbox_uncontrolled = use_state(|| CheckboxState::uncontrolled(false, CheckboxValue::On));
    let switch_controlled = use_state(|| SwitchState::controlled(false, false));
    let switch_uncontrolled = use_state(|| SwitchState::uncontrolled(false, true));
    let radio_controlled = use_state(|| {
        RadioGroupState::controlled(
            vec!["Email".into(), "SMS".into(), "Push".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        )
    });
    let radio_uncontrolled = use_state(|| {
        RadioGroupState::uncontrolled(
            vec!["Daily".into(), "Weekly".into(), "Monthly".into()],
            false,
            RadioOrientation::Vertical,
            Some(1),
        )
    });

    // Controlled checkbox handlers propagate telemetry before synchronising state.
    let checkbox_controlled_change = {
        let channel = checkbox_controlled_channel.clone();
        let state = checkbox_controlled.clone();
        Callback::from(move |event: CheckboxChangeEvent| {
            record_checkbox_change(&channel, &event);
            let mut next = CheckboxState::controlled(event.disabled, event.next);
            if event.disabled {
                next.set_disabled(true);
            }
            state.set(next);
        })
    };
    let checkbox_controlled_focus = {
        let channel = checkbox_controlled_channel.clone();
        let state = checkbox_controlled.clone();
        Callback::from(move |event: CheckboxFocusEvent| {
            record_checkbox_focus(&channel, &event);
            let mut next = (*state).clone();
            if event.focused {
                next.focus();
            } else {
                next.blur();
            }
            state.set(next);
        })
    };

    // Uncontrolled checkbox logs interactions but does not mutate external state.
    let checkbox_uncontrolled_change = {
        let channel = checkbox_uncontrolled_channel.clone();
        Callback::from(move |event: CheckboxChangeEvent| {
            record_checkbox_change(&channel, &event);
        })
    };
    let checkbox_uncontrolled_focus = {
        let channel = checkbox_uncontrolled_channel.clone();
        Callback::from(move |event: CheckboxFocusEvent| {
            record_checkbox_focus(&channel, &event);
        })
    };

    let switch_controlled_change = {
        let channel = switch_controlled_channel.clone();
        let state = switch_controlled.clone();
        Callback::from(move |event: SwitchChangeEvent| {
            record_switch_change(&channel, &event);
            let mut next = SwitchState::controlled(event.disabled, event.next);
            if event.disabled {
                next.set_disabled(true);
            }
            state.set(next);
        })
    };
    let switch_controlled_focus = {
        let channel = switch_controlled_channel.clone();
        let state = switch_controlled.clone();
        Callback::from(move |event: SwitchFocusEvent| {
            record_switch_focus(&channel, &event);
            let mut next = (*state).clone();
            if event.focused {
                next.focus();
            } else {
                next.blur();
            }
            state.set(next);
        })
    };
    let switch_uncontrolled_change = {
        let channel = switch_uncontrolled_channel.clone();
        Callback::from(move |event: SwitchChangeEvent| {
            record_switch_change(&channel, &event);
        })
    };
    let switch_uncontrolled_focus = {
        let channel = switch_uncontrolled_channel.clone();
        Callback::from(move |event: SwitchFocusEvent| {
            record_switch_focus(&channel, &event);
        })
    };

    let radio_controlled_change = {
        let channel = radio_controlled_channel.clone();
        let state = radio_controlled.clone();
        Callback::from(move |event: RadioChangeEvent| {
            record_radio_change(&channel, &event);
            let mut next = (*state).clone();
            next.sync_selected(Some(event.next));
            state.set(next);
        })
    };
    let radio_controlled_focus = {
        let channel = radio_controlled_channel.clone();
        let state = radio_controlled.clone();
        Callback::from(move |event: RadioFocusEvent| {
            record_radio_focus(&channel, &event);
            let mut next = (*state).clone();
            if event.focused {
                next.focus(event.index);
            } else {
                next.blur();
            }
            state.set(next);
        })
    };
    let radio_controlled_key = {
        let channel = radio_controlled_channel.clone();
        Callback::from(move |event: RadioKeyEvent| {
            record_radio_key(&channel, &event);
        })
    };
    let radio_uncontrolled_change = {
        let channel = radio_uncontrolled_channel.clone();
        let state = radio_uncontrolled.clone();
        Callback::from(move |event: RadioChangeEvent| {
            record_radio_change(&channel, &event);
            let mut next = (*state).clone();
            next.sync_selected(Some(event.next));
            state.set(next);
        })
    };
    let radio_uncontrolled_focus = {
        let channel = radio_uncontrolled_channel.clone();
        let state = radio_uncontrolled.clone();
        Callback::from(move |event: RadioFocusEvent| {
            record_radio_focus(&channel, &event);
            let mut next = (*state).clone();
            if event.focused {
                next.focus(event.index);
            } else {
                next.blur();
            }
            state.set(next);
        })
    };
    let radio_uncontrolled_key = {
        let channel = radio_uncontrolled_channel.clone();
        Callback::from(move |event: RadioKeyEvent| {
            record_radio_key(&channel, &event);
        })
    };

    // Render instrumentation snapshot for deterministic logging in CI.
    for (label, state_summary) in [
        (
            "checkbox.controlled",
            checkbox_state_summary(&*checkbox_controlled),
        ),
        (
            "checkbox.uncontrolled",
            checkbox_state_summary(&*checkbox_uncontrolled),
        ),
        (
            "switch.controlled",
            switch_state_summary(&*switch_controlled),
        ),
        (
            "switch.uncontrolled",
            switch_state_summary(&*switch_uncontrolled),
        ),
        ("radio.controlled", radio_state_summary(&*radio_controlled)),
        (
            "radio.uncontrolled",
            radio_state_summary(&*radio_uncontrolled),
        ),
    ] {
        recorder
            .channel(label)
            .record("initial-state", state_summary);
    }

    html! {
        <main class="selection-controls-demo">
            <section>
                <h2>{"Checkboxes"}</h2>
                <article>
                    <h3>{"Controlled"}</h3>
                    <p class="summary">{format!("State → {}", checkbox_state_summary(&*checkbox_controlled))}</p>
                    <YewCheckbox
                        checkbox={checkbox_props("Receive compliance updates", &checkbox_controlled_channel)}
                        state={( *checkbox_controlled ).clone()}
                        on_change={checkbox_controlled_change}
                        on_focus={checkbox_controlled_focus.clone()}
                        on_blur={checkbox_controlled_focus}
                        telemetry_delegate={checkbox_controlled_channel.checkbox_delegate()}
                    />
                </article>
                <article>
                    <h3>{"Uncontrolled"}</h3>
                    <p class="summary">{format!("State → {}", checkbox_state_summary(&*checkbox_uncontrolled))}</p>
                    <YewCheckbox
                        checkbox={checkbox_props("Enable audit trail", &checkbox_uncontrolled_channel)}
                        state={( *checkbox_uncontrolled ).clone()}
                        on_change={checkbox_uncontrolled_change}
                        on_focus={checkbox_uncontrolled_focus.clone()}
                        on_blur={checkbox_uncontrolled_focus}
                        telemetry_delegate={checkbox_uncontrolled_channel.checkbox_delegate()}
                    />
                </article>
            </section>

            <section>
                <h2>{"Switches"}</h2>
                <article>
                    <h3>{"Controlled"}</h3>
                    <p class="summary">{format!("State → {}", switch_state_summary(&*switch_controlled))}</p>
                    <YewSwitch
                        switch={switch_props("Escalate incidents automatically", &switch_controlled_channel)}
                        state={( *switch_controlled ).clone()}
                        on_change={switch_controlled_change}
                        on_focus={switch_controlled_focus.clone()}
                        on_blur={switch_controlled_focus}
                        telemetry_delegate={switch_controlled_channel.switch_delegate()}
                    />
                </article>
                <article>
                    <h3>{"Uncontrolled"}</h3>
                    <p class="summary">{format!("State → {}", switch_state_summary(&*switch_uncontrolled))}</p>
                    <YewSwitch
                        switch={switch_props("Mirror preferences from mobile", &switch_uncontrolled_channel)}
                        state={( *switch_uncontrolled ).clone()}
                        on_change={switch_uncontrolled_change}
                        on_focus={switch_uncontrolled_focus.clone()}
                        on_blur={switch_uncontrolled_focus}
                        telemetry_delegate={switch_uncontrolled_channel.switch_delegate()}
                    />
                </article>
            </section>

            <section>
                <h2>{"Radio groups"}</h2>
                <article>
                    <h3>{"Controlled"}</h3>
                    <p class="summary">{format!("State → {}", radio_state_summary(&*radio_controlled))}</p>
                    <YewRadioGroup
                        group={radio_props(&*radio_controlled, &radio_controlled_channel)}
                        state={( *radio_controlled ).clone()}
                        telemetry={radio_controlled_channel.hooks()}
                        on_change={radio_controlled_change}
                        on_focus={radio_controlled_focus.clone()}
                        on_blur={radio_controlled_focus}
                        on_key={radio_controlled_key.clone()}
                        telemetry_delegate={radio_controlled_channel.radio_delegate()}
                    />
                </article>
                <article>
                    <h3>{"Uncontrolled"}</h3>
                    <p class="summary">{format!("State → {}", radio_state_summary(&*radio_uncontrolled))}</p>
                    <YewRadioGroup
                        group={radio_props(&*radio_uncontrolled, &radio_uncontrolled_channel)}
                        state={( *radio_uncontrolled ).clone()}
                        telemetry={radio_uncontrolled_channel.hooks()}
                        on_change={radio_uncontrolled_change}
                        on_focus={radio_uncontrolled_focus.clone()}
                        on_blur={radio_uncontrolled_focus}
                        on_key={radio_uncontrolled_key.clone()}
                        telemetry_delegate={radio_uncontrolled_channel.radio_delegate()}
                    />
                </article>
            </section>
        </main>
    }
}

fn themed_checkbox_style() -> Style {
    Style::new(css!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
    "#
    ))
    .expect("checkbox style should compile")
}

fn themed_switch_style() -> Style {
    Style::new(css!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
    "#
    ))
    .expect("switch style should compile")
}

fn themed_radio_group_style() -> Style {
    Style::new(css!(
        r#"
        display: grid;
        gap: 0.75rem;
    "#
    ))
    .expect("radio group style should compile")
}

fn themed_radio_option_style() -> Style {
    Style::new(css!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
    "#
    ))
    .expect("radio option style should compile")
}

#[cfg(not(target_arch = "wasm32"))]
fn checkbox_descriptor(
    state: &CheckboxState,
    channel: &TelemetryChannel,
    label: &str,
) -> SelectionControlDescriptor {
    SelectionControlDescriptor::from_headless(
        label,
        themed_checkbox_style(),
        state,
        &SelectionControlThemeTokens::material_defaults().with_data("variant", "checkbox"),
        &SelectionControlTelemetry::from(channel.hooks()),
    )
    .expect("checkbox descriptor should merge telemetry")
}

#[cfg(not(target_arch = "wasm32"))]
fn switch_descriptor(
    state: &SwitchState,
    channel: &TelemetryChannel,
    label: &str,
) -> SelectionControlDescriptor {
    SelectionControlDescriptor::from_headless(
        label,
        themed_switch_style(),
        state,
        &SelectionControlThemeTokens::material_defaults().with_data("variant", "switch"),
        &SelectionControlTelemetry::from(channel.hooks()),
    )
    .expect("switch descriptor should merge telemetry")
}

#[cfg(not(target_arch = "wasm32"))]
fn radio_attributes(state: &RadioGroupState, channel: &TelemetryChannel) -> RadioGroupAttributes {
    let telemetry = SelectionControlTelemetry::from(channel.hooks());
    let mut group_attributes: Vec<(String, String)> = state
        .group_aria_attributes()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    group_attributes.push((
        "data-orientation".into(),
        match state.orientation() {
            RadioOrientation::Horizontal => "horizontal",
            RadioOrientation::Vertical => "vertical",
        }
        .into(),
    ));

    let (mut builder, _, _) = telemetry
        .merge_into_builder(
            RadioGroupAttributes::builder(themed_radio_group_style()),
            group_attributes,
        )
        .expect("radio group telemetry should merge");

    for (index, option) in state.options().iter().enumerate() {
        let mut option_attributes: Vec<(String, String)> = state
            .option_aria_attributes(index)
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect();
        option_attributes.push(("data-index".into(), index.to_string()));
        let (option_builder, _, _) = telemetry
            .merge_into_builder(
                RadioOptionAttributes::builder(option.clone(), themed_radio_option_style()),
                option_attributes,
            )
            .expect("radio option telemetry should merge");
        builder = builder.option(option_builder.build());
    }

    builder.build()
}

/// SSR snapshots covering each ownership model so CI can diff hydration-ready markup.
#[cfg(not(target_arch = "wasm32"))]
pub fn ssr_snapshots() -> Vec<String> {
    let recorder = TelemetryRecorder::new();
    let checkbox_channel = recorder.channel("checkbox.ssr");
    let switch_channel = recorder.channel("switch.ssr");
    let radio_channel = recorder.channel("radio.ssr");

    let checkbox_controlled = CheckboxState::controlled(false, CheckboxValue::Off);
    let checkbox_uncontrolled = CheckboxState::uncontrolled(false, CheckboxValue::On);
    let switch_controlled = SwitchState::controlled(false, false);
    let switch_uncontrolled = SwitchState::uncontrolled(false, true);
    let radio_controlled = RadioGroupState::controlled(
        vec!["Email".into(), "SMS".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    );
    let radio_uncontrolled = RadioGroupState::uncontrolled(
        vec!["Daily".into(), "Weekly".into()],
        false,
        RadioOrientation::Horizontal,
        Some(1),
    );

    vec![
        checkbox_descriptor(
            &checkbox_controlled,
            &checkbox_channel,
            "Controlled checkbox",
        )
        .into_attributes()
        .to_ssr_html(),
        checkbox_descriptor(
            &checkbox_uncontrolled,
            &checkbox_channel,
            "Uncontrolled checkbox",
        )
        .into_attributes()
        .to_ssr_html(),
        switch_descriptor(&switch_controlled, &switch_channel, "Controlled switch")
            .into_attributes()
            .to_ssr_html(),
        switch_descriptor(&switch_uncontrolled, &switch_channel, "Uncontrolled switch")
            .into_attributes()
            .to_ssr_html(),
        radio_attributes(&radio_controlled, &radio_channel).to_ssr_html(),
        radio_attributes(&radio_uncontrolled, &radio_channel).to_ssr_html(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn ssr_markup_contains_automation_ids() {
        let fragments = ssr_snapshots();
        assert!(fragments
            .iter()
            .all(|html| html.contains("data-automation-id")));
    }

    #[test]
    fn checkbox_telemetry_precedes_change_handler() {
        let recorder = TelemetryRecorder::new();
        let channel = recorder.channel("checkbox.test");
        let hooks = channel.hooks();
        instrument_render(&hooks, TelemetryContext::new("test.checkbox"), || ());

        let change_event = CheckboxChangeEvent {
            previous: CheckboxValue::Off,
            next: CheckboxValue::On,
            disabled: false,
            analytics_id: Some(channel.analytics_id()),
            automation_id: Some(channel.automation_id()),
            label: "Test checkbox".into(),
        };

        channel
            .checkbox_delegate()
            .emit(CheckboxTelemetryEvent::Change(change_event.clone()));
        record_checkbox_change(&channel, &change_event);

        let events = recorder.events();
        assert_eq!(events[0].phase, "render");
        assert_eq!(events[1].phase, "telemetry");
        assert_eq!(events[2].phase, "change-handler");
    }

    #[test]
    fn radio_keyboard_sequence_is_tracked() {
        let recorder = TelemetryRecorder::new();
        let channel = recorder.channel("radio.test");
        let hooks = channel.hooks();
        instrument_render(&hooks, TelemetryContext::new("test.radio"), || ());

        let key_event = RadioKeyEvent {
            key: rustic_ui_headless::interaction::ControlKey::ArrowRight,
            previous: Some(0),
            next: Some(1),
            disabled: false,
            analytics_id: Some(channel.analytics_id()),
            automation_id: Some(channel.automation_id()),
            label: "Email".into(),
        };

        channel
            .radio_delegate()
            .emit(RadioTelemetryEvent::Key(key_event.clone()));
        record_radio_key(&channel, &key_event);

        let events = recorder.events();
        assert_eq!(events[0].phase, "render");
        assert_eq!(events[1].phase, "telemetry");
        assert_eq!(events[2].phase, "key");
    }
}
