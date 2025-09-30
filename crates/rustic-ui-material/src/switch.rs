//! Material switch built from the headless [`SwitchState`].
//!
//! Feature-gated adapters expose idiomatic components per framework while
//! sharing styling and accessibility metadata via
//! [`ToggleControlDescriptor`](crate::selection_control::ToggleControlDescriptor):
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
//! between SSR and client renders regardless of framework.

use rustic_ui_headless::switch::SwitchState;
use rustic_ui_styled_engine::{css_with_theme, Style};
use std::collections::HashMap;

use crate::selection_control::{self, ToggleControlDescriptor};

#[derive(Clone, Debug, PartialEq)]
pub struct SwitchProps {
    /// Human friendly label rendered adjacent to the switch track.
    pub label: String,
}

impl SwitchProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[allow(dead_code)]
fn build_descriptor(props: &SwitchProps, state: &SwitchState) -> ToggleControlDescriptor {
    ToggleControlDescriptor::new(props.label.clone(), themed_switch_style())
        .with_attributes(state.aria_attributes())
}

#[allow(dead_code)]
fn render_html(props: &SwitchProps, state: &SwitchState) -> String {
    let descriptor = build_descriptor(props, state);
    selection_control::render_toggle_html(&descriptor)
}

#[allow(dead_code)]
fn attributes_to_map(pairs: Vec<(String, String)>) -> HashMap<String, String> {
    pairs.into_iter().collect()
}

/// Builds the switch track and thumb styling from the active theme tokens. By
/// leaning on `css_with_theme!` we avoid scattering literal values and keep the
/// component responsive to palette or spacing overrides.
#[allow(dead_code)]
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

#[cfg(feature = "react")]
pub mod react {
    //! React adapter returning `Jsx` nodes via the WASM bridge.
    use super::*;
    use js_sys::{Array, Function, Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    /// Type alias representing React nodes produced by the adapter.
    pub type Jsx = JsValue;

    /// Properties consumed by the React switch component.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ReactSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state driving ARIA metadata.
        pub state: SwitchState,
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

    /// React component rendering the Material switch.
    pub fn ReactSwitch(props: &ReactSwitchProps) -> Jsx {
        let descriptor = super::build_descriptor(&props.switch, &props.state);
        let label = descriptor.label().to_string();
        let attributes = descriptor.themed_attributes();
        let props_object = build_props_object(attributes);
        create_element("span", props_object, &[JsValue::from_str(&label)])
    }
}

#[cfg(feature = "yew")]
pub mod yew {
    //! Yew adapter leveraging `#[function_component]` for idiomatic usage.
    use super::*;
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

    /// Properties accepted by [`YewSwitch`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing accessibility metadata.
        pub state: SwitchState,
    }

    /// Switch rendered inside Yew applications.
    #[function_component(YewSwitch)]
    pub fn yew_switch(props: &YewSwitchProps) -> Html {
        let descriptor = super::build_descriptor(&props.switch, &props.state);
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
    //! Leptos adapter returning a [`leptos::View`] that hydrates cleanly across
    //! server and client renders.
    use super::*;
    use leptos::prelude::*;

    /// Properties accepted by [`LeptosSwitch`].
    #[derive(Clone)]
    pub struct LeptosSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
    }

    #[component]
    pub fn LeptosSwitch(props: LeptosSwitchProps) -> impl IntoView {
        let descriptor = super::build_descriptor(&props.switch, &props.state);
        let label = descriptor.label().to_string();
        let mut attr_map = super::attributes_to_map(descriptor.themed_attributes());
        let class = attr_map.remove("class").unwrap_or_default();
        let role = attr_map.remove("role").unwrap_or_else(|| "switch".into());
        let aria_checked = attr_map
            .remove("aria-checked")
            .unwrap_or_else(|| String::from("false"));
        let aria_disabled = attr_map.remove("aria-disabled");
        let tabindex = attr_map
            .remove("tabindex")
            .unwrap_or_else(|| String::from("0"));
        let data_on = attr_map
            .remove("data-on")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));

        view! {
            <span
                class=class
                role=role
                aria-checked=aria_checked
                aria-disabled=aria_disabled
                tabindex=tabindex
                data-on=data_on
                data-focus-visible=data_focus_visible
            >{label}</span>
        }
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter built with `rsx!` for idiomatic usage inside Dioxus
    //! applications.
    use super::*;
    use dioxus::prelude::*;

    /// Properties accepted by [`DioxusSwitch`].
    #[derive(Props, Clone, PartialEq)]
    pub struct DioxusSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
    }

    /// Switch rendered as a Dioxus component.
    pub fn DioxusSwitch(cx: Scope<DioxusSwitchProps>) -> Element {
        let descriptor = super::build_descriptor(&cx.props().switch, &cx.props().state);
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
        let data_on = attr_map
            .remove("data-on")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));

        cx.render(rsx! {
            span {
                class: class,
                role: role,
                aria_checked: aria_checked,
                aria_disabled: aria_disabled,
                tabindex: tabindex,
                data_on: data_on,
                data_focus_visible: data_focus_visible,
                {label}
            }
        })
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter yielding a [`Template`] for reactive dashboards.
    use super::*;
    use sycamore::prelude::*;

    /// Alias mirroring Sycamore's view representation.
    pub type Template<G> = View<G>;

    /// Properties accepted by [`SycamoreSwitch`].
    #[derive(Clone)]
    pub struct SycamoreSwitchProps {
        /// Presentation details for the switch label.
        pub switch: SwitchProps,
        /// Headless state providing ARIA metadata.
        pub state: SwitchState,
    }

    /// Switch rendered within a Sycamore reactive scope.
    #[component]
    pub fn SycamoreSwitch<G: Html>(cx: Scope, props: SycamoreSwitchProps) -> Template<G> {
        let descriptor = super::build_descriptor(&props.switch, &props.state);
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
        let data_on = attr_map
            .remove("data-on")
            .unwrap_or_else(|| String::from("false"));
        let data_focus_visible = attr_map
            .remove("data-focus-visible")
            .unwrap_or_else(|| String::from("false"));

        view! { cx,
            span(
                class=class,
                role=role,
                aria_checked=aria_checked,
                aria_disabled=aria_disabled,
                tabindex=tabindex,
                data_on=data_on,
                data_focus_visible=data_focus_visible,
            ) { (label) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_attributes_include_role() {
        let state = SwitchState::uncontrolled(false, true);
        let descriptor = build_descriptor(&SwitchProps::new("Notifications"), &state);
        assert!(descriptor
            .aria_attributes()
            .any(|(k, v)| k == "role" && v == "switch"));
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
