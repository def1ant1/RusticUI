//! Material flavored checkbox built on the headless [`CheckboxState`].
//!
//! Feature flags expose idiomatic components for each supported framework so
//! enterprise teams can wire the same behavioral core into multi-runtime
//! surfaces without maintaining parallel markup:
//!
//! * `react` – Enables [`react::ReactCheckbox`] which returns [`Jsx`] elements
//!   via the `wasm_bindgen` bridge and delegates style injection to the shared
//!   descriptor helpers.
//! * `yew` – Enables [`yew::YewCheckbox`] driven by `#[function_component]` for
//!   seamless integration with existing Yew applications.
//! * `leptos` – Enables [`leptos::LeptosCheckbox`] composed with
//!   `#[component]`, returning a [`leptos::View`] so Leptos signals can hydrate
//!   without diff churn.
//! * `dioxus` – Enables [`dioxus::DioxusCheckbox`] implemented with `rsx!`
//!   markup for ergonomic client rendering and SSR parity.
//! * `sycamore` – Enables [`sycamore::SycamoreCheckbox`] returning a Sycamore
//!   [`Template`](sycamore::view::Template) for teams building signal-driven
//!   dashboards.
//!
//! All adapters delegate attribute hydration to the shared
//! [`ToggleControlDescriptor`](crate::selection_control::ToggleControlDescriptor)
//! so automation hooks and ARIA metadata remain consistent across frameworks.
//!
//! Downstream orchestration layers should populate the analytics and automation
//! identifiers within [`TelemetryHooks`] *before* calling any render helper so
//! every adapter emits identical instrumentation. Providing the identifiers up
//! front allows SSR, WASM bridges, and component frameworks to produce matching
//! telemetry payloads without bolting on per-platform patches, keeping
//! enterprise monitoring pipelines aligned across runtimes.
//!
//! Every adapter now exposes optional change/focus/blur/key callbacks alongside
//! a telemetry delegate so large applications can bubble structured
//! [`CheckboxTelemetryEvent`] payloads into centralized analytics collectors.
//! The helpers reuse the headless [`CheckboxState`] to compute prior/next
//! values, ensuring automation harnesses attached to React, Yew, Leptos,
//! Dioxus, or Sycamore receive identical event contracts without writing
//! adapter-specific plumbing.

use crate::{
    selection_control::{self, ToggleControlDescriptor},
    telemetry::{instrument_render, TelemetryContext, TelemetryHooks},
};
use rustic_ui_headless::{
    checkbox::{CheckboxState, CheckboxValue},
    interaction::ControlKey,
};
use rustic_ui_styled_engine::{css_with_theme, Style};

#[cfg(feature = "react")]
use wasm_bindgen::JsValue;

/// Canonical payload emitted when checkbox state changes.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxChangeEvent {
    /// Previously resolved checked state prior to the interaction.
    pub previous: CheckboxValue,
    /// Next logical state requested by the user interaction.
    pub next: CheckboxValue,
    /// Whether the checkbox was disabled when the interaction was attempted.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the checkbox.
    pub label: String,
}

/// Canonical payload emitted when focus visibility changes.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxFocusEvent {
    /// Whether focus was gained (`true`) or lost (`false`).
    pub focused: bool,
    /// Current checked state at the time the focus transition occurred.
    pub checked: CheckboxValue,
    /// Whether the checkbox was disabled while the focus event fired.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the checkbox.
    pub label: String,
}

/// Canonical payload emitted for keyboard interactions.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxKeyEvent {
    /// Normalised control key derived from the browser event.
    pub key: ControlKey,
    /// Previously resolved checked state prior to the key interaction.
    pub previous: CheckboxValue,
    /// Next logical state requested by the key press (if any).
    pub next: CheckboxValue,
    /// Whether the checkbox was disabled when the key was pressed.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the checkbox.
    pub label: String,
}

/// Telemetry payload variants surfaced across adapters.
#[derive(Clone, Debug, PartialEq)]
pub enum CheckboxTelemetryEvent {
    /// Change request triggered via pointer or keyboard interaction.
    Change(CheckboxChangeEvent),
    /// Focus gained with the accompanying state metadata.
    Focus(CheckboxFocusEvent),
    /// Focus lost with the accompanying state metadata.
    Blur(CheckboxFocusEvent),
    /// Raw keyboard interaction payload.
    Key(CheckboxKeyEvent),
}

#[allow(dead_code)]
fn analytics_id(props: &CheckboxProps) -> Option<String> {
    props.telemetry.analytics_id.clone()
}

#[allow(dead_code)]
fn automation_id(props: &CheckboxProps) -> Option<String> {
    props.telemetry.automation_id.clone()
}

#[allow(dead_code)]
fn build_change_event(props: &CheckboxProps, state: &CheckboxState) -> CheckboxChangeEvent {
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
        analytics_id: analytics_id(props),
        automation_id: automation_id(props),
        label: props.label.clone(),
    }
}

fn build_focus_event(
    props: &CheckboxProps,
    state: &CheckboxState,
    focused: bool,
) -> CheckboxFocusEvent {
    CheckboxFocusEvent {
        focused,
        checked: state.checked(),
        disabled: state.disabled(),
        analytics_id: analytics_id(props),
        automation_id: automation_id(props),
        label: props.label.clone(),
    }
}

#[allow(dead_code)]
fn build_key_event(
    props: &CheckboxProps,
    state: &CheckboxState,
    key: ControlKey,
) -> CheckboxKeyEvent {
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
        analytics_id: analytics_id(props),
        automation_id: automation_id(props),
        label: props.label.clone(),
    }
}

#[allow(dead_code)]
fn control_key_from_str(key: &str) -> Option<ControlKey> {
    match key {
        " " | "Spacebar" | "Space" => Some(ControlKey::Space),
        "Enter" => Some(ControlKey::Enter),
        _ => None,
    }
}

#[allow(dead_code)]
fn toggled_value(value: CheckboxValue) -> CheckboxValue {
    match value {
        CheckboxValue::Off => CheckboxValue::On,
        CheckboxValue::On => CheckboxValue::Off,
        CheckboxValue::Indeterminate => CheckboxValue::On,
    }
}

/// Props shared across all framework adapters.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxProps {
    /// Visible label rendered alongside the checkbox indicator.
    pub label: String,
    /// Telemetry hooks used to decorate render lifecycles with analytics and
    /// automation identifiers.
    pub telemetry: TelemetryHooks,
}

impl CheckboxProps {
    /// Convenience constructor for tests and examples.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            telemetry: TelemetryHooks::default(),
        }
    }
}

#[allow(dead_code)]
fn build_descriptor(props: &CheckboxProps, state: &CheckboxState) -> ToggleControlDescriptor {
    let descriptor = ToggleControlDescriptor::new(props.label.clone(), themed_checkbox_style())
        .with_attributes(state.aria_attributes());
    let descriptor = if let Some(analytics) = &props.telemetry.analytics_id {
        descriptor.attribute("data-rustic-analytics-id", analytics.clone())
    } else {
        descriptor
    };
    if let Some(automation) = &props.telemetry.automation_id {
        descriptor.attribute("data-automation-id", automation.clone())
    } else {
        descriptor
    }
}

#[allow(dead_code)]
fn render_html(props: &CheckboxProps, state: &CheckboxState) -> String {
    let (context, descriptor, _snapshot) =
        descriptor_with_context("rustic_ui_material::checkbox::render_html", props, state);
    instrument_render(&props.telemetry, context, || {
        selection_control::render_toggle_html(&descriptor)
    })
}

fn merge_descriptor_telemetry(
    mut descriptor: ToggleControlDescriptor,
    telemetry: &TelemetryHooks,
) -> ToggleControlDescriptor {
    let has_analytics = descriptor
        .data_state_attributes()
        .any(|(key, _)| key == "data-rustic-analytics-id");
    if !has_analytics {
        if let Some(analytics) = &telemetry.analytics_id {
            descriptor = descriptor.attribute("data-rustic-analytics-id", analytics.clone());
        }
    }

    let has_automation = descriptor
        .data_state_attributes()
        .any(|(key, _)| key == "data-automation-id");
    if !has_automation {
        if let Some(automation) = &telemetry.automation_id {
            descriptor = descriptor.attribute("data-automation-id", automation.clone());
        }
    }

    descriptor
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CheckboxDescriptorSnapshot {
    label: String,
    themed_attributes: Vec<(String, String)>,
    class: String,
    role: String,
    aria_checked: String,
    aria_disabled: Option<String>,
    tabindex: String,
    data_checked: String,
    data_focus_visible: String,
    data_indeterminate: String,
}

impl CheckboxDescriptorSnapshot {
    fn from_descriptor(descriptor: &ToggleControlDescriptor) -> Self {
        let themed_attributes = descriptor.themed_attributes();
        let mut class = String::new();
        let mut role = String::from("checkbox");
        let mut aria_checked = String::from("false");
        let mut aria_disabled = None;
        let mut tabindex = String::from("0");
        let mut data_checked = String::from("false");
        let mut data_focus_visible = String::from("false");
        let mut data_indeterminate = String::from("false");

        for (key, value) in &themed_attributes {
            match key.as_str() {
                "class" => class = value.clone(),
                "role" => role = value.clone(),
                "aria-checked" => aria_checked = value.clone(),
                "aria-disabled" => aria_disabled = Some(value.clone()),
                "tabindex" => tabindex = value.clone(),
                "data-checked" => data_checked = value.clone(),
                "data-focus-visible" => data_focus_visible = value.clone(),
                "data-indeterminate" => data_indeterminate = value.clone(),
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
            data_indeterminate,
        }
    }
}

fn descriptor_with_context(
    component: &'static str,
    props: &CheckboxProps,
    state: &CheckboxState,
) -> (
    TelemetryContext,
    ToggleControlDescriptor,
    CheckboxDescriptorSnapshot,
) {
    let descriptor = merge_descriptor_telemetry(build_descriptor(props, state), &props.telemetry);
    let snapshot = CheckboxDescriptorSnapshot::from_descriptor(&descriptor);
    let context = TelemetryContext::new(component)
        .with_analytics(props.telemetry.analytics_id.clone())
        .with_automation(props.telemetry.automation_id.clone())
        .with_descriptor_metadata(snapshot.label.clone(), snapshot.themed_attributes.clone());
    (context, descriptor, snapshot)
}

#[cfg(feature = "react")]
fn checkbox_value_to_str(value: CheckboxValue) -> &'static str {
    match value {
        CheckboxValue::Off => "off",
        CheckboxValue::On => "on",
        CheckboxValue::Indeterminate => "indeterminate",
    }
}

#[cfg(feature = "react")]
fn control_key_to_str(key: ControlKey) -> &'static str {
    match key {
        ControlKey::Space => "space",
        ControlKey::Enter => "enter",
        ControlKey::ArrowUp => "arrow-up",
        ControlKey::ArrowDown => "arrow-down",
        ControlKey::ArrowLeft => "arrow-left",
        ControlKey::ArrowRight => "arrow-right",
        ControlKey::Home => "home",
        ControlKey::End => "end",
    }
}

#[cfg(feature = "react")]
fn push_optional_string(
    object: &js_sys::Object,
    key: &str,
    value: &Option<String>,
) -> Result<(), JsValue> {
    if let Some(value) = value {
        js_sys::Reflect::set(object, &JsValue::from_str(key), &JsValue::from_str(value))?;
    }
    Ok(())
}

#[cfg(feature = "react")]
fn telemetry_event_to_js(event: CheckboxTelemetryEvent) -> JsValue {
    use js_sys::Reflect;
    use CheckboxTelemetryEvent as Event;

    let object = js_sys::Object::new();
    match event {
        Event::Change(change) => {
            Reflect::set(
                &object,
                &JsValue::from_str("kind"),
                &JsValue::from_str("change"),
            )
            .expect("set kind");
            Reflect::set(
                &object,
                &JsValue::from_str("previous"),
                &JsValue::from_str(checkbox_value_to_str(change.previous)),
            )
            .expect("set previous");
            Reflect::set(
                &object,
                &JsValue::from_str("next"),
                &JsValue::from_str(checkbox_value_to_str(change.next)),
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
        Event::Focus(focus) => {
            Reflect::set(
                &object,
                &JsValue::from_str("kind"),
                &JsValue::from_str("focus"),
            )
            .expect("set kind");
            Reflect::set(
                &object,
                &JsValue::from_str("focused"),
                &JsValue::from_bool(focus.focused),
            )
            .expect("set focused");
            Reflect::set(
                &object,
                &JsValue::from_str("checked"),
                &JsValue::from_str(checkbox_value_to_str(focus.checked)),
            )
            .expect("set checked");
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
        Event::Blur(focus) => {
            Reflect::set(
                &object,
                &JsValue::from_str("kind"),
                &JsValue::from_str("blur"),
            )
            .expect("set kind");
            Reflect::set(
                &object,
                &JsValue::from_str("focused"),
                &JsValue::from_bool(focus.focused),
            )
            .expect("set focused");
            Reflect::set(
                &object,
                &JsValue::from_str("checked"),
                &JsValue::from_str(checkbox_value_to_str(focus.checked)),
            )
            .expect("set checked");
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
                &JsValue::from_str(control_key_to_str(key.key)),
            )
            .expect("set key");
            Reflect::set(
                &object,
                &JsValue::from_str("previous"),
                &JsValue::from_str(checkbox_value_to_str(key.previous)),
            )
            .expect("set previous");
            Reflect::set(
                &object,
                &JsValue::from_str("next"),
                &JsValue::from_str(checkbox_value_to_str(key.next)),
            )
            .expect("set next");
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
    }

    object.into()
}

/// Generates the themed style for the checkbox container. The macro pulls
/// palette colors, typography metrics and spacing tokens from the active
/// [`Theme`](rustic_ui_styled_engine::Theme) so enterprise teams can rely on global
/// design governance rather than tweaking individual components.
#[allow(dead_code)]
fn themed_checkbox_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        cursor: pointer;
        color: ${text_color};
        position: relative;
        font-family: ${font_family};
        font-size: ${font_size};

        &::before {
            content: "";
            display: inline-block;
            width: ${box_size};
            height: ${box_size};
            margin-right: ${gap};
            border-radius: ${box_radius};
            border: 2px solid ${border_color};
            background: ${box_background};
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
            opacity: 0.38;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        radius = format!("{}px", theme.joy.radius),
        text_color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body1),
        box_size = format!("{}px", theme.spacing(2)),
        box_radius = format!("{}px", theme.joy.radius),
        border_color = theme.palette.text_secondary.clone(),
        box_background = theme.palette.background_paper.clone(),
        checked_background = theme.palette.primary.clone(),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
}

#[cfg(feature = "react")]
pub mod react {
    //! React adapter producing `Jsx` nodes via the `wasm_bindgen` bridge.
    use super::*;
    use js_sys::{Array, Function, Object, Reflect};
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};

    /// Type alias representing React elements returned through the WASM bridge.
    pub type Jsx = JsValue;

    /// Properties consumed by the React checkbox component.
    #[derive(Clone, Debug)]
    pub struct ReactCheckboxProps {
        /// Visual label rendered beside the checkbox indicator.
        pub checkbox: CheckboxProps,
        /// Headless state machine powering ARIA metadata.
        pub state: CheckboxState,
        /// Optional React `onChange` handler bridging to enterprise orchestrators.
        pub on_change: Option<Function>,
        /// Optional React `onFocus` handler forwarding focus analytics payloads.
        pub on_focus: Option<Function>,
        /// Optional React `onBlur` handler forwarding blur analytics payloads.
        pub on_blur: Option<Function>,
        /// Optional React `onKeyDown` handler forwarding keyboard payloads.
        pub on_key: Option<Function>,
        /// Optional telemetry payload delegate invoked by surrounding shells.
        pub telemetry_delegate: Option<Function>,
    }

    fn function_option_eq(lhs: &Option<Function>, rhs: &Option<Function>) -> bool {
        match (lhs, rhs) {
            (Some(a), Some(b)) => JsValue::from(a).strict_eq(&JsValue::from(b)),
            (None, None) => true,
            _ => false,
        }
    }

    impl PartialEq for ReactCheckboxProps {
        fn eq(&self, other: &Self) -> bool {
            self.checkbox == other.checkbox
                && self.state == other.state
                && function_option_eq(&self.on_change, &other.on_change)
                && function_option_eq(&self.on_focus, &other.on_focus)
                && function_option_eq(&self.on_blur, &other.on_blur)
                && function_option_eq(&self.on_key, &other.on_key)
                && function_option_eq(&self.telemetry_delegate, &other.telemetry_delegate)
        }
    }

    fn create_element(tag: &str, props: Object, children: &[JsValue]) -> JsValue {
        let global = js_sys::global();
        let react = Reflect::get(&global, &JsValue::from_str("React"))
            .expect("React global should be present when the `react` feature is enabled");
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

    struct ReactHandlers {
        on_change: Option<Function>,
        on_focus: Option<Function>,
        on_blur: Option<Function>,
        on_key: Option<Function>,
    }

    fn build_props_object(pairs: Vec<(String, String)>, handlers: &ReactHandlers) -> Object {
        let object = Object::new();
        for (key, value) in pairs {
            Reflect::set(
                &object,
                &JsValue::from_str(&key),
                &JsValue::from_str(&value),
            )
            .expect("set React prop");
        }
        if let Some(handler) = &handlers.on_change {
            Reflect::set(&object, &JsValue::from_str("onChange"), handler)
                .expect("set onChange handler");
        }
        if let Some(handler) = &handlers.on_focus {
            Reflect::set(&object, &JsValue::from_str("onFocus"), handler)
                .expect("set onFocus handler");
        }
        if let Some(handler) = &handlers.on_blur {
            Reflect::set(&object, &JsValue::from_str("onBlur"), handler)
                .expect("set onBlur handler");
        }
        if let Some(handler) = &handlers.on_key {
            Reflect::set(&object, &JsValue::from_str("onKeyDown"), handler)
                .expect("set onKeyDown handler");
        }
        object
    }

    fn change_handler(
        props: &ReactCheckboxProps,
        state_handle: &Rc<RefCell<CheckboxState>>,
    ) -> Option<Function> {
        if props.on_change.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_change = props.on_change.clone();
        let checkbox = props.checkbox.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let change = {
                let state = state.borrow();
                build_change_event(&checkbox, &state)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(CheckboxTelemetryEvent::Change(change.clone()));
                let _ = delegate.call1(&JsValue::NULL, &payload);
            }
            if let Some(handler) = on_change.as_ref() {
                let _ = handler.call1(&JsValue::NULL, &event);
            }
            {
                let mut state = state.borrow_mut();
                if !state.is_controlled() {
                    state.toggle(|_| {});
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn focus_handler(
        props: &ReactCheckboxProps,
        state_handle: &Rc<RefCell<CheckboxState>>,
    ) -> Option<Function> {
        if props.on_focus.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_focus = props.on_focus.clone();
        let checkbox = props.checkbox.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let focus = {
                let state = state.borrow();
                build_focus_event(&checkbox, &state, true)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(CheckboxTelemetryEvent::Focus(focus.clone()));
                let _ = delegate.call1(&JsValue::NULL, &payload);
            }
            if let Some(handler) = on_focus.as_ref() {
                let _ = handler.call1(&JsValue::NULL, &event);
            }
            {
                let mut state = state.borrow_mut();
                state.focus();
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn blur_handler(
        props: &ReactCheckboxProps,
        state_handle: &Rc<RefCell<CheckboxState>>,
    ) -> Option<Function> {
        if props.on_blur.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_blur = props.on_blur.clone();
        let checkbox = props.checkbox.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let blur = {
                let state = state.borrow();
                build_focus_event(&checkbox, &state, false)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(CheckboxTelemetryEvent::Blur(blur.clone()));
                let _ = delegate.call1(&JsValue::NULL, &payload);
            }
            if let Some(handler) = on_blur.as_ref() {
                let _ = handler.call1(&JsValue::NULL, &event);
            }
            {
                let mut state = state.borrow_mut();
                state.blur();
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn key_handler(
        props: &ReactCheckboxProps,
        state_handle: &Rc<RefCell<CheckboxState>>,
    ) -> Option<Function> {
        if props.on_key.is_none() && props.on_change.is_none() && props.telemetry_delegate.is_none()
        {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_key = props.on_key.clone();
        let on_change = props.on_change.clone();
        let checkbox = props.checkbox.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let key_value = Reflect::get(&event, &JsValue::from_str("key"))
                .ok()
                .and_then(|value| value.as_string());
            if let Some(key) = key_value.as_deref().and_then(control_key_from_str) {
                if let Ok(prevent) = Reflect::get(&event, &JsValue::from_str("preventDefault")) {
                    if let Ok(prevent) = prevent.dyn_into::<Function>() {
                        let _ = prevent.call0(&event);
                    }
                }

                let (key_event, change_event) = {
                    let state = state.borrow();
                    (
                        build_key_event(&checkbox, &state, key),
                        build_change_event(&checkbox, &state),
                    )
                };

                if let Some(delegate) = telemetry.as_ref() {
                    let key_payload =
                        telemetry_event_to_js(CheckboxTelemetryEvent::Key(key_event.clone()));
                    let _ = delegate.call1(&JsValue::NULL, &key_payload);
                    let change_payload =
                        telemetry_event_to_js(CheckboxTelemetryEvent::Change(change_event.clone()));
                    let _ = delegate.call1(&JsValue::NULL, &change_payload);
                }

                if let Some(handler) = on_key.as_ref() {
                    let _ = handler.call1(&JsValue::NULL, &event);
                }

                if let Some(handler) = on_change.as_ref() {
                    let _ = handler.call1(&JsValue::NULL, &event);
                }

                {
                    let mut state = state.borrow_mut();
                    if !state.is_controlled() {
                        state.on_key(key, |_| {});
                    }
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn handlers(
        props: &ReactCheckboxProps,
        state_handle: Rc<RefCell<CheckboxState>>,
    ) -> ReactHandlers {
        ReactHandlers {
            on_change: change_handler(props, &state_handle),
            on_focus: focus_handler(props, &state_handle),
            on_blur: blur_handler(props, &state_handle),
            on_key: key_handler(props, &state_handle),
        }
    }

    #[cfg(all(test, feature = "react"))]
    mod tests {
        use super::*;
        use js_sys::{Function, Object, Reflect};
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen_test::*;

        wasm_bindgen_test_configure!(run_in_browser);

        fn parse_checkbox_value(value: &JsValue) -> CheckboxValue {
            match value
                .as_string()
                .expect("checkbox telemetry should encode strings")
                .as_str()
            {
                "on" => CheckboxValue::On,
                "off" => CheckboxValue::Off,
                "indeterminate" => CheckboxValue::Indeterminate,
                other => panic!("unexpected checkbox value {other}"),
            }
        }

        fn parse_control_key(value: &JsValue) -> ControlKey {
            match value
                .as_string()
                .expect("key telemetry should encode the control key")
                .as_str()
            {
                "space" => ControlKey::Space,
                "enter" => ControlKey::Enter,
                other => panic!("unexpected control key {other}"),
            }
        }

        fn optional_string(object: &JsValue, key: &str) -> Option<String> {
            Reflect::get(object, &JsValue::from_str(key))
                .ok()
                .and_then(|value| value.as_string())
        }

        fn decode_checkbox_event(value: &JsValue) -> CheckboxTelemetryEvent {
            let kind = Reflect::get(value, &JsValue::from_str("kind"))
                .expect("telemetry payload exposes kind metadata")
                .as_string()
                .expect("telemetry kind should be a string");
            match kind.as_str() {
                "change" => CheckboxTelemetryEvent::Change(CheckboxChangeEvent {
                    previous: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("previous"))
                            .expect("previous should be set"),
                    ),
                    next: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("next"))
                            .expect("next should be set"),
                    ),
                    disabled: Reflect::get(value, &JsValue::from_str("disabled"))
                        .expect("disabled should be set")
                        .as_bool()
                        .expect("disabled encodes a boolean"),
                    analytics_id: optional_string(value, "analyticsId"),
                    automation_id: optional_string(value, "automationId"),
                    label: Reflect::get(value, &JsValue::from_str("label"))
                        .expect("label should be set")
                        .as_string()
                        .expect("label encodes a string"),
                }),
                "focus" => CheckboxTelemetryEvent::Focus(CheckboxFocusEvent {
                    focused: Reflect::get(value, &JsValue::from_str("focused"))
                        .expect("focused should be set")
                        .as_bool()
                        .expect("focused encodes a boolean"),
                    checked: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("checked"))
                            .expect("checked should be set"),
                    ),
                    disabled: Reflect::get(value, &JsValue::from_str("disabled"))
                        .expect("disabled should be set")
                        .as_bool()
                        .expect("disabled encodes a boolean"),
                    analytics_id: optional_string(value, "analyticsId"),
                    automation_id: optional_string(value, "automationId"),
                    label: Reflect::get(value, &JsValue::from_str("label"))
                        .expect("label should be set")
                        .as_string()
                        .expect("label encodes a string"),
                }),
                "blur" => CheckboxTelemetryEvent::Blur(CheckboxFocusEvent {
                    focused: Reflect::get(value, &JsValue::from_str("focused"))
                        .expect("focused should be set")
                        .as_bool()
                        .expect("focused encodes a boolean"),
                    checked: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("checked"))
                            .expect("checked should be set"),
                    ),
                    disabled: Reflect::get(value, &JsValue::from_str("disabled"))
                        .expect("disabled should be set")
                        .as_bool()
                        .expect("disabled encodes a boolean"),
                    analytics_id: optional_string(value, "analyticsId"),
                    automation_id: optional_string(value, "automationId"),
                    label: Reflect::get(value, &JsValue::from_str("label"))
                        .expect("label should be set")
                        .as_string()
                        .expect("label encodes a string"),
                }),
                "key" => CheckboxTelemetryEvent::Key(CheckboxKeyEvent {
                    key: parse_control_key(
                        &Reflect::get(value, &JsValue::from_str("key")).expect("key should be set"),
                    ),
                    previous: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("previous"))
                            .expect("previous should be set"),
                    ),
                    next: parse_checkbox_value(
                        &Reflect::get(value, &JsValue::from_str("next"))
                            .expect("next should be set"),
                    ),
                    disabled: Reflect::get(value, &JsValue::from_str("disabled"))
                        .expect("disabled should be set")
                        .as_bool()
                        .expect("disabled encodes a boolean"),
                    analytics_id: optional_string(value, "analyticsId"),
                    automation_id: optional_string(value, "automationId"),
                    label: Reflect::get(value, &JsValue::from_str("label"))
                        .expect("label should be set")
                        .as_string()
                        .expect("label encodes a string"),
                }),
                other => panic!("unexpected telemetry kind {other}"),
            }
        }

        fn telemetry_recorder() -> (
            Function,
            Rc<RefCell<Vec<CheckboxTelemetryEvent>>>,
            Closure<dyn FnMut(JsValue)>,
        ) {
            let events = Rc::new(RefCell::new(Vec::new()));
            let stored = Rc::clone(&events);
            let closure = Closure::wrap(Box::new(move |value: JsValue| {
                let event = decode_checkbox_event(&value);
                stored.borrow_mut().push(event);
            }) as Box<dyn FnMut(JsValue)>);
            let function: Function = closure.as_ref().clone().unchecked_into();
            (function, events, closure)
        }

        fn event_kinds(events: &[CheckboxTelemetryEvent]) -> Vec<&'static str> {
            events
                .iter()
                .map(|event| match event {
                    CheckboxTelemetryEvent::Change(_) => "change",
                    CheckboxTelemetryEvent::Focus(_) => "focus",
                    CheckboxTelemetryEvent::Blur(_) => "blur",
                    CheckboxTelemetryEvent::Key(_) => "key",
                })
                .collect()
        }

        fn invoke(handler: &Option<Function>, event: JsValue) {
            if let Some(function) = handler {
                let _ = function.call1(&JsValue::NULL, &event);
            }
        }

        fn keyboard_event(key: &str) -> JsValue {
            let event = Object::new();
            Reflect::set(&event, &JsValue::from_str("key"), &JsValue::from_str(key))
                .expect("set key");
            let prevent = Closure::wrap(Box::new(|| {}) as Box<dyn FnMut()>);
            Reflect::set(
                &event,
                &JsValue::from_str("preventDefault"),
                prevent.as_ref(),
            )
            .expect("set preventDefault");
            prevent.forget();
            event.into()
        }

        fn build_props(state: CheckboxState, telemetry: Option<Function>) -> ReactCheckboxProps {
            ReactCheckboxProps {
                checkbox: CheckboxProps::new("Terms of use"),
                state,
                on_change: None,
                on_focus: None,
                on_blur: None,
                on_key: None,
                telemetry_delegate: telemetry,
            }
        }

        #[wasm_bindgen_test]
        fn uncontrolled_handlers_emit_ordered_telemetry() {
            let (delegate, events, closure) = telemetry_recorder();
            let props = build_props(
                CheckboxState::uncontrolled(false, CheckboxValue::Off),
                Some(delegate),
            );
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let handlers = handlers(&props, Rc::clone(&state_handle));

            // Focus metadata must land before any mutation attempts so analytics can
            // observe the untouched state. We therefore expect the focus event to be
            // the first entry in the telemetry log.
            invoke(&handlers.on_focus, JsValue::UNDEFINED);
            invoke(&handlers.on_blur, JsValue::UNDEFINED);

            // Keyboard flows should emit their key payload *before* the change event
            // to honour the analytics → key/focus → change sequencing.
            invoke(&handlers.on_key, keyboard_event(" "));

            // Pointer toggles only emit the change payload; this still runs after the
            // keyboard telemetry so mutation commits happen last.
            invoke(&handlers.on_change, JsValue::UNDEFINED);

            let kinds = event_kinds(&events.borrow());
            assert_eq!(kinds, vec!["focus", "blur", "key", "change", "change"]);
            assert_eq!(state_handle.borrow().checked(), CheckboxValue::Off);

            // The first change toggled the uncontrolled state to `On`, while the
            // pointer interaction toggled it back to `Off`. Verifying the round-trip
            // ensures commit semantics occur after telemetry dispatch.
            let events_ref = events.borrow();
            let mut iter = events_ref.iter();
            assert!(matches!(
                iter.find(|event| matches!(event, CheckboxTelemetryEvent::Key(_))),
                Some(_)
            ));
            drop(events_ref);
            drop(closure);
        }

        #[wasm_bindgen_test]
        fn controlled_handlers_preserve_state() {
            let (delegate, events, closure) = telemetry_recorder();
            let props = build_props(
                CheckboxState::controlled(false, CheckboxValue::Off),
                Some(delegate),
            );
            let state_handle = Rc::new(RefCell::new(props.state.clone()));
            let handlers = handlers(&props, Rc::clone(&state_handle));

            invoke(&handlers.on_key, keyboard_event("Enter"));
            invoke(&handlers.on_change, JsValue::UNDEFINED);

            // Controlled flows must emit telemetry but keep their state untouched,
            // reinforcing analytics → key → change → commit ordering without
            // mutating local caches.
            assert_eq!(
                event_kinds(&events.borrow()),
                vec!["key", "change", "change"]
            );
            assert_eq!(state_handle.borrow().checked(), CheckboxValue::Off);
            drop(closure);
        }
    }

    /// React component rendering the Material checkbox.
    ///
    /// The adapter mirrors the documentation-heavy style used by other modules
    /// (see [`click_away`](crate::click_away) or [`collapsible`](crate::collapsible))
    /// so enterprise governance teams understand the render lifecycle:
    ///
    /// * A [`TelemetryContext`] seeded with the fully-qualified component path
    ///   and decorated with descriptor metadata is constructed so downstream
    ///   spans, analytics sinks, and error hooks can attribute metrics and
    ///   attribute snapshots back to this adapter.
    /// * [`instrument_render`] enters the context span, ensures success/error
    ///   hooks run, and propagates analytics/automation identifiers extracted
    ///   from [`CheckboxProps::telemetry`].
    /// * Prior to hydration, telemetry defaults are merged into the descriptor
    ///   attributes so SSR and CSR renders emit identical `data-*` markers even
    ///   when the caller omits explicit identifiers.
    /// * Event handlers wrap consumer callbacks, delivering normalized
    ///   [`CheckboxTelemetryEvent`] payloads to the optional telemetry delegate
    ///   **before** invoking user logic. This guarantees analytics capture
    ///   precedes side effects, aligning with audit requirements in regulated
    ///   environments.
    /// * After telemetry delegates and consumer callbacks run, the captured
    ///   [`CheckboxState`] mutates via [`CheckboxState::toggle`],
    ///   [`CheckboxState::focus`], [`CheckboxState::blur`], and
    ///   [`CheckboxState::on_key`] whenever it owns its value. Controlled
    ///   integrations trigger the same telemetry/change notifications but the
    ///   bridge guards the mutation with [`CheckboxState::is_controlled`] so
    ///   local snapshots stay in sync with external sources of truth.
    pub fn ReactCheckbox(props: &ReactCheckboxProps) -> Jsx {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::checkbox::react::ReactCheckbox",
            &props.checkbox,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.checkbox.telemetry, context, || {
            let label = snapshot.label.clone();
            let attributes = snapshot.themed_attributes.clone();
            let handlers = handlers(props, Rc::clone(&state_handle));
            let props_object = build_props_object(attributes, &handlers);
            create_element("span", props_object, &[JsValue::from_str(&label)])
        })
    }
}

#[cfg(feature = "yew")]
pub mod yew {
    //! Yew adapter implemented with `#[function_component]` so downstream apps
    //! can compose the checkbox like any other Yew widget.
    use super::*;
    use yew::events::{FocusEvent, KeyboardEvent, MouseEvent};
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

    /// Properties consumed by [`YewCheckbox`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine providing accessibility metadata.
        pub state: CheckboxState,
        /// Optional change callback invoked with [`CheckboxChangeEvent`].
        #[prop_or_default]
        pub on_change: Option<Callback<CheckboxChangeEvent>>,
        /// Optional focus callback invoked when the checkbox gains focus.
        #[prop_or_default]
        pub on_focus: Option<Callback<CheckboxFocusEvent>>,
        /// Optional blur callback invoked when the checkbox loses focus.
        #[prop_or_default]
        pub on_blur: Option<Callback<CheckboxFocusEvent>>,
        /// Optional keyboard callback invoked with normalized control keys.
        #[prop_or_default]
        pub on_key: Option<Callback<CheckboxKeyEvent>>,
        /// Optional telemetry delegate invoked with structured payloads.
        #[prop_or_default]
        pub telemetry_delegate: Option<Callback<CheckboxTelemetryEvent>>,
    }

    /// Checkbox rendered as a Yew component.
    ///
    /// The function mirrors the inline documentation style adopted by other
    /// enterprise adapters, spelling out the render lifecycle so governance and
    /// QA teams can trace telemetry:
    ///
    /// * [`TelemetryContext`] is seeded with the component name, populated with
    ///   analytics/automation identifiers from [`CheckboxProps`], and enriched
    ///   with descriptor metadata so diagnostics capture the rendered
    ///   attributes.
    /// * [`instrument_render`] enters the span and invokes success/error hooks,
    ///   ensuring render panics propagate structured
    ///   [`crate::telemetry::TelemetryError`] events.
    /// * Descriptor telemetry defaults are merged prior to calling
    ///   [`ToggleControlDescriptor::themed_attributes`], keeping SSR/CSR output
    ///   aligned even when consumers omit explicit identifiers.
    /// * Event handlers route structured [`CheckboxTelemetryEvent`] payloads to
    ///   the optional telemetry delegate **before** invoking user callbacks so
    ///   analytics capture consistently precedes consumer side effects.
    /// * After telemetry flows, the shared [`CheckboxState`] transitions via
    ///   [`CheckboxState::toggle`], [`CheckboxState::focus`],
    ///   [`CheckboxState::blur`], and [`CheckboxState::on_key`] whenever it owns
    ///   its value. Controlled parents still receive telemetry/change
    ///   callbacks, yet the bridge checks [`CheckboxState::is_controlled`] so
    ///   the local snapshot stays untouched until the external owner syncs it.
    #[function_component(YewCheckbox)]
    pub fn yew_checkbox(props: &YewCheckboxProps) -> Html {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::checkbox::yew::YewCheckbox",
            &props.checkbox,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.checkbox.telemetry, context, || {
            let label = snapshot.label.clone();
            let attrs = snapshot.themed_attributes.clone();
            let change_handler = {
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: MouseEvent| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&checkbox, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate.emit(CheckboxTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb.emit(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        if !state.is_controlled() {
                            state.toggle(|_| {});
                        }
                    }
                })
            };
            let focus_handler = {
                let on_focus = props.on_focus.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: FocusEvent| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate.emit(CheckboxTelemetryEvent::Focus(focus.clone()));
                    }
                    if let Some(cb) = &on_focus {
                        cb.emit(focus.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.focus();
                    }
                })
            };
            let blur_handler = {
                let on_blur = props.on_blur.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: FocusEvent| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate.emit(CheckboxTelemetryEvent::Blur(blur.clone()));
                    }
                    if let Some(cb) = &on_blur {
                        cb.emit(blur.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.blur();
                    }
                })
            };
            let key_handler = {
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |event: KeyboardEvent| {
                    if let Some(control) = control_key_from_str(event.key().as_str()) {
                        event.prevent_default();
                        let (key_event, change) = {
                            let state = state.borrow();
                            (
                                build_key_event(&checkbox, &state, control),
                                build_change_event(&checkbox, &state),
                            )
                        };
                        if let Some(delegate) = &telemetry {
                            delegate.emit(CheckboxTelemetryEvent::Key(key_event.clone()));
                            delegate.emit(CheckboxTelemetryEvent::Change(change.clone()));
                        }
                        if let Some(cb) = &on_key {
                            cb.emit(key_event.clone());
                        }
                        if let Some(change_cb) = &on_change {
                            change_cb.emit(change.clone());
                        }
                        {
                            let mut state = state.borrow_mut();
                            if !state.is_controlled() {
                                state.on_key(control, |_| {});
                            }
                        }
                    }
                })
            };
            let mut node = html! {
                <span onclick={change_handler} onfocus={focus_handler} onblur={blur_handler} onkeydown={key_handler}>{label}</span>
            };
            if let VNode::VTag(ref mut tag) = node {
                for (key, value) in attrs {
                    tag.add_attribute(key, value);
                }
            }
            node
        })
    }
}

#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter returning a [`leptos::View`] so reactive signals can drive
    //! the checkbox while sharing the descriptor wiring used by other
    //! frameworks.
    use super::*;
    use leptos::ev::{FocusEvent, KeyboardEvent, MouseEvent};
    use leptos::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    /// Properties accepted by [`LeptosCheckbox`].
    #[derive(Clone)]
    pub struct LeptosCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine describing behavior and ARIA metadata.
        pub state: CheckboxState,
        /// Optional change callback emitted when toggles occur.
        pub on_change: Option<Rc<dyn Fn(CheckboxChangeEvent)>>,
        /// Optional focus callback emitted when focus is gained.
        pub on_focus: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional blur callback emitted when focus is lost.
        pub on_blur: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional key callback emitted with normalized control keys.
        pub on_key: Option<Rc<dyn Fn(CheckboxKeyEvent)>>,
        /// Optional telemetry delegate invoked with structured payloads.
        pub telemetry_delegate: Option<Rc<dyn Fn(CheckboxTelemetryEvent)>>,
    }

    #[component]
    /// Leptos adapter mirroring the telemetry-heavy lifecycle described for the
    /// Yew/React integrations. The documentation explicitly calls out the
    /// render order so observability and QA teams can validate automation:
    ///
    /// * A [`TelemetryContext`] seeded with the component name and descriptor
    ///   metadata feeds [`instrument_render`], wiring analytics/automation
    ///   identifiers and attribute snapshots into the tracing span.
    /// * Descriptor telemetry defaults are merged before extracting themed
    ///   attributes, guaranteeing SSR and CSR renders emit identical `data-*`
    ///   markers.
    /// * Event closures emit [`CheckboxTelemetryEvent`] payloads to the
    ///   telemetry delegate **before** executing user callbacks, keeping
    ///   analytics capture deterministic.
    /// * [`CheckboxState`] transitions (`toggle`/`focus`/`blur`/`on_key`) execute
    ///   after telemetry delegates fire whenever the checkbox owns its value, so
    ///   the UI mutates through the shared headless state machine while
    ///   preserving analytics ordering. Controlled flows emit the same telemetry
    ///   yet skip the mutation because the bridge checks
    ///   [`CheckboxState::is_controlled`], keeping host-owned truth authoritative.
    pub fn LeptosCheckbox(props: LeptosCheckboxProps) -> impl IntoView {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::checkbox::leptos::LeptosCheckbox",
            &props.checkbox,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.checkbox.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_checked = snapshot.data_checked.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let data_indeterminate = snapshot.data_indeterminate.clone();
            let on_click = {
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: MouseEvent| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&checkbox, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        if !state.is_controlled() {
                            state.toggle(|_| {});
                        }
                    }
                }
            };
            let on_focus = {
                let on_focus = props.on_focus.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Focus(focus.clone()));
                    }
                    if let Some(cb) = &on_focus {
                        cb(focus.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.focus();
                    }
                }
            };
            let on_blur = {
                let on_blur = props.on_blur.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Blur(blur.clone()));
                    }
                    if let Some(cb) = &on_blur {
                        cb(blur.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.blur();
                    }
                }
            };
            let on_key = {
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    if let Some(control) = control_key_from_str(event.key().as_str()) {
                        event.prevent_default();
                        let (key_event, change) = {
                            let state = state.borrow();
                            (
                                build_key_event(&checkbox, &state, control),
                                build_change_event(&checkbox, &state),
                            )
                        };
                        if let Some(delegate) = &telemetry {
                            delegate(CheckboxTelemetryEvent::Key(key_event.clone()));
                            delegate(CheckboxTelemetryEvent::Change(change.clone()));
                        }
                        if let Some(cb) = &on_key {
                            cb(key_event.clone());
                        }
                        if let Some(change_cb) = &on_change {
                            change_cb(change.clone());
                        }
                        {
                            let mut state = state.borrow_mut();
                            if !state.is_controlled() {
                                state.on_key(control, |_| {});
                            }
                        }
                    }
                }
            };

            view! {
                <span
                    class=class
                    role=role
                    aria-checked=aria_checked
                    aria-disabled=aria_disabled
                    tabindex=tabindex
                    data-checked=data_checked
                    data-focus-visible=data_focus_visible
                    data-indeterminate=data_indeterminate
                    on:click=on_click
                    on:focus=on_focus
                    on:blur=on_blur
                    on:keydown=on_key
                >{label}</span>
            }
        })
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter using `rsx!` so teams can hydrate the checkbox inside
    //! Dioxus shells without falling back to raw HTML strings.
    use super::*;
    use ::dioxus::prelude::events::{FocusEvent, KeyboardEvent, MouseEvent};
    use ::dioxus::prelude::*;
    use keyboard_types::Key;
    use std::{cell::RefCell, rc::Rc};

    /// Properties accepted by [`DioxusCheckbox`].
    #[derive(Props, Clone)]
    pub struct DioxusCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine describing accessibility metadata.
        pub state: CheckboxState,
        /// Optional change callback executed by client integrations.
        #[props(optional)]
        pub on_change: Option<Rc<dyn Fn(CheckboxChangeEvent)>>,
        /// Optional focus callback executed by client integrations.
        #[props(optional)]
        pub on_focus: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional blur callback executed by client integrations.
        #[props(optional)]
        pub on_blur: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional keyboard callback executed by client integrations.
        #[props(optional)]
        pub on_key: Option<Rc<dyn Fn(CheckboxKeyEvent)>>,
        /// Optional telemetry delegate invoked by automation shells.
        #[props(optional)]
        pub telemetry_delegate: Option<Rc<dyn Fn(CheckboxTelemetryEvent)>>,
    }

    fn rc_option_eq<T: ?Sized>(lhs: &Option<Rc<T>>, rhs: &Option<Rc<T>>) -> bool {
        match (lhs, rhs) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }

    impl PartialEq for DioxusCheckboxProps {
        fn eq(&self, other: &Self) -> bool {
            self.checkbox == other.checkbox
                && self.state.checked() == other.state.checked()
                && self.state.disabled() == other.state.disabled()
                && self.state.focus_visible() == other.state.focus_visible()
                && rc_option_eq(&self.on_change, &other.on_change)
                && rc_option_eq(&self.on_focus, &other.on_focus)
                && rc_option_eq(&self.on_blur, &other.on_blur)
                && rc_option_eq(&self.on_key, &other.on_key)
                && rc_option_eq(&self.telemetry_delegate, &other.telemetry_delegate)
        }
    }

    /// Checkbox rendered through the Dioxus virtual DOM.
    ///
    /// The adapter mirrors the analytics-first lifecycle established in the
    /// React/Yew/Leptos integrations:
    ///
    /// * A [`TelemetryContext`] enriched with descriptor metadata scopes the
    ///   render span so tracing subscribers record both component identity and
    ///   attribute snapshots.
    /// * Descriptor telemetry defaults are merged before deriving themed
    ///   attributes which keeps SSR and client renders aligned even when
    ///   orchestration layers omit explicit identifiers.
    /// * Event handlers deliver [`CheckboxTelemetryEvent`] payloads to optional
    ///   telemetry delegates **before** invoking consumer callbacks, ensuring
    ///   analytics capture precedes business logic.
    /// * After telemetry dispatch, the shared [`CheckboxState`] transitions via
    ///   [`CheckboxState::toggle`], [`CheckboxState::focus`],
    ///   [`CheckboxState::blur`], and [`CheckboxState::on_key`] whenever it owns
    ///   its value, guaranteeing UI updates flow through the headless state
    ///   machine. Controlled consumers still receive telemetry/change payloads
    ///   while the guard around [`CheckboxState::is_controlled`] prevents local
    ///   mutation until hosts sync external truth.
    pub fn DioxusCheckbox(cx: Scope<DioxusCheckboxProps>) -> Element {
        let props = cx.props();
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::checkbox::dioxus::DioxusCheckbox",
            &props.checkbox,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.checkbox.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_checked = snapshot.data_checked.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let data_indeterminate = snapshot.data_indeterminate.clone();
            let onclick = {
                let telemetry = props.telemetry_delegate.clone();
                let on_change = props.on_change.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: MouseEvent| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&checkbox, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        if !state.is_controlled() {
                            state.toggle(|_| {});
                        }
                    }
                }
            };
            let on_focus = {
                let telemetry = props.telemetry_delegate.clone();
                let on_focus = props.on_focus.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Focus(focus.clone()));
                    }
                    if let Some(cb) = &on_focus {
                        cb(focus.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.focus();
                    }
                }
            };
            let on_blur = {
                let telemetry = props.telemetry_delegate.clone();
                let on_blur = props.on_blur.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Blur(blur.clone()));
                    }
                    if let Some(cb) = &on_blur {
                        cb(blur.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.blur();
                    }
                }
            };
            let on_key = {
                let telemetry = props.telemetry_delegate.clone();
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    let key = match event.data.key() {
                        Key::Space | Key::Character(ref ch) if ch == " " => Some(" ".to_string()),
                        Key::Enter => Some("Enter".to_string()),
                        _ => None,
                    };
                    if let Some(name) = key {
                        if let Some(control) = control_key_from_str(&name) {
                            let (key_event, change) = {
                                let state = state.borrow();
                                (
                                    build_key_event(&checkbox, &state, control),
                                    build_change_event(&checkbox, &state),
                                )
                            };
                            if let Some(delegate) = &telemetry {
                                delegate(CheckboxTelemetryEvent::Key(key_event.clone()));
                                delegate(CheckboxTelemetryEvent::Change(change.clone()));
                            }
                            if let Some(cb) = &on_key {
                                cb(key_event.clone());
                            }
                            if let Some(change_cb) = &on_change {
                                change_cb(change.clone());
                            }
                            {
                                let mut state = state.borrow_mut();
                                if !state.is_controlled() {
                                    state.on_key(control, |_| {});
                                }
                            }
                        }
                    }
                }
            };

            cx.render(rsx! {
                span {
                    class: class,
                    role: role,
                    aria_checked: aria_checked,
                    aria_disabled: aria_disabled,
                    tabindex: tabindex,
                    data_checked: data_checked,
                    data_focus_visible: data_focus_visible,
                    data_indeterminate: data_indeterminate,
                    onclick: onclick,
                    onfocus: on_focus,
                    onblur: on_blur,
                    onkeydown: on_key,
                    {label}
                }
            })
        })
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter returning a [`Template`] for signal driven surfaces.
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    use sycamore::prelude::*;
    use sycamore::web::html::event::KeyboardEvent;

    /// Alias matching the return type expected by Sycamore component macros.
    pub type Template<G> = View<G>;

    /// Properties accepted by [`SycamoreCheckbox`].
    #[derive(Clone)]
    pub struct SycamoreCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine wiring ARIA metadata.
        pub state: CheckboxState,
        /// Optional change callback executed by client integrations.
        pub on_change: Option<Rc<dyn Fn(CheckboxChangeEvent)>>,
        /// Optional focus callback executed by client integrations.
        pub on_focus: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional blur callback executed by client integrations.
        pub on_blur: Option<Rc<dyn Fn(CheckboxFocusEvent)>>,
        /// Optional keyboard callback executed by client integrations.
        pub on_key: Option<Rc<dyn Fn(CheckboxKeyEvent)>>,
        /// Optional telemetry delegate invoked by automation shells.
        pub telemetry_delegate: Option<Rc<dyn Fn(CheckboxTelemetryEvent)>>,
    }

    /// Checkbox rendered within a Sycamore reactive scope.
    ///
    /// The adapter mirrors the analytics-first lifecycle of the other modules:
    /// descriptor telemetry is merged up-front, a [`TelemetryContext`] enriched
    /// with attribute metadata scopes the render span, telemetry delegates fire
    /// before consumer callbacks, and [`CheckboxState`] transitions occur via
    /// [`CheckboxState::toggle`], [`CheckboxState::focus`],
    /// [`CheckboxState::blur`], and [`CheckboxState::on_key`] once analytics
    /// capture completes **and** the state machine owns its value. Controlled
    /// flows still emit telemetry/change callbacks, yet the guard around
    /// [`CheckboxState::is_controlled`] keeps the local snapshot pristine until
    /// hosts reconcile external truth.
    #[component]
    pub fn SycamoreCheckbox<G: Html>(cx: Scope, props: SycamoreCheckboxProps) -> Template<G> {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::checkbox::sycamore::SycamoreCheckbox",
            &props.checkbox,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.checkbox.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_checked = snapshot.data_checked.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let data_indeterminate = snapshot.data_indeterminate.clone();
            let on_click = {
                let telemetry = props.telemetry_delegate.clone();
                let on_change = props.on_change.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&checkbox, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        if !state.is_controlled() {
                            state.toggle(|_| {});
                        }
                    }
                }
            };
            let on_focus = {
                let telemetry = props.telemetry_delegate.clone();
                let on_focus = props.on_focus.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Focus(focus.clone()));
                    }
                    if let Some(cb) = &on_focus {
                        cb(focus.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.focus();
                    }
                }
            };
            let on_blur = {
                let telemetry = props.telemetry_delegate.clone();
                let on_blur = props.on_blur.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&checkbox, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(CheckboxTelemetryEvent::Blur(blur.clone()));
                    }
                    if let Some(cb) = &on_blur {
                        cb(blur.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.blur();
                    }
                }
            };
            let on_key = {
                let telemetry = props.telemetry_delegate.clone();
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let checkbox = props.checkbox.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    let key = event.key();
                    if let Some(control) = control_key_from_str(&key) {
                        let (key_event, change) = {
                            let state = state.borrow();
                            (
                                build_key_event(&checkbox, &state, control),
                                build_change_event(&checkbox, &state),
                            )
                        };
                        if let Some(delegate) = &telemetry {
                            delegate(CheckboxTelemetryEvent::Key(key_event.clone()));
                            delegate(CheckboxTelemetryEvent::Change(change.clone()));
                        }
                        if let Some(cb) = &on_key {
                            cb(key_event.clone());
                        }
                        if let Some(change_cb) = &on_change {
                            change_cb(change.clone());
                        }
                        {
                            let mut state = state.borrow_mut();
                            if !state.is_controlled() {
                                state.on_key(control, |_| {});
                            }
                        }
                    }
                }
            };

            view! { cx,
                span(
                    class=class,
                    role=role,
                    aria_checked=aria_checked,
                    aria_disabled=aria_disabled,
                    tabindex=tabindex,
                    data_checked=data_checked,
                    data_focus_visible=data_focus_visible,
                    data_indeterminate=data_indeterminate,
                    on:click=on_click,
                    on:focus=on_focus,
                    on:blur=on_blur,
                    on:keydown=on_key,
                ) { (label) }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_attributes_include_role() {
        let state = CheckboxState::uncontrolled(false, true);
        let attrs = build_descriptor(&CheckboxProps::new("Accept"), &state);
        assert!(attrs
            .aria_attributes()
            .any(|(k, v)| k == "role" && v == "checkbox"));
    }

    #[test]
    fn render_html_includes_label() {
        let props = CheckboxProps::new("Accept");
        let state = CheckboxState::uncontrolled(false, false);
        let html = render_html(&props, &state);
        assert!(html.contains(">Accept<"));
        assert!(html.contains("aria-checked"));
    }

    #[test]
    fn telemetry_attributes_are_applied_when_provided() {
        let state = CheckboxState::uncontrolled(false, false);
        let mut props = CheckboxProps::new("Instrumented");
        props.telemetry.analytics_id = Some("analytics-42".into());
        props.telemetry.automation_id = Some("automation-42".into());

        let descriptor = build_descriptor(&props, &state);
        let data_attrs: Vec<_> = descriptor
            .data_state_attributes()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        assert!(data_attrs
            .iter()
            .any(|(key, value)| key == "data-rustic-analytics-id" && value == "analytics-42"));
        assert!(data_attrs
            .iter()
            .any(|(key, value)| key == "data-automation-id" && value == "automation-42"));
    }
}
