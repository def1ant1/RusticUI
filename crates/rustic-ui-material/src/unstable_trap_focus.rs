#![deny(missing_docs)]
//! Material renderers for the experimental focus trap instrumentation.
//!
//! The helpers in this module mirror [`crate::focus_trap`] but accept the
//! telemetry-enhanced [`UnstableFocusTrapState`]
//! (`rustic_ui_headless::unstable_trap_focus`).  While the API remains gated
//! behind the `unstable` feature flag, adapters can observe how often keyboard
//! users wrap between sentinels and feed the data into automation pipelines
//! without rewriting markup across frameworks.
//!
//! The additional attributes are intentionally verbose:
//! - `data-rustic-focus-loop-count` exposes a monotonically increasing counter
//!   so QA tooling can assert how frequently loops occur while end-to-end tests
//!   exercise overlays.
//! - `data-rustic-focus-loop-last-direction` indicates whether the latest loop
//!   happened while moving forward or backward, simplifying analytics queries.
//! - `aria-roledescription="focus trap instrumentation sentinel"` documents to
//!   assistive tech that the node exists purely for focus management research.
//!
//! As the API stabilizes the data attributes may migrate into the stable focus
//! trap module, so downstream integrations should avoid hard-coding selectors
//! against the `unstable` names.

use crate::focus_trap::{
    focus_trap_sentinel_attributes, sentinel_style, FocusTrapSentinelKind, FocusTrapSentinelOptions,
};
use rustic_ui_headless::unstable_trap_focus::UnstableFocusTrapState;

/// Build automation-friendly attribute pairs for an unstable focus trap sentinel.
#[must_use]
pub fn unstable_focus_trap_sentinel_attributes(
    state: &UnstableFocusTrapState,
    kind: FocusTrapSentinelKind,
    options: &FocusTrapSentinelOptions,
    fallback_prefix: &str,
) -> Vec<(String, String)> {
    let attrs = match kind {
        FocusTrapSentinelKind::Start => state.start_sentinel_attributes(),
        FocusTrapSentinelKind::End => state.end_sentinel_attributes(),
    };
    let mut pairs = focus_trap_sentinel_attributes(attrs, kind, options, fallback_prefix);
    pairs.push((
        "data-rustic-focus-loop-count".into(),
        state.loop_event_count().to_string(),
    ));
    if let Some(event) = state.last_loop_event() {
        pairs.push((
            "data-rustic-focus-loop-last-direction".into(),
            event.direction.as_str().into(),
        ));
    }
    pairs
}

/// Render a sentinel as HTML including instrumentation-specific attributes.
#[must_use]
pub fn render_unstable_focus_trap_sentinel_html(
    state: &UnstableFocusTrapState,
    kind: FocusTrapSentinelKind,
    options: &FocusTrapSentinelOptions,
    fallback_prefix: &str,
) -> String {
    let mut pairs = unstable_focus_trap_sentinel_attributes(state, kind, options, fallback_prefix);
    pairs.push(("tabindex".into(), "0".into()));
    pairs.push(("aria-hidden".into(), "true".into()));
    pairs.push((
        "aria-roledescription".into(),
        "focus trap instrumentation sentinel".into(),
    ));
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

    /// Yew component rendering an unstable focus trap sentinel.
    #[derive(Properties, Clone)]
    pub struct UnstableFocusTrapSentinelProps {
        /// Shared experimental focus trap state machine managed by the overlay controller.
        pub state: Rc<UnstableFocusTrapState>,
        /// Whether this sentinel sits before or after the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix. When omitted the dialog prefix is reused.
        #[prop_or_default]
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix used when `options.automation_prefix` is empty.
        #[prop_or_else(|| AttrValue::from("dialog"))]
        pub fallback_prefix: AttrValue,
    }

    impl PartialEq for UnstableFocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.state, &other.state)
                && self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    #[function_component(UnstableFocusTrapSentinel)]
    pub fn unstable_focus_trap_sentinel(props: &UnstableFocusTrapSentinelProps) -> Html {
        let pairs = unstable_focus_trap_sentinel_attributes(
            &props.state,
            props.kind,
            &props.options,
            props.fallback_prefix.as_str(),
        );
        let mut node = html! { <span></span> };
        if let Html::VTag(ref mut tag) = node {
            tag.add_attribute(
                "class",
                crate::style_helpers::themed_class(sentinel_style()),
            );
            tag.add_attribute("tabindex", "0");
            tag.add_attribute("aria-hidden", "true");
            tag.add_attribute(
                "aria-roledescription",
                "focus trap instrumentation sentinel",
            );
            for (key, value) in pairs {
                tag.add_attribute(key, value);
            }
        }
        node
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{UnstableFocusTrapSentinel, UnstableFocusTrapSentinelProps};

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::prelude::*;
    use std::sync::Arc;

    /// Leptos component mirroring the Yew API for the unstable sentinel.
    #[derive(Clone)]
    pub struct UnstableFocusTrapSentinelProps {
        /// Shared experimental focus trap state machine.
        pub state: Arc<UnstableFocusTrapState>,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl Default for UnstableFocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: Arc::new(UnstableFocusTrapState::new(true)),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    impl PartialEq for UnstableFocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.state, &other.state)
                && self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    #[component]
    pub fn UnstableFocusTrapSentinel(props: UnstableFocusTrapSentinelProps) -> impl IntoView {
        let pairs = unstable_focus_trap_sentinel_attributes(
            &props.state,
            props.kind,
            &props.options,
            &props.fallback_prefix,
        );
        let mut element = leptos::html::span().attr("tabindex", "0");
        element = element.attr("aria-hidden", "true");
        element = element.attr(
            "aria-roledescription",
            "focus trap instrumentation sentinel",
        );
        element = element.class(crate::style_helpers::themed_class(sentinel_style()));
        for (key, value) in pairs {
            element = element.attr(key, value);
        }
        element.into_view()
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{UnstableFocusTrapSentinel, UnstableFocusTrapSentinelProps};

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties for the Dioxus sentinel renderer.
    #[derive(Clone)]
    pub struct UnstableFocusTrapSentinelProps {
        /// Focus trap state machine mirrored from the controller.
        pub state: UnstableFocusTrapState,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl PartialEq for UnstableFocusTrapSentinelProps {
        fn eq(&self, other: &Self) -> bool {
            self.kind == other.kind
                && self.options == other.options
                && self.fallback_prefix == other.fallback_prefix
        }
    }

    impl Default for UnstableFocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: UnstableFocusTrapState::new(true),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    /// Render the sentinel into HTML.
    pub fn render(props: &UnstableFocusTrapSentinelProps) -> String {
        render_unstable_focus_trap_sentinel_html(
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
    pub struct UnstableFocusTrapSentinelProps {
        /// Focus trap state machine mirrored from the controller.
        pub state: UnstableFocusTrapState,
        /// Sentinel position relative to the tabbable content.
        pub kind: FocusTrapSentinelKind,
        /// Optional automation prefix overriding the fallback.
        pub options: FocusTrapSentinelOptions,
        /// Fallback automation prefix when the options omit one.
        pub fallback_prefix: String,
    }

    impl Default for UnstableFocusTrapSentinelProps {
        fn default() -> Self {
            Self {
                state: UnstableFocusTrapState::new(true),
                kind: FocusTrapSentinelKind::Start,
                options: FocusTrapSentinelOptions::default(),
                fallback_prefix: "dialog".into(),
            }
        }
    }

    /// Render the sentinel into HTML.
    pub fn render(props: &UnstableFocusTrapSentinelProps) -> String {
        render_unstable_focus_trap_sentinel_html(
            &props.state,
            props.kind,
            &props.options,
            &props.fallback_prefix,
        )
    }
}
