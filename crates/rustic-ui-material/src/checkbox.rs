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

use crate::{
    selection_control::{self, ToggleControlDescriptor},
    telemetry::{instrument_render, TelemetryContext, TelemetryHooks},
};
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_styled_engine::{css_with_theme, Style};
use std::collections::HashMap;

/// Props shared across all framework adapters.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxProps {
    /// Visible label rendered alongside the checkbox indicator.
    pub label: String,
}

impl CheckboxProps {
    /// Convenience constructor for tests and examples.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[allow(dead_code)]
fn build_descriptor(props: &CheckboxProps, state: &CheckboxState) -> ToggleControlDescriptor {
    ToggleControlDescriptor::new(props.label.clone(), themed_checkbox_style())
        .with_attributes(state.aria_attributes())
}

#[allow(dead_code)]
fn render_html(props: &CheckboxProps, state: &CheckboxState) -> String {
    let descriptor = build_descriptor(props, state);
    selection_control::render_toggle_html(&descriptor)
}

#[allow(dead_code)]
fn attributes_to_map(pairs: Vec<(String, String)>) -> HashMap<String, String> {
    pairs.into_iter().collect()
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
    use wasm_bindgen::{JsCast, JsValue};

    /// Type alias representing React elements returned through the WASM bridge.
    pub type Jsx = JsValue;

    /// Properties consumed by the React checkbox component.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ReactCheckboxProps {
        /// Visual label rendered beside the checkbox indicator.
        pub checkbox: CheckboxProps,
        /// Headless state machine powering ARIA metadata.
        pub state: CheckboxState,
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

    /// React component rendering the Material checkbox.
    pub fn ReactCheckbox(props: &ReactCheckboxProps) -> Jsx {
        let descriptor = super::build_descriptor(&props.checkbox, &props.state);
        let label = descriptor.label().to_string();
        let attributes = descriptor.themed_attributes();
        let props_object = build_props_object(attributes);
        create_element("span", props_object, &[JsValue::from_str(&label)])
    }
}

#[cfg(feature = "yew")]
pub mod yew {
    //! Yew adapter implemented with `#[function_component]` so downstream apps
    //! can compose the checkbox like any other Yew widget.
    use super::*;
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

    /// Properties consumed by [`YewCheckbox`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine providing accessibility metadata.
        pub state: CheckboxState,
    }

    /// Checkbox rendered as a Yew component.
    #[function_component(YewCheckbox)]
    pub fn yew_checkbox(props: &YewCheckboxProps) -> Html {
        let descriptor = super::build_descriptor(&props.checkbox, &props.state);
        let label = descriptor.label().to_string();
        let attrs = descriptor.themed_attributes();
        let mut node = html! { <span>{label}</span> };
        if let VNode::VTag(ref mut tag) = node {
            for (key, value) in attrs {
                tag.add_attribute(key, value);
            }
        }
        node
    }
}

#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter returning a [`leptos::View`] so reactive signals can drive
    //! the checkbox while sharing the descriptor wiring used by other
    //! frameworks.
    use super::*;
    use leptos::prelude::*;

    /// Properties accepted by [`LeptosCheckbox`].
    #[derive(Clone)]
    pub struct LeptosCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine describing behavior and ARIA metadata.
        pub state: CheckboxState,
    }

    #[component]
    pub fn LeptosCheckbox(props: LeptosCheckboxProps) -> impl IntoView {
        let descriptor = super::build_descriptor(&props.checkbox, &props.state);
        let label = descriptor.label().to_string();
        let mut attr_map = super::attributes_to_map(descriptor.themed_attributes());
        let class = attr_map.remove("class").unwrap_or_default();
        let role = attr_map.remove("role").unwrap_or_else(|| "checkbox".into());
        let aria_checked = attr_map
            .remove("aria-checked")
            .unwrap_or_else(|| String::from("false"));
        let aria_disabled = attr_map.remove("aria-disabled");
        let tabindex = attr_map
            .remove("tabindex")
            .unwrap_or_else(|| String::from("0"));
        let data_checked = attr_map
            .remove("data-checked")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));
        let data_indeterminate = attr_map
            .remove("data-indeterminate")
            .unwrap_or_else(|| String::from("false"));

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
            >{label}</span>
        }
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter using `rsx!` so teams can hydrate the checkbox inside
    //! Dioxus shells without falling back to raw HTML strings.
    use super::*;
    use dioxus::prelude::*;

    /// Properties accepted by [`DioxusCheckbox`].
    #[derive(Props, Clone, PartialEq)]
    pub struct DioxusCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine describing accessibility metadata.
        pub state: CheckboxState,
    }

    /// Checkbox rendered through the Dioxus virtual DOM.
    pub fn DioxusCheckbox(cx: Scope<DioxusCheckboxProps>) -> Element {
        let descriptor = super::build_descriptor(&cx.props().checkbox, &cx.props().state);
        let label = descriptor.label().to_string();
        let mut attr_map = super::attributes_to_map(descriptor.themed_attributes());
        let class = attr_map.remove("class").unwrap_or_default();
        let role = attr_map.remove("role").unwrap_or_default();
        let aria_checked = attr_map
            .remove("aria-checked")
            .unwrap_or_else(|| String::from("false"));
        let aria_disabled = attr_map.remove("aria-disabled");
        let tabindex = attr_map
            .remove("tabindex")
            .unwrap_or_else(|| String::from("0"));
        let data_checked = attr_map
            .remove("data-checked")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));
        let data_indeterminate = attr_map
            .remove("data-indeterminate")
            .unwrap_or_else(|| String::from("false"));

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
                {label}
            }
        })
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter returning a [`Template`] for signal driven surfaces.
    use super::*;
    use sycamore::prelude::*;

    /// Alias matching the return type expected by Sycamore component macros.
    pub type Template<G> = View<G>;

    /// Properties accepted by [`SycamoreCheckbox`].
    #[derive(Clone)]
    pub struct SycamoreCheckboxProps {
        /// Visual configuration for the checkbox.
        pub checkbox: CheckboxProps,
        /// Headless state machine wiring ARIA metadata.
        pub state: CheckboxState,
    }

    /// Checkbox rendered within a Sycamore reactive scope.
    #[component]
    pub fn SycamoreCheckbox<G: Html>(cx: Scope, props: SycamoreCheckboxProps) -> Template<G> {
        let descriptor = super::build_descriptor(&props.checkbox, &props.state);
        let label = descriptor.label().to_string();
        let mut attr_map = super::attributes_to_map(descriptor.themed_attributes());
        let class = attr_map.remove("class").unwrap_or_default();
        let role = attr_map.remove("role").unwrap_or_default();
        let aria_checked = attr_map
            .remove("aria-checked")
            .unwrap_or_else(|| String::from("false"));
        let aria_disabled = attr_map.remove("aria-disabled");
        let tabindex = attr_map
            .remove("tabindex")
            .unwrap_or_else(|| String::from("0"));
        let data_checked = attr_map
            .remove("data-checked")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));
        let data_indeterminate = attr_map
            .remove("data-indeterminate")
            .unwrap_or_else(|| String::from("false"));

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
            ) { (label) }
        }
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
}
