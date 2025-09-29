#![deny(missing_docs)]
//! Click-away renderer primitives that wrap [`ClickAwayState`]
//! instances from `rustic_ui_headless` and expose framework
//! adapters mirroring the ergonomics of [`dialog`](crate::dialog)
//! and [`drawer`](crate::drawer).
//!
//! The goal of this module is to make it effortless to bolt the
//! pointer/focus detection logic onto existing surfaces without
//! duplicating orchestration code. Every helper funnels through
//! automation-friendly attribute builders so analytics pipelines,
//! QA harnesses, and distributed telemetry collectors observe the
//! exact same identifiers regardless of whether the consumer is
//! running Yew, Leptos, Dioxus, Sycamore, or a server-side renderer.

use crate::telemetry::{instrument_render, TelemetryContext, TelemetryHooks};
use rustic_ui_headless::click_away::{ClickAwayRootAttributes, ClickAwayState};
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Declarative overrides for the root boundary exposed by [`ClickAwayState`].
///
/// The struct intentionally mirrors the ergonomics of
/// [`DialogSurfaceOptions`](crate::dialog::DialogSurfaceOptions) so the same
/// orchestration layers can be re-used for modals, drawers, menus and any other
/// overlay that relies on click-away dismissal.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClickAwayBoundaryOptions {
    /// Optional DOM identifier applied to the boundary element.
    pub id: Option<String>,
    /// Optional analytics marker propagated to `data-rustic-analytics-id`.
    pub analytics_id: Option<String>,
    /// Optional automation identifier surfaced via `data-automation-id`.
    pub automation_id: Option<String>,
}

fn merge_boundary_options(
    options: &ClickAwayBoundaryOptions,
    telemetry: &TelemetryHooks,
) -> ClickAwayBoundaryOptions {
    let mut merged = options.clone();
    if merged.analytics_id.is_none() {
        merged.analytics_id = telemetry.analytics_id.clone();
    }
    if merged.automation_id.is_none() {
        merged.automation_id = telemetry.automation_id.clone();
    }
    merged
}

fn resolved_automation_id(options: &ClickAwayBoundaryOptions, fallback: &str) -> String {
    options
        .automation_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| fallback.to_string())
}

fn apply_boundary_options<'a>(
    mut attrs: ClickAwayRootAttributes<'a>,
    options: &'a ClickAwayBoundaryOptions,
) -> ClickAwayRootAttributes<'a> {
    if let Some(id) = &options.id {
        attrs = attrs.id(id);
    }
    if let Some(analytics) = &options.analytics_id {
        attrs = attrs.analytics_id(analytics);
    }
    attrs
}

/// Convert the root attribute builder into automation-friendly key/value pairs.
///
/// * `automation_fallback` lets orchestration layers derive deterministic
///   automation identifiers when none are supplied via [`ClickAwayBoundaryOptions`].
/// * The helper preserves the canonical `data-rustic-click-away` tuple emitted by
///   the headless state so centralised event listeners continue to function.
#[must_use]
pub fn click_away_root_attributes(
    attrs: ClickAwayRootAttributes<'_>,
    automation_fallback: &str,
    options: &ClickAwayBoundaryOptions,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(4);
    let (controller_key, controller_value) = attrs.controller_attribute();
    pairs.push((controller_key.into(), controller_value.into()));
    if let Some((key, value)) = attrs.id_attribute() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    let automation_id = options
        .automation_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or(automation_fallback);
    pairs.push(("data-automation-id".into(), automation_id.to_string()));
    pairs
}

/// Render the click-away boundary into HTML so SSR frameworks can reuse the
/// exact same automation identifiers as their CSR counterparts.
#[must_use]
pub fn render_click_away_boundary_html(
    state: &ClickAwayState,
    options: &ClickAwayBoundaryOptions,
    automation_fallback: &str,
    children: &str,
) -> String {
    let attrs = state.root_attributes();
    let attrs = apply_boundary_options(attrs, options);
    let pairs = click_away_root_attributes(attrs, automation_fallback, options);
    crate::render_helpers::render_element_html("div", boundary_style(), pairs, children)
}

fn boundary_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: block;
        isolation: isolate;
        &[data-automation-id]{
            /* Provide a stable hook for scripted telemetry collectors */
            contain: layout paint;
        }
        "#
    )
}

/// Derive a deterministic automation identifier for dialog overlays.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
    feature = "react",
))]
#[must_use]
pub fn dialog_click_away_automation(surface: &crate::dialog::DialogSurfaceOptions) -> String {
    surface
        .analytics_id
        .as_deref()
        .map(|id| format!("dialog::{id}::click-away"))
        .unwrap_or_else(|| "dialog::click-away".into())
}

/// Derive a deterministic automation identifier for drawer overlays.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
    feature = "react",
))]
#[must_use]
pub fn drawer_click_away_automation(props: &crate::drawer::DrawerProps<'_>) -> String {
    props
        .on_toggle_event
        .map(|event| format!("drawer::{event}::click-away"))
        .unwrap_or_else(|| "drawer::click-away".into())
}

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
pub mod yew {
    use super::*;
    use std::rc::Rc;
    use yew::prelude::*;

    /// Yew component that decorates the subtree with click-away telemetry
    /// attributes while delegating actual event handling to the shared
    /// orchestration layer.
    #[derive(Properties, Clone)]
    pub struct ClickAwayBoundaryProps {
        /// Shared state machine driving the click-away lifecycle. The caller is
        /// responsible for mutating the state in response to pointer/focus
        /// events; this component only renders attributes and automation hooks.
        pub state: Rc<ClickAwayState>,
        /// Optional attribute overrides controlling id/analytics/automation ids.
        #[prop_or_default]
        pub options: ClickAwayBoundaryOptions,
        /// Deterministic fallback automation identifier when `options` omit it.
        #[prop_or_else(|| AttrValue::from("rustic-ui::click-away"))]
        pub automation_fallback: AttrValue,
        /// Render subtree managed by the click-away detector.
        #[prop_or_default]
        pub children: Children,
        /// Optional telemetry hooks executed around render.
        #[prop_or_default]
        pub telemetry: TelemetryHooks,
    }

    impl PartialEq for ClickAwayBoundaryProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.children == other.children
                && self.telemetry == other.telemetry
        }
    }

    #[function_component(ClickAwayBoundary)]
    pub fn click_away_boundary(props: &ClickAwayBoundaryProps) -> Html {
        let options = merge_boundary_options(&props.options, &props.telemetry);
        let automation = resolved_automation_id(&options, props.automation_fallback.as_str());
        let context =
            TelemetryContext::new("rustic_ui_material::click_away::yew::ClickAwayBoundary")
                .with_analytics(options.analytics_id.clone())
                .with_automation(Some(automation));
        instrument_render(&props.telemetry, context, || {
            let attrs = props.state.root_attributes();
            let attrs = apply_boundary_options(attrs, &options);
            let pairs =
                click_away_root_attributes(attrs, props.automation_fallback.as_str(), &options);
            let mut node = html! { <div>{ for props.children.iter() }</div> };
            if let Html::VTag(ref mut tag) = node {
                for (key, value) in pairs {
                    tag.add_attribute(key, value);
                }
                tag.add_attribute(
                    "class",
                    crate::style_helpers::themed_class(boundary_style()),
                );
            }
            node
        })
    }
}

#[cfg(feature = "yew")]
pub use yew::{ClickAwayBoundary, ClickAwayBoundaryProps};

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;
    use leptos::prelude::*;
    use std::sync::Arc;

    /// Leptos adapter exposing identical knobs as the Yew variant while
    /// returning a fully composed `View` so SSR/CSR remain consistent.
    #[derive(Clone)]
    pub struct ClickAwayBoundaryProps {
        /// Shared click-away state machine tracked by the hosting application.
        pub state: Arc<ClickAwayState>,
        /// Optional attribute overrides controlling ids and analytics markers.
        pub options: ClickAwayBoundaryOptions,
        /// Fallback automation identifier when none is supplied in `options`.
        pub automation_fallback: String,
        /// Child nodes rendered inside the boundary.
        pub children: Box<dyn Fn() -> View + Send + Sync>,
        /// Optional telemetry hooks executed around render.
        pub telemetry: TelemetryHooks,
    }

    impl Default for ClickAwayBoundaryProps {
        fn default() -> Self {
            Self {
                state: Arc::new(ClickAwayState::new()),
                options: ClickAwayBoundaryOptions::default(),
                automation_fallback: "rustic-ui::click-away".into(),
                children: Box::new(|| View::empty()),
                telemetry: TelemetryHooks::default(),
            }
        }
    }

    impl PartialEq for ClickAwayBoundaryProps {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
                && self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.telemetry == other.telemetry
        }
    }

    #[component]
    pub fn ClickAwayBoundary(props: ClickAwayBoundaryProps) -> impl IntoView {
        let options = merge_boundary_options(&props.options, &props.telemetry);
        let automation = resolved_automation_id(&options, props.automation_fallback.as_str());
        let context =
            TelemetryContext::new("rustic_ui_material::click_away::leptos::ClickAwayBoundary")
                .with_analytics(options.analytics_id.clone())
                .with_automation(Some(automation));
        instrument_render(&props.telemetry, context, || {
            let attrs = props.state.root_attributes();
            let attrs = apply_boundary_options(attrs, &options);
            let pairs =
                click_away_root_attributes(attrs, props.automation_fallback.as_str(), &options);
            let mut element = leptos::html::div();
            element = element.class(crate::style_helpers::themed_class(boundary_style()));
            for (key, value) in pairs {
                element = element.attr(key, value);
            }
            element.child((props.children)()).into_view()
        })
    }
}

#[cfg(feature = "leptos")]
pub use leptos::{ClickAwayBoundary, ClickAwayBoundaryProps};

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus renderer.
    #[derive(Clone)]
    pub struct ClickAwayBoundaryProps {
        /// Click-away state machine mirrored from the application layer.
        pub state: ClickAwayState,
        /// Declarative overrides for analytics/automation metadata.
        pub options: ClickAwayBoundaryOptions,
        /// Fallback automation identifier when the options omit one.
        pub automation_fallback: String,
        /// Serialized children rendered inside the boundary.
        pub children: String,
        /// Optional telemetry hooks executed around render.
        pub telemetry: TelemetryHooks,
    }

    impl PartialEq for ClickAwayBoundaryProps {
        fn eq(&self, other: &Self) -> bool {
            self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.children == other.children
                && self.telemetry == other.telemetry
        }
    }

    impl Default for ClickAwayBoundaryProps {
        fn default() -> Self {
            Self {
                state: ClickAwayState::new(),
                options: ClickAwayBoundaryOptions::default(),
                automation_fallback: "rustic-ui::click-away".into(),
                children: String::new(),
                telemetry: TelemetryHooks::default(),
            }
        }
    }

    /// Render the boundary into HTML so Dioxus SSR and client renderers stay in
    /// lockstep with the Yew/Leptos adapters.
    pub fn render(props: &ClickAwayBoundaryProps) -> String {
        let options = merge_boundary_options(&props.options, &props.telemetry);
        let automation = resolved_automation_id(&options, props.automation_fallback.as_str());
        let context =
            TelemetryContext::new("rustic_ui_material::click_away::dioxus::ClickAwayBoundary")
                .with_analytics(options.analytics_id.clone())
                .with_automation(Some(automation));
        instrument_render(&props.telemetry, context, || {
            render_click_away_boundary_html(
                &props.state,
                &options,
                props.automation_fallback.as_str(),
                &props.children,
            )
        })
    }
}

// ---------------------------------------------------------------------------
// Sycamore adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore renderer mirroring the Dioxus properties for API consistency.
    #[derive(Clone)]
    pub struct ClickAwayBoundaryProps {
        /// Click-away state machine mirrored from the application layer.
        pub state: ClickAwayState,
        /// Declarative overrides for analytics/automation metadata.
        pub options: ClickAwayBoundaryOptions,
        /// Fallback automation identifier when the options omit one.
        pub automation_fallback: String,
        /// Serialized children rendered inside the boundary.
        pub children: String,
        /// Optional telemetry hooks executed around render.
        pub telemetry: TelemetryHooks,
    }

    impl PartialEq for ClickAwayBoundaryProps {
        fn eq(&self, other: &Self) -> bool {
            self.options == other.options
                && self.automation_fallback == other.automation_fallback
                && self.children == other.children
                && self.telemetry == other.telemetry
        }
    }

    impl Default for ClickAwayBoundaryProps {
        fn default() -> Self {
            Self {
                state: ClickAwayState::new(),
                options: ClickAwayBoundaryOptions::default(),
                automation_fallback: "rustic-ui::click-away".into(),
                children: String::new(),
                telemetry: TelemetryHooks::default(),
            }
        }
    }

    /// Render the boundary into HTML string form.
    pub fn render(props: &ClickAwayBoundaryProps) -> String {
        let options = merge_boundary_options(&props.options, &props.telemetry);
        let automation = resolved_automation_id(&options, props.automation_fallback.as_str());
        let context =
            TelemetryContext::new("rustic_ui_material::click_away::sycamore::ClickAwayBoundary")
                .with_analytics(options.analytics_id.clone())
                .with_automation(Some(automation));
        instrument_render(&props.telemetry, context, || {
            render_click_away_boundary_html(
                &props.state,
                &options,
                props.automation_fallback.as_str(),
                &props.children,
            )
        })
    }
}
