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

use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_styled_engine::{css_with_theme, Style};
use std::collections::HashMap;

use crate::selection_control::{self, RadioGroupDescriptor, RadioOptionDescriptor};

#[derive(Clone, Debug, PartialEq)]
pub struct RadioGroupProps {
    /// Optional custom labels for each option. When omitted the state's option
    /// names are reused.
    pub option_labels: Vec<String>,
}

impl RadioGroupProps {
    pub fn new(option_labels: impl Into<Vec<String>>) -> Self {
        Self {
            option_labels: option_labels.into(),
        }
    }

    pub fn from_state(state: &RadioGroupState) -> Self {
        Self {
            option_labels: state.options().to_vec(),
        }
    }
}

#[allow(dead_code)]
fn build_descriptor(props: &RadioGroupProps, state: &RadioGroupState) -> RadioGroupDescriptor {
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

    for (index, option) in state.options().iter().enumerate() {
        let label = labels.get(index).cloned().unwrap_or_else(|| option.clone());
        let option_descriptor = RadioOptionDescriptor::new(label, themed_radio_option_style())
            .with_attributes(state.option_aria_attributes(index))
            .attribute("data-index", index.to_string());
        descriptor = descriptor.option(option_descriptor);
    }

    descriptor
}

#[allow(dead_code)]
fn render_html(props: &RadioGroupProps, state: &RadioGroupState) -> String {
    let descriptor = build_descriptor(props, state);
    selection_control::render_radio_group_html(&descriptor)
}

#[allow(dead_code)]
fn attributes_to_map(pairs: Vec<(String, String)>) -> HashMap<String, String> {
    pairs.into_iter().collect()
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
    use wasm_bindgen::{JsCast, JsValue};

    /// Type alias representing React elements emitted by the adapter.
    pub type Jsx = JsValue;

    /// Properties accepted by the React radio group component.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ReactRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state describing option metadata and focus handling.
        pub state: RadioGroupState,
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

    /// React component rendering a Material radio group.
    pub fn ReactRadioGroup(props: &ReactRadioGroupProps) -> Jsx {
        let descriptor = super::build_descriptor(&props.group, &props.state);
        let group_attrs = descriptor.group_thematic_attributes();
        let group_props = build_props_object(group_attrs);

        let option_children: Vec<JsValue> = descriptor
            .options()
            .iter()
            .map(|option| {
                let option_props = build_props_object(option.themed_attributes());
                let label = option.label().to_string();
                create_element("span", option_props, &[JsValue::from_str(&label)])
            })
            .collect();

        create_element("div", group_props, option_children.as_slice())
    }
}

#[cfg(feature = "yew")]
pub mod yew {
    //! Yew adapter implemented with `#[function_component]` for idiomatic usage.
    use super::*;
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

    /// Properties accepted by [`YewRadioGroup`].
    #[derive(Properties, Clone, PartialEq)]
    pub struct YewRadioGroupProps {
        /// Optional labels applied to each radio option.
        pub group: RadioGroupProps,
        /// Headless state providing focus and selection metadata.
        pub state: RadioGroupState,
    }

    /// Radio group rendered via Yew.
    #[function_component(YewRadioGroup)]
    pub fn yew_radio_group(props: &YewRadioGroupProps) -> Html {
        let descriptor = super::build_descriptor(&props.group, &props.state);
        let group_attrs = descriptor.group_thematic_attributes();
        let mut node = html! {
            <div>
                { for descriptor.options().iter().map(|option| {
                    let option_label = option.label().to_string();
                    let option_attrs = option.themed_attributes();
                    let mut child = html! { <span>{option_label}</span> };
                    if let VNode::VTag(ref mut tag) = child {
                        for (key, value) in option_attrs {
                            tag.add_attribute(key, value);
                        }
                    }
                    child
                }) }
            </div>
        };

        if let VNode::VTag(ref mut tag) = node {
            for (key, value) in group_attrs {
                tag.add_attribute(key, value);
            }
        }

        node
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
    }

    #[component]
    pub fn LeptosRadioGroup(props: LeptosRadioGroupProps) -> impl IntoView {
        let descriptor = super::build_descriptor(&props.group, &props.state);
        let mut group_map = super::attributes_to_map(descriptor.group_thematic_attributes());
        let class = group_map.remove("class").unwrap_or_default();
        let role = group_map
            .remove("role")
            .unwrap_or_else(|| String::from("radiogroup"));
        let aria_orientation = group_map
            .remove("aria-orientation")
            .unwrap_or_else(|| String::from("horizontal"));
        let aria_disabled = group_map.remove("aria-disabled");
        let data_orientation = group_map
            .remove("data-orientation")
            .unwrap_or_else(|| String::from("horizontal"));

        let option_views: Vec<View> = descriptor
            .options()
            .iter()
            .map(|option| {
                let mut option_map = super::attributes_to_map(option.themed_attributes());
                let option_class = option_map.remove("class").unwrap_or_default();
                let option_role = option_map
                    .remove("role")
                    .unwrap_or_else(|| String::from("radio"));
                let aria_checked = option_map
                    .remove("aria-checked")
                    .unwrap_or_else(|| String::from("false"));
                let aria_disabled_opt = option_map.remove("aria-disabled");
                let tabindex = option_map
                    .remove("tabindex")
                    .unwrap_or_else(|| String::from("0"));
                let data_checked = option_map
                    .remove("data-checked")
                    .unwrap_or_else(|| String::from("false"));
                let data_focus_visible = option_map
                    .remove("data-focus-visible")
                    .unwrap_or_else(|| String::from("false"));
                let data_index = option_map
                    .remove("data-index")
                    .unwrap_or_else(|| String::from("0"));
                let label = option.label().to_string();

                view! {
                    <span
                        class=option_class
                        role=option_role
                        aria-checked=aria_checked
                        aria-disabled=aria_disabled_opt.clone()
                        tabindex=tabindex
                        data_checked=data_checked
                        data_focus_visible=data_focus_visible
                        data_index=data_index
                    >{label}</span>
                }
            })
            .collect();

        let options_fragment = View::new_fragment(option_views);

        view! {
            <div
                class=class
                role=role
                aria-orientation=aria_orientation
                aria-disabled=aria_disabled
                data-orientation=data_orientation
            >{options_fragment}</div>
        }
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
    }

    /// Radio group rendered as a Dioxus component.
    pub fn DioxusRadioGroup(cx: Scope<DioxusRadioGroupProps>) -> Element {
        let descriptor = super::build_descriptor(&cx.props().group, &cx.props().state);
        let mut group_map = super::attributes_to_map(descriptor.group_thematic_attributes());
        let class = group_map.remove("class").unwrap_or_default();
        let role = group_map
            .remove("role")
            .unwrap_or_else(|| String::from("radiogroup"));
        let aria_orientation = group_map
            .remove("aria-orientation")
            .unwrap_or_else(|| String::from("horizontal"));
        let aria_disabled = group_map.remove("aria-disabled");
        let data_orientation = group_map
            .remove("data-orientation")
            .unwrap_or_else(|| String::from("horizontal"));

        cx.render(rsx! {
            div {
                class: class,
                role: role,
                aria_orientation: aria_orientation,
                aria_disabled: aria_disabled,
                data_orientation: data_orientation,
                { descriptor.options().iter().map(|option| {
                    let mut option_map = super::attributes_to_map(option.themed_attributes());
                    let option_class = option_map.remove("class").unwrap_or_default();
                    let option_role = option_map
                        .remove("role")
                        .unwrap_or_else(|| String::from("radio"));
                    let aria_checked = option_map
                        .remove("aria-checked")
                        .unwrap_or_else(|| String::from("false"));
                    let aria_disabled_opt = option_map.remove("aria-disabled");
                    let tabindex = option_map
                        .remove("tabindex")
                        .unwrap_or_else(|| String::from("0"));
                    let data_checked = option_map
                        .remove("data-checked")
                        .unwrap_or_else(|| String::from("false"));
                    let data_focus_visible = option_map
                        .remove("data-focus-visible")
                        .unwrap_or_else(|| String::from("false"));
                    let data_index = option_map
                        .remove("data-index")
                        .unwrap_or_else(|| String::from("0"));
                    let label = option.label().to_string();

                    rsx! {
                        span {
                            class: option_class,
                            role: option_role,
                            aria_checked: aria_checked,
                            aria_disabled: aria_disabled_opt,
                            tabindex: tabindex,
                            data_checked: data_checked,
                            data_focus_visible: data_focus_visible,
                            data_index: data_index,
                            {label}
                        }
                    }
                }) }
            }
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
    }

    /// Radio group rendered within a Sycamore reactive scope.
    #[component]
    pub fn SycamoreRadioGroup<G: Html>(cx: Scope, props: SycamoreRadioGroupProps) -> Template<G> {
        let descriptor = super::build_descriptor(&props.group, &props.state);
        let mut group_map = super::attributes_to_map(descriptor.group_thematic_attributes());
        let class = group_map.remove("class").unwrap_or_default();
        let role = group_map
            .remove("role")
            .unwrap_or_else(|| String::from("radiogroup"));
        let aria_orientation = group_map
            .remove("aria-orientation")
            .unwrap_or_else(|| String::from("horizontal"));
        let aria_disabled = group_map.remove("aria-disabled");
        let data_orientation = group_map
            .remove("data-orientation")
            .unwrap_or_else(|| String::from("horizontal"));

        let option_views: Vec<View<G>> = descriptor
            .options()
            .iter()
            .map(|option| {
                let mut option_map = super::attributes_to_map(option.themed_attributes());
                let option_class = option_map.remove("class").unwrap_or_default();
                let option_role = option_map
                    .remove("role")
                    .unwrap_or_else(|| String::from("radio"));
                let aria_checked = option_map
                    .remove("aria-checked")
                    .unwrap_or_else(|| String::from("false"));
                let aria_disabled_opt = option_map.remove("aria-disabled");
                let tabindex = option_map
                    .remove("tabindex")
                    .unwrap_or_else(|| String::from("0"));
                let data_checked = option_map
                    .remove("data-checked")
                    .unwrap_or_else(|| String::from("false"));
                let data_focus_visible = option_map
                    .remove("data-focus-visible")
                    .unwrap_or_else(|| String::from("false"));
                let data_index = option_map
                    .remove("data-index")
                    .unwrap_or_else(|| String::from("0"));
                let label = option.label().to_string();

                view! { cx,
                    span(
                        class=option_class,
                        role=option_role,
                        aria_checked=aria_checked,
                        aria_disabled=aria_disabled_opt,
                        tabindex=tabindex,
                        data_checked=data_checked,
                        data_focus_visible=data_focus_visible,
                        data_index=data_index,
                    ) { (label) }
                }
            })
            .collect();

        let options_fragment = View::new_fragment(option_views);

        view! { cx,
            div(
                class=class,
                role=role,
                aria_orientation=aria_orientation,
                aria_disabled=aria_disabled,
                data_orientation=data_orientation,
            ) { (options_fragment) }
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
        let descriptor = build_descriptor(&props, &state);
        assert!(descriptor.aria_attributes().any(|(k, _)| k == "role"));
        assert!(descriptor.options().iter().any(|option| option
            .aria_attributes()
            .any(|(k, v)| k == "aria-checked" && v == "true")));
    }
}
