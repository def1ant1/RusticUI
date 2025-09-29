//! Renderer for the responsive [`HiddenState`](rustic_ui_headless::hidden::HiddenState).
//!
//! Enterprise applications often hide layouts responsively while still exposing
//! them to assistive technologies.  Centralising the translation from the
//! headless `HiddenState` into deterministic CSS variables ensures every
//! framework adapter (React, Yew, Leptos, etc.) preserves the exact same SSR
//! footprint which keeps hydration and automated screenshot pipelines aligned.

use std::collections::BTreeMap;

use rustic_ui_headless::hidden::{HiddenEvaluation, HiddenState};

use crate::render_helpers::{
    bool_to_css_flag, collect_responsive_variables, css_variables_to_style, visibility_to_display,
    visibility_to_visibility, CssVariableMap,
};

/// Output returned by [`render_hidden`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl HiddenRenderOutput {
    /// Access the generated CSS variable map so adapters can forward it to
    /// CSS-in-Rust abstractions or attribute builders.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Serialised inline style string suitable for SSR attribute injection.
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Render the provided [`HiddenState`] into deterministic CSS variables and an
/// inline style string.  The renderer walks every breakpoint configured on the
/// state machine and stores visibility, ARIA and inert flags under the
/// `--rustic_ui_hidden_*` namespace.
#[must_use]
pub fn render_hidden(state: &HiddenState) -> HiddenRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "hidden",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            hidden_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    HiddenRenderOutput {
        css_variables,
        inline_style,
    }
}

fn hidden_tokens_for_breakpoint(evaluation: &HiddenEvaluation) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(6);

    tokens.push(("display", visibility_to_display(evaluation.hidden)));
    tokens.push(("visibility", visibility_to_visibility(evaluation.hidden)));
    tokens.push(("aria_hidden", bool_to_css_flag(evaluation.hidden)));
    tokens.push(("role", evaluation.role.as_str().to_string()));
    tokens.push(("inert", bool_to_css_flag(evaluation.inert)));

    if matches!(
        evaluation.breakpoint,
        rustic_ui_headless::layout::Breakpoint::Base
    ) {
        tokens.push((
            "active_breakpoint",
            evaluation.breakpoint.as_token().to_string(),
        ));
    }

    tokens
}

/// Adapter props shared across framework specific shims.
#[derive(Clone, Copy, Debug)]
pub struct HiddenAdapterProps<'a> {
    /// Headless hidden state driving responsive visibility.
    pub state: &'a HiddenState,
}

impl<'a> HiddenAdapterProps<'a> {
    /// Convenience constructor so integrations can write `HiddenAdapterProps::new(&state)`.
    #[inline]
    pub fn new(state: &'a HiddenState) -> Self {
        Self { state }
    }
}

/// Internal helper leveraged by the framework adapters.  The indirection keeps
/// the adapter shims tiny while ensuring every runtime delegates to the shared
/// renderer.
#[cfg_attr(
    not(any(
        feature = "react",
        feature = "yew",
        feature = "leptos",
        feature = "dioxus",
        feature = "sycamore"
    )),
    allow(dead_code)
)]
fn render_hidden_with_props(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
    render_hidden(props.state)
}

/// React adapter invoking [`render_hidden`].
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Execute the canonical renderer during React's render lifecycle so SSR and
    /// hydration both observe identical inline style payloads.
    pub fn render(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
        super::render_hidden_with_props(props)
    }
}

/// Yew adapter exposing the same renderer behind the `yew` feature flag.
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Produce CSS variables for Yew based applications.  Controlled and
    /// uncontrolled flows both delegate to the shared renderer which keeps
    /// automation hooks consistent across runtimes.
    pub fn render(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
        super::render_hidden_with_props(props)
    }
}

/// Leptos adapter bridging signal driven updates to the canonical renderer.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Feed the [`HiddenState`] from Leptos signals into the shared renderer so
    /// SSR streams and client hydration stay perfectly aligned.
    pub fn render(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
        super::render_hidden_with_props(props)
    }
}

/// Dioxus adapter forwarding to [`render_hidden`].
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Dioxus orchestrators call this helper to obtain deterministic CSS
    /// variables before streaming markup or mounting on the client.
    pub fn render(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
        super::render_hidden_with_props(props)
    }
}

/// Sycamore adapter mirroring the other framework integrations.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Whether the [`HiddenState`] lives inside a Sycamore signal or plain Rust
    /// struct, this helper returns the canonical CSS variables for SSR.
    pub fn render(props: HiddenAdapterProps<'_>) -> HiddenRenderOutput {
        super::render_hidden_with_props(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::hidden::{HiddenRole, HiddenState};
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

    #[test]
    fn hidden_renderer_serialises_visibility_flags() {
        let state = HiddenState::new(
            ResponsiveValue::new(false)
                .with_override(Breakpoint::Md, true)
                .with_override(Breakpoint::Xl, false),
            BreakpointConfig::material(),
        )
        .with_role(HiddenRole::Group)
        .inert(true);

        let output = render_hidden(&state);

        assert_eq!(
            output
                .css_variables()
                .get("--rustic_ui_hidden_visibility")
                .map(String::as_str),
            Some("visible"),
        );
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_hidden_visibility-md"));
        assert!(output
            .inline_style()
            .contains("--rustic_ui_hidden_aria_hidden"));
    }

    #[test]
    fn adapter_delegates_to_shared_renderer() {
        let state = HiddenState::new(ResponsiveValue::new(true), BreakpointConfig::material());

        let base = render_hidden(&state);
        let adapter = super::render_hidden_with_props(HiddenAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
