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

use crate::{
    selection_control::{self, RadioGroupDescriptor, RadioOptionDescriptor},
    telemetry::{instrument_render, TelemetryContext, TelemetryHooks},
};

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
        /// Telemetry hooks applied around the React render.
        pub telemetry: TelemetryHooks,
    }

    impl ReactRadioGroupProps {
        /// Convenience constructor mirroring the previous two-field struct so
        /// downstream callers remain source compatible.
        pub fn new(group: RadioGroupProps, state: RadioGroupState) -> Self {
            Self {
                group,
                state,
                telemetry: TelemetryHooks::default(),
            }
        }

        #[allow(dead_code)]
        pub fn with_telemetry(mut self, telemetry: TelemetryHooks) -> Self {
            self.telemetry = telemetry;
            self
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
            let option_children: Vec<JsValue> = snapshot
                .options
                .iter()
                .map(|option| {
                    let option_props = build_props_object(option.themed_attributes.clone());
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
    use yew::prelude::*;
    use yew::virtual_dom::VNode;

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
    }

    /// Radio group rendered via Yew.
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
            let mut node = html! {
                <div>
                    { for snapshot.options.iter().map(|option| {
                        let option_attrs = option.themed_attributes.clone();
                        let mut child = html! { <span>{option.label.clone()}</span> };
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
