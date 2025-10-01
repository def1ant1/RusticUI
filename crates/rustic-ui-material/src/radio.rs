//! Material radio group built atop the headless [`RadioGroupState`].
//!
//! Feature gates expose first-class components for each supported framework
//! while the shared [`RadioGroupDescriptor`](crate::selection_control::RadioGroupDescriptor)
//! guarantees consistent styling and automation hooks:
//!
//! * `react` – [`react::ReactRadioGroup`] yields [`Jsx`] through the
//!   `wasm_bindgen` bridge, wiring descriptors directly into React elements.
//! * `yew` – [`yew::YewRadioGroup`] leverages `#[function_component]` so Yew apps
//!   can bind to the group without string conversions.
//! * `leptos` – [`leptos::LeptosRadioGroup`] composes with `#[component]` and
//!   returns a [`leptos::View`].
//! * `dioxus` – [`dioxus::DioxusRadioGroup`] uses `rsx!` for idiomatic Dioxus
//!   rendering.
//! * `sycamore` – [`sycamore::SycamoreRadioGroup`] returns a Sycamore
//!   [`Template`](sycamore::view::Template) for signal driven dashboards.
//!
//! Each adapter reads from the same descriptor so automation selectors and ARIA
//! metadata stay synchronized across frameworks and SSR pipelines.

use rustic_ui_headless::{
    interaction::ControlKey,
    radio::{RadioGroupState, RadioOrientation},
};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::{
    selection_control::{self, RadioGroupDescriptor, RadioOptionDescriptor},
    telemetry::{instrument_render, TelemetryContext, TelemetryHooks},
};

/// Telemetry payload emitted when radio option analytics identifiers are
/// observed by the adapter event handlers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioAnalyticsEvent {
    /// Index of the option that triggered the interaction.
    pub index: usize,
    /// Index currently selected before the event is processed.
    pub selected: Option<usize>,
    /// Whether the group is disabled, mirroring the descriptor attributes.
    pub disabled: bool,
    /// Analytics identifier mirrored from the descriptor, if present.
    pub analytics_id: Option<String>,
    /// Automation identifier mirrored from the descriptor, if present.
    pub automation_id: Option<String>,
    /// Human readable label rendered beside the faux radio control.
    pub label: String,
}

/// Telemetry payload emitted when an option gains or loses focus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioFocusEvent {
    /// Index of the option that changed focus state.
    pub index: usize,
    /// Whether focus is now active (`true`) or cleared (`false`).
    pub focused: bool,
    /// Whether the group is disabled at the time of the focus change.
    pub disabled: bool,
    /// Analytics identifier mirrored from the descriptor, if present.
    pub analytics_id: Option<String>,
    /// Automation identifier mirrored from the descriptor, if present.
    pub automation_id: Option<String>,
    /// Human readable label rendered beside the faux radio control.
    pub label: String,
}

/// Telemetry payload emitted whenever the selection intent changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioChangeEvent {
    /// Previously selected index prior to the change.
    pub previous: Option<usize>,
    /// Index requested by the interaction.
    pub next: usize,
    /// Whether the group is disabled when the change was requested.
    pub disabled: bool,
    /// Analytics identifier mirrored from the descriptor, if present.
    pub analytics_id: Option<String>,
    /// Automation identifier mirrored from the descriptor, if present.
    pub automation_id: Option<String>,
    /// Label describing the option the user attempted to select.
    pub label: String,
}

/// Telemetry payload emitted after [`RadioGroupState::select`] completes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioCommitEvent {
    /// Selected index reported after the mutation request finishes.
    pub selected: Option<usize>,
    /// Whether the state machine is operating in controlled mode.
    pub controlled: bool,
    /// Analytics identifier mirrored from the descriptor, if present.
    pub analytics_id: Option<String>,
    /// Automation identifier mirrored from the descriptor, if present.
    pub automation_id: Option<String>,
    /// Label describing the option confirmed by the commit.
    pub label: String,
}

/// Canonical payload emitted for keyboard interactions across adapters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioKeyEvent {
    /// Normalised control key derived from the browser event.
    pub key: ControlKey,
    /// Previously selected index prior to the key interaction.
    pub previous: Option<usize>,
    /// Next index requested by the interaction (if any).
    pub next: Option<usize>,
    /// Whether the group was disabled while the event fired.
    pub disabled: bool,
    /// Analytics identifier mirrored from the descriptor, if present.
    pub analytics_id: Option<String>,
    /// Automation identifier mirrored from the descriptor, if present.
    pub automation_id: Option<String>,
    /// Human friendly label rendered beside the originating option.
    pub label: String,
}

/// Unified telemetry event surfaced to React consumers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RadioTelemetryEvent {
    /// Analytics metadata observed prior to any other telemetry.
    Analytics(RadioAnalyticsEvent),
    /// Focus gained for a specific option.
    Focus(RadioFocusEvent),
    /// Focus lost for a specific option.
    Blur(RadioFocusEvent),
    /// Selection intent captured before state mutation.
    Change(RadioChangeEvent),
    /// Final selection snapshot captured after the commit path completes.
    Commit(RadioCommitEvent),
}

fn build_analytics_event(
    option: &RadioOptionSnapshot,
    state: &RadioGroupState,
    index: usize,
) -> RadioAnalyticsEvent {
    RadioAnalyticsEvent {
        index,
        selected: state.selected_index(),
        disabled: state.disabled(),
        analytics_id: option.analytics_id.clone(),
        automation_id: option.automation_id.clone(),
        label: option.label.clone(),
    }
}

fn build_focus_event(
    option: &RadioOptionSnapshot,
    state: &RadioGroupState,
    index: usize,
    focused: bool,
) -> RadioFocusEvent {
    RadioFocusEvent {
        index,
        focused,
        disabled: state.disabled(),
        analytics_id: option.analytics_id.clone(),
        automation_id: option.automation_id.clone(),
        label: option.label.clone(),
    }
}

fn build_change_event(
    option: &RadioOptionSnapshot,
    previous: Option<usize>,
    next: usize,
    disabled: bool,
) -> RadioChangeEvent {
    RadioChangeEvent {
        previous,
        next,
        disabled,
        analytics_id: option.analytics_id.clone(),
        automation_id: option.automation_id.clone(),
        label: option.label.clone(),
    }
}

fn build_commit_event(
    option: &RadioOptionSnapshot,
    selected: Option<usize>,
    controlled: bool,
) -> RadioCommitEvent {
    RadioCommitEvent {
        selected,
        controlled,
        analytics_id: option.analytics_id.clone(),
        automation_id: option.automation_id.clone(),
        label: option.label.clone(),
    }
}

fn build_key_event(
    option: &RadioOptionSnapshot,
    key: ControlKey,
    previous: Option<usize>,
    next: Option<usize>,
    disabled: bool,
) -> RadioKeyEvent {
    RadioKeyEvent {
        key,
        previous,
        next,
        disabled,
        analytics_id: option.analytics_id.clone(),
        automation_id: option.automation_id.clone(),
        label: option.label.clone(),
    }
}

fn control_key_from_str(key: &str) -> Option<ControlKey> {
    match key {
        " " | "Space" | "Spacebar" => Some(ControlKey::Space),
        "Enter" => Some(ControlKey::Enter),
        "ArrowUp" => Some(ControlKey::ArrowUp),
        "ArrowDown" => Some(ControlKey::ArrowDown),
        "ArrowLeft" => Some(ControlKey::ArrowLeft),
        "ArrowRight" => Some(ControlKey::ArrowRight),
        "Home" => Some(ControlKey::Home),
        "End" => Some(ControlKey::End),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadioGroupProps {
    /// Optional custom labels for each option. When omitted the state's option
    /// names are reused.
    pub option_labels: Vec<String>,
    /// Telemetry hooks invoked when rendering adapters for analytics and
    /// automation instrumentation.
    pub telemetry: TelemetryHooks,
}

impl RadioGroupProps {
    pub fn new(option_labels: impl Into<Vec<String>>) -> Self {
        Self {
            option_labels: option_labels.into(),
            telemetry: TelemetryHooks::default(),
        }
    }

    pub fn from_state(state: &RadioGroupState) -> Self {
        Self {
            option_labels: state.options().to_vec(),
            telemetry: TelemetryHooks::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_telemetry(mut self, telemetry: TelemetryHooks) -> Self {
        self.telemetry = telemetry;
        self
    }
}

#[allow(dead_code)]
fn build_descriptor(
    props: &RadioGroupProps,
    telemetry: &TelemetryHooks,
    state: &RadioGroupState,
) -> RadioGroupDescriptor {
    let orientation_value = match state.orientation() {
        RadioOrientation::Horizontal => "horizontal",
        RadioOrientation::Vertical => "vertical",
    };

    let labels = if props.option_labels.is_empty() {
        state.options().to_vec()
    } else {
        props.option_labels.clone()
    };

    let mut descriptor = RadioGroupDescriptor::new(themed_radio_group_style())
        .with_group_attributes(state.group_aria_attributes())
        .group_attribute("data-orientation", orientation_value);

    descriptor = apply_group_telemetry(descriptor, telemetry);

    for (index, option) in state.options().iter().enumerate() {
        let label = labels.get(index).cloned().unwrap_or_else(|| option.clone());
        let option_descriptor = RadioOptionDescriptor::new(label, themed_radio_option_style())
            .with_attributes(state.option_aria_attributes(index))
            .attribute("data-index", index.to_string());
        let option_descriptor = apply_option_telemetry(option_descriptor, telemetry);
        descriptor = descriptor.option(option_descriptor);
    }

    descriptor
}

fn apply_group_telemetry(
    mut descriptor: RadioGroupDescriptor,
    telemetry: &TelemetryHooks,
) -> RadioGroupDescriptor {
    let has_analytics = descriptor
        .data_state_attributes()
        .any(|(key, _)| key == "data-rustic-analytics-id");
    if !has_analytics {
        if let Some(analytics) = &telemetry.analytics_id {
            descriptor = descriptor.group_attribute("data-rustic-analytics-id", analytics.clone());
        }
    }

    let has_automation = descriptor
        .data_state_attributes()
        .any(|(key, _)| key == "data-automation-id");
    if !has_automation {
        if let Some(automation) = &telemetry.automation_id {
            descriptor = descriptor.group_attribute("data-automation-id", automation.clone());
        }
    }

    descriptor
}

fn apply_option_telemetry(
    mut option: RadioOptionDescriptor,
    telemetry: &TelemetryHooks,
) -> RadioOptionDescriptor {
    let has_analytics = option
        .data_state_attributes()
        .any(|(key, _)| key == "data-rustic-analytics-id");
    if !has_analytics {
        if let Some(analytics) = &telemetry.analytics_id {
            option = option.attribute("data-rustic-analytics-id", analytics.clone());
        }
    }

    let has_automation = option
        .data_state_attributes()
        .any(|(key, _)| key == "data-automation-id");
    if !has_automation {
        if let Some(automation) = &telemetry.automation_id {
            option = option.attribute("data-automation-id", automation.clone());
        }
    }

    option
}

#[allow(dead_code)]
fn render_html(props: &RadioGroupProps, state: &RadioGroupState) -> String {
    let telemetry = props.telemetry.clone();
    let (context, descriptor, _snapshot) = descriptor_with_context(
        "rustic_ui_material::radio::render_html",
        props,
        &telemetry,
        state,
    );
    instrument_render(&telemetry, context, || {
        selection_control::render_radio_group_html(&descriptor)
    })
}

fn merged_telemetry(primary: &TelemetryHooks, fallback: &TelemetryHooks) -> TelemetryHooks {
    TelemetryHooks {
        analytics_id: primary
            .analytics_id
            .clone()
            .or_else(|| fallback.analytics_id.clone()),
        automation_id: primary
            .automation_id
            .clone()
            .or_else(|| fallback.automation_id.clone()),
        span: primary.span.clone().or_else(|| fallback.span.clone()),
        on_render: primary
            .on_render
            .clone()
            .or_else(|| fallback.on_render.clone()),
        on_error: primary
            .on_error
            .clone()
            .or_else(|| fallback.on_error.clone()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RadioOptionSnapshot {
    label: String,
    themed_attributes: Vec<(String, String)>,
    class: String,
    role: String,
    aria_checked: String,
    aria_disabled: Option<String>,
    tabindex: String,
    data_checked: String,
    data_focus_visible: String,
    data_index: String,
    analytics_id: Option<String>,
    automation_id: Option<String>,
}

impl RadioOptionSnapshot {
    fn from_descriptor(descriptor: &RadioOptionDescriptor) -> Self {
        let themed_attributes = descriptor.themed_attributes();
        let mut class = String::new();
        let mut role = String::from("radio");
        let mut aria_checked = String::from("false");
        let mut aria_disabled = None;
        let mut tabindex = String::from("0");
        let mut data_checked = String::from("false");
        let mut data_focus_visible = String::from("false");
        let mut data_index = String::from("0");
        let mut analytics_id = None;
        let mut automation_id = None;

        for (key, value) in &themed_attributes {
            match key.as_str() {
                "class" => class = value.clone(),
                "role" => role = value.clone(),
                "aria-checked" => aria_checked = value.clone(),
                "aria-disabled" => aria_disabled = Some(value.clone()),
                "tabindex" => tabindex = value.clone(),
                "data-checked" => data_checked = value.clone(),
                "data-focus-visible" => data_focus_visible = value.clone(),
                "data-index" => data_index = value.clone(),
                "data-rustic-analytics-id" => analytics_id = Some(value.clone()),
                "data-automation-id" => automation_id = Some(value.clone()),
                _ => {}
            }
        }

        Self {
            label: descriptor.label().to_string(),
            themed_attributes,
            class,
            role,
            aria_checked,
            aria_disabled,
            tabindex,
            data_checked,
            data_focus_visible,
            data_index,
            analytics_id,
            automation_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RadioGroupDescriptorSnapshot {
    label: String,
    group_thematic_attributes: Vec<(String, String)>,
    class: String,
    role: String,
    aria_orientation: String,
    aria_disabled: Option<String>,
    data_orientation: String,
    analytics_id: Option<String>,
    automation_id: Option<String>,
    options: Vec<RadioOptionSnapshot>,
}

impl RadioGroupDescriptorSnapshot {
    fn from_descriptor(descriptor: &RadioGroupDescriptor) -> Self {
        let group_thematic_attributes = descriptor.group_thematic_attributes();
        let mut class = String::new();
        let mut role = String::from("radiogroup");
        let mut aria_orientation = String::from("horizontal");
        let mut aria_disabled = None;
        let mut data_orientation = String::from("horizontal");
        let mut analytics_id = None;
        let mut automation_id = None;

        for (key, value) in &group_thematic_attributes {
            match key.as_str() {
                "class" => class = value.clone(),
                "role" => role = value.clone(),
                "aria-orientation" => aria_orientation = value.clone(),
                "aria-disabled" => aria_disabled = Some(value.clone()),
                "data-orientation" => data_orientation = value.clone(),
                "data-rustic-analytics-id" => analytics_id = Some(value.clone()),
                "data-automation-id" => automation_id = Some(value.clone()),
                _ => {}
            }
        }

        let options = descriptor
            .options()
            .iter()
            .map(RadioOptionSnapshot::from_descriptor)
            .collect::<Vec<_>>();

        let label = format!("radio-group::{}-options", options.len());

        Self {
            label,
            group_thematic_attributes,
            class,
            role,
            aria_orientation,
            aria_disabled,
            data_orientation,
            analytics_id,
            automation_id,
            options,
        }
    }
}

fn descriptor_with_context(
    component: &'static str,
    props: &RadioGroupProps,
    telemetry: &TelemetryHooks,
    state: &RadioGroupState,
) -> (
    TelemetryContext,
    RadioGroupDescriptor,
    RadioGroupDescriptorSnapshot,
) {
    let descriptor = build_descriptor(props, telemetry, state);
    let snapshot = RadioGroupDescriptorSnapshot::from_descriptor(&descriptor);
    let context = TelemetryContext::new(component)
        .with_analytics(telemetry.analytics_id.clone())
        .with_automation(telemetry.automation_id.clone())
        .with_descriptor_metadata(
            snapshot.label.clone(),
            snapshot.group_thematic_attributes.clone(),
        );
    (context, descriptor, snapshot)
}

/// Generates layout styling for the radio group container, including
/// orientation-aware flex direction toggles.
#[allow(dead_code)]
fn themed_radio_group_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        flex-direction: column;
        gap: ${gap};

        &[data-orientation='horizontal'] {
            flex-direction: row;
        }

        &[aria-disabled='true'] {
            opacity: 0.38;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
    )
}

/// Visual styling for individual radio options including the faux dot used to
/// communicate selection.
#[allow(dead_code)]
fn themed_radio_option_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        cursor: pointer;
        font-family: ${font_family};
        font-size: ${font_size};
        color: ${text_color};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};

        &::before {
            content: "";
            width: ${dot_size};
            height: ${dot_size};
            border-radius: 9999px;
            border: 2px solid ${border_color};
            margin-right: ${gap};
            box-sizing: border-box;
            transition: background-color 160ms ease, border-color 160ms ease;
        }

        &[data-checked='true']::before {
            background: ${checked_background};
            border-color: ${checked_background};
        }

        &[data-focus-visible='true'] {
            outline: ${focus_outline_width} solid ${focus_outline_color};
            outline-offset: 2px;
        }

        &[aria-disabled='true'] {
            cursor: not-allowed;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body1),
        text_color = theme.palette.text_primary.clone(),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        radius = format!("{}px", theme.joy.radius),
        dot_size = format!("{}px", theme.spacing(1)),
        border_color = theme.palette.text_secondary.clone(),
        checked_background = theme.palette.primary.clone(),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
}

#[cfg(feature = "react")]
pub mod react {
    //! React adapter returning [`Jsx`] nodes via the shared descriptor.
    use super::*;
    use js_sys::{Array, Function, Object, Reflect};
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};

    /// Type alias representing React elements emitted by the adapter.
    pub type Jsx = JsValue;

    /// Properties accepted by the React radio group component.
    #[derive(Clone, Debug)]
    pub struct ReactRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state describing option metadata and focus handling.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the React render.
        pub telemetry: TelemetryHooks,
        /// Optional React `onChange` handler executed after telemetry.
        pub on_change: Option<Function>,
        /// Optional React `onFocus` handler executed after telemetry.
        pub on_focus: Option<Function>,
        /// Optional React `onBlur` handler executed after telemetry.
        pub on_blur: Option<Function>,
        /// Optional React `onKeyDown` handler executed after telemetry.
        pub on_key_down: Option<Function>,
        /// Optional telemetry delegate receiving structured payloads.
        pub telemetry_delegate: Option<Function>,
    }

    impl ReactRadioGroupProps {
        /// Convenience constructor mirroring the previous two-field struct so
        /// downstream callers remain source compatible.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key_down: None,
                telemetry_delegate: None,
            }
        }

        #[allow(dead_code)]
        pub fn with_telemetry(mut self, telemetry: TelemetryHooks) -> Self {
            self.telemetry = telemetry;
            self
        }
    }

    fn function_option_eq(lhs: &Option<Function>, rhs: &Option<Function>) -> bool {
        match (lhs, rhs) {
            (Some(a), Some(b)) => JsValue::from(a).strict_eq(&JsValue::from(b)),
            (None, None) => true,
            _ => false,
        }
    }

    impl PartialEq for ReactRadioGroupProps {
        fn eq(&self, other: &Self) -> bool {
            self.group == other.group
                && self.state == other.state
                && self.telemetry == other.telemetry
                && function_option_eq(&self.on_change, &other.on_change)
                && function_option_eq(&self.on_focus, &other.on_focus)
                && function_option_eq(&self.on_blur, &other.on_blur)
                && function_option_eq(&self.on_key_down, &other.on_key_down)
                && function_option_eq(&self.telemetry_delegate, &other.telemetry_delegate)
        }
    }

    fn emit_telemetry(delegate: &Option<Function>, events: &[RadioTelemetryEvent]) {
        if let Some(function) = delegate {
            for event in events {
                let payload = telemetry_event_to_js(event);
                let _ = function.call1(&JsValue::NULL, &payload);
            }
        }
    }

    #[derive(Default)]
    struct ReactOptionHandlers {
        on_select: Option<Function>,
        on_focus: Option<Function>,
        on_blur: Option<Function>,
        on_key: Option<Function>,
    }

    struct ReactOptionHandlerBuilder {
        state: Rc<RefCell<RadioGroupState>>,
        options: Rc<Vec<RadioOptionSnapshot>>,
        on_change: Option<Function>,
        on_focus: Option<Function>,
        on_blur: Option<Function>,
        on_key_down: Option<Function>,
        telemetry_delegate: Option<Function>,
    }

    impl ReactOptionHandlerBuilder {
        fn new(
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            on_change: Option<Function>,
            on_focus: Option<Function>,
            on_blur: Option<Function>,
            on_key_down: Option<Function>,
            telemetry_delegate: Option<Function>,
        ) -> Self {
            Self {
                state,
                options,
                on_change,
                on_focus,
                on_blur,
                on_key_down,
                telemetry_delegate,
            }
        }

        fn build(&self, index: usize) -> ReactOptionHandlers {
            ReactOptionHandlers {
                on_select: self.build_select_handler(index),
                on_focus: self.build_focus_handler(index),
                on_blur: self.build_blur_handler(index),
                on_key: self.build_key_handler(index),
            }
        }

        fn build_select_handler(&self, index: usize) -> Option<Function> {
            if self.on_change.is_none() && self.telemetry_delegate.is_none() {
                return None;
            }

            let telemetry = self.telemetry_delegate.clone();
            let on_change = self.on_change.clone();
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();

            // Each handler adheres to the same telemetry choreography:
            // 1. Emit analytics metadata immediately so upstream pipelines
            //    capture the raw interaction context before mutation.
            // 2. Emit the domain specific event (change in this case).
            // 3. Invoke the shared headless state machine (`select`).
            // 4. Emit the commit snapshot reflecting the resulting state.
            // 5. Delegate to user provided callbacks, guaranteeing side
            //    effects only observe telemetry that already shipped.
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let (analytics_event, previous, controlled, disabled) = {
                    let state_ref = state.borrow();
                    (
                        RadioTelemetryEvent::Analytics(build_analytics_event(
                            &option, &state_ref, index,
                        )),
                        state_ref.selected_index(),
                        state_ref.is_controlled(),
                        state_ref.disabled(),
                    )
                };

                let mut events = Vec::with_capacity(3);
                events.push(analytics_event);
                events.push(RadioTelemetryEvent::Change(build_change_event(
                    &option, previous, index, disabled,
                )));

                {
                    let mut state_mut = state.borrow_mut();
                    state_mut.select(index, |_| {});
                }

                let selected_after = {
                    let state_ref = state.borrow();
                    state_ref.selected_index().or(Some(index))
                };

                events.push(RadioTelemetryEvent::Commit(build_commit_event(
                    &option,
                    selected_after,
                    controlled,
                )));

                emit_telemetry(&telemetry, &events);

                if let Some(handler) = on_change.as_ref() {
                    let _ = handler.call1(&JsValue::NULL, &event);
                }
            }) as Box<dyn FnMut(JsValue)>);

            let function: Function = closure.as_ref().clone().unchecked_into();
            closure.forget();
            Some(function)
        }

        fn build_focus_handler(&self, index: usize) -> Option<Function> {
            if self.on_focus.is_none() && self.telemetry_delegate.is_none() {
                return None;
            }

            let telemetry = self.telemetry_delegate.clone();
            let on_focus = self.on_focus.clone();
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();

            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let (analytics_event, focus_event) = {
                    let state_ref = state.borrow();
                    (
                        RadioTelemetryEvent::Analytics(build_analytics_event(
                            &option, &state_ref, index,
                        )),
                        RadioTelemetryEvent::Focus(build_focus_event(
                            &option, &state_ref, index, true,
                        )),
                    )
                };

                let mut events = Vec::with_capacity(2);
                events.push(analytics_event);
                events.push(focus_event);
                emit_telemetry(&telemetry, &events);

                {
                    let mut state_mut = state.borrow_mut();
                    state_mut.focus(index);
                }

                if let Some(handler) = on_focus.as_ref() {
                    let _ = handler.call1(&JsValue::NULL, &event);
                }
            }) as Box<dyn FnMut(JsValue)>);

            let function: Function = closure.as_ref().clone().unchecked_into();
            closure.forget();
            Some(function)
        }

        fn build_blur_handler(&self, index: usize) -> Option<Function> {
            if self.on_blur.is_none() && self.telemetry_delegate.is_none() {
                return None;
            }

            let telemetry = self.telemetry_delegate.clone();
            let on_blur = self.on_blur.clone();
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();

            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let (analytics_event, blur_event) = {
                    let state_ref = state.borrow();
                    (
                        RadioTelemetryEvent::Analytics(build_analytics_event(
                            &option, &state_ref, index,
                        )),
                        RadioTelemetryEvent::Blur(build_focus_event(
                            &option, &state_ref, index, false,
                        )),
                    )
                };

                let mut events = Vec::with_capacity(2);
                events.push(analytics_event);
                events.push(blur_event);
                emit_telemetry(&telemetry, &events);

                {
                    let mut state_mut = state.borrow_mut();
                    state_mut.blur();
                }

                if let Some(handler) = on_blur.as_ref() {
                    let _ = handler.call1(&JsValue::NULL, &event);
                }
            }) as Box<dyn FnMut(JsValue)>);

            let function: Function = closure.as_ref().clone().unchecked_into();
            closure.forget();
            Some(function)
        }

        fn build_key_handler(&self, index: usize) -> Option<Function> {
            if self.on_key_down.is_none()
                && self.on_change.is_none()
                && self.telemetry_delegate.is_none()
            {
                return None;
            }

            let telemetry = self.telemetry_delegate.clone();
            let on_key = self.on_key_down.clone();
            let on_change = self.on_change.clone();
            let state = Rc::clone(&self.state);
            let options = Rc::clone(&self.options);
            let origin_option = self.options[index].clone();

            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let key = Reflect::get(&event, &JsValue::from_str("key"))
                    .ok()
                    .and_then(|value| value.as_string())
                    .and_then(|value| control_key_from_str(&value));

                if let Some(control) = key {
                    if let Ok(prevent) = Reflect::get(&event, &JsValue::from_str("preventDefault"))
                    {
                        if let Ok(prevent) = prevent.dyn_into::<Function>() {
                            let _ = prevent.call0(&event);
                        }
                    }

                    let (analytics_event, previous, controlled, disabled) = {
                        let state_ref = state.borrow();
                        (
                            RadioTelemetryEvent::Analytics(build_analytics_event(
                                &origin_option,
                                &state_ref,
                                index,
                            )),
                            state_ref.selected_index(),
                            state_ref.is_controlled(),
                            state_ref.disabled(),
                        )
                    };

                    let mut events = Vec::with_capacity(5);
                    events.push(analytics_event);

                    let selected_after = Rc::new(RefCell::new(None));
                    {
                        let mut state_mut = state.borrow_mut();
                        let recorder = Rc::clone(&selected_after);
                        state_mut.on_key(control, move |selected| {
                            recorder.borrow_mut().replace(selected);
                        });
                    }

                    if let Some(next_index) = *selected_after.borrow() {
                        let focused_option = options[next_index].clone();
                        let focus_event = {
                            let state_ref = state.borrow();
                            RadioTelemetryEvent::Focus(build_focus_event(
                                &focused_option,
                                &state_ref,
                                next_index,
                                true,
                            ))
                        };
                        events.push(focus_event);

                        if next_index != index {
                            let blur_event = {
                                let state_ref = state.borrow();
                                RadioTelemetryEvent::Blur(build_focus_event(
                                    &origin_option,
                                    &state_ref,
                                    index,
                                    false,
                                ))
                            };
                            events.push(blur_event);
                        }

                        events.push(RadioTelemetryEvent::Change(build_change_event(
                            &focused_option,
                            previous,
                            next_index,
                            disabled,
                        )));

                        let committed = {
                            let state_ref = state.borrow();
                            state_ref.selected_index().or(Some(next_index))
                        };

                        events.push(RadioTelemetryEvent::Commit(build_commit_event(
                            &focused_option,
                            committed,
                            controlled,
                        )));
                    }

                    emit_telemetry(&telemetry, &events);

                    if let Some(handler) = on_key.as_ref() {
                        let _ = handler.call1(&JsValue::NULL, &event);
                    }

                    if let Some(handler) = on_change.as_ref() {
                        let _ = handler.call1(&JsValue::NULL, &event);
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            let function: Function = closure.as_ref().clone().unchecked_into();
            closure.forget();
            Some(function)
        }
    }

    fn create_element(tag: &str, props: Object, children: &[JsValue]) -> JsValue {
        let global = js_sys::global();
        let react = Reflect::get(&global, &JsValue::from_str("React"))
            .expect("React global missing; ensure it is registered before rendering");
        let create_element = Reflect::get(&react, &JsValue::from_str("createElement"))
            .expect("React.createElement missing")
            .dyn_into::<Function>()
            .expect("React.createElement should be callable");

        let args = Array::new();
        args.push(&JsValue::from_str(tag));
        args.push(&props.into());
        for child in children {
            args.push(child);
        }

        create_element
            .apply(&JsValue::NULL, &args)
            .expect("React.createElement invocation")
    }

    fn build_props_object(pairs: Vec<(String, String)>) -> Object {
        let object = Object::new();
        for (key, value) in pairs {
            Reflect::set(
                &object,
                &JsValue::from_str(&key),
                &JsValue::from_str(&value),
            )
            .expect("set React prop");
        }
        object
    }

    fn build_option_props_object(
        option: &RadioOptionSnapshot,
        handlers: &ReactOptionHandlers,
    ) -> Object {
        let object = Object::new();
        for (key, value) in &option.themed_attributes {
            Reflect::set(&object, &JsValue::from_str(key), &JsValue::from_str(value))
                .expect("set option prop");
        }

        if let Some(handler) = &handlers.on_select {
            Reflect::set(&object, &JsValue::from_str("onClick"), handler)
                .expect("set onClick handler");
            Reflect::set(&object, &JsValue::from_str("onclick"), handler)
                .expect("set onclick handler");
            Reflect::set(&object, &JsValue::from_str("onChange"), handler)
                .expect("set onChange handler");
            Reflect::set(&object, &JsValue::from_str("onchange"), handler)
                .expect("set onchange handler");
        }

        if let Some(handler) = &handlers.on_focus {
            Reflect::set(&object, &JsValue::from_str("onFocus"), handler)
                .expect("set onFocus handler");
            Reflect::set(&object, &JsValue::from_str("onfocus"), handler)
                .expect("set onfocus handler");
        }

        if let Some(handler) = &handlers.on_blur {
            Reflect::set(&object, &JsValue::from_str("onBlur"), handler)
                .expect("set onBlur handler");
            Reflect::set(&object, &JsValue::from_str("onblur"), handler)
                .expect("set onblur handler");
        }

        if let Some(handler) = &handlers.on_key {
            Reflect::set(&object, &JsValue::from_str("onKeyDown"), handler)
                .expect("set onKeyDown handler");
            Reflect::set(&object, &JsValue::from_str("onkeydown"), handler)
                .expect("set onkeydown handler");
        }

        object
    }

    fn push_optional_string(
        object: &Object,
        key: &str,
        value: &Option<String>,
    ) -> Result<(), JsValue> {
        if let Some(value) = value {
            Reflect::set(object, &JsValue::from_str(key), &JsValue::from_str(value))?;
        }
        Ok(())
    }

    fn push_optional_usize(
        object: &Object,
        key: &str,
        value: Option<usize>,
    ) -> Result<(), JsValue> {
        if let Some(value) = value {
            Reflect::set(
                object,
                &JsValue::from_str(key),
                &JsValue::from_f64(value as f64),
            )?;
        }
        Ok(())
    }

    fn telemetry_event_to_js(event: &RadioTelemetryEvent) -> JsValue {
        use RadioTelemetryEvent as Event;

        let object = Object::new();
        match event {
            Event::Analytics(analytics) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("analytics"),
                )
                .expect("set kind");
                Reflect::set(
                    &object,
                    &JsValue::from_str("index"),
                    &JsValue::from_f64(analytics.index as f64),
                )
                .expect("set index");
                push_optional_usize(&object, "selected", analytics.selected).expect("set selected");
                Reflect::set(
                    &object,
                    &JsValue::from_str("disabled"),
                    &JsValue::from_bool(analytics.disabled),
                )
                .expect("set disabled");
                push_optional_string(&object, "analyticsId", &analytics.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &analytics.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&analytics.label),
                )
                .expect("set label");
            }
            Event::Focus(focus) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("focus"),
                )
                .expect("set kind");
                Reflect::set(
                    &object,
                    &JsValue::from_str("index"),
                    &JsValue::from_f64(focus.index as f64),
                )
                .expect("set index");
                Reflect::set(
                    &object,
                    &JsValue::from_str("focused"),
                    &JsValue::from_bool(focus.focused),
                )
                .expect("set focused");
                Reflect::set(
                    &object,
                    &JsValue::from_str("disabled"),
                    &JsValue::from_bool(focus.disabled),
                )
                .expect("set disabled");
                push_optional_string(&object, "analyticsId", &focus.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &focus.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&focus.label),
                )
                .expect("set label");
            }
            Event::Blur(blur) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("blur"),
                )
                .expect("set kind");
                Reflect::set(
                    &object,
                    &JsValue::from_str("index"),
                    &JsValue::from_f64(blur.index as f64),
                )
                .expect("set index");
                Reflect::set(
                    &object,
                    &JsValue::from_str("focused"),
                    &JsValue::from_bool(blur.focused),
                )
                .expect("set focused");
                Reflect::set(
                    &object,
                    &JsValue::from_str("disabled"),
                    &JsValue::from_bool(blur.disabled),
                )
                .expect("set disabled");
                push_optional_string(&object, "analyticsId", &blur.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &blur.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&blur.label),
                )
                .expect("set label");
            }
            Event::Change(change) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("change"),
                )
                .expect("set kind");
                push_optional_usize(&object, "previous", change.previous).expect("set previous");
                Reflect::set(
                    &object,
                    &JsValue::from_str("next"),
                    &JsValue::from_f64(change.next as f64),
                )
                .expect("set next");
                Reflect::set(
                    &object,
                    &JsValue::from_str("disabled"),
                    &JsValue::from_bool(change.disabled),
                )
                .expect("set disabled");
                push_optional_string(&object, "analyticsId", &change.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &change.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&change.label),
                )
                .expect("set label");
            }
            Event::Commit(commit) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("commit"),
                )
                .expect("set kind");
                push_optional_usize(&object, "selected", commit.selected).expect("set selected");
                Reflect::set(
                    &object,
                    &JsValue::from_str("controlled"),
                    &JsValue::from_bool(commit.controlled),
                )
                .expect("set controlled");
                push_optional_string(&object, "analyticsId", &commit.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &commit.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&commit.label),
                )
                .expect("set label");
            }
        }

        object.into()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::{cell::RefCell, rc::Rc};
        use wasm_bindgen::JsValue;
        use wasm_bindgen_test::*;

        wasm_bindgen_test_configure!(run_in_browser);

        fn sample_state_uncontrolled() -> RadioGroupState {
            RadioGroupState::uncontrolled(
                vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()],
                false,
                RadioOrientation::Horizontal,
                Some(0),
            )
        }

        fn sample_state_controlled() -> RadioGroupState {
            RadioGroupState::controlled(
                vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()],
                false,
                RadioOrientation::Horizontal,
                Some(1),
            )
        }

        fn build_snapshot(state: &RadioGroupState) -> RadioGroupDescriptorSnapshot {
            let props = RadioGroupProps::from_state(state);
            let telemetry = TelemetryHooks::default();
            let (_ctx, _descriptor, snapshot) = super::super::descriptor_with_context(
                "rustic_ui_material::radio::react::tests::snapshot",
                &props,
                &telemetry,
                state,
            );
            snapshot
        }

        fn telemetry_collector() -> (Function, js_sys::Array) {
            let events = js_sys::Array::new();
            let stored = events.clone();
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                stored.push(&event);
            }) as Box<dyn FnMut(JsValue)>);
            let function: Function = closure.as_ref().clone().unchecked_into();
            closure.forget();
            (function, events)
        }

        fn build_option_props(
            state: Rc<RefCell<RadioGroupState>>,
            snapshot: &RadioGroupDescriptorSnapshot,
            telemetry: Option<Function>,
            index: usize,
        ) -> (Object, Rc<RefCell<RadioGroupState>>) {
            let options = Rc::new(snapshot.options.clone());
            let builder = ReactOptionHandlerBuilder::new(
                Rc::clone(&state),
                Rc::clone(&options),
                None,
                None,
                None,
                None,
                telemetry,
            );
            let handlers = builder.build(index);
            let props = build_option_props_object(&options[index], &handlers);
            (props, state)
        }

        fn call_handler(props: &Object, key: &str) {
            if let Ok(handler) = Reflect::get(props, &JsValue::from_str(key)) {
                if let Ok(function) = handler.dyn_into::<Function>() {
                    let _ = function.call1(&JsValue::NULL, &JsValue::UNDEFINED);
                }
            }
        }

        fn event_kinds(events: &js_sys::Array) -> Vec<String> {
            events
                .iter()
                .map(|value| {
                    Reflect::get(&value, &JsValue::from_str("kind"))
                        .ok()
                        .and_then(|kind| kind.as_string())
                        .unwrap_or_default()
                })
                .collect()
        }

        #[wasm_bindgen_test]
        fn uncontrolled_click_emits_change_commit_sequence() {
            let state = Rc::new(RefCell::new(sample_state_uncontrolled()));
            let snapshot = build_snapshot(&state.borrow());
            let (delegate, events) = telemetry_collector();
            let (props, state_handle) =
                build_option_props(Rc::clone(&state), &snapshot, Some(delegate), 1);

            call_handler(&props, "onClick");

            let kinds = event_kinds(&events);
            assert_eq!(kinds, vec!["analytics", "change", "commit"]);
            assert_eq!(state_handle.borrow().selected_index(), Some(1));
        }

        #[wasm_bindgen_test]
        fn controlled_click_reports_commit_without_mutating_state() {
            let state = Rc::new(RefCell::new(sample_state_controlled()));
            let snapshot = build_snapshot(&state.borrow());
            let (delegate, events) = telemetry_collector();
            let (props, state_handle) =
                build_option_props(Rc::clone(&state), &snapshot, Some(delegate), 2);

            call_handler(&props, "onClick");

            assert_eq!(state_handle.borrow().selected_index(), Some(1));
            let last_event = events.get(events.length() - 1);
            let selected = Reflect::get(&last_event, &JsValue::from_str("selected"))
                .unwrap()
                .as_f64()
                .map(|value| value as usize);
            assert_eq!(selected, Some(2));
            let kinds = event_kinds(&events);
            assert_eq!(kinds, vec!["analytics", "change", "commit"]);
        }

        #[wasm_bindgen_test]
        fn preserves_option_attributes() {
            let state = Rc::new(RefCell::new(sample_state_uncontrolled()));
            let snapshot = build_snapshot(&state.borrow());
            let options = Rc::new(snapshot.options.clone());
            let builder = ReactOptionHandlerBuilder::new(
                Rc::clone(&state),
                Rc::clone(&options),
                None,
                None,
                None,
                None,
                None,
            );
            let handlers = builder.build(0);
            let props = build_option_props_object(&options[0], &handlers);

            let role = Reflect::get(&props, &JsValue::from_str("role"))
                .unwrap()
                .as_string()
                .unwrap();
            let aria_checked = Reflect::get(&props, &JsValue::from_str("aria-checked"))
                .unwrap()
                .as_string()
                .unwrap();
            let data_index = Reflect::get(&props, &JsValue::from_str("data-index"))
                .unwrap()
                .as_string()
                .unwrap();

            assert_eq!(role, "radio");
            assert_eq!(aria_checked, "true");
            assert_eq!(data_index, "0");
        }
    }

    /// React component rendering a Material radio group.
    pub fn ReactRadioGroup(props: &ReactRadioGroupProps) -> Jsx {
        let telemetry = super::merged_telemetry(&props.telemetry, &props.group.telemetry);
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::react::ReactRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        instrument_render(&telemetry, context, || {
            let group_props = build_props_object(snapshot.group_thematic_attributes.clone());
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let options = Rc::new(snapshot.options.clone());
            let handler_builder = ReactOptionHandlerBuilder::new(
                Rc::clone(&state_handle),
                Rc::clone(&options),
                props.on_change.clone(),
                props.on_focus.clone(),
                props.on_blur.clone(),
                props.on_key_down.clone(),
                props.telemetry_delegate.clone(),
            );

            let option_children: Vec<JsValue> = options
                .iter()
                .enumerate()
                .map(|(index, option)| {
                    let handlers = handler_builder.build(index);
                    let option_props = build_option_props_object(option, &handlers);
                    create_element("span", option_props, &[JsValue::from_str(&option.label)])
                })
                .collect();
            create_element("div", group_props, option_children.as_slice())
        })
    }
}

#[cfg(feature = "yew")]
pub mod yew {
    //! Yew adapter implemented with `#[function_component]` for idiomatic usage.
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    use yew::events::{Event, FocusEvent, KeyboardEvent, MouseEvent};
    use yew::html::{onblur, onchange, onclick, onfocus, onkeydown};
    use yew::prelude::*;
    use yew::virtual_dom::{Listener, VNode};

    /// Properties accepted by [`YewRadioGroup`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Yew render lifecycle.
        #[prop_or_default]
        pub telemetry: TelemetryHooks,
        /// Optional change callback invoked with [`RadioChangeEvent`].
        #[prop_or_default]
        pub on_change: Option<Callback<RadioChangeEvent>>,
        /// Optional focus callback invoked when an option gains focus.
        #[prop_or_default]
        pub on_focus: Option<Callback<RadioFocusEvent>>,
        /// Optional blur callback invoked when an option loses focus.
        #[prop_or_default]
        pub on_blur: Option<Callback<RadioFocusEvent>>,
        /// Optional keyboard callback invoked with normalized key payloads.
        #[prop_or_default]
        pub on_key: Option<Callback<RadioKeyEvent>>,
        /// Optional telemetry delegate receiving structured payloads.
        #[prop_or_default]
        pub telemetry_delegate: Option<Callback<RadioTelemetryEvent>>,
    }

    fn emit_telemetry(
        delegate: &Option<Callback<RadioTelemetryEvent>>,
        events: &[RadioTelemetryEvent],
    ) {
        if let Some(callback) = delegate {
            for event in events {
                callback.emit(event.clone());
            }
        }
    }

    #[derive(Clone)]
    struct YewOptionHandlers {
        onclick: Rc<dyn Listener>,
        onchange: Rc<dyn Listener>,
        onfocus: Rc<dyn Listener>,
        onblur: Rc<dyn Listener>,
        onkeydown: Rc<dyn Listener>,
    }

    #[derive(Clone)]
    struct YewOptionHandlerBuilder {
        state: Rc<RefCell<RadioGroupState>>,
        options: Rc<Vec<RadioOptionSnapshot>>,
        on_change: Option<Callback<RadioChangeEvent>>,
        on_focus: Option<Callback<RadioFocusEvent>>,
        on_blur: Option<Callback<RadioFocusEvent>>,
        on_key: Option<Callback<RadioKeyEvent>>,
        telemetry_delegate: Option<Callback<RadioTelemetryEvent>>,
    }

    impl YewOptionHandlerBuilder {
        fn new(
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            on_change: Option<Callback<RadioChangeEvent>>,
            on_focus: Option<Callback<RadioFocusEvent>>,
            on_blur: Option<Callback<RadioFocusEvent>>,
            on_key: Option<Callback<RadioKeyEvent>>,
            telemetry_delegate: Option<Callback<RadioTelemetryEvent>>,
        ) -> Self {
            Self {
                state,
                options,
                on_change,
                on_focus,
                on_blur,
                on_key,
                telemetry_delegate,
            }
        }

        fn build(&self, index: usize) -> YewOptionHandlers {
            let (onclick, onchange) = self.build_select_handlers(index);
            let onfocus = self.build_focus_handler(index);
            let onblur = self.build_blur_handler(index);
            let onkeydown = self.build_key_handler(index);
            YewOptionHandlers {
                onclick,
                onchange,
                onfocus,
                onblur,
                onkeydown,
            }
        }

        fn build_select_handlers(&self, index: usize) -> (Rc<dyn Listener>, Rc<dyn Listener>) {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_change = self.on_change.clone();

            // Wrap the choreography in a single runner so both `onclick` and `onchange`
            // listeners can delegate to the same logic without duplicating telemetry
            // ordering. Automation pipelines rely on this deterministic sequence:
            // 1. Capture analytics metadata before mutating any state.
            // 2. Emit the `RadioChangeEvent` snapshot requested by the interaction.
            // 3. Drive the shared [`RadioGroupState`] via [`RadioGroupState::select`].
            // 4. Emit the post-mutation `RadioCommitEvent` snapshot.
            // 5. Finally invoke user callbacks, ensuring analytics is always emitted
            //    before consumer side effects run.
            let runner: Rc<RefCell<Box<dyn FnMut()>>> = Rc::new(RefCell::new(Box::new({
                move || {
                    let (analytics_event, previous, controlled, disabled) = {
                        let state_ref = state.borrow();
                        (
                            RadioTelemetryEvent::Analytics(build_analytics_event(
                                &option, &state_ref, index,
                            )),
                            state_ref.selected_index(),
                            state_ref.is_controlled(),
                            state_ref.disabled(),
                        )
                    };

                    let change_event = build_change_event(&option, previous, index, disabled);
                    let mut telemetry_events = Vec::with_capacity(3);
                    telemetry_events.push(analytics_event);
                    telemetry_events.push(RadioTelemetryEvent::Change(change_event.clone()));

                    {
                        let mut state_mut = state.borrow_mut();
                        state_mut.select(index, |_| {});
                        state_mut.focus(index);
                    }

                    let selected_after = {
                        let state_ref = state.borrow();
                        state_ref.selected_index().or(Some(index))
                    };
                    let commit_event = build_commit_event(&option, selected_after, controlled);
                    telemetry_events.push(RadioTelemetryEvent::Commit(commit_event));

                    emit_telemetry(&telemetry, &telemetry_events);

                    if let Some(callback) = &on_change {
                        callback.emit(change_event);
                    }
                }
            })));

            let click_runner = Rc::clone(&runner);
            let onclick_callback = Callback::from(move |_event: MouseEvent| {
                (click_runner.borrow_mut())();
            });
            let onchange_runner = Rc::clone(&runner);
            let onchange_callback = Callback::from(move |_event: Event| {
                (onchange_runner.borrow_mut())();
            });

            let onclick_listener: Rc<dyn Listener> =
                Rc::new(onclick::Wrapper::new(onclick_callback));
            let onchange_listener: Rc<dyn Listener> =
                Rc::new(onchange::Wrapper::new(onchange_callback));

            (onclick_listener, onchange_listener)
        }

        fn build_focus_handler(&self, index: usize) -> Rc<dyn Listener> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_focus = self.on_focus.clone();

            let callback = Callback::from(move |_event: FocusEvent| {
                let (analytics_event, focus_payload) = {
                    let state_ref = state.borrow();
                    (
                        RadioTelemetryEvent::Analytics(build_analytics_event(
                            &option, &state_ref, index,
                        )),
                        build_focus_event(&option, &state_ref, index, true),
                    )
                };

                let telemetry_events = vec![
                    analytics_event,
                    RadioTelemetryEvent::Focus(focus_payload.clone()),
                ];
                emit_telemetry(&telemetry, &telemetry_events);

                {
                    let mut state_mut = state.borrow_mut();
                    state_mut.focus(index);
                }

                if let Some(callback) = &on_focus {
                    callback.emit(focus_payload);
                }
            });

            Rc::new(onfocus::Wrapper::new(callback))
        }

        fn build_blur_handler(&self, index: usize) -> Rc<dyn Listener> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_blur = self.on_blur.clone();

            let callback = Callback::from(move |_event: FocusEvent| {
                let (analytics_event, blur_payload) = {
                    let state_ref = state.borrow();
                    (
                        RadioTelemetryEvent::Analytics(build_analytics_event(
                            &option, &state_ref, index,
                        )),
                        build_focus_event(&option, &state_ref, index, false),
                    )
                };

                let telemetry_events = vec![
                    analytics_event,
                    RadioTelemetryEvent::Blur(blur_payload.clone()),
                ];
                emit_telemetry(&telemetry, &telemetry_events);

                {
                    let mut state_mut = state.borrow_mut();
                    state_mut.blur();
                }

                if let Some(callback) = &on_blur {
                    callback.emit(blur_payload);
                }
            });

            Rc::new(onblur::Wrapper::new(callback))
        }

        fn build_key_handler(&self, index: usize) -> Rc<dyn Listener> {
            let state = Rc::clone(&self.state);
            let options = Rc::clone(&self.options);
            let telemetry = self.telemetry_delegate.clone();
            let on_key = self.on_key.clone();
            let on_change = self.on_change.clone();
            let origin_option = self.options[index].clone();

            let callback = Callback::from(move |event: KeyboardEvent| {
                if let Some(control) = control_key_from_str(event.key().as_str()) {
                    event.prevent_default();

                    let (analytics_event, previous, controlled, disabled) = {
                        let state_ref = state.borrow();
                        (
                            RadioTelemetryEvent::Analytics(build_analytics_event(
                                &origin_option,
                                &state_ref,
                                index,
                            )),
                            state_ref.selected_index(),
                            state_ref.is_controlled(),
                            state_ref.disabled(),
                        )
                    };

                    let selected_after = Rc::new(RefCell::new(None));
                    {
                        let mut state_mut = state.borrow_mut();
                        let recorder = Rc::clone(&selected_after);
                        state_mut.on_key(control, move |selected| {
                            recorder.borrow_mut().replace(selected);
                        });
                    }

                    let mut telemetry_events = Vec::with_capacity(5);
                    telemetry_events.push(analytics_event);

                    let next_index = *selected_after.borrow();
                    let mut change_payload = None;
                    let mut key_payload =
                        build_key_event(&origin_option, control, previous, next_index, disabled);

                    if let Some(next_index) = next_index {
                        let focused_option = options[next_index].clone();
                        let focus_event = {
                            let state_ref = state.borrow();
                            RadioTelemetryEvent::Focus(build_focus_event(
                                &focused_option,
                                &state_ref,
                                next_index,
                                true,
                            ))
                        };
                        telemetry_events.push(focus_event);

                        if next_index != index {
                            let blur_event = {
                                let state_ref = state.borrow();
                                RadioTelemetryEvent::Blur(build_focus_event(
                                    &origin_option,
                                    &state_ref,
                                    index,
                                    false,
                                ))
                            };
                            telemetry_events.push(blur_event);
                        }

                        let change_event =
                            build_change_event(&focused_option, previous, next_index, disabled);
                        telemetry_events.push(RadioTelemetryEvent::Change(change_event.clone()));

                        let committed = {
                            let state_ref = state.borrow();
                            state_ref.selected_index().or(Some(next_index))
                        };
                        telemetry_events.push(RadioTelemetryEvent::Commit(build_commit_event(
                            &focused_option,
                            committed,
                            controlled,
                        )));

                        change_payload = Some(change_event);
                    }

                    emit_telemetry(&telemetry, &telemetry_events);

                    if let Some(callback) = &on_key {
                        callback.emit(key_payload);
                    }

                    if let (Some(callback), Some(change_event)) =
                        (on_change.as_ref(), change_payload)
                    {
                        callback.emit(change_event);
                    }
                }
            });

            Rc::new(onkeydown::Wrapper::new(callback))
        }
    }

    /// Radio group rendered via Yew.
    ///
    /// The implementation mirrors the enterprise telemetry choreography used by
    /// the React adapter to keep analytics, automation and state transitions in
    /// lockstep:
    ///
    /// * [`TelemetryHooks`] from the props and [`RadioGroupProps`] are merged so
    ///   analytics identifiers flow consistently regardless of where they are
    ///   configured.
    /// * [`descriptor_with_context`] seeds a [`TelemetryContext`] that captures
    ///   the fully-qualified component path and descriptor metadata.
    /// * Event handler factories centralise wiring per option, guaranteeing each
    ///   closure emits telemetry **before** invoking consumer callbacks while
    ///   also funnelling mutations through the shared [`RadioGroupState`].
    /// * Pointer and keyboard interactions emit analytics → change → commit in
    ///   order, focus transitions emit analytics → focus/blur, and keyboard
    ///   flows optionally append change/commit snapshots when selection changes.
    ///
    /// Extensive inline documentation exists so governance teams can audit the
    /// behaviour and so future contributors can extend the logic without
    /// repeating boilerplate across options.
    #[function_component(YewRadioGroup)]
    pub fn yew_radio_group(props: &YewRadioGroupProps) -> Html {
        let telemetry = super::merged_telemetry(&props.telemetry, &props.group.telemetry);
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::yew::YewRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        instrument_render(&telemetry, context, move || {
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let options = Rc::new(snapshot.options.clone());
            let handler_builder = Rc::new(YewOptionHandlerBuilder::new(
                Rc::clone(&state_handle),
                Rc::clone(&options),
                props.on_change.clone(),
                props.on_focus.clone(),
                props.on_blur.clone(),
                props.on_key.clone(),
                props.telemetry_delegate.clone(),
            ));

            let option_nodes = options.iter().enumerate().map({
                let builder = Rc::clone(&handler_builder);
                move |(index, option)| {
                    let option_snapshot = option.clone();
                    let handlers = builder.build(index);
                    let mut child = html! { <span>{option_snapshot.label.clone()}</span> };
                    if let VNode::VTag(ref mut tag) = child {
                        for (key, value) in option_snapshot.themed_attributes.clone() {
                            tag.add_attribute(key, value);
                        }
                        tag.add_listener(Rc::clone(&handlers.onclick));
                        tag.add_listener(Rc::clone(&handlers.onchange));
                        tag.add_listener(Rc::clone(&handlers.onfocus));
                        tag.add_listener(Rc::clone(&handlers.onblur));
                        tag.add_listener(Rc::clone(&handlers.onkeydown));
                    }
                    child
                }
            });

            let mut node = html! { <div>{ for option_nodes }</div> };

            if let VNode::VTag(ref mut tag) = node {
                for (key, value) in snapshot.group_thematic_attributes.clone() {
                    tag.add_attribute(key, value);
                }
            }

            node
        })
    }
}

#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter returning a [`leptos::View`] built from the descriptor
    //! metadata.
    use super::*;
    use leptos::prelude::*;

    /// Properties accepted by [`LeptosRadioGroup`].
    #[derive(Clone)]
    pub struct LeptosRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Leptos render lifecycle.
        pub telemetry: TelemetryHooks,
    }

    impl LeptosRadioGroupProps {
        /// Convenience constructor retaining backward-compatible ergonomics.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
            }
        }
    }

    #[component]
    pub fn LeptosRadioGroup(props: LeptosRadioGroupProps) -> impl IntoView {
        let telemetry = super::merged_telemetry(&props.telemetry, &props.group.telemetry);
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::leptos::LeptosRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        instrument_render(&telemetry, context, move || {
            let option_views: Vec<View> = snapshot
                .options
                .iter()
                .map(|option| {
                    view! {
                        <span
                            class=option.class.clone()
                            role=option.role.clone()
                            aria-checked=option.aria_checked.clone()
                            aria-disabled=option.aria_disabled.clone()
                            tabindex=option.tabindex.clone()
                            data_checked=option.data_checked.clone()
                            data_focus_visible=option.data_focus_visible.clone()
                            data_index=option.data_index.clone()
                            data_rustic_analytics_id=option.analytics_id.clone()
                            data_automation_id=option.automation_id.clone()
                        >{option.label.clone()}</span>
                    }
                })
                .collect();

            let options_fragment = View::new_fragment(option_views);

            view! {
                <div
                    class=snapshot.class.clone()
                    role=snapshot.role.clone()
                    aria-orientation=snapshot.aria_orientation.clone()
                    aria-disabled=snapshot.aria_disabled.clone()
                    data-orientation=snapshot.data_orientation.clone()
                    data_rustic_analytics_id=snapshot.analytics_id.clone()
                    data_automation_id=snapshot.automation_id.clone()
                >{options_fragment}</div>
            }
        })
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter constructed with `rsx!` for idiomatic use in Dioxus apps.
    use super::*;
    use dioxus::prelude::*;

    /// Properties accepted by [`DioxusRadioGroup`].
    #[derive(Props, Clone, PartialEq)]
    pub struct DioxusRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Dioxus render lifecycle.
        #[props(default = None)]
        pub telemetry: Option<TelemetryHooks>,
    }

    /// Radio group rendered as a Dioxus component.
    pub fn DioxusRadioGroup(cx: Scope<DioxusRadioGroupProps>) -> Element {
        let props = cx.props();
        let telemetry_override = props.telemetry.clone().unwrap_or_default();
        let telemetry = super::merged_telemetry(&telemetry_override, &props.group.telemetry);
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::dioxus::DioxusRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        let scope = cx;
        instrument_render(&telemetry, context, move || {
            let options = snapshot.options.clone();
            scope.render(rsx! {
                div {
                    class: snapshot.class.clone(),
                    role: snapshot.role.clone(),
                    aria_orientation: snapshot.aria_orientation.clone(),
                    aria_disabled: snapshot.aria_disabled.clone(),
                    data_orientation: snapshot.data_orientation.clone(),
                    data_rustic_analytics_id: snapshot.analytics_id.clone(),
                    data_automation_id: snapshot.automation_id.clone(),
                    { options.iter().map(|option| {
                        let label = option.label.clone();
                        rsx! {
                            span {
                                class: option.class.clone(),
                                role: option.role.clone(),
                                aria_checked: option.aria_checked.clone(),
                                aria_disabled: option.aria_disabled.clone(),
                                tabindex: option.tabindex.clone(),
                                data_checked: option.data_checked.clone(),
                                data_focus_visible: option.data_focus_visible.clone(),
                                data_index: option.data_index.clone(),
                                data_rustic_analytics_id: option.analytics_id.clone(),
                                data_automation_id: option.automation_id.clone(),
                                {label}
                            }
                        }
                    }) }
                }
            })
        })
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter returning a [`Template`] for reactive dashboards.
    use super::*;
    use sycamore::prelude::*;

    /// Alias matching Sycamore's view representation.
    pub type Template<G> = View<G>;

    /// Properties accepted by [`SycamoreRadioGroup`].
    #[derive(Clone)]
    pub struct SycamoreRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Sycamore render lifecycle.
        pub telemetry: TelemetryHooks,
    }

    impl SycamoreRadioGroupProps {
        /// Convenience constructor mirroring the previous struct layout.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
            }
        }
    }

    /// Radio group rendered within a Sycamore reactive scope.
    #[component]
    pub fn SycamoreRadioGroup<G: Html>(cx: Scope, props: SycamoreRadioGroupProps) -> Template<G> {
        let telemetry = super::merged_telemetry(&props.telemetry, &props.group.telemetry);
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::sycamore::SycamoreRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        instrument_render(&telemetry, context, move || {
            let option_views: Vec<View<G>> = snapshot
                .options
                .iter()
                .map(|option| {
                    let label = option.label.clone();
                    view! { cx,
                        span(
                            class=option.class.clone(),
                            role=option.role.clone(),
                            aria_checked=option.aria_checked.clone(),
                            aria_disabled=option.aria_disabled.clone(),
                            tabindex=option.tabindex.clone(),
                            data_checked=option.data_checked.clone(),
                            data_focus_visible=option.data_focus_visible.clone(),
                            data_index=option.data_index.clone(),
                            data_rustic_analytics_id=option.analytics_id.clone(),
                            data_automation_id=option.automation_id.clone(),
                        ) { (label) }
                    }
                })
                .collect();

            let options_fragment = View::new_fragment(option_views);

            view! { cx,
                div(
                    class=snapshot.class.clone(),
                    role=snapshot.role.clone(),
                    aria_orientation=snapshot.aria_orientation.clone(),
                    aria_disabled=snapshot.aria_disabled.clone(),
                    data_orientation=snapshot.data_orientation.clone(),
                    data_rustic_analytics_id=snapshot.analytics_id.clone(),
                    data_automation_id=snapshot.automation_id.clone(),
                ) { (options_fragment) }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_html_includes_all_options() {
        let props = RadioGroupProps::new(vec!["A".to_string(), "B".to_string()]);
        let state = RadioGroupState::uncontrolled(
            vec!["A".into(), "B".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let html = render_html(&props, &state);
        assert!(html.contains("data-index=\"0\""));
        assert!(html.contains("data-index=\"1\""));
    }

    #[test]
    fn descriptor_exposes_aria_metadata() {
        let props = RadioGroupProps::new(vec!["A".to_string(), "B".to_string()]);
        let state = RadioGroupState::uncontrolled(
            vec!["A".into(), "B".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let descriptor = build_descriptor(&props, &props.telemetry, &state);
        assert!(descriptor.aria_attributes().any(|(k, _)| k == "role"));
        assert!(descriptor.options().iter().any(|option| option
            .aria_attributes()
            .any(|(k, v)| k == "aria-checked" && v == "true")));
    }
}
