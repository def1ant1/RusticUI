#![deny(missing_docs)]
//! Collapsible region renderers that decorate [`CollapsibleRegionState`]
//! with automation identifiers and multi-framework adapters. The helpers
//! keep orchestration consistent with [`accordion`](crate::accordion)
//! while exposing telemetry hooks identical to our dialog/drawer stacks.

use rustic_ui_headless::collapsible_region::{
    CollapsibleContentAttributes, CollapsibleRegionState, CollapsibleTriggerAttributes,
};
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Declarative overrides for the trigger element controlling the region.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CollapsibleTriggerOptions {
    /// Optional id referenced by analytics pipelines.
    pub analytics_id: Option<String>,
    /// Optional `aria-controls` identifier.
    pub controls: Option<String>,
    /// Optional automation identifier surfaced via `data-automation-id`.
    pub automation_id: Option<String>,
}

/// Declarative overrides for the collapsible region itself.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CollapsibleRegionOptions {
    /// Optional DOM identifier referenced by triggers and tests.
    pub id: Option<String>,
    /// Optional analytics identifier mirrored to `data-rustic-analytics-id`.
    pub analytics_id: Option<String>,
    /// Optional automation identifier surfaced via `data-automation-id`.
    pub automation_id: Option<String>,
}

fn apply_trigger_options<'a>(
    mut attrs: CollapsibleTriggerAttributes<'a>,
    options: &'a CollapsibleTriggerOptions,
) -> CollapsibleTriggerAttributes<'a> {
    if let Some(controls) = &options.controls {
        attrs = attrs.controls(controls);
    }
    if let Some(analytics) = &options.analytics_id {
        attrs = attrs.analytics_id(analytics);
    }
    attrs
}

fn apply_region_options<'a>(
    mut attrs: CollapsibleContentAttributes<'a>,
    options: &'a CollapsibleRegionOptions,
) -> CollapsibleContentAttributes<'a> {
    if let Some(id) = &options.id {
        attrs = attrs.id(id);
    }
    if let Some(analytics) = &options.analytics_id {
        attrs = attrs.analytics_id(analytics);
    }
    attrs
}

/// Convert trigger attributes into automation-friendly key/value pairs.
#[must_use]
pub fn collapsible_trigger_attributes(
    attrs: CollapsibleTriggerAttributes<'_>,
    options: &CollapsibleTriggerOptions,
    automation_fallback: &str,
) -> Vec<(String, String)> {
    let attrs = attrs.as_pairs();
    let mut pairs = Vec::with_capacity(attrs.len() + 1);
    for (key, value) in attrs {
        pairs.push((key.into(), value));
    }
    let automation = options
        .automation_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or(automation_fallback);
    pairs.push(("data-automation-id".into(), automation.to_string()));
    pairs
}

/// Convert region attributes into automation-friendly key/value pairs.
#[must_use]
pub fn collapsible_region_attributes(
    attrs: CollapsibleContentAttributes<'_>,
    options: &CollapsibleRegionOptions,
    automation_fallback: &str,
) -> Vec<(String, String)> {
    let attrs = attrs.as_pairs();
    let mut pairs = Vec::with_capacity(attrs.len() + 1);
    for (key, value) in attrs {
        pairs.push((key.into(), value));
    }
    let automation = options
        .automation_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or(automation_fallback);
    pairs.push(("data-automation-id".into(), automation.to_string()));
    pairs
}

fn trigger_style() -> Style {
    css_with_theme!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        background: transparent;
        border: none;
        padding: ${padding};
        cursor: pointer;
        font: inherit;
        color: inherit;
        &[data-hidden="true"] { opacity: 0.6; }
        "#,
        padding = format!("{}px", theme.spacing(2)),
    )
}

fn region_style() -> Style {
    css_with_theme!(
        r#"
        overflow: hidden;
        transition: height 200ms ease, opacity 200ms ease;
        &[data-hidden="true"] {
            height: 0;
            opacity: 0;
            visibility: hidden;
        }
        &:not([data-hidden="true"]) {
            opacity: 1;
            visibility: visible;
        }
        "#
    )
}

/// Render the trigger as HTML for SSR pipelines.
#[must_use]
pub fn render_collapsible_trigger_html(
    state: &CollapsibleRegionState,
    options: &CollapsibleTriggerOptions,
    automation_fallback: &str,
    label: &str,
) -> String {
    let attrs = state.trigger_attributes();
    let attrs = apply_trigger_options(attrs, options);
    let pairs = collapsible_trigger_attributes(attrs, options, automation_fallback);
    crate::render_helpers::render_element_html("button", trigger_style(), pairs, label)
}

/// Render the collapsible region as HTML for SSR pipelines.
#[must_use]
pub fn render_collapsible_region_html(
    state: &CollapsibleRegionState,
    options: &CollapsibleRegionOptions,
    automation_fallback: &str,
    children: &str,
) -> String {
    let attrs = state.region_attributes();
    let attrs = apply_region_options(attrs, options);
    let pairs = collapsible_region_attributes(attrs, options, automation_fallback);
    crate::render_helpers::render_element_html("div", region_style(), pairs, children)
}

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use std::rc::Rc;
    use yew::prelude::*;

    /// Yew trigger component. Event listeners remain the responsibility of the
    /// hosting accordion or disclosure widget; this wrapper simply mirrors
    /// attributes and telemetry hooks so orchestration layers stay centralised.
    #[derive(Properties, Clone)]
    pub struct CollapsibleTriggerProps {
        /// Shared region state machine.
        pub state: Rc<CollapsibleRegionState>,
        /// Optional attribute overrides.
        #[prop_or_default]
        pub options: CollapsibleTriggerOptions,
        /// Fallback automation identifier when options omit one.
        #[prop_or_else(|| AttrValue::from("rustic-ui::collapsible-trigger"))]
        pub automation_fallback: AttrValue,
        /// Optional custom class merged with the themed baseline.
        #[prop_or_default]
        pub class: Option<AttrValue>,
        /// Trigger label children.
        #[prop_or_default]
        pub children: Children,
    }

    impl PartialEq for CollapsibleTriggerProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.class == other.class
                && self.children == other.children
        }
    }

    #[function_component(CollapsibleTrigger)]
    pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
        let attrs = props.state.trigger_attributes();
        let attrs = apply_trigger_options(attrs, &props.options);
        let pairs = collapsible_trigger_attributes(
            attrs,
            &props.options,
            props.automation_fallback.as_str(),
        );
        let themed = crate::style_helpers::themed_class(trigger_style());
        let class = match &props.class {
            Some(custom) if !custom.is_empty() => format!("{themed} {custom}"),
            _ => themed,
        };
        let mut node =
            html! { <button type="button" class={class}>{ for props.children.iter() }</button> };
        if let Html::VTag(ref mut tag) = node {
            for (key, value) in pairs {
                tag.add_attribute(key, value);
            }
        }
        node
    }

    /// Yew region component mirroring the trigger API surface.
    #[derive(Properties, Clone)]
    pub struct CollapsibleRegionProps {
        /// Shared region state machine.
        pub state: Rc<CollapsibleRegionState>,
        /// Optional attribute overrides.
        #[prop_or_default]
        pub options: CollapsibleRegionOptions,
        /// Fallback automation identifier when options omit one.
        #[prop_or_else(|| AttrValue::from("rustic-ui::collapsible-region"))]
        pub automation_fallback: AttrValue,
        /// Optional custom class merged with the themed baseline.
        #[prop_or_default]
        pub class: Option<AttrValue>,
        /// Region contents.
        #[prop_or_default]
        pub children: Children,
    }

    impl PartialEq for CollapsibleRegionProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.class == other.class
                && self.children == other.children
        }
    }

    #[function_component(CollapsibleRegion)]
    pub fn collapsible_region(props: &CollapsibleRegionProps) -> Html {
        let attrs = props.state.region_attributes();
        let attrs = apply_region_options(attrs, &props.options);
        let pairs = collapsible_region_attributes(
            attrs,
            &props.options,
            props.automation_fallback.as_str(),
        );
        let themed = crate::style_helpers::themed_class(region_style());
        let class = match &props.class {
            Some(custom) if !custom.is_empty() => format!("{themed} {custom}"),
            _ => themed,
        };
        let mut node = html! { <div class={class}>{ for props.children.iter() }</div> };
        if let Html::VTag(ref mut tag) = node {
            for (key, value) in pairs {
                tag.add_attribute(key, value);
            }
        }
        node
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{
    CollapsibleRegion, CollapsibleRegionProps, CollapsibleTrigger, CollapsibleTriggerProps,
};

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::prelude::*;
    use std::sync::Arc;

    /// Leptos variant of [`CollapsibleTrigger`].
    #[derive(Clone)]
    pub struct CollapsibleTriggerProps {
        /// Shared region state machine.
        pub state: Arc<CollapsibleRegionState>,
        /// Attribute overrides.
        pub options: CollapsibleTriggerOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Optional custom class appended to the themed baseline.
        pub class: Option<String>,
        /// Trigger children view factory.
        pub children: Box<dyn Fn() -> View + Send + Sync>,
    }

    impl Default for CollapsibleTriggerProps {
        fn default() -> Self {
            Self {
                state: Arc::new(CollapsibleRegionState::uncontrolled(false)),
                options: CollapsibleTriggerOptions::default(),
                automation_fallback: "rustic-ui::collapsible-trigger".into(),
                class: None,
                children: Box::new(|| View::empty()),
            }
        }
    }

    impl PartialEq for CollapsibleTriggerProps {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.class == other.class
        }
    }

    #[component]
    pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> impl IntoView {
        let attrs = props.state.trigger_attributes();
        let attrs = super::apply_trigger_options(attrs, &props.options);
        let pairs = super::collapsible_trigger_attributes(
            attrs,
            &props.options,
            &props.automation_fallback,
        );
        let themed = crate::style_helpers::themed_class(super::trigger_style());
        let class = props
            .class
            .as_ref()
            .map(|custom| format!("{themed} {custom}"))
            .unwrap_or(themed);
        let mut element = leptos::html::button().attr("type", "button");
        element = element.class(class);
        for (key, value) in pairs {
            element = element.attr(key, value);
        }
        element.child((props.children)()).into_view()
    }

    /// Leptos variant of the collapsible region wrapper.
    #[derive(Clone)]
    pub struct CollapsibleRegionProps {
        /// Shared region state machine.
        pub state: Arc<CollapsibleRegionState>,
        /// Attribute overrides.
        pub options: CollapsibleRegionOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Optional custom class appended to the themed baseline.
        pub class: Option<String>,
        /// Region children view factory.
        pub children: Box<dyn Fn() -> View + Send + Sync>,
    }

    impl Default for CollapsibleRegionProps {
        fn default() -> Self {
            Self {
                state: Arc::new(CollapsibleRegionState::uncontrolled(false)),
                options: CollapsibleRegionOptions::default(),
                automation_fallback: "rustic-ui::collapsible-region".into(),
                class: None,
                children: Box::new(|| View::empty()),
            }
        }
    }

    impl PartialEq for CollapsibleRegionProps {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.class == other.class
        }
    }

    #[component]
    pub fn CollapsibleRegion(props: CollapsibleRegionProps) -> impl IntoView {
        let attrs = props.state.region_attributes();
        let attrs = super::apply_region_options(attrs, &props.options);
        let pairs =
            super::collapsible_region_attributes(attrs, &props.options, &props.automation_fallback);
        let themed = crate::style_helpers::themed_class(super::region_style());
        let class = props
            .class
            .as_ref()
            .map(|custom| format!("{themed} {custom}"))
            .unwrap_or(themed);
        let mut element = leptos::html::div().class(class);
        for (key, value) in pairs {
            element = element.attr(key, value);
        }
        element.child((props.children)()).into_view()
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{
    CollapsibleRegion, CollapsibleRegionProps, CollapsibleTrigger, CollapsibleTriggerProps,
};

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus trigger renderer.
    #[derive(Clone, PartialEq)]
    pub struct CollapsibleTriggerProps {
        /// Region state machine mirrored from the controller.
        pub state: CollapsibleRegionState,
        /// Attribute overrides.
        pub options: CollapsibleTriggerOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Trigger children HTML.
        pub children: String,
    }

    impl Default for CollapsibleTriggerProps {
        fn default() -> Self {
            Self {
                state: CollapsibleRegionState::uncontrolled(false),
                options: CollapsibleTriggerOptions::default(),
                automation_fallback: "rustic-ui::collapsible-trigger".into(),
                children: String::new(),
            }
        }
    }

    /// Render the trigger into HTML.
    pub fn render_trigger(props: &CollapsibleTriggerProps) -> String {
        super::render_collapsible_trigger_html(
            &props.state,
            &props.options,
            &props.automation_fallback,
            &props.children,
        )
    }

    /// Properties consumed by the Dioxus region renderer.
    #[derive(Clone, PartialEq)]
    pub struct CollapsibleRegionProps {
        /// Region state machine mirrored from the controller.
        pub state: CollapsibleRegionState,
        /// Attribute overrides.
        pub options: CollapsibleRegionOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Region children HTML.
        pub children: String,
    }

    impl Default for CollapsibleRegionProps {
        fn default() -> Self {
            Self {
                state: CollapsibleRegionState::uncontrolled(false),
                options: CollapsibleRegionOptions::default(),
                automation_fallback: "rustic-ui::collapsible-region".into(),
                children: String::new(),
            }
        }
    }

    /// Render the region into HTML.
    pub fn render_region(props: &CollapsibleRegionProps) -> String {
        super::render_collapsible_region_html(
            &props.state,
            &props.options,
            &props.automation_fallback,
            &props.children,
        )
    }
}

// ---------------------------------------------------------------------------
// Sycamore adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore trigger renderer properties.
    #[derive(Clone, PartialEq)]
    pub struct CollapsibleTriggerProps {
        /// Region state machine mirrored from the controller.
        pub state: CollapsibleRegionState,
        /// Attribute overrides.
        pub options: CollapsibleTriggerOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Trigger children HTML.
        pub children: String,
    }

    impl Default for CollapsibleTriggerProps {
        fn default() -> Self {
            Self {
                state: CollapsibleRegionState::uncontrolled(false),
                options: CollapsibleTriggerOptions::default(),
                automation_fallback: "rustic-ui::collapsible-trigger".into(),
                children: String::new(),
            }
        }
    }

    /// Render the trigger into HTML.
    pub fn render_trigger(props: &CollapsibleTriggerProps) -> String {
        super::render_collapsible_trigger_html(
            &props.state,
            &props.options,
            &props.automation_fallback,
            &props.children,
        )
    }

    /// Sycamore region renderer properties.
    #[derive(Clone, PartialEq)]
    pub struct CollapsibleRegionProps {
        /// Region state machine mirrored from the controller.
        pub state: CollapsibleRegionState,
        /// Attribute overrides.
        pub options: CollapsibleRegionOptions,
        /// Fallback automation identifier.
        pub automation_fallback: String,
        /// Region children HTML.
        pub children: String,
    }

    impl Default for CollapsibleRegionProps {
        fn default() -> Self {
            Self {
                state: CollapsibleRegionState::uncontrolled(false),
                options: CollapsibleRegionOptions::default(),
                automation_fallback: "rustic-ui::collapsible-region".into(),
                children: String::new(),
            }
        }
    }

    /// Render the region into HTML.
    pub fn render_region(props: &CollapsibleRegionProps) -> String {
        super::render_collapsible_region_html(
            &props.state,
            &props.options,
            &props.automation_fallback,
            &props.children,
        )
    }
}
