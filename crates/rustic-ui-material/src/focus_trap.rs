#![deny(missing_docs)]
//! Focus trap renderers that decorate the headless [`FocusTrapState`]
//! with automation identifiers and framework adapters. The helpers
//! intentionally mirror the ergonomics of [`dialog`](crate::dialog)
//! because most enterprise overlays compose a dialog surface with
//! sentinels on either side to keep keyboard users within the modal
//! while telemetry systems observe focus churn.

use rustic_ui_headless::focus_trap::{FocusTrapSentinelAttributes, FocusTrapState};
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Enumerates the sentinel nodes that bookend a focus trap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusTrapSentinelKind {
    /// Sentinel rendered before the tabbable content.
    Start,
    /// Sentinel rendered after the tabbable content.
    End,
}

impl FocusTrapSentinelKind {
    fn automation_suffix(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Optional overrides shared by both sentinels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FocusTrapSentinelOptions {
    /// Optional automation prefix mirrored to `data-automation-id`.
    pub automation_prefix: Option<String>,
}

/// Convert a sentinel attribute builder into automation-friendly pairs.
#[must_use]
pub fn focus_trap_sentinel_attributes(
    attrs: FocusTrapSentinelAttributes<'_>,
    kind: FocusTrapSentinelKind,
    options: &FocusTrapSentinelOptions,
    fallback_prefix: &str,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(3);
    let (key, value) = attrs.controller_attribute();
    pairs.push((key.into(), value));
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    let prefix = options
        .automation_prefix
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback_prefix);
    pairs.push((
        "data-automation-id".into(),
        format!("{prefix}::focus-trap-{}", kind.automation_suffix()),
    ));
    pairs
}

fn sentinel_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        width: 1px;
        height: 1px;
        margin: -1px;
        padding: 0;
        border: 0;
        clip: rect(0, 0, 0, 0);
        overflow: hidden;
        /* Sentinels should never steal layout real estate yet must remain */
        /* discoverable for analytics, hence we isolate them visually. */
        &[data-automation-id] { opacity: 0; }
        "#
    )
}

/// Render a sentinel as HTML so SSR retains the automation markers.
#[must_use]
pub fn render_focus_trap_sentinel_html(
    state: &FocusTrapState,
    kind: FocusTrapSentinelKind,
    options: &FocusTrapSentinelOptions,
    fallback_prefix: &str,
) -> String {
    let attrs = match kind {
        FocusTrapSentinelKind::Start => state.start_sentinel_attributes(),
        FocusTrapSentinelKind::End => state.end_sentinel_attributes(),
    };
    let pairs = focus_trap_sentinel_attributes(attrs, kind, options, fallback_prefix);
    crate::render_helpers::render_element_html("span", sentinel_style(), pairs, "")
}

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use std::rc::Rc;
    use yew::prelude::*;

    /// Yew component rendering a focus trap sentinel.
    #[derive(Properties, Clone)]
    pub struct FocusTrapSentinelProps {
        /// Shared focus trap state machine. The hosting overlay owns lifecycle
        /// transitions (registering focus, rebuilding node lists, etc.) while
        /// this adapter mirrors the analytics attributes.
        pub state: Rc<FocusTrapState>,
        /// Whether this sentinel sits before or after the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix. When omitted the component falls back to
        /// the dialog automation prefix so telemetry streams remain stable.
        #[prop_or_default]
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix used when `options.automation_prefix` is empty.
        #[prop_or_else(|| AttrValue::from("dialog"))]
        pub fallback_prefix: AttrValue,
    }

    impl PartialEq for FocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.state, &other.state)
                && self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    #[function_component(FocusTrapSentinel)]
    pub fn focus_trap_sentinel(props: &FocusTrapSentinelProps) -> Html {
        // Sentinels never subscribe to DOM events directly. Instead the overlay
        // listens for focus/keyboard changes and mutates `FocusTrapState` which
        // keeps teardown predictable during dialog unmounts.
        let attrs = match props.kind {
            FocusTrapSentinelKind::Start => props.state.start_sentinel_attributes(),
            FocusTrapSentinelKind::End => props.state.end_sentinel_attributes(),
        };
        let pairs = focus_trap_sentinel_attributes(
            attrs,
            props.kind,
            &props.options,
            props.fallback_prefix.as_str(),
        );
        let mut node = html! { <span tabindex="0" aria-hidden="true"></span> };
        if let Html::VTag(ref mut tag) = node {
            tag.add_attribute(
                "class",
                crate::style_helpers::themed_class(sentinel_style()),
            );
            for (key, value) in pairs {
                tag.add_attribute(key, value);
            }
        }
        node
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{FocusTrapSentinel, FocusTrapSentinelProps};

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::prelude::*;
    use std::sync::Arc;

    /// Leptos sentinel component mirroring the Yew API surface.
    #[derive(Clone)]
    pub struct FocusTrapSentinelProps {
        /// Shared focus trap state machine managed by the overlay controller.
        pub state: Arc<FocusTrapState>,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl Default for FocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: Arc::new(FocusTrapState::new(true)),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    impl PartialEq for FocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
                && self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    #[component]
    pub fn FocusTrapSentinel(props: FocusTrapSentinelProps) -> impl IntoView {
        let attrs = match props.kind {
            FocusTrapSentinelKind::Start => props.state.start_sentinel_attributes(),
            FocusTrapSentinelKind::End => props.state.end_sentinel_attributes(),
        };
        let pairs = focus_trap_sentinel_attributes(
            attrs,
            props.kind,
            &props.options,
            &props.fallback_prefix,
        );
        let mut element = leptos::html::span().attr("tabindex", "0");
        element = element.attr("aria-hidden", "true");
        element = element.class(crate::style_helpers::themed_class(sentinel_style()));
        for (key, value) in pairs {
            element = element.attr(key, value);
        }
        element.into_view()
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{FocusTrapSentinel, FocusTrapSentinelProps};

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties for the Dioxus sentinel renderer.
    #[derive(Clone)]
    pub struct FocusTrapSentinelProps {
        /// Focus trap state machine mirrored from the controller.
        pub state: FocusTrapState,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl PartialEq for FocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    impl Default for FocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: FocusTrapState::new(true),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    /// Render the sentinel into HTML.
    pub fn render(props: &FocusTrapSentinelProps) -> String {
        render_focus_trap_sentinel_html(
            &props.state,
            props.kind,
            &props.options,
            &props.fallback_prefix,
        )
    }
}

// ---------------------------------------------------------------------------
// Sycamore adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Properties for the Sycamore sentinel renderer mirroring the Dioxus API.
    #[derive(Clone, PartialEq)]
    pub struct FocusTrapSentinelProps {
        /// Focus trap state machine mirrored from the controller.
        pub state: FocusTrapState,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl Default for FocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: FocusTrapState::new(true),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    /// Render the sentinel into HTML.
    pub fn render(props: &FocusTrapSentinelProps) -> String {
        render_focus_trap_sentinel_html(
            &props.state,
            props.kind,
            &props.options,
            &props.fallback_prefix,
        )
    }
}
