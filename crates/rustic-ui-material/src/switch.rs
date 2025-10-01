//! Material switch built from the headless [`SwitchState`].
//!
//! Feature-gated adapters expose idiomatic components per framework while
//! sharing styling, accessibility metadata, and telemetry ordering via
//! [`ToggleControlDescriptor`](crate::selection_control::ToggleControlDescriptor)
//! and the helper dispatchers defined below.
//!
//! * `react` – [`react::ReactSwitch`] returns [`Jsx`] using the `wasm_bindgen`
//!   bridge and the shared descriptor metadata.
//! * `yew` – [`yew::YewSwitch`] is decorated with `#[function_component]` for
//!   seamless use in Yew apps.
//! * `leptos` – [`leptos::LeptosSwitch`] leverages the Leptos `#[component]`
//!   macro and returns a [`leptos::View`].
//! * `dioxus` – [`dioxus::DioxusSwitch`] renders markup with `rsx!` so Dioxus
//!   shells gain first-class primitives instead of raw HTML strings.
//! * `sycamore` – [`sycamore::SycamoreSwitch`] yields a Sycamore
//!   [`Template`](sycamore::view::Template) for signal-driven experiences.
//!
//! All adapters derive their attributes from the same descriptor ensuring parity
//! between SSR and client renders regardless of framework. Telemetry hooks and
//! analytics callbacks are routed through [`instrument_render`],
//! [`TelemetryContext`], and the dispatch helpers so analytics capture always
//! precedes local side effects.

use crate::{
    selection_control::{self, ToggleControlDescriptor},
    telemetry::{instrument_render, TelemetryContext, TelemetryHooks},
};
use rustic_ui_headless::{interaction::ControlKey, switch::SwitchState};
use rustic_ui_styled_engine::{css_with_theme, Style};

#[cfg(feature = "react")]
use wasm_bindgen::JsValue;

/// Props shared across all framework adapters.
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchProps {
    /// Human friendly label rendered adjacent to the switch track.
    pub label: String,
    /// Telemetry hooks used to decorate render lifecycles with analytics and
    /// automation identifiers.
    pub telemetry: TelemetryHooks,
}

impl SwitchProps {
    /// Convenience constructor for tests and examples.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            telemetry: TelemetryHooks::default(),
        }
    }
}

/// Canonical payload emitted when switch state changes.
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchChangeEvent {
    /// Previously resolved on/off state prior to the interaction.
    pub previous: bool,
    /// Next logical state requested by the user interaction.
    pub next: bool,
    /// Whether the switch was disabled when the interaction was attempted.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the switch.
    pub label: String,
}

/// Canonical payload emitted when focus visibility changes.
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchFocusEvent {
    /// Whether focus was gained (`true`) or lost (`false`).
    pub focused: bool,
    /// Current on/off state at the time the focus transition occurred.
    pub on: bool,
    /// Whether the switch was disabled while the focus event fired.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the switch.
    pub label: String,
}

/// Canonical payload emitted for keyboard interactions.
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchKeyEvent {
    /// Normalised control key derived from the browser event.
    pub key: ControlKey,
    /// Previously resolved on/off state prior to the key interaction.
    pub previous: bool,
    /// Next logical state requested by the key press (if any).
    pub next: bool,
    /// Whether the switch was disabled when the key was pressed.
    pub disabled: bool,
    /// Identifier mirrored to analytics sinks (if configured).
    pub analytics_id: Option<String>,
    /// Identifier mirrored to automation sinks (if configured).
    pub automation_id: Option<String>,
    /// Human friendly label rendered alongside the switch.
    pub label: String,
}

/// Telemetry payload variants surfaced across adapters.
#[derive(Clone, Debug, PartialEq)]
pub enum SwitchTelemetryEvent {
    /// Change request triggered via pointer or keyboard interaction.
    Change(SwitchChangeEvent),
    /// Focus gained with the accompanying state metadata.
    Focus(SwitchFocusEvent),
    /// Focus lost with the accompanying state metadata.
    Blur(SwitchFocusEvent),
    /// Raw keyboard interaction payload.
    Key(SwitchKeyEvent),
}

#[allow(dead_code)]
fn analytics_id(props: &SwitchProps) -> Option<String> {
    props.telemetry.analytics_id.clone()
}

#[allow(dead_code)]
fn automation_id(props: &SwitchProps) -> Option<String> {
    props.telemetry.automation_id.clone()
}

#[allow(dead_code)]
fn build_change_event(props: &SwitchProps, state: &SwitchState) -> SwitchChangeEvent {
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
        analytics_id: analytics_id(props),
        automation_id: automation_id(props),
        label: props.label.clone(),
    }
}

fn build_focus_event(props: &SwitchProps, state: &SwitchState, focused: bool) -> SwitchFocusEvent {
    SwitchFocusEvent {
        focused,
        on: state.on(),
        disabled: state.disabled(),
        analytics_id: analytics_id(props),
        automation_id: automation_id(props),
        label: props.label.clone(),
    }
}

#[allow(dead_code)]
fn build_key_event(props: &SwitchProps, state: &SwitchState, key: ControlKey) -> SwitchKeyEvent {
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
fn build_descriptor(props: &SwitchProps, state: &SwitchState) -> ToggleControlDescriptor {
    let descriptor = ToggleControlDescriptor::new(props.label.clone(), themed_switch_style())
        .with_attributes(state.aria_attributes());
    merge_descriptor_telemetry(descriptor, &props.telemetry)
}

#[allow(dead_code)]
fn render_html(props: &SwitchProps, state: &SwitchState) -> String {
    let (context, descriptor, _snapshot) =
        descriptor_with_context("rustic_ui_material::switch::render_html", props, state);
    instrument_render(&props.telemetry, context, || {
        selection_control::render_toggle_html(&descriptor)
    })
}

/// Builds the switch track and thumb styling from the active theme tokens. By
/// leaning on `css_with_theme!` we avoid scattering literal values and keep the
/// component responsive to palette or spacing overrides.
fn themed_switch_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        cursor: pointer;
        font-family: ${font_family};
        color: ${text_color};
        position: relative;
        padding: ${padding_y} ${padding_x};

        &::before {
            content: "";
            width: ${track_width};
            height: ${track_height};
            background: ${track_off};
            border-radius: ${track_radius};
            transition: background-color 160ms ease;
            display: inline-block;
            margin-right: ${gap};
        }

        &::after {
            content: "";
            position: absolute;
            left: ${thumb_offset};
            top: 50%;
            transform: translateY(-50%);
            width: ${thumb_size};
            height: ${thumb_size};
            background: ${thumb_color};
            border-radius: 9999px;
            transition: transform 160ms ease;
        }

        &[data-on='true']::before {
            background: ${track_on};
        }

        &[data-on='true']::after {
            transform: translate(${thumb_translate}, -50%);
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
        font_family = theme.typography.font_family.clone(),
        text_color = theme.palette.text_primary.clone(),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        track_width = format!("{}px", theme.spacing(4)),
        track_height = format!("{}px", theme.spacing(1)),
        track_radius = format!("{}px", theme.spacing(1)),
        track_off = theme.palette.text_secondary.clone(),
        track_on = theme.palette.primary.clone(),
        thumb_size = format!("{}px", theme.spacing(2)),
        thumb_color = theme.palette.background_paper.clone(),
        thumb_offset = format!("{}px", theme.spacing(0)),
        thumb_translate = format!("{}px", theme.spacing(2)),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
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
struct SwitchDescriptorSnapshot {
    label: String,
    themed_attributes: Vec<(String, String)>,
    class: String,
    role: String,
    aria_checked: String,
    aria_disabled: Option<String>,
    tabindex: String,
    data_on: String,
    data_focus_visible: String,
}

impl SwitchDescriptorSnapshot {
    fn from_descriptor(descriptor: &ToggleControlDescriptor) -> Self {
        let themed_attributes = descriptor.themed_attributes();
        let mut class = String::new();
        let mut role = String::from("switch");
        let mut aria_checked = String::from("false");
        let mut aria_disabled = None;
        let mut tabindex = String::from("0");
        let mut data_on = String::from("false");
        let mut data_focus_visible = String::from("false");

        for (key, value) in &themed_attributes {
            match key.as_str() {
                "class" => class = value.clone(),
                "role" => role = value.clone(),
                "aria-checked" => aria_checked = value.clone(),
                "aria-disabled" => aria_disabled = Some(value.clone()),
                "tabindex" => tabindex = value.clone(),
                "data-on" => data_on = value.clone(),
                "data-focus-visible" => data_focus_visible = value.clone(),
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
            data_on,
            data_focus_visible,
        }
    }
}

fn descriptor_with_context(
    component: &'static str,
    props: &SwitchProps,
    state: &SwitchState,
) -> (
    TelemetryContext,
    ToggleControlDescriptor,
    SwitchDescriptorSnapshot,
) {
    let descriptor = merge_descriptor_telemetry(build_descriptor(props, state), &props.telemetry);
    let snapshot = SwitchDescriptorSnapshot::from_descriptor(&descriptor);
    let context = TelemetryContext::new(component)
        .with_analytics(props.telemetry.analytics_id.clone())
        .with_automation(props.telemetry.automation_id.clone())
        .with_descriptor_metadata(snapshot.label.clone(), snapshot.themed_attributes.clone());
    (context, descriptor, snapshot)
}

fn dispatch_change_event(
    props: &SwitchProps,
    state: &mut SwitchState,
    telemetry: Option<&mut dyn FnMut(SwitchTelemetryEvent)>,
    change_callback: Option<&mut dyn FnMut(SwitchChangeEvent)>,
) -> SwitchChangeEvent {
    let change = build_change_event(props, state);
    if let Some(callback) = telemetry {
        callback(SwitchTelemetryEvent::Change(change.clone()));
    }
    if let Some(callback) = change_callback {
        callback(change.clone());
    }
    state.toggle(|_| {});
    change
}

fn dispatch_focus_event(
    props: &SwitchProps,
    state: &mut SwitchState,
    focused: bool,
    telemetry: Option<&mut dyn FnMut(SwitchTelemetryEvent)>,
    focus_callback: Option<&mut dyn FnMut(SwitchFocusEvent)>,
) -> SwitchFocusEvent {
    let focus = build_focus_event(props, state, focused);
    if let Some(callback) = telemetry {
        let event = if focused {
            SwitchTelemetryEvent::Focus(focus.clone())
        } else {
            SwitchTelemetryEvent::Blur(focus.clone())
        };
        callback(event);
    }
    if let Some(callback) = focus_callback {
        callback(focus.clone());
    }
    if focused {
        state.focus();
    } else {
        state.blur();
    }
    focus
}

fn dispatch_key_event(
    props: &SwitchProps,
    state: &mut SwitchState,
    key: ControlKey,
    telemetry: Option<&mut dyn FnMut(SwitchTelemetryEvent)>,
    key_callback: Option<&mut dyn FnMut(SwitchKeyEvent)>,
    change_callback: Option<&mut dyn FnMut(SwitchChangeEvent)>,
) -> (SwitchKeyEvent, SwitchChangeEvent) {
    let key_event = build_key_event(props, state, key);
    let change_event = build_change_event(props, state);

    if let Some(callback) = telemetry {
        callback(SwitchTelemetryEvent::Key(key_event.clone()));
        callback(SwitchTelemetryEvent::Change(change_event.clone()));
    }
    if let Some(callback) = key_callback {
        callback(key_event.clone());
    }
    if let Some(callback) = change_callback {
        callback(change_event.clone());
    }
    state.on_key(key, |_| {});
    (key_event, change_event)
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
fn telemetry_event_to_js(event: SwitchTelemetryEvent) -> JsValue {
    use js_sys::Reflect;
    use SwitchTelemetryEvent as Event;

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
                &JsValue::from_bool(change.previous),
            )
            .expect("set previous");
            Reflect::set(
                &object,
                &JsValue::from_str("next"),
                &JsValue::from_bool(change.next),
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
                &JsValue::from_str("on"),
                &JsValue::from_bool(focus.on),
            )
            .expect("set on");
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
                &JsValue::from_str("on"),
                &JsValue::from_bool(focus.on),
            )
            .expect("set on");
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
                &JsValue::from_bool(key.previous),
            )
            .expect("set previous");
            Reflect::set(
                &object,
                &JsValue::from_str("next"),
                &JsValue::from_bool(key.next),
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
#[cfg(feature = "react")]
pub mod react {
    //! React adapter returning `Jsx` nodes via the WASM bridge.
    use super::*;
    use js_sys::{Array, Function, Object, Reflect};
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen::{closure::Closure, JsCast};

    /// Type alias representing React nodes produced by the adapter.
    pub type Jsx = JsValue;

    /// Properties consumed by the React switch component.
    #[derive(Clone, Debug)]
    pub struct ReactSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state driving ARIA metadata.
        pub state: SwitchState,
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

    impl PartialEq for ReactSwitchProps {
        fn eq(&self, other: &Self) -> bool {
            self.switch == other.switch
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
            .expect("React global missing; ensure the runtime registers React");
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
        props: &ReactSwitchProps,
        state_handle: &Rc<RefCell<SwitchState>>,
    ) -> Option<Function> {
        if props.on_change.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_change = props.on_change.clone();
        let switch_props = props.switch.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let change = {
                let state = state.borrow();
                build_change_event(&switch_props, &state)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(SwitchTelemetryEvent::Change(change.clone()));
                let _ = delegate.call1(&JsValue::NULL, &payload);
            }
            if let Some(handler) = on_change.as_ref() {
                let _ = handler.call1(&JsValue::NULL, &event);
            }
            {
                let mut state = state.borrow_mut();
                state.toggle(|_| {});
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn focus_handler(
        props: &ReactSwitchProps,
        state_handle: &Rc<RefCell<SwitchState>>,
    ) -> Option<Function> {
        if props.on_focus.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_focus = props.on_focus.clone();
        let switch_props = props.switch.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let focus = {
                let state = state.borrow();
                build_focus_event(&switch_props, &state, true)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(SwitchTelemetryEvent::Focus(focus.clone()));
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
        props: &ReactSwitchProps,
        state_handle: &Rc<RefCell<SwitchState>>,
    ) -> Option<Function> {
        if props.on_blur.is_none() && props.telemetry_delegate.is_none() {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_blur = props.on_blur.clone();
        let switch_props = props.switch.clone();
        let state = Rc::clone(state_handle);

        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            let blur = {
                let state = state.borrow();
                build_focus_event(&switch_props, &state, false)
            };
            if let Some(delegate) = telemetry.as_ref() {
                let payload = telemetry_event_to_js(SwitchTelemetryEvent::Blur(blur.clone()));
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
        props: &ReactSwitchProps,
        state_handle: &Rc<RefCell<SwitchState>>,
    ) -> Option<Function> {
        if props.on_key.is_none() && props.on_change.is_none() && props.telemetry_delegate.is_none()
        {
            return None;
        }

        let telemetry = props.telemetry_delegate.clone();
        let on_key = props.on_key.clone();
        let on_change = props.on_change.clone();
        let switch_props = props.switch.clone();
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
                        build_key_event(&switch_props, &state, key),
                        build_change_event(&switch_props, &state),
                    )
                };

                if let Some(delegate) = telemetry.as_ref() {
                    let key_payload =
                        telemetry_event_to_js(SwitchTelemetryEvent::Key(key_event.clone()));
                    let _ = delegate.call1(&JsValue::NULL, &key_payload);
                    let change_payload =
                        telemetry_event_to_js(SwitchTelemetryEvent::Change(change_event.clone()));
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
                    state.on_key(key, |_| {});
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        let function: Function = closure.as_ref().clone().unchecked_into();
        closure.forget();
        Some(function)
    }

    fn handlers(props: &ReactSwitchProps, state_handle: Rc<RefCell<SwitchState>>) -> ReactHandlers {
        ReactHandlers {
            on_change: change_handler(props, &state_handle),
            on_focus: focus_handler(props, &state_handle),
            on_blur: blur_handler(props, &state_handle),
            on_key: key_handler(props, &state_handle),
        }
    }

    /// React component rendering the Material switch.
    ///
    /// * A [`TelemetryContext`] seeded with the fully-qualified component path
    ///   and decorated with descriptor metadata is constructed so downstream
    ///   spans, analytics sinks, and error hooks can attribute metrics and
    ///   attribute snapshots back to this adapter.
    /// * [`instrument_render`] enters the context span, ensures success/error
    ///   hooks run, and propagates analytics/automation identifiers extracted
    ///   from [`SwitchProps::telemetry`].
    /// * Prior to hydration, telemetry defaults are merged into the descriptor
    ///   attributes so SSR and CSR renders emit identical `data-*` markers even
    ///   when the caller omits explicit identifiers.
    /// * Event handlers wrap consumer callbacks, delivering normalized
    ///   [`SwitchTelemetryEvent`] payloads to the optional telemetry delegate
    ///   **before** invoking user logic. This guarantees analytics capture
    ///   precedes side effects, aligning with audit requirements in regulated
    ///   environments.
    /// * After telemetry delegates and consumer callbacks run, the captured
    ///   [`SwitchState`] is mutated via [`SwitchState::toggle`],
    ///   [`SwitchState::focus`], [`SwitchState::blur`], and
    ///   [`SwitchState::on_key`] so UI transitions always flow through the
    ///   shared headless state machine.
    pub fn ReactSwitch(props: &ReactSwitchProps) -> Jsx {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::switch::react::ReactSwitch",
            &props.switch,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.switch.telemetry, context, || {
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
    //! Yew adapter leveraging `#[function_component]` for idiomatic usage.
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    use yew::events::{FocusEvent, KeyboardEvent, MouseEvent};
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

    /// Properties accepted by [`YewSwitch`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing accessibility metadata.
        pub state: SwitchState,
        /// Optional change callback invoked with [`SwitchChangeEvent`].
        #[prop_or_default]
        pub on_change: Option<Callback<SwitchChangeEvent>>,
        /// Optional focus callback invoked when the switch gains focus.
        #[prop_or_default]
        pub on_focus: Option<Callback<SwitchFocusEvent>>,
        /// Optional blur callback invoked when the switch loses focus.
        #[prop_or_default]
        pub on_blur: Option<Callback<SwitchFocusEvent>>,
        /// Optional keyboard callback invoked with normalized control keys.
        #[prop_or_default]
        pub on_key: Option<Callback<SwitchKeyEvent>>,
        /// Optional telemetry delegate invoked with structured payloads.
        #[prop_or_default]
        pub telemetry_delegate: Option<Callback<SwitchTelemetryEvent>>,
    }

    /// Switch rendered inside Yew applications.
    #[function_component(YewSwitch)]
    pub fn yew_switch(props: &YewSwitchProps) -> Html {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::switch::yew::YewSwitch",
            &props.switch,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.switch.telemetry, context, || {
            let label = snapshot.label.clone();
            let attrs = snapshot.themed_attributes.clone();
            let change_handler = {
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: MouseEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate.emit(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut change_runner = on_change.clone().map(|callback| {
                        Box::new(move |event: SwitchChangeEvent| callback.emit(event))
                            as Box<dyn FnMut(SwitchChangeEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let change_ref = change_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchChangeEvent));
                    let mut state = state.borrow_mut();
                    dispatch_change_event(&switch_props, &mut state, telemetry_ref, change_ref);
                })
            };
            let focus_handler = {
                let on_focus = props.on_focus.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: FocusEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate.emit(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut focus_runner = on_focus.clone().map(|callback| {
                        Box::new(move |event: SwitchFocusEvent| callback.emit(event))
                            as Box<dyn FnMut(SwitchFocusEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let focus_ref = focus_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchFocusEvent));
                    let mut state = state.borrow_mut();
                    dispatch_focus_event(&switch_props, &mut state, true, telemetry_ref, focus_ref);
                })
            };
            let blur_handler = {
                let on_blur = props.on_blur.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |_event: FocusEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate.emit(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut blur_runner = on_blur.clone().map(|callback| {
                        Box::new(move |event: SwitchFocusEvent| callback.emit(event))
                            as Box<dyn FnMut(SwitchFocusEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let blur_ref = blur_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchFocusEvent));
                    let mut state = state.borrow_mut();
                    dispatch_focus_event(&switch_props, &mut state, false, telemetry_ref, blur_ref);
                })
            };
            let key_handler = {
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                Callback::from(move |event: KeyboardEvent| {
                    if let Some(control) = control_key_from_str(event.key().as_str()) {
                        event.prevent_default();
                        let mut telemetry_runner = telemetry.clone().map(|delegate| {
                            Box::new(move |event: SwitchTelemetryEvent| delegate.emit(event))
                                as Box<dyn FnMut(SwitchTelemetryEvent)>
                        });
                        let mut key_runner = on_key.clone().map(|callback| {
                            Box::new(move |event: SwitchKeyEvent| callback.emit(event))
                                as Box<dyn FnMut(SwitchKeyEvent)>
                        });
                        let mut change_runner = on_change.clone().map(|callback| {
                            Box::new(move |event: SwitchChangeEvent| callback.emit(event))
                                as Box<dyn FnMut(SwitchChangeEvent)>
                        });
                        let telemetry_ref = telemetry_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                        let key_ref = key_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchKeyEvent));
                        let change_ref = change_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchChangeEvent));
                        let mut state = state.borrow_mut();
                        dispatch_key_event(
                            &switch_props,
                            &mut state,
                            control,
                            telemetry_ref,
                            key_ref,
                            change_ref,
                        );
                    }
                })
            };

            let mut node = html! { <span>{label}</span> };
            if let VNode::VTag(ref mut tag) = node {
                for (key, value) in attrs {
                    tag.add_attribute(key, value);
                }
                tag.add_listener(change_handler);
                tag.add_listener(focus_handler);
                tag.add_listener(blur_handler);
                tag.add_listener(key_handler);
            }
            node
        })
    }
}
#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter returning a [`leptos::View`] that hydrates cleanly across
    //! server and client renders.
    use super::*;
    use leptos::ev::{FocusEvent, KeyboardEvent, MouseEvent};
    use leptos::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    /// Properties accepted by [`LeptosSwitch`].
    #[derive(Clone)]
    pub struct LeptosSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
        /// Optional change callback emitted when toggles occur.
        pub on_change: Option<Rc<dyn Fn(SwitchChangeEvent)>>,
        /// Optional focus callback emitted when focus is gained.
        pub on_focus: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional blur callback emitted when focus is lost.
        pub on_blur: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional key callback emitted with normalized control keys.
        pub on_key: Option<Rc<dyn Fn(SwitchKeyEvent)>>,
        /// Optional telemetry delegate invoked with structured payloads.
        pub telemetry_delegate: Option<Rc<dyn Fn(SwitchTelemetryEvent)>>,
    }

    #[component]
    pub fn LeptosSwitch(props: LeptosSwitchProps) -> impl IntoView {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::switch::leptos::LeptosSwitch",
            &props.switch,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.switch.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_on = snapshot.data_on.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let on_click = {
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: MouseEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut change_runner = on_change.clone().map(|callback| {
                        Box::new(move |event: SwitchChangeEvent| callback(event))
                            as Box<dyn FnMut(SwitchChangeEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let change_ref = change_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchChangeEvent));
                    let mut state = state.borrow_mut();
                    dispatch_change_event(&switch_props, &mut state, telemetry_ref, change_ref);
                }
            };
            let on_focus = {
                let on_focus = props.on_focus.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut focus_runner = on_focus.clone().map(|callback| {
                        Box::new(move |event: SwitchFocusEvent| callback(event))
                            as Box<dyn FnMut(SwitchFocusEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let focus_ref = focus_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchFocusEvent));
                    let mut state = state.borrow_mut();
                    dispatch_focus_event(&switch_props, &mut state, true, telemetry_ref, focus_ref);
                }
            };
            let on_blur = {
                let on_blur = props.on_blur.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let mut telemetry_runner = telemetry.clone().map(|delegate| {
                        Box::new(move |event: SwitchTelemetryEvent| delegate(event))
                            as Box<dyn FnMut(SwitchTelemetryEvent)>
                    });
                    let mut blur_runner = on_blur.clone().map(|callback| {
                        Box::new(move |event: SwitchFocusEvent| callback(event))
                            as Box<dyn FnMut(SwitchFocusEvent)>
                    });
                    let telemetry_ref = telemetry_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                    let blur_ref = blur_runner
                        .as_mut()
                        .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchFocusEvent));
                    let mut state = state.borrow_mut();
                    dispatch_focus_event(&switch_props, &mut state, false, telemetry_ref, blur_ref);
                }
            };
            let on_key_down = {
                let on_key = props.on_key.clone();
                let on_change = props.on_change.clone();
                let telemetry = props.telemetry_delegate.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    if let Some(control) = control_key_from_str(event.key().as_str()) {
                        event.prevent_default();
                        let mut telemetry_runner = telemetry.clone().map(|delegate| {
                            Box::new(move |event: SwitchTelemetryEvent| delegate(event))
                                as Box<dyn FnMut(SwitchTelemetryEvent)>
                        });
                        let mut key_runner = on_key.clone().map(|callback| {
                            Box::new(move |event: SwitchKeyEvent| callback(event))
                                as Box<dyn FnMut(SwitchKeyEvent)>
                        });
                        let mut change_runner = on_change.clone().map(|callback| {
                            Box::new(move |event: SwitchChangeEvent| callback(event))
                                as Box<dyn FnMut(SwitchChangeEvent)>
                        });
                        let telemetry_ref = telemetry_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchTelemetryEvent));
                        let key_ref = key_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchKeyEvent));
                        let change_ref = change_runner
                            .as_mut()
                            .map(|runner| runner.as_mut() as &mut dyn FnMut(SwitchChangeEvent));
                        let mut state = state.borrow_mut();
                        dispatch_key_event(
                            &switch_props,
                            &mut state,
                            control,
                            telemetry_ref,
                            key_ref,
                            change_ref,
                        );
                    }
                }
            };

            leptos::view! {
                leptos::html::span()
                    .class(class)
                    .attr("role", role)
                    .attr("aria-checked", aria_checked)
                    .attr("tabindex", tabindex)
                    .attr("data-on", data_on)
                    .attr("data-focus-visible", data_focus_visible)
                    .attr_optional("aria-disabled", aria_disabled)
                    .on(ev::click, on_click)
                    .on(ev::focus, on_focus)
                    .on(ev::blur, on_blur)
                    .on(ev::keydown, on_key_down)
                    .child(label)
            }
        })
    }
}
#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter built with `rsx!` for idiomatic usage inside Dioxus
    //! applications.
    use super::*;
    use dioxus::events::{FocusEvent, KeyboardEvent, MouseEvent};
    use dioxus::prelude::*;
    use std::{cell::RefCell, rc::Rc};

    /// Properties accepted by [`DioxusSwitch`].
    #[derive(Props, Clone, PartialEq)]
    pub struct DioxusSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
        /// Optional change callback emitted when toggles occur.
        #[props(default = None)]
        pub on_change: Option<Rc<dyn Fn(SwitchChangeEvent)>>,
        /// Optional focus callback emitted when focus is gained.
        #[props(default = None)]
        pub on_focus: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional blur callback emitted when focus is lost.
        #[props(default = None)]
        pub on_blur: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional key callback emitted with normalized control keys.
        #[props(default = None)]
        pub on_key: Option<Rc<dyn Fn(SwitchKeyEvent)>>,
        /// Optional telemetry delegate invoked with structured payloads.
        #[props(default = None)]
        pub telemetry_delegate: Option<Rc<dyn Fn(SwitchTelemetryEvent)>>,
    }

    /// Switch rendered as a Dioxus component.
    pub fn DioxusSwitch(cx: Scope<DioxusSwitchProps>) -> Element {
        let props = cx.props();
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::switch::dioxus::DioxusSwitch",
            &props.switch,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.switch.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_on = snapshot.data_on.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let onclick = {
                let telemetry = props.telemetry_delegate.clone();
                let on_change = props.on_change.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: MouseEvent| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&switch_props, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.toggle(|_| {});
                    }
                }
            };
            let on_focus = {
                let telemetry = props.telemetry_delegate.clone();
                let on_focus = props.on_focus.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&switch_props, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Focus(focus.clone()));
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
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_event: FocusEvent| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&switch_props, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Blur(blur.clone()));
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
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    let key = match event.data.key() {
                        Key::Space | Key::Character(ref ch) if ch == " " => Some(" ".to_string()),
                        Key::Enter => Some("Enter".to_string()),
                        _ => None,
                    };
                    if let Some(name) = key.as_deref() {
                        if let Some(control) = control_key_from_str(name) {
                            let (key_event, change) = {
                                let state = state.borrow();
                                (
                                    build_key_event(&switch_props, &state, control),
                                    build_change_event(&switch_props, &state),
                                )
                            };
                            if let Some(delegate) = &telemetry {
                                delegate(SwitchTelemetryEvent::Key(key_event.clone()));
                                delegate(SwitchTelemetryEvent::Change(change.clone()));
                            }
                            if let Some(cb) = &on_key {
                                cb(key_event.clone());
                            }
                            if let Some(change_cb) = &on_change {
                                change_cb(change.clone());
                            }
                            {
                                let mut state = state.borrow_mut();
                                state.on_key(control, |_| {});
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
                    data_on: data_on,
                    data_focus_visible: data_focus_visible,
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
    //! Sycamore adapter yielding a [`Template`] for reactive dashboards.
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    use sycamore::prelude::*;
    use sycamore::web::html::event::KeyboardEvent;

    /// Alias mirroring Sycamore's view representation.
    pub type Template<G> = View<G>;

    /// Properties accepted by [`SycamoreSwitch`].
    #[derive(Clone)]
    pub struct SycamoreSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
        /// Optional change callback emitted when toggles occur.
        pub on_change: Option<Rc<dyn Fn(SwitchChangeEvent)>>,
        /// Optional focus callback emitted when focus is gained.
        pub on_focus: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional blur callback emitted when focus is lost.
        pub on_blur: Option<Rc<dyn Fn(SwitchFocusEvent)>>,
        /// Optional key callback emitted with normalized control keys.
        pub on_key: Option<Rc<dyn Fn(SwitchKeyEvent)>>,
        /// Optional telemetry delegate invoked with structured payloads.
        pub telemetry_delegate: Option<Rc<dyn Fn(SwitchTelemetryEvent)>>,
    }

    /// Switch rendered within a Sycamore reactive scope.
    #[component]
    pub fn SycamoreSwitch<G: Html>(cx: Scope, props: SycamoreSwitchProps) -> Template<G> {
        let (context, _descriptor, snapshot) = descriptor_with_context(
            "rustic_ui_material::switch::sycamore::SycamoreSwitch",
            &props.switch,
            &props.state,
        );
        let state_handle = Rc::new(RefCell::new(props.state.clone()));
        instrument_render(&props.switch.telemetry, context, || {
            let label = snapshot.label.clone();
            let class = snapshot.class.clone();
            let role = snapshot.role.clone();
            let aria_checked = snapshot.aria_checked.clone();
            let aria_disabled = snapshot.aria_disabled.clone();
            let tabindex = snapshot.tabindex.clone();
            let data_on = snapshot.data_on.clone();
            let data_focus_visible = snapshot.data_focus_visible.clone();
            let on_click = {
                let telemetry = props.telemetry_delegate.clone();
                let on_change = props.on_change.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let change = {
                        let state = state.borrow();
                        build_change_event(&switch_props, &state)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Change(change.clone()));
                    }
                    if let Some(cb) = &on_change {
                        cb(change.clone());
                    }
                    {
                        let mut state = state.borrow_mut();
                        state.toggle(|_| {});
                    }
                }
            };
            let on_focus = {
                let telemetry = props.telemetry_delegate.clone();
                let on_focus = props.on_focus.clone();
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let focus = {
                        let state = state.borrow();
                        build_focus_event(&switch_props, &state, true)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Focus(focus.clone()));
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
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |_| {
                    let blur = {
                        let state = state.borrow();
                        build_focus_event(&switch_props, &state, false)
                    };
                    if let Some(delegate) = &telemetry {
                        delegate(SwitchTelemetryEvent::Blur(blur.clone()));
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
                let switch_props = props.switch.clone();
                let state = Rc::clone(&state_handle);
                move |event: KeyboardEvent| {
                    let key = event.key();
                    if let Some(control) = control_key_from_str(&key) {
                        let (key_event, change) = {
                            let state = state.borrow();
                            (
                                build_key_event(&switch_props, &state, control),
                                build_change_event(&switch_props, &state),
                            )
                        };
                        if let Some(delegate) = &telemetry {
                            delegate(SwitchTelemetryEvent::Key(key_event.clone()));
                            delegate(SwitchTelemetryEvent::Change(change.clone()));
                        }
                        if let Some(cb) = &on_key {
                            cb(key_event.clone());
                        }
                        if let Some(change_cb) = &on_change {
                            change_cb(change.clone());
                        }
                        {
                            let mut state = state.borrow_mut();
                            state.on_key(control, |_| {});
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
                    data_on=data_on,
                    data_focus_visible=data_focus_visible,
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
    fn dispatch_change_updates_state_and_telemetry() {
        let mut props = SwitchProps::new("Notifications");
        props.telemetry.analytics_id = Some("switch.analytics".into());
        let mut state = SwitchState::uncontrolled(false, false);
        let mut telemetry_events = Vec::new();
        let mut change_events = Vec::new();
        {
            let mut telemetry = |event: SwitchTelemetryEvent| telemetry_events.push(event);
            let mut change = |event: SwitchChangeEvent| change_events.push(event);
            dispatch_change_event(&props, &mut state, Some(&mut telemetry), Some(&mut change));
        }
        assert!(state.on());
        assert_eq!(telemetry_events.len(), 1);
        match &telemetry_events[0] {
            SwitchTelemetryEvent::Change(change) => {
                assert!(!change.previous);
                assert!(change.next);
                assert_eq!(change.analytics_id.as_deref(), Some("switch.analytics"));
            }
            other => panic!("unexpected telemetry event: {other:?}"),
        }
        assert_eq!(change_events.len(), 1);
        assert!(change_events[0].next);
    }

    #[test]
    fn dispatch_focus_tracks_focus_visibility_and_telemetry() {
        let props = SwitchProps::new("Notifications");
        let mut state = SwitchState::uncontrolled(false, false);
        let mut telemetry_events = Vec::new();
        let mut focus_events = Vec::new();
        {
            let mut telemetry = |event: SwitchTelemetryEvent| telemetry_events.push(event);
            let mut focus = |event: SwitchFocusEvent| focus_events.push(event);
            dispatch_focus_event(
                &props,
                &mut state,
                true,
                Some(&mut telemetry),
                Some(&mut focus),
            );
        }
        assert!(state.focus_visible());
        assert_eq!(telemetry_events.len(), 1);
        matches!(telemetry_events[0], SwitchTelemetryEvent::Focus(_));
        assert_eq!(focus_events.len(), 1);
        assert!(focus_events[0].focused);
    }

    #[test]
    fn dispatch_key_emits_key_and_change_telemetry() {
        let mut props = SwitchProps::new("Notifications");
        let mut state = SwitchState::uncontrolled(false, false);
        let mut telemetry_events = Vec::new();
        let mut key_events = Vec::new();
        let mut change_events = Vec::new();
        {
            let mut telemetry = |event: SwitchTelemetryEvent| telemetry_events.push(event);
            let mut key = |event: SwitchKeyEvent| key_events.push(event);
            let mut change = |event: SwitchChangeEvent| change_events.push(event);
            dispatch_key_event(
                &props,
                &mut state,
                ControlKey::Space,
                Some(&mut telemetry),
                Some(&mut key),
                Some(&mut change),
            );
        }
        assert!(state.on());
        assert_eq!(telemetry_events.len(), 2);
        assert!(matches!(telemetry_events[0], SwitchTelemetryEvent::Key(_)));
        assert!(matches!(
            telemetry_events[1],
            SwitchTelemetryEvent::Change(_)
        ));
        assert_eq!(key_events.len(), 1);
        assert_eq!(change_events.len(), 1);
    }

    #[test]
    fn render_html_contains_label_and_data_state() {
        let props = SwitchProps::new("Notifications");
        let state = SwitchState::uncontrolled(false, false);
        let html = render_html(&props, &state);
        assert!(html.contains(">Notifications<"));
        assert!(html.contains("data-on"));
    }
}
