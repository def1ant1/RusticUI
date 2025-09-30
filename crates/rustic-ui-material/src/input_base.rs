//! Material renderer for [`InputState`](rustic_ui_headless::input_base::InputState).
//!
//! The module intentionally mirrors the ergonomics of [`text_field`](crate::text_field)
//! so React, Yew, Leptos, Dioxus and Sycamore adapters can reuse the same headless
//! [`InputState`] across controlled and uncontrolled scenarios.  Every helper is
//! heavily documented to help enterprise integrators understand how analytics and
//! accessibility metadata is surfaced through deterministic `data-*` attributes.
//! The comments double as living runbooks for downstream platform teams so new
//! form controls inherit the automation hooks without duplicating boilerplate.
//!
//! ## Automation and observability
//! * All automation selectors are generated via [`style_helpers::automation_data_attr`]
//!   to guarantee stable prefixes (`data-rustic-*`).  Playwright/Cypress suites can
//!   glob on the prefix without worrying about future renames.
//! * Headless analytics emitted by [`InputState`] (dirty/visited/focused/errors and
//!   selection snapshots) are mirrored into `data-*` attributes.  React SSR, WASM
//!   clients and server rendered frameworks all render the same metadata which keeps
//!   golden snapshots and runtime instrumentation perfectly aligned.
//! * The shared HTML builders use [`style_helpers::themed_attributes_html`]
//!   so server side renders remain hydration-safe.  Hydration friendly markup is
//!   vital for large organisations who diff server/client output during release
//!   certification.
//!
//! ## Minimising repetitive work
//! Every adapter defers to the helpers below to assemble HTML/attributes.  This keeps
//! controlled/uncontrolled wiring, analytics mirrors and theme integration in one
//! place.  New frameworks or custom wrappers can lean on the same helpers which
//! dramatically reduces manual, error-prone duplication.

use rustic_ui_headless::input_base::{
    InputChangeEvent, InputCommitEvent, InputResetEvent, InputSelection, InputState,
};
use rustic_ui_styled_engine::{css_with_theme, use_theme, Style, Theme};

use crate::style_helpers;

pub use crate::macros::{
    Color as InputBaseColor, Size as InputBaseSize, Variant as InputBaseVariant,
};

const COMPONENT_NAME: &str = "input-base";

/// Convenience helper returning the canonical "true"/"false" strings.
#[must_use]
fn bool_token(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

#[cfg(any(
    feature = "react",
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn compute_parts(
    theme: &Theme,
    color: InputBaseColor,
    size: InputBaseSize,
    variant: InputBaseVariant,
) -> (String, &'static str, String) {
    let color = match color {
        InputBaseColor::Primary => theme.palette.primary.clone(),
        InputBaseColor::Secondary => theme.palette.secondary.clone(),
    };
    let font_size = match size {
        InputBaseSize::Small => "0.8rem",
        InputBaseSize::Medium => "1rem",
        InputBaseSize::Large => "1.2rem",
    };
    let border = match variant {
        InputBaseVariant::Outlined => format!("1px solid {}", color.clone()),
        InputBaseVariant::Contained => format!("1px solid {}", color.clone()),
        InputBaseVariant::Text => "none".to_string(),
    };
    (color, font_size, border)
}

#[cfg(any(
    feature = "react",
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn resolve_style(
    color: InputBaseColor,
    size: InputBaseSize,
    variant: InputBaseVariant,
    style_overrides: Option<String>,
) -> Style {
    let theme = use_theme();
    let (color, font_size, border) = compute_parts(&theme, color, size, variant);
    let extra = style_overrides.unwrap_or_default();
    css_with_theme!(
        theme,
        r#"
        color: ${color};
        font-size: ${font_size};
        border: ${border};
        padding: 4px 8px;
        width: 100%;
        box-sizing: border-box;
        ${extra}
        "#,
        color = color,
        font_size = font_size,
        border = border,
        extra = extra
    )
}

/// Construct the automation/data attributes that mirror headless analytics state.
#[must_use]
fn analytics_attributes(state: &InputState, analytics_id: Option<&str>) -> Vec<(String, String)> {
    let mut attrs = Vec::new();

    // Stable automation markers used across SSR and WASM clients.
    attrs.push((
        "data-component".into(),
        style_helpers::component_marker(COMPONENT_NAME),
    ));

    let dirty_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["dirty"]);
    attrs.push((dirty_attr, bool_token(state.dirty())));

    let visited_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["visited"]);
    attrs.push((visited_attr, bool_token(state.visited())));

    let focus_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["focused"]);
    attrs.push((focus_attr, bool_token(state.focused())));

    let error_count_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["error-count"]);
    attrs.push((error_count_attr, state.errors().len().to_string()));

    if let Some(selection) = state.selection() {
        let start_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["selection-start"]);
        let end_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["selection-end"]);
        attrs.push((start_attr, selection.start.to_string()));
        attrs.push((end_attr, selection.end.to_string()));
    }

    if !state.errors().is_empty() {
        let status_attr = style_helpers::automation_data_attr(COMPONENT_NAME, ["status-message"]);
        attrs.push((status_attr.clone(), state.errors().join("\n")));
        // Mirror under the generic `data-status-message` hook for legacy parity.
        attrs.push(("data-status-message".into(), state.errors().join("\n")));
    }

    if let Some(id) = analytics_id {
        attrs.push(("data-analytics-id".into(), id.to_string()));
        let automation_attr =
            style_helpers::automation_data_attr(COMPONENT_NAME, ["analytics", "id"]);
        attrs.push((automation_attr, id.to_string()));
    }

    attrs
}

/// Build the attribute collection consumed by the various adapters.
#[must_use]
fn input_attribute_pairs(
    state: &InputState,
    status_id: Option<&str>,
    analytics_id: Option<&str>,
    input_type: &str,
    placeholder: &str,
    aria_label: &str,
) -> Vec<(String, String)> {
    let mut pairs = analytics_attributes(state, analytics_id);

    pairs.push(("type".into(), input_type.to_string()));
    pairs.push(("value".into(), state.value().to_string()));
    pairs.push(("placeholder".into(), placeholder.to_string()));
    pairs.push(("aria-label".into(), aria_label.to_string()));

    // Backwards compatible flags used by existing QA suites.
    pairs.push(("data-dirty".into(), bool_token(state.dirty())));
    pairs.push(("data-visited".into(), bool_token(state.visited())));
    pairs.push(("data-focused".into(), bool_token(state.focused())));

    if !state.errors().is_empty() {
        pairs.push(("aria-invalid".into(), "true".into()));
    }

    if let Some(id) = status_id {
        pairs.push(("aria-describedby".into(), id.to_string()));
    }

    if let Some(selection) = state.selection() {
        pairs.push(("data-selection-start".into(), selection.start.to_string()));
        pairs.push(("data-selection-end".into(), selection.end.to_string()));
    }

    pairs
}

/// Assemble SSR friendly attributes used by non-DOM frameworks.
#[must_use]
fn ssr_input_attributes(
    state: &InputState,
    status_id: Option<&str>,
    analytics_id: Option<&str>,
    input_type: &str,
    placeholder: &str,
    aria_label: &str,
) -> Vec<(String, String)> {
    input_attribute_pairs(
        state,
        status_id,
        analytics_id,
        input_type,
        placeholder,
        aria_label,
    )
}

/// Shared configuration driving HTML renderers.
#[derive(Clone)]
pub struct InputBaseRenderConfig<'a> {
    /// Headless state powering the input element.
    pub state: &'a InputState,
    /// Input placeholder text.
    pub placeholder: &'a str,
    /// ARIA label describing the control.
    pub aria_label: &'a str,
    /// Input `type` attribute (text, email, number, etc.).
    pub input_type: &'a str,
    /// Optional validation status element identifier.
    pub status_id: Option<&'a str>,
    /// Optional analytics identifier mirrored to the DOM.
    pub analytics_id: Option<&'a str>,
    /// Visual color scheme.
    pub color: InputBaseColor,
    /// Stylistic variant.
    pub variant: InputBaseVariant,
    /// Component sizing token.
    pub size: InputBaseSize,
    /// Additional CSS declarations appended to the themed style.
    pub style_overrides: Option<&'a str>,
}

impl<'a> InputBaseRenderConfig<'a> {
    /// Convenience constructor with sensible defaults.
    pub fn new(state: &'a InputState) -> Self {
        Self {
            state,
            placeholder: "",
            aria_label: "",
            input_type: "text",
            status_id: None,
            analytics_id: None,
            color: InputBaseColor::Primary,
            variant: InputBaseVariant::Text,
            size: InputBaseSize::Medium,
            style_overrides: None,
        }
    }
}

/// Render output mirroring other Material primitives.
#[derive(Clone, PartialEq, Eq)]
pub struct InputBaseRenderOutput {
    /// Serialized `<input>` markup suitable for SSR and hydration.
    pub html: String,
}

/// Render the input base into HTML so SSR frameworks can hydrate safely later.
#[must_use]
pub fn render_input_base_html(config: &InputBaseRenderConfig<'_>) -> String {
    let style = resolve_style(
        config.color.clone(),
        config.size.clone(),
        config.variant.clone(),
        config.style_overrides.map(|s| s.to_string()),
    );
    let attrs = ssr_input_attributes(
        config.state,
        config.status_id,
        config.analytics_id,
        config.input_type,
        config.placeholder,
        config.aria_label,
    );
    let attr_string = crate::style_helpers::themed_attributes_html(style, attrs);
    format!("<input {attrs} />", attrs = attr_string)
}

/// Helper returning both the serialized HTML and attribute map for SSR harnesses.
#[must_use]
pub fn render_input_base(config: &InputBaseRenderConfig<'_>) -> InputBaseRenderOutput {
    InputBaseRenderOutput {
        html: render_input_base_html(config),
    }
}

#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// React adapter props mirror [`InputBaseRenderConfig`] while owning their inputs.
    #[derive(Clone)]
    pub struct InputBaseProps<'a> {
        /// Shared headless state.
        pub state: &'a InputState,
        /// Placeholder hint rendered inside the input.
        pub placeholder: &'a str,
        /// Accessibility label for screen readers.
        pub aria_label: &'a str,
        /// Input `type` (defaults to `text`).
        pub input_type: &'a str,
        /// Optional status element identifier linked via `aria-describedby`.
        pub status_id: Option<&'a str>,
        /// Optional analytics identifier forwarded to automation hooks.
        pub analytics_id: Option<&'a str>,
        /// Theme color token.
        pub color: InputBaseColor,
        /// Variant token (Text/Contained/Outlined).
        pub variant: InputBaseVariant,
        /// Size token controlling padding/font-size.
        pub size: InputBaseSize,
        /// Additional CSS declarations appended to the themed class.
        pub style_overrides: Option<&'a str>,
    }

    /// Render the input for React SSR pipelines.
    pub fn render(props: &InputBaseProps<'_>) -> String {
        let config = InputBaseRenderConfig {
            state: props.state,
            placeholder: props.placeholder,
            aria_label: props.aria_label,
            input_type: props.input_type,
            status_id: props.status_id,
            analytics_id: props.analytics_id,
            color: props.color.clone(),
            variant: props.variant.clone(),
            size: props.size.clone(),
            style_overrides: props.style_overrides,
        };
        render_input_base(&config).html
    }
}

#[cfg(any(feature = "yew", feature = "leptos"))]
mod shared_state_handle {
    use super::*;
    use std::cell::{Ref, RefCell, RefMut};
    use std::rc::Rc;

    /// Handle offering interior mutability over [`InputState`].
    #[derive(Clone)]
    pub struct InputBaseStateHandle {
        inner: Rc<RefCell<InputState>>,
    }

    impl InputBaseStateHandle {
        /// Construct a new handle from an owned state.
        pub fn new(state: InputState) -> Self {
            Self {
                inner: Rc::new(RefCell::new(state)),
            }
        }

        /// Immutable borrow of the inner state.
        pub fn borrow(&self) -> Ref<'_, InputState> {
            self.inner.borrow()
        }

        /// Mutable borrow of the inner state.
        pub fn borrow_mut(&self) -> RefMut<'_, InputState> {
            self.inner.borrow_mut()
        }
    }

    impl From<InputState> for InputBaseStateHandle {
        fn from(state: InputState) -> Self {
            Self::new(state)
        }
    }

    impl PartialEq for InputBaseStateHandle {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.inner, &other.inner)
        }
    }
}

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use shared_state_handle::InputBaseStateHandle;

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlInputElement, KeyboardEvent};
    use yew::{prelude::*, virtual_dom::VNode};

    fn apply_input_attributes(tag: &mut yew::virtual_dom::VTag, attrs: Vec<(String, String)>) {
        for (key, value) in attrs {
            tag.add_attribute(key, value);
        }
    }

    fn extract_selection(element: &HtmlInputElement) -> Option<InputSelection> {
        let start = element.selection_start().ok().flatten().map(|v| v as usize);
        let end = element.selection_end().ok().flatten().map(|v| v as usize);
        match (start, end) {
            (Some(s), Some(e)) => Some(InputSelection::new(s, e)),
            _ => None,
        }
    }

    /// Properties consumed by the Yew input base component.
    #[derive(Properties, Clone, PartialEq)]
    pub struct InputBaseProps {
        /// Shared headless state powering the input.
        pub state: InputBaseStateHandle,
        /// Placeholder hint rendered inside the control.
        #[prop_or_default]
        pub placeholder: AttrValue,
        /// Accessibility label for the input element.
        #[prop_or_default]
        pub aria_label: AttrValue,
        /// Input `type` attribute.
        #[prop_or(AttrValue::Static("text"))]
        pub input_type: AttrValue,
        /// Optional status element identifier linked through `aria-describedby`.
        #[prop_or_default]
        pub status_id: Option<AttrValue>,
        /// Optional analytics identifier mirrored to the DOM.
        #[prop_or_default]
        pub analytics_id: Option<AttrValue>,
        /// Additional CSS declarations appended to the themed class.
        #[prop_or_default]
        pub style_overrides: Option<String>,
        /// Theme color token.
        #[prop_or_default]
        pub color: InputBaseColor,
        /// Variant token controlling borders/background.
        #[prop_or_default]
        pub variant: InputBaseVariant,
        /// Size token controlling padding and font size.
        #[prop_or_default]
        pub size: InputBaseSize,
        /// Change callback emitted after `InputState::change` completes.
        #[prop_or_default]
        pub on_change: Option<Callback<InputChangeEvent>>,
        /// Commit callback emitted after blur/enter.
        #[prop_or_default]
        pub on_commit: Option<Callback<InputCommitEvent>>,
        /// Reset callback emitted when escape restores the baseline value.
        #[prop_or_default]
        pub on_reset: Option<Callback<InputResetEvent>>,
    }

    /// Controlled text input primitive that mirrors [`InputState`].
    #[function_component(InputBase)]
    pub fn input_base(props: &InputBaseProps) -> Html {
        let class = crate::style_helpers::themed_class(resolve_style(
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
            props.style_overrides.clone(),
        ));

        let version = use_state(|| 0u64);

        let status_id = props.status_id.as_ref().map(|value| value.as_str());
        let analytics_id = props.analytics_id.as_ref().map(|value| value.as_str());
        let placeholder = props.placeholder.clone();
        let aria_label = props.aria_label.clone();
        let input_type = props.input_type.clone();
        let attrs = {
            let state = props.state.borrow();
            input_attribute_pairs(
                &state,
                status_id,
                analytics_id,
                input_type.as_str(),
                placeholder.as_str(),
                aria_label.as_str(),
            )
        };

        let on_change_cb = props.on_change.clone();
        let state_for_input = props.state.clone();
        let version_for_input = version.clone();
        let analytics_id_input = analytics_id.map(str::to_string);
        let status_id_input = status_id.map(str::to_string);
        let placeholder_input = placeholder.to_string();
        let aria_label_input = aria_label.to_string();
        let input_type_value = input_type.to_string();
        let oninput = Callback::from(move |event: InputEvent| {
            let element = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value = element
                .as_ref()
                .map(HtmlInputElement::value)
                .unwrap_or_default();
            let selection = element.as_ref().and_then(extract_selection);
            let callback = on_change_cb.clone();
            {
                let mut state = state_for_input.borrow_mut();
                let change = state.change(value, selection);
                if let Some(cb) = callback {
                    cb.emit(InputChangeEvent::from(change));
                }
            }
            // Force a re-render so attribute getters reflect the new state.
            let next = (*version_for_input).wrapping_add(1);
            version_for_input.set(next);
        });

        let on_focus_state = props.state.clone();
        let version_for_focus = version.clone();
        let onfocus = Callback::from(move |_event: FocusEvent| {
            {
                let mut state = on_focus_state.borrow_mut();
                state.set_focused(true);
            }
            let next = (*version_for_focus).wrapping_add(1);
            version_for_focus.set(next);
        });

        let on_commit_cb = props.on_commit.clone();
        let state_for_blur = props.state.clone();
        let version_for_blur = version.clone();
        let onblur = Callback::from(move |_event: FocusEvent| {
            let callback = on_commit_cb.clone();
            {
                let mut state = state_for_blur.borrow_mut();
                state.set_focused(false);
                let commit = state.commit();
                if let Some(cb) = callback {
                    cb.emit(InputCommitEvent::from(commit));
                }
            }
            let next = (*version_for_blur).wrapping_add(1);
            version_for_blur.set(next);
        });

        let commit_cb_key = props.on_commit.clone();
        let reset_cb_key = props.on_reset.clone();
        let state_for_keys = props.state.clone();
        let version_for_keys = version.clone();
        let onkeydown = Callback::from(move |event: KeyboardEvent| {
            let mut should_refresh = false;
            match event.key().as_str() {
                "Enter" => {
                    event.prevent_default();
                    let callback = commit_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        let snapshot = state.commit();
                        if let Some(cb) = callback {
                            cb.emit(InputCommitEvent::from(snapshot));
                        }
                    }
                    should_refresh = true;
                }
                "Escape" => {
                    event.prevent_default();
                    let callback = reset_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        let snapshot = state.reset();
                        if let Some(cb) = callback {
                            cb.emit(InputResetEvent::from(snapshot));
                        }
                    }
                    should_refresh = true;
                }
                _ => {}
            }
            if should_refresh {
                let next = (*version_for_keys).wrapping_add(1);
                version_for_keys.set(next);
            }
        });

        let mut node = html! {
            <input
                class={class}
                oninput={oninput}
                onfocus={onfocus}
                onblur={onblur}
                onkeydown={onkeydown}
            />
        };
        if let VNode::VTag(ref mut tag) = node {
            apply_input_attributes(tag, attrs);
        }
        node
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{InputBase, InputBaseProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::{
        component, create_memo, create_signal,
        ev::{Event, FocusEvent, KeyboardEvent},
        event_target_value, view, IntoView, SignalGet, SignalSet, SignalUpdate,
    };
    use std::rc::Rc;

    fn attr_lookup(attrs: &[(String, String)], key: &str) -> Option<String> {
        attrs
            .iter()
            .find(|(candidate, _)| candidate == key)
            .map(|(_, value)| value.clone())
    }

    /// Properties consumed by the Leptos input base component.
    #[derive(leptos::Props, Clone, PartialEq)]
    pub struct InputBaseProps {
        /// Shared headless state powering the input element.
        pub state: InputBaseStateHandle,
        /// Placeholder hint rendered inside the control.
        #[prop(optional, into)]
        pub placeholder: Option<String>,
        /// Accessibility label describing the input.
        #[prop(optional, into)]
        pub aria_label: Option<String>,
        /// Input `type` attribute.
        #[prop(optional, into)]
        pub input_type: Option<String>,
        /// Optional status element identifier.
        #[prop(optional, into)]
        pub status_id: Option<String>,
        /// Optional analytics identifier forwarded to automation hooks.
        #[prop(optional, into)]
        pub analytics_id: Option<String>,
        /// Additional CSS declarations appended to the themed class.
        #[prop(optional)]
        pub style_overrides: Option<String>,
        /// Theme color token.
        #[prop(optional)]
        pub color: Option<InputBaseColor>,
        /// Variant token controlling borders/background.
        #[prop(optional)]
        pub variant: Option<InputBaseVariant>,
        /// Size token controlling padding and font size.
        #[prop(optional)]
        pub size: Option<InputBaseSize>,
        /// Change callback invoked with [`InputChangeEvent`].
        #[prop(optional)]
        pub on_change: Option<Rc<dyn Fn(InputChangeEvent)>>,
        /// Commit callback invoked with [`InputCommitEvent`].
        #[prop(optional)]
        pub on_commit: Option<Rc<dyn Fn(InputCommitEvent)>>,
        /// Reset callback invoked with [`InputResetEvent`].
        #[prop(optional)]
        pub on_reset: Option<Rc<dyn Fn(InputResetEvent)>>,
    }

    /// Leptos component mirroring the Yew implementation.
    #[component]
    pub fn InputBase(props: InputBaseProps) -> impl IntoView {
        let InputBaseProps {
            state,
            placeholder,
            aria_label,
            input_type,
            status_id,
            analytics_id,
            style_overrides,
            color,
            variant,
            size,
            on_change,
            on_commit,
            on_reset,
        } = props;

        let placeholder = placeholder.unwrap_or_default();
        let aria_label = aria_label.unwrap_or_default();
        let input_type = input_type.unwrap_or_else(|| "text".to_string());
        let color = color.unwrap_or_default();
        let variant = variant.unwrap_or_default();
        let size = size.unwrap_or_default();

        let class = crate::style_helpers::themed_class(resolve_style(
            color.clone(),
            size.clone(),
            variant.clone(),
            style_overrides.clone(),
        ));

        let (version, set_version) = create_signal(0u64);
        let state_for_snapshot = state.clone();
        let status_id_snapshot = status_id.clone();
        let analytics_id_snapshot = analytics_id.clone();
        let placeholder_snapshot = placeholder.clone();
        let aria_label_snapshot = aria_label.clone();
        let input_type_snapshot = input_type.clone();
        let attributes = create_memo(move |_| {
            version.get();
            let state = state_for_snapshot.borrow();
            input_attribute_pairs(
                &state,
                status_id_snapshot.as_deref(),
                analytics_id_snapshot.as_deref(),
                input_type_snapshot.as_str(),
                placeholder_snapshot.as_str(),
                aria_label_snapshot.as_str(),
            )
        });

        let change_cb = on_change.clone();
        let state_for_input = state.clone();
        let set_version_input = set_version.clone();
        let on_input_handler = move |ev: Event| {
            let value = event_target_value(&ev);
            let callback = change_cb.clone();
            {
                let mut state = state_for_input.borrow_mut();
                let change = state.change(value, None);
                if let Some(cb) = callback {
                    cb(InputChangeEvent::from(change));
                }
            }
            set_version_input.update(|tick| *tick = tick.wrapping_add(1));
        };

        let focus_state = state.clone();
        let set_version_focus = set_version.clone();
        let on_focus_handler = move |_ev: FocusEvent| {
            {
                let mut state = focus_state.borrow_mut();
                state.set_focused(true);
            }
            set_version_focus.update(|tick| *tick = tick.wrapping_add(1));
        };

        let commit_cb = on_commit.clone();
        let state_for_blur = state.clone();
        let set_version_blur = set_version.clone();
        let on_blur_handler = move |_ev: FocusEvent| {
            let callback = commit_cb.clone();
            {
                let mut state = state_for_blur.borrow_mut();
                state.set_focused(false);
                let commit = state.commit();
                if let Some(cb) = callback {
                    cb(InputCommitEvent::from(commit));
                }
            }
            set_version_blur.update(|tick| *tick = tick.wrapping_add(1));
        };

        let commit_cb_key = on_commit.clone();
        let reset_cb_key = on_reset.clone();
        let state_for_keys = state.clone();
        let set_version_keys = set_version.clone();
        let on_keydown_handler = move |ev: KeyboardEvent| {
            let mut should_refresh = false;
            match ev.key().as_str() {
                "Enter" => {
                    ev.prevent_default();
                    let callback = commit_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        let commit = state.commit();
                        if let Some(cb) = callback {
                            cb(InputCommitEvent::from(commit));
                        }
                    }
                    should_refresh = true;
                }
                "Escape" => {
                    ev.prevent_default();
                    let callback = reset_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        let reset = state.reset();
                        if let Some(cb) = callback {
                            cb(InputResetEvent::from(reset));
                        }
                    }
                    should_refresh = true;
                }
                _ => {}
            }
            if should_refresh {
                set_version_keys.update(|tick| *tick = tick.wrapping_add(1));
            }
        };

        view! {
            <input
                class=class
                prop:value=move || {
                    attr_lookup(&attributes.get(), "value").unwrap_or_default()
                }
                attr:placeholder=move || attr_lookup(&attributes.get(), "placeholder")
                attr:aria-label=move || attr_lookup(&attributes.get(), "aria-label")
                attr:aria-invalid=move || attr_lookup(&attributes.get(), "aria-invalid")
                attr:aria-describedby=move || attr_lookup(&attributes.get(), "aria-describedby")
                attr:data-dirty=move || {
                    attr_lookup(&attributes.get(), "data-dirty").unwrap_or_else(|| "false".into())
                }
                attr:data-visited=move || {
                    attr_lookup(&attributes.get(), "data-visited").unwrap_or_else(|| "false".into())
                }
                attr:data-focused=move || {
                    attr_lookup(&attributes.get(), "data-focused").unwrap_or_else(|| "false".into())
                }
                attr:data-status-message=move || attr_lookup(&attributes.get(), "data-status-message")
                attr:data-analytics-id=move || attr_lookup(&attributes.get(), "data-analytics-id")
                on:input=on_input_handler
                on:focus=on_focus_handler
                on:blur=on_blur_handler
                on:keydown=on_keydown_handler
            />
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{InputBase, InputBaseProps};

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct InputBaseProps {
        /// Placeholder hint rendered inside the input.
        pub placeholder: String,
        /// Accessibility label.
        pub aria_label: String,
        /// Input `type` attribute.
        pub input_type: String,
        /// Theme color token.
        pub color: InputBaseColor,
        /// Variant token.
        pub variant: InputBaseVariant,
        /// Size token.
        pub size: InputBaseSize,
        /// Optional style overrides appended to the themed class.
        pub style_overrides: Option<String>,
        /// Optional status identifier for validation messaging.
        pub status_id: Option<String>,
        /// Optional analytics identifier mirrored to automation hooks.
        pub analytics_id: Option<String>,
    }

    /// Render the input into HTML for SSR/hydration pipelines.
    pub fn render(props: &InputBaseProps, state: &InputState) -> String {
        let config = InputBaseRenderConfig {
            state,
            placeholder: props.placeholder.as_str(),
            aria_label: props.aria_label.as_str(),
            input_type: props.input_type.as_str(),
            status_id: props.status_id.as_deref(),
            analytics_id: props.analytics_id.as_deref(),
            color: props.color.clone(),
            variant: props.variant.clone(),
            size: props.size.clone(),
            style_overrides: props.style_overrides.as_deref(),
        };
        render_input_base(&config).html
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Properties consumed by the Sycamore adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct InputBaseProps {
        /// Placeholder hint rendered inside the input.
        pub placeholder: String,
        /// Accessibility label for assistive tech.
        pub aria_label: String,
        /// Input `type` attribute.
        pub input_type: String,
        /// Theme color token.
        pub color: InputBaseColor,
        /// Variant token.
        pub variant: InputBaseVariant,
        /// Size token.
        pub size: InputBaseSize,
        /// Optional style overrides appended to the themed class.
        pub style_overrides: Option<String>,
        /// Optional status identifier for validation messaging.
        pub status_id: Option<String>,
        /// Optional analytics identifier mirrored to automation hooks.
        pub analytics_id: Option<String>,
    }

    /// Render the input into deterministic HTML for SSR.
    pub fn render(props: &InputBaseProps, state: &InputState) -> String {
        let config = InputBaseRenderConfig {
            state,
            placeholder: props.placeholder.as_str(),
            aria_label: props.aria_label.as_str(),
            input_type: props.input_type.as_str(),
            status_id: props.status_id.as_deref(),
            analytics_id: props.analytics_id.as_deref(),
            color: props.color.clone(),
            variant: props.variant.clone(),
            size: props.size.clone(),
            style_overrides: props.style_overrides.as_deref(),
        };
        render_input_base(&config).html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analytics_attributes_include_selection_and_status() {
        let mut state = InputState::uncontrolled("seed", Some(InputSelection::new(0, 4)));
        state.change("updated", Some(InputSelection::collapsed(7)));
        state.set_errors(vec![String::from("Required"), String::from("Unique")]);
        state.commit();
        let attrs = input_attribute_pairs(
            &state,
            Some("status"),
            Some("analytics-1"),
            "text",
            "Placeholder",
            "Label",
        );
        let lookup = |key: &str| {
            attrs
                .iter()
                .find(|(candidate, _)| candidate == key)
                .map(|(_, value)| value.clone())
        };
        assert_eq!(lookup("data-dirty"), Some("true".into()));
        assert_eq!(lookup("data-visited"), Some("true".into()));
        assert_eq!(lookup("aria-describedby"), Some("status".into()));
        assert_eq!(
            lookup("data-status-message"),
            Some("Required\nUnique".into())
        );
        assert_eq!(lookup("data-selection-start"), Some("7".into()));
        let automation_attr =
            style_helpers::automation_data_attr(COMPONENT_NAME, ["analytics", "id"]);
        assert_eq!(lookup(&automation_attr), Some("analytics-1".into()));
    }

    #[test]
    fn render_output_produces_html_fragment() {
        let state = InputState::uncontrolled("seed", None);
        let config = InputBaseRenderConfig::new(&state);
        let html = render_input_base(&config).html;
        assert!(html.starts_with("<input"));
        assert!(html.contains("data-component=\""));
    }
}
