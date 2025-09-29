//! Renderer for [`DividerState`](rustic_ui_headless::divider::DividerState).
//!
//! Dividers expose orientation, thickness and inset tokens.  Rendering the
//! responsive evaluation into CSS variables ensures consistency across server
//! and client adapters without duplicating formatting logic.

use std::collections::BTreeMap;

use rustic_ui_headless::divider::{DividerEvaluation, DividerState};

use crate::render_helpers::{
    collect_responsive_variables, css_variables_to_style, normalise_css_token,
    normalise_spacing_token, CssVariableMap,
};

/// Output returned by [`render_divider`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DividerRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl DividerRenderOutput {
    /// Exposes the generated CSS variables in insertion order.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Inline style string derived from the CSS variable map.
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Render the responsive [`DividerState`] into CSS variables and inline styles.
#[must_use]
pub fn render_divider(state: &DividerState) -> DividerRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "divider",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            divider_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    DividerRenderOutput {
        css_variables,
        inline_style,
    }
}

fn divider_tokens_for_breakpoint(
    evaluation: &DividerEvaluation<'_>,
) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(5);

    tokens.push(("orientation", evaluation.orientation.as_str().to_string()));
    tokens.push((
        "thickness",
        normalise_css_token(evaluation.thickness.as_str(), "1px"),
    ));
    tokens.push(("inset", normalise_spacing_token(evaluation.inset.as_str())));
    tokens.push(("role", evaluation.role.as_str().to_string()));

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

/// Adapter props shared by all frameworks.
#[derive(Clone, Copy, Debug)]
pub struct DividerAdapterProps<'a> {
    /// Headless divider state describing orientation/thickness/inset.
    pub state: &'a DividerState,
}

impl<'a> DividerAdapterProps<'a> {
    /// Helper constructor so integration code can express intent succinctly.
    #[inline]
    pub fn new(state: &'a DividerState) -> Self {
        Self { state }
    }
}

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
fn render_divider_with_props(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
    render_divider(props.state)
}

/// React adapter delegating to the shared renderer.
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Run the renderer during React SSR or client renders for deterministic CSS variables.
    pub fn render(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
        super::render_divider_with_props(props)
    }
}

/// Yew adapter bridging to [`render_divider`].
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Produce CSS variables for Yew components without replicating orientation
    /// math in user land code.
    pub fn render(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
        super::render_divider_with_props(props)
    }
}

/// Leptos adapter forwarding to the canonical renderer.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Feed the [`DividerState`] into the renderer from reactive signals.
    pub fn render(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
        super::render_divider_with_props(props)
    }
}

/// Dioxus adapter.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Invoke during Dioxus renders to retrieve deterministic CSS variables.
    pub fn render(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
        super::render_divider_with_props(props)
    }
}

/// Sycamore adapter mirroring the others.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore orchestrations can simply pass the state through this helper to
    /// obtain SSR ready CSS variables.
    pub fn render(props: DividerAdapterProps<'_>) -> DividerRenderOutput {
        super::render_divider_with_props(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::divider::{DividerState, DividerTokens};
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

    #[test]
    fn divider_renderer_emits_orientation_and_thickness() {
        let tokens = DividerTokens {
            orientation: ResponsiveValue::new(
                rustic_ui_headless::divider::DividerOrientation::Horizontal,
            )
            .with_override(
                Breakpoint::Lg,
                rustic_ui_headless::divider::DividerOrientation::Vertical,
            ),
            thickness: ResponsiveValue::from(String::from("2px")),
            inset: ResponsiveValue::from(String::from("8px")),
        };
        let state = DividerState::new(tokens, BreakpointConfig::material());

        let output = render_divider(&state);

        assert_eq!(
            output
                .css_variables()
                .get("--rustic_ui_divider_orientation")
                .map(String::as_str),
            Some("horizontal"),
        );
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_divider_orientation-lg"));
        assert!(output
            .inline_style()
            .contains("--rustic_ui_divider_thickness"));
    }

    #[test]
    fn adapter_delegates_to_renderer() {
        let tokens = DividerTokens::horizontal("1px");
        let state = DividerState::new(tokens, BreakpointConfig::material());

        let base = render_divider(&state);
        let adapter = super::render_divider_with_props(DividerAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
