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
    /// Keyboard interaction payload captured before focus or selection updates.
    Key(RadioKeyEvent),
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

fn preview_keyboard_target(state: &RadioGroupState, key: ControlKey) -> Option<usize> {
    if state.disabled() || state.len() == 0 {
        return None;
    }

    let len = state.len();
    let mut focus_index = state
        .focus_visible_index()
        .or(state.selected_index())
        .unwrap_or(0)
        .min(len - 1);

    match key {
        ControlKey::Space | ControlKey::Enter => Some(focus_index),
        ControlKey::Home => Some(0),
        ControlKey::End => Some(len - 1),
        ControlKey::ArrowRight => match state.orientation() {
            RadioOrientation::Horizontal => Some((focus_index + 1) % len),
            RadioOrientation::Vertical => None,
        },
        ControlKey::ArrowDown => Some((focus_index + 1) % len),
        ControlKey::ArrowLeft => match state.orientation() {
            RadioOrientation::Horizontal => Some(if focus_index == 0 {
                len - 1
            } else {
                focus_index - 1
            }),
            RadioOrientation::Vertical => None,
        },
        ControlKey::ArrowUp => Some(if focus_index == 0 {
            len - 1
        } else {
            focus_index - 1
        }),
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
    /// Additional attributes appended to the group container prior to theming.
    /// The helper exists so tests (and eventually applications) can request
    /// inline overrides without touching adapter internals.
    pub additional_group_attributes: Vec<(String, String)>,
    /// Additional attributes appended to individual options before theming.
    /// The outer index maps directly to the option index.
    pub additional_option_attributes: Vec<Vec<(String, String)>>,
}

impl RadioGroupProps {
    pub fn new(option_labels: impl Into<Vec<String>>) -> Self {
        Self {
            option_labels: option_labels.into(),
            telemetry: TelemetryHooks::default(),
            additional_group_attributes: Vec::new(),
            additional_option_attributes: Vec::new(),
        }
    }

    pub fn from_state(state: &RadioGroupState) -> Self {
        Self {
            option_labels: state.options().to_vec(),
            telemetry: TelemetryHooks::default(),
            additional_group_attributes: Vec::new(),
            additional_option_attributes: Vec::new(),
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

    for (key, value) in &props.additional_group_attributes {
        descriptor = descriptor.group_attribute(key.clone(), value.clone());
    }

    descriptor = apply_group_telemetry(descriptor, telemetry);

    for (index, option) in state.options().iter().enumerate() {
        let label = labels.get(index).cloned().unwrap_or_else(|| option.clone());
        let option_descriptor = RadioOptionDescriptor::new(label, themed_radio_option_style())
            .with_attributes(state.option_aria_attributes(index))
            .attribute("data-index", index.to_string());
        let option_descriptor = apply_option_telemetry(option_descriptor, telemetry);
        let option_descriptor =
            if let Some(additional) = props.additional_option_attributes.get(index) {
                additional
                    .iter()
                    .fold(option_descriptor, |descriptor, (key, value)| {
                        descriptor.attribute(key.clone(), value.clone())
                    })
            } else {
                option_descriptor
            };
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
    /// Attribute pairs produced by [`RadioOptionDescriptor::themed_attributes`]
    /// that the snapshot does not recognise yet.
    ///
    /// We store these eagerly so framework adapters can forward future theme
    /// metadata (for example, `style` or `data-*` hooks emitted by new
    /// automation features) without requiring a release that teaches every
    /// renderer about the new key.  Doing so keeps downstream apps free from
    /// manual "patch in this attribute" workarounds and lets CI highlight the
    /// precise spot when additional attribute awareness is eventually required.
    extra_attributes: Vec<(String, String)>,
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
        let mut extra_attributes = Vec::new();

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
                _ => extra_attributes.push((key.clone(), value.clone())),
            }
        }

        Self {
            label: descriptor.label().to_string(),
            themed_attributes,
            extra_attributes,
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
    /// Attribute pairs emitted by [`RadioGroupDescriptor::group_thematic_attributes`]
    /// that do not have a dedicated field in the snapshot yet.
    ///
    /// Preserving these values allows framework adapters to forward newly
    /// introduced theme data (think inline `style` rules injected by
    /// automation-driven layout experiments) without shipping a synchronized
    /// update across every renderer.  Future releases can still promote the
    /// attributes to first-class fields, but until then the passthrough keeps
    /// enterprise pipelines free from bespoke patches.
    extra_attributes: Vec<(String, String)>,
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
        let mut extra_attributes = Vec::new();

        for (key, value) in &group_thematic_attributes {
            match key.as_str() {
                "class" => class = value.clone(),
                "role" => role = value.clone(),
                "aria-orientation" => aria_orientation = value.clone(),
                "aria-disabled" => aria_disabled = Some(value.clone()),
                "data-orientation" => data_orientation = value.clone(),
                "data-rustic-analytics-id" => analytics_id = Some(value.clone()),
                "data-automation-id" => automation_id = Some(value.clone()),
                _ => extra_attributes.push((key.clone(), value.clone())),
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
            extra_attributes,
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
            // 3. When uncontrolled, invoke the shared headless state machine
            //    (`select`). Controlled callers intentionally skip the
            //    mutation so local snapshots remain untouched while telemetry
            //    still records the attempt for auditors.
            // 4. Emit the commit snapshot reflecting the resulting state –
            //    controlled flows reuse the pre-interaction selection so
            //    analytics can reconcile the unchanged data point.
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

                if !controlled {
                    let mut state_mut = state.borrow_mut();
                    state_mut.select(index, |_| {});
                }

                let selected_after = if controlled {
                    previous
                } else {
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

                    let mut events = Vec::with_capacity(6);
                    events.push(analytics_event);

                    let selected_after = Rc::new(RefCell::new(None));
                    {
                        let mut state_mut = state.borrow_mut();
                        let recorder = Rc::clone(&selected_after);
                        // Controlled adapters still pipeline keyboard intents
                        // through the headless state so the roving focus marker is
                        // updated by `RadioGroupState::focus`. The callback only
                        // records the requested index, leaving controlled React
                        // callers responsible for syncing selection while avoiding
                        // accidental mutations because the `select` helper skips
                        // persisting state when `ToggleMode::Controlled` is active.
                        state_mut.on_key(control, move |selected| {
                            recorder.borrow_mut().replace(selected);
                        });
                    }

                    let next_index = *selected_after.borrow();
                    let key_payload =
                        build_key_event(&origin_option, control, previous, next_index, disabled);
                    events.push(RadioTelemetryEvent::Key(key_payload));

                    if let Some(next_index) = next_index {
                        let focused_option = options[next_index].clone();
                        let focus_event = {
                            let state_ref = state.borrow();
                            // Focus telemetry is emitted immediately after the
                            // keyboard handler runs so downstream analytics can
                            // observe the roving focus destination even when the
                            // external React store keeps selection unchanged.
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

                        let committed = if controlled {
                            previous
                        } else {
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
            Event::Key(key) => {
                Reflect::set(
                    &object,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("key"),
                )
                .expect("set kind");
                Reflect::set(
                    &object,
                    &JsValue::from_str("key"),
                    &JsValue::from_str(match key.key {
                        ControlKey::Space => "Space",
                        ControlKey::Enter => "Enter",
                        ControlKey::ArrowUp => "ArrowUp",
                        ControlKey::ArrowDown => "ArrowDown",
                        ControlKey::ArrowLeft => "ArrowLeft",
                        ControlKey::ArrowRight => "ArrowRight",
                        ControlKey::Home => "Home",
                        ControlKey::End => "End",
                    }),
                )
                .expect("set key");
                push_optional_usize(&object, "previous", key.previous).expect("set previous");
                push_optional_usize(&object, "next", key.next).expect("set next");
                Reflect::set(
                    &object,
                    &JsValue::from_str("disabled"),
                    &JsValue::from_bool(key.disabled),
                )
                .expect("set disabled");
                push_optional_string(&object, "analyticsId", &key.analytics_id)
                    .expect("set analyticsId");
                push_optional_string(&object, "automationId", &key.automation_id)
                    .expect("set automationId");
                Reflect::set(
                    &object,
                    &JsValue::from_str("label"),
                    &JsValue::from_str(&key.label),
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
            call_handler_with_event(props, key, JsValue::UNDEFINED);
        }

        fn call_handler_with_event(props: &Object, key: &str, event: JsValue) {
            if let Ok(handler) = Reflect::get(props, &JsValue::from_str(key)) {
                if let Ok(function) = handler.dyn_into::<Function>() {
                    let _ = function.call1(&JsValue::NULL, &event);
                }
            }
        }

        fn keyboard_event(key: &str) -> JsValue {
            let event = Object::new();
            Reflect::set(&event, &JsValue::from_str("key"), &JsValue::from_str(key))
                .expect("set key");

            let prevent = Closure::wrap(Box::new(|| {}) as Box<dyn FnMut()>);
            let function = prevent.as_ref().clone();
            Reflect::set(&event, &JsValue::from_str("preventDefault"), &function)
                .expect("set preventDefault");
            prevent.forget();

            event.into()
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

        #[wasm_bindgen_test]
        fn keydown_emits_key_focus_and_commit_sequence() {
            let state = Rc::new(RefCell::new(sample_state_uncontrolled()));
            let snapshot = build_snapshot(&state.borrow());
            let (delegate, events) = telemetry_collector();
            let (props, state_handle) =
                build_option_props(Rc::clone(&state), &snapshot, Some(delegate), 0);

            call_handler_with_event(&props, "onKeyDown", keyboard_event("ArrowRight"));

            let kinds = event_kinds(&events);
            assert_eq!(
                kinds,
                vec!["analytics", "key", "focus", "blur", "change", "commit"]
            );
            assert_eq!(state_handle.borrow().selected_index(), Some(1));
        }

        #[wasm_bindgen_test]
        fn controlled_keydown_advances_focus_and_reports_next_index() {
            let state = Rc::new(RefCell::new(sample_state_controlled()));
            let snapshot = build_snapshot(&state.borrow());
            let (delegate, events) = telemetry_collector();
            let (props, state_handle) =
                build_option_props(Rc::clone(&state), &snapshot, Some(delegate), 1);

            call_handler_with_event(&props, "onKeyDown", keyboard_event("ArrowRight"));

            let handle = state_handle.borrow();
            assert_eq!(handle.selected_index(), Some(1));
            assert_eq!(handle.focus_visible_index(), Some(2));
            drop(handle);

            let kinds = event_kinds(&events);
            assert_eq!(
                kinds,
                vec!["analytics", "key", "focus", "blur", "change", "commit"]
            );

            let key_event = events.get(1);
            let key_next = Reflect::get(&key_event, &JsValue::from_str("next"))
                .ok()
                .and_then(|value| value.as_f64())
                .map(|value| value as usize);
            assert_eq!(key_next, Some(2));

            let focus_event = events.get(2);
            let focus_index = Reflect::get(&focus_event, &JsValue::from_str("index"))
                .ok()
                .and_then(|value| value.as_f64())
                .map(|value| value as usize);
            assert_eq!(focus_index, Some(2));

            let blur_event = events.get(3);
            let blur_index = Reflect::get(&blur_event, &JsValue::from_str("index"))
                .ok()
                .and_then(|value| value.as_f64())
                .map(|value| value as usize);
            assert_eq!(blur_index, Some(1));

            let commit_event = events.get(5);
            let commit_selected = Reflect::get(&commit_event, &JsValue::from_str("selected"))
                .ok()
                .and_then(|value| value.as_f64())
                .map(|value| value as usize);
            assert_eq!(commit_selected, Some(1));
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
            // 3. Drive the shared [`RadioGroupState`] via [`RadioGroupState::select`]
            //    when operating uncontrolled. Controlled flows intentionally skip
            //    the mutation so render snapshots remain in lock-step with the
            //    external owner while still documenting the attempt.
            // 4. Emit the post-mutation `RadioCommitEvent` snapshot. In controlled
            //    mode the event reuses the previous selection, providing auditors a
            //    clear signal that UI state did not shift.
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
                        if !controlled {
                            state_mut.select(index, |_| {});
                        }
                        state_mut.focus(index);
                    }

                    let selected_after = {
                        if controlled {
                            previous
                        } else {
                            let state_ref = state.borrow();
                            state_ref.selected_index().or(Some(index))
                        }
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
                        // Focus guard: borrow the state mutably only long enough to
                        // delegate to the headless state machine.  Enterprise
                        // governance teams audit this hand-off to confirm roving
                        // focus is always updated by the canonical logic, even when
                        // React/Yew consumers operate the component in controlled
                        // mode.  By funnelling the key into `on_key` we capture the
                        // callback-selected index for telemetry without taking over
                        // ownership of the eventual commit.
                        let mut state_mut = state.borrow_mut();
                        let recorder = Rc::clone(&selected_after);
                        state_mut.on_key(control, move |selected| {
                            recorder.borrow_mut().replace(selected);
                        });
                    }

                    let next_index = selected_after.borrow().copied();

                    let mut telemetry_events = Vec::with_capacity(6);
                    telemetry_events.push(analytics_event);

                    let key_payload =
                        build_key_event(&origin_option, control, previous, next_index, disabled);
                    telemetry_events.push(RadioTelemetryEvent::Key(key_payload.clone()));
                    let mut change_payload = None;

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

                        let committed = if controlled {
                            previous
                        } else {
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
    /// * Pointer interactions emit analytics → change → commit in order, focus
    ///   transitions emit analytics → focus/blur, and keyboard flows emit
    ///   analytics → key → focus/blur → change → commit so automation suites can
    ///   validate navigation intent separately from the resulting selection.
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
    use leptos::ev::{Event, FocusEvent, KeyboardEvent, MouseEvent};
    use leptos::{prelude::*, Attribute};
    use std::{cell::RefCell, rc::Rc};

    /// Convenience alias for telemetry delegates captured by handler closures.
    type TelemetryDelegate = Rc<dyn Fn(RadioTelemetryEvent)>;
    /// Convenience alias for change callbacks supplied by consuming Leptos apps.
    type ChangeCallback = Rc<dyn Fn(RadioChangeEvent)>;
    /// Convenience alias for focus callbacks supplied by consuming Leptos apps.
    type FocusCallback = Rc<dyn Fn(RadioFocusEvent)>;
    /// Convenience alias for keyboard callbacks supplied by consuming Leptos apps.
    type KeyCallback = Rc<dyn Fn(RadioKeyEvent)>;

    fn emit_telemetry(delegate: &Option<TelemetryDelegate>, events: &[RadioTelemetryEvent]) {
        if let Some(callback) = delegate {
            for event in events {
                callback(event.clone());
            }
        }
    }

    #[derive(Clone)]
    struct LeptosOptionHandlers {
        /// Runner invoked by both `on:click` and `on:change` so analytics is
        /// emitted once regardless of which DOM event the browser fires.
        select: Rc<dyn Fn()>,
        /// Runner triggered by `on:focus` to emit analytics + focus telemetry
        /// before invoking consumer callbacks.
        focus: Rc<dyn Fn()>,
        /// Runner triggered by `on:blur` to emit analytics + blur telemetry
        /// before invoking consumer callbacks.
        blur: Rc<dyn Fn()>,
        /// Runner triggered by `on:keydown` once a control key is detected.
        key: Rc<dyn Fn(ControlKey)>,
    }

    struct LeptosOptionHandlerBuilder {
        state: Rc<RefCell<RadioGroupState>>,
        options: Rc<Vec<RadioOptionSnapshot>>,
        on_change: Option<ChangeCallback>,
        on_focus: Option<FocusCallback>,
        on_blur: Option<FocusCallback>,
        on_key: Option<KeyCallback>,
        telemetry_delegate: Option<TelemetryDelegate>,
        refresh: Rc<dyn Fn()>,
    }

    impl LeptosOptionHandlerBuilder {
        fn new(
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            on_change: Option<ChangeCallback>,
            on_focus: Option<FocusCallback>,
            on_blur: Option<FocusCallback>,
            on_key: Option<KeyCallback>,
            telemetry_delegate: Option<TelemetryDelegate>,
            refresh: Rc<dyn Fn()>,
        ) -> Self {
            Self {
                state,
                options,
                on_change,
                on_focus,
                on_blur,
                on_key,
                telemetry_delegate,
                refresh,
            }
        }

        fn build(&self, index: usize) -> LeptosOptionHandlers {
            let select_runner = self.build_select_handler(index);
            let focus_runner = self.build_focus_handler(index);
            let blur_runner = self.build_blur_handler(index);
            let key_runner = self.build_key_handler(index);

            LeptosOptionHandlers {
                select: select_runner,
                focus: focus_runner,
                blur: blur_runner,
                key: key_runner,
            }
        }

        fn build_select_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_change = self.on_change.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
                // Snapshot the current state before any mutation so telemetry
                // delegates receive the analytics payload tied to the pre-event
                // selection. This mirrors the React/Yew choreography and keeps
                // QA automation deterministic.
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
                    if !controlled {
                        state_mut.select(index, |_| {});
                    }
                    state_mut.focus(index);
                }

                refresh();

                let selected_after = if controlled {
                    previous
                } else {
                    let state_ref = state.borrow();
                    state_ref.selected_index().or(Some(index))
                };
                let commit_event = build_commit_event(&option, selected_after, controlled);
                telemetry_events.push(RadioTelemetryEvent::Commit(commit_event));

                emit_telemetry(&telemetry, &telemetry_events);

                if let Some(callback) = &on_change {
                    callback(change_event);
                }
            })
        }

        fn build_focus_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_focus = self.on_focus.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
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

                refresh();

                if let Some(callback) = &on_focus {
                    callback(focus_payload);
                }
            })
        }

        fn build_blur_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_blur = self.on_blur.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
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

                refresh();

                if let Some(callback) = &on_blur {
                    callback(blur_payload);
                }
            })
        }

        fn build_key_handler(&self, index: usize) -> Rc<dyn Fn(ControlKey)> {
            let state = Rc::clone(&self.state);
            let options = Rc::clone(&self.options);
            let telemetry = self.telemetry_delegate.clone();
            let on_key = self.on_key.clone();
            let on_change = self.on_change.clone();
            let origin_option = self.options[index].clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move |control: ControlKey| {
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
                    // Governance-grade focus audit trail: even in controlled mode we
                    // forward the key press into the headless state machine so the
                    // canonical roving focus algorithm records which option would be
                    // highlighted.  Framework adapters only own the commit step; by
                    // sourcing the focus intent from `on_key` we ensure telemetry is
                    // anchored to the same index the core engine calculated, keeping
                    // accessibility reviewers and SOC teams in sync.
                    let mut state_mut = state.borrow_mut();
                    let recorder = Rc::clone(&selected_after);
                    state_mut.on_key(control, move |selected| {
                        recorder.borrow_mut().replace(selected);
                    });
                }

                let next_index = {
                    let resolved = selected_after.borrow().copied();
                    if resolved.is_some() {
                        resolved
                    } else if controlled {
                        let state_ref = state.borrow();
                        preview_keyboard_target(&state_ref, control)
                    } else {
                        None
                    }
                };

                refresh();

                let mut telemetry_events = Vec::with_capacity(6);
                telemetry_events.push(analytics_event);
                let key_payload =
                    build_key_event(&origin_option, control, previous, next_index, disabled);
                telemetry_events.push(RadioTelemetryEvent::Key(key_payload.clone()));
                let mut change_payload = None;

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

                    let committed = if controlled {
                        previous
                    } else {
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
                    callback(key_payload.clone());
                }

                if let (Some(callback), Some(change_event)) = (on_change.as_ref(), change_payload) {
                    callback(change_event);
                }
            })
        }
    }

    /// Properties accepted by [`LeptosRadioGroup`].
    #[derive(Clone)]
    pub struct LeptosRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Leptos render lifecycle.
        pub telemetry: TelemetryHooks,
        /// Optional change callback invoked after telemetry delegates fire.
        pub on_change: Option<ChangeCallback>,
        /// Optional focus callback invoked when an option gains focus.
        pub on_focus: Option<FocusCallback>,
        /// Optional blur callback invoked when an option loses focus.
        pub on_blur: Option<FocusCallback>,
        /// Optional keyboard callback invoked with normalised control keys.
        pub on_key: Option<KeyCallback>,
        /// Optional telemetry delegate receiving structured analytics payloads.
        pub telemetry_delegate: Option<TelemetryDelegate>,
    }

    impl LeptosRadioGroupProps {
        /// Convenience constructor retaining backward-compatible ergonomics.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: None,
            }
        }
    }

    /// Helper retained for SSR convenience. This mirrors the existing
    /// `render_*` helpers exposed by other adapters so teams can generate static
    /// markup without instantiating the interactive component.
    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }

    #[component]
    pub fn LeptosRadioGroup(props: LeptosRadioGroupProps) -> impl IntoView {
        let telemetry = super::merged_telemetry(&props.telemetry, &props.group.telemetry);
        let telemetry_for_closure = telemetry.clone();
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::leptos::LeptosRadioGroup",
            &props.group,
            &telemetry,
            &props.state,
        );
        instrument_render(&telemetry, context, move || {
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let options = Rc::new(snapshot.options.clone());
            let (version, set_version) = create_signal(0u64);
            let refresh = {
                let set_version = set_version.clone();
                Rc::new(move || set_version.update(|tick| *tick = tick.wrapping_add(1)))
            };
            let handler_builder = Rc::new(LeptosOptionHandlerBuilder::new(
                Rc::clone(&state_handle),
                Rc::clone(&options),
                props.on_change.clone(),
                props.on_focus.clone(),
                props.on_blur.clone(),
                props.on_key.clone(),
                props.telemetry_delegate.clone(),
                Rc::clone(&refresh),
            ));

            let state_for_snapshot = Rc::clone(&state_handle);
            let group_for_snapshot = props.group.clone();
            let telemetry_for_snapshot = telemetry_for_closure.clone();
            let snapshot_signal = create_memo(move |_| {
                version.get();
                let state_ref = state_for_snapshot.borrow();
                let descriptor = super::build_descriptor(
                    &group_for_snapshot,
                    &telemetry_for_snapshot,
                    &state_ref,
                );
                RadioGroupDescriptorSnapshot::from_descriptor(&descriptor)
            });

            let option_views: Vec<View> = options
                .iter()
                .enumerate()
                .map({
                    let builder = Rc::clone(&handler_builder);
                    let snapshot_signal = snapshot_signal.clone();
                    move |(index, _option)| {
                        let handlers = builder.build(index);
                        let select_runner = handlers.select.clone();
                        let focus_runner = handlers.focus.clone();
                        let blur_runner = handlers.blur.clone();
                        let key_runner = handlers.key.clone();
                        let snapshot_signal_for_option = snapshot_signal.clone();
                        let index_for_option = index;

                        view! {
                            <span
                                class=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .class
                                        .clone()
                                }
                                attr:role=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .role
                                        .clone()
                                }
                                attr:aria-checked=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .aria_checked
                                        .clone()
                                }
                                attr:aria-disabled=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .aria_disabled
                                        .clone()
                                }
                                attr:tabindex=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .tabindex
                                        .clone()
                                }
                                attr:data-checked=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .data_checked
                                        .clone()
                                }
                                attr:data-focus-visible=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .data_focus_visible
                                        .clone()
                                }
                                attr:data-index=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .data_index
                                        .clone()
                                }
                                attr:data-rustic-analytics-id=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .analytics_id
                                        .clone()
                                }
                                attr:data-automation-id=move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .automation_id
                                        .clone()
                                }
                                // Keep the passthrough inline so automation owners can
                                // lean on descriptor evolution instead of hand-updating
                                // every adapter when new attributes land. Returning the
                                // collected [`Attribute`] items ensures Leptos applies
                                // them without duplicate boilerplate.
                                ..move || {
                                    snapshot_signal_for_option
                                        .get()
                                        .options[index_for_option]
                                        .extra_attributes
                                        .iter()
                                        .cloned()
                                        .map(|(key, value)| Attribute::from((key, value)))
                                        .collect::<Vec<_>>()
                                }
                                on:click=move |_event: MouseEvent| select_runner()
                                on:change=move |_event: Event| select_runner()
                                on:focus=move |_event: FocusEvent| focus_runner()
                                on:blur=move |_event: FocusEvent| blur_runner()
                                on:keydown=move |event: KeyboardEvent| {
                                    if let Some(control) = control_key_from_str(event.key().as_str()) {
                                        event.prevent_default();
                                        key_runner(control);
                                    }
                                }
                            >{move || {
                                snapshot_signal_for_option
                                    .get()
                                    .options[index_for_option]
                                    .label
                                    .clone()
                            }}</span>
                        }
                    }
                })
                .collect();

            let options_fragment = View::new_fragment(option_views);

            let group_snapshot = snapshot_signal.clone();
            view! {
                <div
                    class=move || group_snapshot.get().class.clone()
                    attr:role=move || group_snapshot.get().role.clone()
                    attr:aria-orientation=move || {
                        group_snapshot.get().aria_orientation.clone()
                    }
                    attr:aria-disabled=move || group_snapshot.get().aria_disabled.clone()
                    attr:data-orientation=move || group_snapshot.get().data_orientation.clone()
                    attr:data-rustic-analytics-id=move || {
                        group_snapshot.get().analytics_id.clone()
                    }
                    attr:data-automation-id=move || {
                        group_snapshot.get().automation_id.clone()
                    }
                    // The dynamic spread mirrors the option handling above so
                    // CI covers both the container and child spans whenever
                    // designers introduce fresh theme-driven attributes. This
                    // keeps downstream teams from copy/pasting attribute
                    // wiring in automation suites.
                    ..move || {
                        group_snapshot
                            .get()
                            .extra_attributes
                            .iter()
                            .cloned()
                            .map(|(key, value)| Attribute::from((key, value)))
                            .collect::<Vec<_>>()
                    }
                >{options_fragment}</div>
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct Harness {
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            order: Rc<RefCell<Vec<String>>>,
            telemetry_events: Rc<RefCell<Vec<RadioTelemetryEvent>>>,
            change_events: Rc<RefCell<Vec<RadioChangeEvent>>>,
            focus_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            blur_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            key_events: Rc<RefCell<Vec<RadioKeyEvent>>>,
            refresh_counter: Rc<RefCell<usize>>,
            builder: Rc<LeptosOptionHandlerBuilder>,
        }

        impl Harness {
            fn new(controlled: bool) -> Self {
                let state = if controlled {
                    RadioGroupState::controlled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                } else {
                    RadioGroupState::uncontrolled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                };
                let group = RadioGroupProps::from_state(&state);
                let telemetry = TelemetryHooks::default();
                let (_context, _descriptor, snapshot) = super::super::descriptor_with_context(
                    "rustic_ui_material::radio::leptos::tests::Harness",
                    &group,
                    &telemetry,
                    &state,
                );

                let state_handle = Rc::new(RefCell::new(state));
                let options = Rc::new(snapshot.options.clone());
                let order = Rc::new(RefCell::new(Vec::new()));
                let telemetry_events = Rc::new(RefCell::new(Vec::new()));
                let change_events = Rc::new(RefCell::new(Vec::new()));
                let focus_events = Rc::new(RefCell::new(Vec::new()));
                let blur_events = Rc::new(RefCell::new(Vec::new()));
                let key_events = Rc::new(RefCell::new(Vec::new()));
                let refresh_counter = Rc::new(RefCell::new(0usize));

                let telemetry_delegate: TelemetryDelegate = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&telemetry_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push(format!("telemetry::{:?}", event));
                        events.borrow_mut().push(event);
                    })
                };

                let on_change: ChangeCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&change_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::change".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_focus: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&focus_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::focus".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_blur: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&blur_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::blur".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_key: KeyCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&key_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::key".into());
                        events.borrow_mut().push(event);
                    })
                };

                let refresh = {
                    let counter = Rc::clone(&refresh_counter);
                    Rc::new(move || {
                        let mut count = counter.borrow_mut();
                        *count = count.wrapping_add(1);
                    }) as Rc<dyn Fn()>
                };

                let builder = Rc::new(LeptosOptionHandlerBuilder::new(
                    Rc::clone(&state_handle),
                    Rc::clone(&options),
                    Some(on_change),
                    Some(on_focus),
                    Some(on_blur),
                    Some(on_key),
                    Some(telemetry_delegate),
                    refresh,
                ));

                Self {
                    state: state_handle,
                    options,
                    order,
                    telemetry_events,
                    change_events,
                    focus_events,
                    blur_events,
                    key_events,
                    refresh_counter,
                    builder,
                }
            }
        }

        #[test]
        fn uncontrolled_select_emits_change_and_commit_before_callbacks() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 1
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(1)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 1);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 4);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Change"));
            assert!(order[2].starts_with("telemetry::Commit"));
            assert_eq!(order[3], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn controlled_select_notifies_without_mutating_selection() {
            let harness = Harness::new(true);
            let handlers = harness.builder.build(2);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(0));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 2
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(0)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 2);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order[3], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn focus_and_blur_emit_telemetry_and_update_focus_state() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.focus();
            assert_eq!(harness.state.borrow().focus_visible_index(), Some(1));
            handlers.blur();
            assert_eq!(harness.state.borrow().focus_visible_index(), None);

            let focus_events = harness.focus_events.borrow();
            assert_eq!(focus_events.len(), 1);
            assert_eq!(focus_events[0].index, 1);
            drop(focus_events);

            let blur_events = harness.blur_events.borrow();
            assert_eq!(blur_events.len(), 1);
            assert_eq!(blur_events[0].index, 1);
            drop(blur_events);

            let telemetry = harness.telemetry_events.borrow();
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            drop(telemetry);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 6);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Focus"));
            assert_eq!(order[2], "callback::focus");
            assert!(order[3].starts_with("telemetry::Analytics"));
            assert!(order[4].starts_with("telemetry::Blur"));
            assert_eq!(order[5], "callback::blur");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 2);
        }

        #[test]
        fn keyboard_navigation_emits_key_change_and_commit() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(0);
            handlers.key(ControlKey::ArrowRight);

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 6);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Key(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            assert!(matches!(telemetry[4], RadioTelemetryEvent::Change(_)));
            assert!(matches!(telemetry[5], RadioTelemetryEvent::Commit(_)));
            drop(telemetry);

            let keys = harness.key_events.borrow();
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0].key, ControlKey::ArrowRight);
            drop(keys);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 1);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 8);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Key"));
            assert!(order[2].starts_with("telemetry::Focus"));
            assert!(order[3].starts_with("telemetry::Blur"));
            assert!(order[4].starts_with("telemetry::Change"));
            assert!(order[5].starts_with("telemetry::Commit"));
            assert_eq!(order[6], "callback::key");
            assert_eq!(order[7], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn ssr_render_preserves_descriptor_metadata() {
            let state = RadioGroupState::uncontrolled(
                vec!["North".into(), "South".into()],
                false,
                RadioOrientation::Vertical,
                Some(0),
            );
            let props =
                LeptosRadioGroupProps::new(RadioGroupProps::from_state(&state), state.clone());
            let html = leptos::ssr::render_to_string({
                let props = props.clone();
                move || LeptosRadioGroup(props.clone())
            });

            let markup = html.to_string();
            assert!(markup.contains("role=\"radiogroup\""));
            assert!(markup.contains("data-index=\"0\""));
            assert!(markup.contains("data-orientation=\"vertical\""));
        }

        #[test]
        fn leptos_dynamic_attributes_forward_new_theme_pairs() {
            // Regression guard: when the descriptor emits additional themed
            // attributes (e.g. inline `style` overrides rolled out by design
            // automation), Leptos must surface them automatically so QA does
            // not need to duplicate attribute wiring manually.
            let state = RadioGroupState::uncontrolled(
                vec!["Primary".into(), "Secondary".into(), "Tertiary".into()],
                false,
                RadioOrientation::Horizontal,
                Some(1),
            );
            let mut group_props = RadioGroupProps::from_state(&state);
            group_props
                .additional_group_attributes
                .push(("style".into(), "position: relative;".into()));
            group_props.additional_option_attributes = vec![
                vec![("style".into(), "color: red;".into())],
                vec![("style".into(), "color: blue;".into())],
                Vec::new(),
            ];
            let props = LeptosRadioGroupProps {
                group: group_props,
                state: state.clone(),
                telemetry: TelemetryHooks::default(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: None,
            };

            let html = leptos::ssr::render_to_string({
                let props = props.clone();
                move || LeptosRadioGroup(props.clone())
            });

            let markup = html.to_string();
            assert!(
                markup.contains("style=\"position: relative;\""),
                "group style attribute missing: {markup}"
            );
            assert!(
                markup.contains("style=\"color: red;\""),
                "option style attribute missing: {markup}"
            );
            assert!(
                markup.contains("class=\""),
                "class attribute should still be rendered alongside dynamic spread: {markup}"
            );
        }
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter constructed with `rsx!` for idiomatic use in Dioxus apps.
    use super::*;
    use ::dioxus::core::{Attribute, AttributeValue, DynamicNode, VNode};
    use ::dioxus::prelude::events::{FocusEvent, KeyboardEvent, MouseEvent};
    use ::dioxus::prelude::*;
    use keyboard_types::Key;
    use std::{cell::RefCell, rc::Rc};

    /// Shared type alias mirroring the `Leptos` adapter so integrations can
    /// forward telemetry in a consistent `Rc` shape.
    type TelemetryDelegate = Rc<dyn Fn(RadioTelemetryEvent)>;
    /// Alias for change callbacks supplied by Dioxus shells.
    type ChangeCallback = Rc<dyn Fn(RadioChangeEvent)>;
    /// Alias for focus/blur callbacks supplied by Dioxus shells.
    type FocusCallback = Rc<dyn Fn(RadioFocusEvent)>;
    /// Alias for keyboard callbacks supplied by Dioxus shells.
    type KeyCallback = Rc<dyn Fn(RadioKeyEvent)>;

    fn emit_telemetry(delegate: &Option<TelemetryDelegate>, events: &[RadioTelemetryEvent]) {
        if let Some(callback) = delegate {
            for event in events {
                callback(event.clone());
            }
        }
    }

    fn rc_option_eq<T: ?Sized>(lhs: &Option<Rc<T>>, rhs: &Option<Rc<T>>) -> bool {
        match (lhs, rhs) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }

    /// Properties accepted by [`DioxusRadioGroup`].
    #[derive(Props, Clone)]
    pub struct DioxusRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Optional change callback invoked after the shared runner completes.
        #[props(optional)]
        pub on_change: Option<ChangeCallback>,
        /// Optional focus callback invoked when an option gains focus.
        #[props(optional)]
        pub on_focus: Option<FocusCallback>,
        /// Optional blur callback invoked when an option loses focus.
        #[props(optional)]
        pub on_blur: Option<FocusCallback>,
        /// Optional keyboard callback invoked with normalized control keys.
        #[props(optional)]
        pub on_key: Option<KeyCallback>,
        /// Optional telemetry delegate receiving structured payloads.
        #[props(optional)]
        pub telemetry_delegate: Option<TelemetryDelegate>,
        /// Telemetry hooks applied around the Dioxus render lifecycle.
        #[props(default = None)]
        pub telemetry: Option<TelemetryHooks>,
    }

    impl PartialEq for DioxusRadioGroupProps {
        fn eq(&self, other: &Self) -> bool {
            self.group == other.group
                && self.state.options() == other.state.options()
                && self.state.orientation() == other.state.orientation()
                && self.state.disabled() == other.state.disabled()
                && self.state.selected_index() == other.state.selected_index()
                && self.state.focus_visible_index() == other.state.focus_visible_index()
                && rc_option_eq(&self.on_change, &other.on_change)
                && rc_option_eq(&self.on_focus, &other.on_focus)
                && rc_option_eq(&self.on_blur, &other.on_blur)
                && rc_option_eq(&self.on_key, &other.on_key)
                && rc_option_eq(&self.telemetry_delegate, &other.telemetry_delegate)
                && self.telemetry == other.telemetry
        }
    }

    #[derive(Clone)]
    struct DioxusOptionHandlers {
        select: Rc<dyn Fn()>,
        focus: Rc<dyn Fn()>,
        blur: Rc<dyn Fn()>,
        key: Rc<dyn Fn(ControlKey)>,
    }

    struct DioxusOptionHandlerBuilder {
        state: Rc<RefCell<RadioGroupState>>,
        options: Rc<Vec<RadioOptionSnapshot>>,
        on_change: Option<ChangeCallback>,
        on_focus: Option<FocusCallback>,
        on_blur: Option<FocusCallback>,
        on_key: Option<KeyCallback>,
        telemetry_delegate: Option<TelemetryDelegate>,
        refresh: Rc<dyn Fn()>,
    }

    impl DioxusOptionHandlerBuilder {
        fn new(
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            on_change: Option<ChangeCallback>,
            on_focus: Option<FocusCallback>,
            on_blur: Option<FocusCallback>,
            on_key: Option<KeyCallback>,
            telemetry_delegate: Option<TelemetryDelegate>,
            refresh: Rc<dyn Fn()>,
        ) -> Self {
            Self {
                state,
                options,
                on_change,
                on_focus,
                on_blur,
                on_key,
                telemetry_delegate,
                refresh,
            }
        }

        fn build(&self, index: usize) -> DioxusOptionHandlers {
            let select_runner = self.build_select_handler(index);
            let focus_runner = self.build_focus_handler(index);
            let blur_runner = self.build_blur_handler(index);
            let key_runner = self.build_key_handler(index);

            DioxusOptionHandlers {
                select: select_runner,
                focus: focus_runner,
                blur: blur_runner,
                key: key_runner,
            }
        }

        fn build_select_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_change = self.on_change.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
                // Telemetry is captured **before** any mutation to ensure
                // analytics systems observe the pre-interaction state. This
                // mirrors the React/Yew/Leptos lifecycles and keeps automation
                // flows deterministic across adapters.
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
                    // Drive the shared state machine so keyboard focus follows
                    // the selection just like the other renderers.
                    let mut state_mut = state.borrow_mut();
                    if !controlled {
                        state_mut.select(index, |_| {});
                    }
                    state_mut.focus(index);
                }

                refresh();

                let selected_after = if controlled {
                    previous
                } else {
                    let state_ref = state.borrow();
                    state_ref.selected_index().or(Some(index))
                };
                telemetry_events.push(RadioTelemetryEvent::Commit(build_commit_event(
                    &option,
                    selected_after,
                    controlled,
                )));

                emit_telemetry(&telemetry, &telemetry_events);

                if let Some(callback) = &on_change {
                    callback(change_event);
                }
            })
        }

        fn build_focus_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_focus = self.on_focus.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
                // Focus telemetry mirrors the analytics-first lifecycle so
                // automation can assert deterministic ordering.
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

                refresh();

                if let Some(callback) = &on_focus {
                    callback(focus_payload);
                }
            })
        }

        fn build_blur_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_blur = self.on_blur.clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move || {
                // Blur mirrors the focus handler with an explicit `false`
                // visibility flag so analytics sees the transition order.
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

                refresh();

                if let Some(callback) = &on_blur {
                    callback(blur_payload);
                }
            })
        }

        fn build_key_handler(&self, index: usize) -> Rc<dyn Fn(ControlKey)> {
            let state = Rc::clone(&self.state);
            let options = Rc::clone(&self.options);
            let telemetry = self.telemetry_delegate.clone();
            let on_key = self.on_key.clone();
            let on_change = self.on_change.clone();
            let origin_option = self.options[index].clone();
            let refresh = Rc::clone(&self.refresh);

            Rc::new(move |control: ControlKey| {
                // Capture a consistent snapshot before mutating so telemetry
                // ordering matches the other adapters.
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

                // Regulatory audit requirement: the headless state machine must
                // always observe the keyboard control flow so the roving focus
                // cursor advances identically for controlled and uncontrolled
                // inputs.  We therefore invoke `on_key` unconditionally and
                // capture the resulting index in a scoped cell.  Controlled
                // callers ignore the commit produced by the state machine, but
                // the focus mutation remains authoritative for accessibility
                // attestations and SOC reviewers.
                let selected_after = Rc::new(RefCell::new(None));
                {
                    // Compliance note: this guard intentionally borrows the
                    // state mutably even when the group is controlled.  The
                    // inner callback records the computed target so telemetry
                    // can cite the same index as the roving focus algorithm,
                    // while the controlled contract keeps the commit owner in
                    // the host application.  This mirrors the sequencing used
                    // by the React/Yew adapters and keeps our analytics trace
                    // consistent across renderers.
                    let mut state_mut = state.borrow_mut();
                    let recorder = Rc::clone(&selected_after);
                    state_mut.on_key(control, move |selected| {
                        recorder.borrow_mut().replace(selected);
                    });
                }

                let next_index = {
                    let resolved = selected_after.borrow().copied();
                    if resolved.is_some() {
                        resolved
                    } else if controlled {
                        // Fallback for defensive parity: if the controlled flow
                        // short-circuited selection we still synthesize the
                        // focus target using the deterministic preview helper so
                        // auditors see a stable trajectory in the telemetry
                        // payloads.
                        let state_ref = state.borrow();
                        preview_keyboard_target(&state_ref, control)
                    } else {
                        None
                    }
                };

                refresh();

                let mut telemetry_events = Vec::with_capacity(6);
                telemetry_events.push(analytics_event);
                let key_payload =
                    build_key_event(&origin_option, control, previous, next_index, disabled);
                telemetry_events.push(RadioTelemetryEvent::Key(key_payload.clone()));
                let mut change_payload = None;

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

                    let committed = if controlled {
                        previous
                    } else {
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
                    callback(key_payload.clone());
                }

                if let (Some(callback), Some(change_event)) = (on_change.as_ref(), change_payload) {
                    callback(change_event);
                }
            })
        }
    }

    fn sync_state_from_props(state: &Rc<RefCell<RadioGroupState>>, props_state: &RadioGroupState) {
        let mut state_mut = state.borrow_mut();
        if state_mut.options() != props_state.options() {
            *state_mut = props_state.clone();
            return;
        }

        state_mut.set_disabled(props_state.disabled());
        state_mut.set_orientation(props_state.orientation());

        if props_state.is_controlled() {
            state_mut.sync_selected(props_state.selected_index());
        }
    }

    /// Appends arbitrary theme attributes to an existing [`VNode`], matching the
    /// spread behaviour used by our React and Leptos adapters so partners receive
    /// the same automation surface without hand-editing the Dioxus tree.
    fn extend_node_attributes<'a>(
        scope: &'a ScopeState,
        node: &mut VNode<'a>,
        attributes: &[(String, String)],
    ) {
        if attributes.is_empty() {
            return;
        }

        let existing = node.dynamic_attrs.len();
        let bump = scope.bump();
        let mut merged =
            bumpalo::collections::Vec::with_capacity_in(existing + attributes.len(), bump);

        // SAFETY: the dynamic attribute slice lives in the scope-local bump
        // arena. Copying the bytes into a new bump allocation is equivalent to
        // how the `rsx!` macro hydrates attributes, and the original slice is
        // discarded with the frame, so we avoid aliasing while keeping parity
        // with the other adapters.
        unsafe {
            merged.set_len(existing);
            std::ptr::copy_nonoverlapping(
                node.dynamic_attrs.as_ptr(),
                merged.as_mut_ptr(),
                existing,
            );
        }

        for (key, value) in attributes {
            let name = bump.alloc_str(key);
            let value = bump.alloc_str(value);
            merged.push(Attribute::new(
                name,
                AttributeValue::Text(value),
                None,
                false,
            ));
        }

        node.dynamic_attrs = merged.into_bump_slice();
    }

    /// Radio group rendered as a Dioxus component.
    pub fn DioxusRadioGroup(cx: Scope<DioxusRadioGroupProps>) -> Element {
        let props = cx.props();
        let telemetry_override = props.telemetry.clone().unwrap_or_default();
        let telemetry = super::merged_telemetry(&telemetry_override, &props.group.telemetry);
        let state_handle = {
            let initial = Rc::new(RefCell::new(props.state.clone()));
            let cell = use_ref(cx, || initial);
            cell.read().clone()
        };
        sync_state_from_props(&state_handle, &props.state);

        let state_snapshot = state_handle.borrow();
        let (context, _descriptor, snapshot) = super::descriptor_with_context(
            "rustic_ui_material::radio::dioxus::DioxusRadioGroup",
            &props.group,
            &telemetry,
            &state_snapshot,
        );
        drop(state_snapshot);

        let scope = cx;
        let on_change = props.on_change.clone();
        let on_focus = props.on_focus.clone();
        let on_blur = props.on_blur.clone();
        let on_key = props.on_key.clone();
        let telemetry_delegate = props.telemetry_delegate.clone();
        let options_snapshot = snapshot.options.clone();
        // Collect the group-level attributes eagerly so the Dioxus adapter stays
        // aligned with the React/Leptos/Yew renderers when new theme metadata is
        // introduced.  Centralising the spread here keeps enterprise builds free
        // from repetitive "add this new key" patches.
        let group_attributes = {
            let mut attributes = snapshot.extra_attributes.clone();
            attributes.push(("class".into(), snapshot.class.clone()));
            attributes.push(("role".into(), snapshot.role.clone()));
            attributes.push(("aria-orientation".into(), snapshot.aria_orientation.clone()));
            if let Some(value) = snapshot.aria_disabled.clone() {
                attributes.push(("aria-disabled".into(), value));
            }
            attributes.push(("data-orientation".into(), snapshot.data_orientation.clone()));
            if let Some(value) = snapshot.analytics_id.clone() {
                attributes.push(("data-rustic-analytics-id".into(), value));
            }
            if let Some(value) = snapshot.automation_id.clone() {
                attributes.push(("data-automation-id".into(), value));
            }
            attributes
        };
        // Mirror the group strategy for each option to keep theme tweaks (class,
        // style, data hooks, etc.) flowing through automatically.
        let option_attribute_sets = options_snapshot
            .iter()
            .map(|option| {
                let mut attributes = option.extra_attributes.clone();
                attributes.push(("class".into(), option.class.clone()));
                attributes.push(("role".into(), option.role.clone()));
                attributes.push(("aria-checked".into(), option.aria_checked.clone()));
                if let Some(value) = option.aria_disabled.clone() {
                    attributes.push(("aria-disabled".into(), value));
                }
                attributes.push(("tabindex".into(), option.tabindex.clone()));
                attributes.push(("data-checked".into(), option.data_checked.clone()));
                attributes.push((
                    "data-focus-visible".into(),
                    option.data_focus_visible.clone(),
                ));
                attributes.push(("data-index".into(), option.data_index.clone()));
                if let Some(value) = option.analytics_id.clone() {
                    attributes.push(("data-rustic-analytics-id".into(), value));
                }
                if let Some(value) = option.automation_id.clone() {
                    attributes.push(("data-automation-id".into(), value));
                }
                attributes
            })
            .collect::<Vec<_>>();

        let options = Rc::new(options_snapshot);
        let option_attribute_sets = Rc::new(option_attribute_sets);
        let group_attributes = Rc::new(group_attributes);

        instrument_render(&telemetry, context, move || {
            let refresh: Rc<dyn Fn()> = {
                let scope = scope;
                Rc::new(move || scope.needs_update())
            };
            let handler_builder = Rc::new(DioxusOptionHandlerBuilder::new(
                Rc::clone(&state_handle),
                Rc::clone(&options),
                on_change.clone(),
                on_focus.clone(),
                on_blur.clone(),
                on_key.clone(),
                telemetry_delegate.clone(),
                refresh,
            ));

            let options = Rc::clone(&options);
            let option_attribute_sets = Rc::clone(&option_attribute_sets);
            let group_attributes = Rc::clone(&group_attributes);

            scope.render(LazyNodes::new(move |scope_state| {
                let mut node = rsx! {
                    div {
                        { options.iter().enumerate().map(|(index, option)| {
                            let label = option.label.clone();
                            let handlers = handler_builder.build(index);
                            let select_runner = handlers.select.clone();
                            let focus_runner = handlers.focus.clone();
                            let blur_runner = handlers.blur.clone();
                            let key_runner = handlers.key.clone();

                            rsx! {
                                span {
                                    // Clicking funnels through the shared runner so telemetry,
                                    // state updates, and consumer callbacks remain in lock-step.
                                    onclick: move |_event: MouseEvent| {
                                        select_runner();
                                    },
                                    // Focus hooks emit analytics before mutating focus state to
                                    // keep automation ordering deterministic.
                                    onfocus: move |_event: FocusEvent| {
                                        focus_runner();
                                    },
                                    // Blur mirrors focus to capture analytics prior to clearing
                                    // the headless focus-visible flag.
                                    onblur: move |_event: FocusEvent| {
                                        blur_runner();
                                    },
                                    // Keyboard interactions normalize the control key before
                                    // delegating into the shared handler, guaranteeing identical
                                    // telemetry sequencing across renderers.
                                    onkeydown: move |event: KeyboardEvent| {
                                        let raw_key = event.data.key();
                                        let name = match raw_key {
                                            Key::Character(ch) => ch,
                                            Key::Space => " ".to_string(),
                                            Key::Enter => "Enter".to_string(),
                                            other => other.to_string(),
                                        };
                                        if let Some(control) = control_key_from_str(name.as_str()) {
                                            event.prevent_default();
                                            key_runner(control);
                                        }
                                    },
                                    {label}
                                }
                            }
                        }) }
                    }
                }
                .call(scope_state);

                // Rehydrate the group container with the precomputed attributes so
                // any freshly introduced theme hooks (style, data-*, analytics) are
                // forwarded automatically.
                extend_node_attributes(scope_state, &mut node, &group_attributes);

                let mut option_attributes = option_attribute_sets.iter();
                for dynamic in node.dynamic_nodes.iter() {
                    if let DynamicNode::Fragment(children) = dynamic {
                        for child in *children {
                            if let Some(attributes) = option_attributes.next() {
                                // SAFETY: the VNode instances were just constructed for this render,
                                // so mutating them in place keeps parity with the other adapters
                                // while guaranteeing no aliasing across scopes.
                                let child = unsafe { &mut *(child as *const VNode as *mut VNode) };
                                // Apply the option attributes lazily so design system experiments
                                // (think inline `style` or `data-*` analytics probes) flow through
                                // without revisiting the adapter.
                                extend_node_attributes(scope_state, child, attributes);
                            }
                        }
                    }
                }

                debug_assert!(
                    option_attributes.next().is_none(),
                    "every option attribute bundle should have been applied",
                );

                node
            }))
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ::dioxus::prelude::VirtualDom;

        struct Harness {
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            order: Rc<RefCell<Vec<String>>>,
            telemetry_events: Rc<RefCell<Vec<RadioTelemetryEvent>>>,
            change_events: Rc<RefCell<Vec<RadioChangeEvent>>>,
            focus_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            blur_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            key_events: Rc<RefCell<Vec<RadioKeyEvent>>>,
            refresh_counter: Rc<RefCell<usize>>,
            builder: Rc<DioxusOptionHandlerBuilder>,
        }

        impl Harness {
            fn new(controlled: bool) -> Self {
                let state = if controlled {
                    RadioGroupState::controlled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                } else {
                    RadioGroupState::uncontrolled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                };
                let group = RadioGroupProps::from_state(&state);
                let telemetry = TelemetryHooks::default();
                let (_context, _descriptor, snapshot) = super::descriptor_with_context(
                    "rustic_ui_material::radio::dioxus::tests::Harness",
                    &group,
                    &telemetry,
                    &state,
                );

                let state_handle = Rc::new(RefCell::new(state));
                let options = Rc::new(snapshot.options.clone());
                let order = Rc::new(RefCell::new(Vec::new()));
                let telemetry_events = Rc::new(RefCell::new(Vec::new()));
                let change_events = Rc::new(RefCell::new(Vec::new()));
                let focus_events = Rc::new(RefCell::new(Vec::new()));
                let blur_events = Rc::new(RefCell::new(Vec::new()));
                let key_events = Rc::new(RefCell::new(Vec::new()));
                let refresh_counter = Rc::new(RefCell::new(0usize));

                let telemetry_delegate: TelemetryDelegate = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&telemetry_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push(format!("telemetry::{:?}", event));
                        events.borrow_mut().push(event);
                    })
                };

                let on_change: ChangeCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&change_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::change".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_focus: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&focus_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::focus".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_blur: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&blur_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::blur".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_key: KeyCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&key_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::key".into());
                        events.borrow_mut().push(event);
                    })
                };

                let refresh = {
                    let counter = Rc::clone(&refresh_counter);
                    Rc::new(move || {
                        let mut value = counter.borrow_mut();
                        *value = value.wrapping_add(1);
                    }) as Rc<dyn Fn()>
                };

                let builder = Rc::new(DioxusOptionHandlerBuilder::new(
                    Rc::clone(&state_handle),
                    Rc::clone(&options),
                    Some(on_change),
                    Some(on_focus),
                    Some(on_blur),
                    Some(on_key),
                    Some(telemetry_delegate),
                    refresh,
                ));

                Self {
                    state: state_handle,
                    options,
                    order,
                    telemetry_events,
                    change_events,
                    focus_events,
                    blur_events,
                    key_events,
                    refresh_counter,
                    builder,
                }
            }
        }

        #[test]
        fn uncontrolled_select_emits_change_and_commit_before_callbacks() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 1
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(1)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 1);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 4);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Change"));
            assert!(order[2].starts_with("telemetry::Commit"));
            assert_eq!(order[3], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn controlled_select_notifies_without_mutating_selection() {
            let harness = Harness::new(true);
            let handlers = harness.builder.build(2);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(0));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 2
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(0)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 2);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order[3], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn focus_and_blur_emit_telemetry_and_update_focus_state() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.focus();
            assert_eq!(harness.state.borrow().focus_visible_index(), Some(1));
            handlers.blur();
            assert_eq!(harness.state.borrow().focus_visible_index(), None);

            let focus_events = harness.focus_events.borrow();
            assert_eq!(focus_events.len(), 1);
            assert_eq!(focus_events[0].index, 1);
            drop(focus_events);

            let blur_events = harness.blur_events.borrow();
            assert_eq!(blur_events.len(), 1);
            assert_eq!(blur_events[0].index, 1);
            drop(blur_events);

            let telemetry = harness.telemetry_events.borrow();
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            drop(telemetry);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 6);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Focus"));
            assert_eq!(order[2], "callback::focus");
            assert!(order[3].starts_with("telemetry::Analytics"));
            assert!(order[4].starts_with("telemetry::Blur"));
            assert_eq!(order[5], "callback::blur");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 2);
        }

        #[test]
        fn keyboard_navigation_emits_key_change_and_commit() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(0);
            handlers.key(ControlKey::ArrowRight);

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 6);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Key(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            assert!(matches!(telemetry[4], RadioTelemetryEvent::Change(_)));
            assert!(matches!(telemetry[5], RadioTelemetryEvent::Commit(_)));
            drop(telemetry);

            let keys = harness.key_events.borrow();
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0].key, ControlKey::ArrowRight);
            drop(keys);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 1);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 8);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Key"));
            assert!(order[2].starts_with("telemetry::Focus"));
            assert!(order[3].starts_with("telemetry::Blur"));
            assert!(order[4].starts_with("telemetry::Change"));
            assert!(order[5].starts_with("telemetry::Commit"));
            assert_eq!(order[6], "callback::key");
            assert_eq!(order[7], "callback::change");
            drop(order);

            assert_eq!(*harness.refresh_counter.borrow(), 1);
        }

        #[test]
        fn dioxus_extra_attributes_round_trip_through_virtual_dom() {
            let state = RadioGroupState::uncontrolled(
                vec!["Primary".into(), "Secondary".into()],
                false,
                RadioOrientation::Horizontal,
                Some(0),
            );
            let mut group = RadioGroupProps::from_state(&state);
            group
                .additional_group_attributes
                .push(("style".into(), "position: relative;".into()));
            group.additional_option_attributes =
                vec![vec![("style".into(), "color: crimson;".into())], Vec::new()];

            let props = DioxusRadioGroupProps {
                group,
                state: state.clone(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: None,
                telemetry: None,
            };

            let mut dom = VirtualDom::new_with_props(DioxusRadioGroup, props);
            dom.rebuild();
            let markup = dioxus_ssr::render(&dom).to_string();

            // Guard the "theme attributes propagate automatically" contract so
            // enterprise theming hooks (class names, inline experiments, data-*
            // analytics probes) do not regress silently.
            assert!(
                markup.contains("class=\""),
                "expected class attribute to survive the Dioxus round-trip: {markup}"
            );
            assert!(
                markup.contains("style=\"position: relative;\""),
                "group style attribute missing after VirtualDom render: {markup}"
            );
            assert!(
                markup.contains("style=\"color: crimson;\""),
                "option style attribute missing after VirtualDom render: {markup}"
            );
        }

        #[test]
        fn virtual_dom_preserves_descriptor_attributes() {
            let state = RadioGroupState::uncontrolled(
                vec!["North".into(), "South".into()],
                false,
                RadioOrientation::Vertical,
                Some(0),
            );
            let props = DioxusRadioGroupProps {
                group: RadioGroupProps::from_state(&state),
                state: state.clone(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: None,
                telemetry: None,
            };
            let mut dom = VirtualDom::new_with_props(DioxusRadioGroup, props);
            dom.rebuild();
            let markup = dioxus_ssr::render(&dom);

            assert!(markup.contains("role=\"radiogroup\""));
            assert!(markup.contains("data-index=\"0\""));
            assert!(markup.contains("data-orientation=\"vertical\""));
        }
    }

    /// Helper retained for SSR convenience matching the other adapters.
    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter returning a [`Template`] for reactive dashboards.
    use super::*;
    use ::sycamore as sycamore_crate;
    use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc, sync::OnceLock};
    use sycamore_crate::prelude::*;
    use sycamore_crate::reactive::MaybeDyn;
    use sycamore_crate::web::html::event::KeyboardEvent;
    use sycamore_crate::{component, view};

    /// Alias matching Sycamore's view representation.
    pub type Template<G> = View<G>;

    /// Telemetry delegates and callbacks share the same `Rc` plumbing used by the
    /// Leptos/Dioxus adapters so enterprise automation can plug in once and
    /// receive identical payload sequencing regardless of renderer.
    type TelemetryDelegate = Rc<dyn Fn(RadioTelemetryEvent)>;
    type ChangeCallback = Rc<dyn Fn(RadioChangeEvent)>;
    type FocusCallback = Rc<dyn Fn(RadioFocusEvent)>;
    type KeyCallback = Rc<dyn Fn(RadioKeyEvent)>;

    fn emit_telemetry(delegate: &Option<TelemetryDelegate>, events: &[RadioTelemetryEvent]) {
        if let Some(callback) = delegate {
            for event in events {
                callback(event.clone());
            }
        }
    }

    #[derive(Clone)]
    struct SycamoreOptionHandlers {
        /// Runner invoked by both `on:click` and `on:change` so analytics only
        /// fires once per user gesture.
        select: Rc<dyn Fn()>,
        /// Runner triggered by `on:focus` to emit analytics before callbacks.
        focus: Rc<dyn Fn()>,
        /// Runner triggered by `on:blur` to emit analytics before callbacks.
        blur: Rc<dyn Fn()>,
        /// Runner triggered by `on:keydown` once a control key is detected.
        key: Rc<dyn Fn(ControlKey)>,
    }

    /// Closure factory shared across options so instrumentation, state mutation,
    /// and callback ordering stay centralised.  Each method clones the minimal
    /// data needed for a particular option index and returns a zero-argument
    /// runner the view can invoke from Sycamore event bindings.  This avoids
    /// repeating telemetry scaffolding for every `view!` invocation while
    /// documenting the analytics-first lifecycle in one auditable location.
    struct SycamoreOptionHandlerBuilder {
        state: Rc<RefCell<RadioGroupState>>,
        options: Rc<Vec<RadioOptionSnapshot>>,
        on_change: Option<ChangeCallback>,
        on_focus: Option<FocusCallback>,
        on_blur: Option<FocusCallback>,
        on_key: Option<KeyCallback>,
        telemetry_delegate: Option<TelemetryDelegate>,
    }

    impl SycamoreOptionHandlerBuilder {
        fn new(
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            on_change: Option<ChangeCallback>,
            on_focus: Option<FocusCallback>,
            on_blur: Option<FocusCallback>,
            on_key: Option<KeyCallback>,
            telemetry_delegate: Option<TelemetryDelegate>,
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

        fn build(&self, index: usize) -> SycamoreOptionHandlers {
            let select_runner = self.build_select_handler(index);
            let focus_runner = self.build_focus_handler(index);
            let blur_runner = self.build_blur_handler(index);
            let key_runner = self.build_key_handler(index);

            SycamoreOptionHandlers {
                select: select_runner,
                focus: focus_runner,
                blur: blur_runner,
                key: key_runner,
            }
        }

        fn build_select_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_change = self.on_change.clone();

            Rc::new(move || {
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
                    if !controlled {
                        state_mut.select(index, |_| {});
                    }
                    state_mut.focus(index);
                }

                let selected_after = if controlled {
                    previous
                } else {
                    let state_ref = state.borrow();
                    state_ref.selected_index().or(Some(index))
                };
                telemetry_events.push(RadioTelemetryEvent::Commit(build_commit_event(
                    &option,
                    selected_after,
                    controlled,
                )));

                emit_telemetry(&telemetry, &telemetry_events);

                if let Some(callback) = &on_change {
                    callback(change_event);
                }
            })
        }

        fn build_focus_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_focus = self.on_focus.clone();

            Rc::new(move || {
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
                    callback(focus_payload);
                }
            })
        }

        fn build_blur_handler(&self, index: usize) -> Rc<dyn Fn()> {
            let state = Rc::clone(&self.state);
            let option = self.options[index].clone();
            let telemetry = self.telemetry_delegate.clone();
            let on_blur = self.on_blur.clone();

            Rc::new(move || {
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
                    callback(blur_payload);
                }
            })
        }

        fn build_key_handler(&self, index: usize) -> Rc<dyn Fn(ControlKey)> {
            let state = Rc::clone(&self.state);
            let options = Rc::clone(&self.options);
            let telemetry = self.telemetry_delegate.clone();
            let on_key = self.on_key.clone();
            let on_change = self.on_change.clone();
            let origin_option = self.options[index].clone();

            Rc::new(move |control: ControlKey| {
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

                let next_index = if controlled {
                    let state_ref = state.borrow();
                    preview_keyboard_target(&state_ref, control)
                } else {
                    let selected_after = Rc::new(RefCell::new(None));
                    {
                        let mut state_mut = state.borrow_mut();
                        let recorder = Rc::clone(&selected_after);
                        state_mut.on_key(control, move |selected| {
                            recorder.borrow_mut().replace(selected);
                        });
                    }
                    *selected_after.borrow()
                };

                let mut telemetry_events = Vec::with_capacity(6);
                telemetry_events.push(analytics_event);
                let key_payload =
                    build_key_event(&origin_option, control, previous, next_index, disabled);
                telemetry_events.push(RadioTelemetryEvent::Key(key_payload.clone()));
                let mut change_payload = None;

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

                    let committed = if controlled {
                        previous
                    } else {
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
                    callback(key_payload.clone());
                }

                if let (Some(callback), Some(change_event)) = (on_change.as_ref(), change_payload) {
                    callback(change_event);
                }
            })
        }
    }

    /// Cache converting descriptor attribute keys into `'static` references so the
    /// Sycamore spread APIs can mirror whatever new metadata the snapshot
    /// delivers without adding yet another `match` statement every time the
    /// theming engine grows.  We intentionally intern the strings instead of
    /// leaking per render so long-lived dashboards remain leak free even as new
    /// experiments introduce additional attributes.
    fn intern_attribute_key(key: &str) -> &'static str {
        static INTERNER: OnceLock<std::sync::Mutex<HashMap<String, &'static str>>> =
            OnceLock::new();
        let map = INTERNER.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
        let mut guard = map.lock().expect("attribute interner poisoned");
        if let Some(existing) = guard.get(key) {
            return *existing;
        }

        let owned = key.to_string();
        let leaked: &'static str = Box::leak(owned.clone().into_boxed_str());
        guard.insert(owned, leaked);
        leaked
    }

    /// Translate the descriptor-provided attribute pairs into Sycamore's
    /// [`Attributes`] spread helper so every current and future key from the
    /// snapshot automatically lands on the DOM element.  By funnelling the
    /// conversion through this helper we avoid scattering manual bindings (and
    /// the inevitable missed updates) throughout the view! macro.
    fn sycamore_attributes_from_pairs(pairs: &[(String, String)]) -> Attributes {
        let mut attrs = Attributes::new();
        for (key, value) in pairs {
            let interned = intern_attribute_key(key);
            let attr_value = MaybeDyn::Static(Some(Cow::Owned(value.clone())));
            attrs.set_attribute(interned, attr_value);
        }
        attrs
    }

    /// Properties accepted by [`SycamoreRadioGroup`].
    #[derive(Clone)]
    pub struct SycamoreRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
        /// Telemetry hooks applied around the Sycamore render lifecycle.
        pub telemetry: TelemetryHooks,
        /// Optional change callback invoked after telemetry delegates fire.
        pub on_change: Option<ChangeCallback>,
        /// Optional focus callback invoked when an option gains focus.
        pub on_focus: Option<FocusCallback>,
        /// Optional blur callback invoked when an option loses focus.
        pub on_blur: Option<FocusCallback>,
        /// Optional keyboard callback invoked with normalised control keys.
        pub on_key: Option<KeyCallback>,
        /// Optional telemetry delegate receiving structured analytics payloads.
        pub telemetry_delegate: Option<TelemetryDelegate>,
    }

    impl SycamoreRadioGroupProps {
        /// Convenience constructor mirroring the previous struct layout.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: None,
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
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let options = Rc::new(snapshot.options.clone());
            let handler_builder = Rc::new(SycamoreOptionHandlerBuilder::new(
                Rc::clone(&state_handle),
                Rc::clone(&options),
                props.on_change.clone(),
                props.on_focus.clone(),
                props.on_blur.clone(),
                props.on_key.clone(),
                props.telemetry_delegate.clone(),
            ));

            let option_views: Vec<View<G>> = options
                .iter()
                .enumerate()
                .map({
                    let builder = Rc::clone(&handler_builder);
                    move |(index, option)| {
                        let handlers = builder.build(index);
                        let select_runner = handlers.select.clone();
                        let focus_runner = handlers.focus.clone();
                        let blur_runner = handlers.blur.clone();
                        let key_runner = handlers.key.clone();
                        let label = option.label.clone();
                        let themed_attributes = option.themed_attributes.clone();
                        // Convert the descriptor snapshot into a Sycamore spread so any
                        // new keys added by the theming pipeline flow directly into the
                        // rendered span without editing this adapter again.  Future
                        // contributors should extend `RadioOptionDescriptor` rather than
                        // binding individual attributes here.
                        let option_attributes = sycamore_attributes_from_pairs(&themed_attributes);

                        view! { cx,
                            span(
                                ..option_attributes,
                                on:click=move |_| select_runner(),
                                on:change=move |_| select_runner(),
                                on:focus=move |_| focus_runner(),
                                on:blur=move |_| blur_runner(),
                                on:keydown=move |event: KeyboardEvent| {
                                    let key = event.key();
                                    if let Some(control) = control_key_from_str(&key) {
                                        event.prevent_default();
                                        key_runner(control);
                                    }
                                },
                            ) { (label.clone()) }
                        }
                    }
                })
                .collect();

            let options_fragment = View::new_fragment(option_views);
            let group_attribute_pairs = snapshot.group_thematic_attributes.clone();
            // Mirror the option spread for the container so the automation/theme
            // pipeline can add new keys (for example, inline styles or
            // experiment-driven data hooks) in one place: the descriptor.
            let group_attributes = sycamore_attributes_from_pairs(&group_attribute_pairs);

            view! { cx,
                div(
                    ..group_attributes,
                ) { (options_fragment) }
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct Harness {
            state: Rc<RefCell<RadioGroupState>>,
            options: Rc<Vec<RadioOptionSnapshot>>,
            order: Rc<RefCell<Vec<String>>>,
            telemetry_events: Rc<RefCell<Vec<RadioTelemetryEvent>>>,
            change_events: Rc<RefCell<Vec<RadioChangeEvent>>>,
            focus_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            blur_events: Rc<RefCell<Vec<RadioFocusEvent>>>,
            key_events: Rc<RefCell<Vec<RadioKeyEvent>>>,
            builder: Rc<SycamoreOptionHandlerBuilder>,
        }

        impl Harness {
            fn new(controlled: bool) -> Self {
                let state = if controlled {
                    RadioGroupState::controlled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                } else {
                    RadioGroupState::uncontrolled(
                        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
                        false,
                        RadioOrientation::Horizontal,
                        Some(0),
                    )
                };
                let group = RadioGroupProps::from_state(&state);
                let telemetry = TelemetryHooks::default();
                let (_context, _descriptor, snapshot) = super::super::descriptor_with_context(
                    "rustic_ui_material::radio::sycamore::tests::Harness",
                    &group,
                    &telemetry,
                    &state,
                );

                let state_handle = Rc::new(RefCell::new(state));
                let options = Rc::new(snapshot.options.clone());
                let order = Rc::new(RefCell::new(Vec::new()));
                let telemetry_events = Rc::new(RefCell::new(Vec::new()));
                let change_events = Rc::new(RefCell::new(Vec::new()));
                let focus_events = Rc::new(RefCell::new(Vec::new()));
                let blur_events = Rc::new(RefCell::new(Vec::new()));
                let key_events = Rc::new(RefCell::new(Vec::new()));

                let telemetry_delegate: TelemetryDelegate = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&telemetry_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push(format!("telemetry::{:?}", event));
                        events.borrow_mut().push(event);
                    })
                };

                let on_change: ChangeCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&change_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::change".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_focus: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&focus_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::focus".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_blur: FocusCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&blur_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::blur".into());
                        events.borrow_mut().push(event);
                    })
                };

                let on_key: KeyCallback = {
                    let order = Rc::clone(&order);
                    let events = Rc::clone(&key_events);
                    Rc::new(move |event| {
                        order.borrow_mut().push("callback::key".into());
                        events.borrow_mut().push(event);
                    })
                };

                let builder = Rc::new(SycamoreOptionHandlerBuilder::new(
                    Rc::clone(&state_handle),
                    Rc::clone(&options),
                    Some(on_change),
                    Some(on_focus),
                    Some(on_blur),
                    Some(on_key),
                    Some(telemetry_delegate),
                ));

                Self {
                    state: state_handle,
                    options,
                    order,
                    telemetry_events,
                    change_events,
                    focus_events,
                    blur_events,
                    key_events,
                    builder,
                }
            }
        }

        #[test]
        fn uncontrolled_select_updates_state_and_callbacks() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 1
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(1)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 1);
            drop(changes);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 4);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Change"));
            assert!(order[2].starts_with("telemetry::Commit"));
            assert_eq!(order[3], "callback::change");
        }

        #[test]
        fn controlled_select_emits_commit_without_mutation() {
            let harness = Harness::new(true);
            let handlers = harness.builder.build(2);
            handlers.select();

            assert_eq!(harness.state.borrow().selected_index(), Some(0));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 3);
            assert!(matches!(
                telemetry[1],
                RadioTelemetryEvent::Change(ref evt) if evt.next == 2
            ));
            assert!(matches!(
                telemetry[2],
                RadioTelemetryEvent::Commit(ref evt) if evt.selected == Some(0)
            ));
            drop(telemetry);

            let changes = harness.change_events.borrow();
            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].next, 2);
        }

        #[test]
        fn focus_and_blur_emit_telemetry_and_update_focus_state() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(1);
            handlers.focus();
            assert_eq!(harness.state.borrow().focus_visible_index(), Some(1));
            handlers.blur();
            assert_eq!(harness.state.borrow().focus_visible_index(), None);

            let focus_events = harness.focus_events.borrow();
            assert_eq!(focus_events.len(), 1);
            assert_eq!(focus_events[0].index, 1);
            drop(focus_events);

            let blur_events = harness.blur_events.borrow();
            assert_eq!(blur_events.len(), 1);
            assert_eq!(blur_events[0].index, 1);
            drop(blur_events);

            let telemetry = harness.telemetry_events.borrow();
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            drop(telemetry);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 6);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Focus"));
            assert_eq!(order[2], "callback::focus");
            assert!(order[3].starts_with("telemetry::Analytics"));
            assert!(order[4].starts_with("telemetry::Blur"));
            assert_eq!(order[5], "callback::blur");
        }

        #[test]
        fn keyboard_navigation_emits_key_change_and_commit() {
            let harness = Harness::new(false);
            let handlers = harness.builder.build(0);
            handlers.key(ControlKey::ArrowRight);

            assert_eq!(harness.state.borrow().selected_index(), Some(1));

            let telemetry = harness.telemetry_events.borrow();
            assert_eq!(telemetry.len(), 6);
            assert!(matches!(telemetry[0], RadioTelemetryEvent::Analytics(_)));
            assert!(matches!(telemetry[1], RadioTelemetryEvent::Key(_)));
            assert!(matches!(telemetry[2], RadioTelemetryEvent::Focus(_)));
            assert!(matches!(telemetry[3], RadioTelemetryEvent::Blur(_)));
            assert!(matches!(telemetry[4], RadioTelemetryEvent::Change(_)));
            assert!(matches!(telemetry[5], RadioTelemetryEvent::Commit(_)));
            drop(telemetry);

            let keys = harness.key_events.borrow();
            assert_eq!(keys.len(), 1);
            assert_eq!(keys[0].key, ControlKey::ArrowRight);

            let order = harness.order.borrow();
            assert_eq!(order.len(), 8);
            assert!(order[0].starts_with("telemetry::Analytics"));
            assert!(order[1].starts_with("telemetry::Key"));
            assert!(order[2].starts_with("telemetry::Focus"));
            assert!(order[3].starts_with("telemetry::Blur"));
            assert!(order[4].starts_with("telemetry::Change"));
            assert!(order[5].starts_with("telemetry::Commit"));
            assert_eq!(order[6], "callback::key");
            assert_eq!(order[7], "callback::change");
        }

        #[test]
        fn ssr_render_preserves_descriptor_metadata() {
            use sycamore_crate::render_to_string;

            let state = RadioGroupState::uncontrolled(
                vec!["North".into(), "South".into()],
                false,
                RadioOrientation::Vertical,
                Some(0),
            );
            let props =
                SycamoreRadioGroupProps::new(RadioGroupProps::from_state(&state), state.clone());
            let markup = render_to_string(|cx| {
                SycamoreRadioGroup::<sycamore_crate::web::Html>(cx, props.clone())
            });

            let html = markup.to_string();
            assert!(html.contains("role=\"radiogroup\""));
            assert!(html.contains("data-index=\"0\""));
            assert!(html.contains("data-orientation=\"vertical\""));
        }

        #[test]
        fn sycamore_spreads_descriptor_extra_attributes() {
            use sycamore_crate::render_to_string;

            let state = RadioGroupState::uncontrolled(
                vec!["North".into(), "South".into()],
                false,
                RadioOrientation::Vertical,
                Some(0),
            );
            let mut props = RadioGroupProps::from_state(&state);
            // Simulate a theme override and inline style injected by automation so
            // we can confirm the Sycamore adapter defers to the descriptor
            // snapshot rather than re-listing attributes manually.
            props
                .additional_group_attributes
                .push(("style".into(), "display:grid".into()));
            props
                .additional_group_attributes
                .push(("class".into(), "group-override".into()));
            props.additional_option_attributes = vec![
                vec![
                    ("style".into(), "color:red".into()),
                    ("class".into(), "option-override".into()),
                ],
                Vec::new(),
            ];

            let markup = render_to_string(|cx| {
                let props = SycamoreRadioGroupProps::new(props.clone(), state.clone());
                SycamoreRadioGroup::<sycamore_crate::web::Html>(cx, props)
            });

            let html = markup.to_string();
            assert!(html.contains("group-override"));
            assert!(html.contains("style=\"display:grid\""));
            assert!(html.contains("option-override"));
            assert!(html.contains("style=\"color:red\""));
        }
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
