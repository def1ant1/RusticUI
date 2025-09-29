//! Renderer for [`ContainerState`](rustic_ui_headless::container::ContainerState).
//!
//! The container primitive enforces horizontal rhythm using max-width and
//! padding tokens that vary per breakpoint.  This renderer consumes the
//! headless state and emits CSS variables under the `--rustic_ui_container_*`
//! namespace so adapters do not have to implement the bookkeeping themselves.
//! Keeping the rendering logic server side also guarantees that SSR snapshots
//! align with hydration output which is crucial for enterprise workflows that
//! diff markup in CI.

use std::collections::BTreeMap;

use rustic_ui_headless::container::{ContainerEvaluation, ContainerState};

use crate::render_helpers::{
    collect_responsive_variables, css_variables_to_style, normalise_css_token,
    normalise_spacing_token, CssVariableMap,
};

/// Result returned by [`render_container`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl ContainerRenderOutput {
    /// CSS variables keyed by `--rustic_ui_container_*` names.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Inline style attribute that can be injected into SSR markup.
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Convert a [`ContainerState`] into CSS variables and an inline style string.
#[must_use]
pub fn render_container(state: &ContainerState) -> ContainerRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "container",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            container_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    ContainerRenderOutput {
        css_variables,
        inline_style,
    }
}

fn container_tokens_for_breakpoint(
    evaluation: &ContainerEvaluation<'_>,
) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(4);

    tokens.push((
        "max_width",
        normalise_css_token(evaluation.max_width.as_str(), "100%"),
    ));
    tokens.push((
        "padding_inline",
        normalise_spacing_token(evaluation.padding_inline.as_str()),
    ));
    tokens.push(("role", evaluation.role.as_str().to_string()));
    tokens.push((
        "fixed",
        if evaluation.fixed {
            "true".into()
        } else {
            "false".into()
        },
    ));

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

    #[test]
    fn container_renderer_emits_breakpoint_specific_tokens() {
        let tokens = rustic_ui_headless::container::ContainerTokens {
            max_width: ResponsiveValue::new("600px".to_string())
                .with_override(Breakpoint::Md, "900px".to_string()),
            padding_inline: ResponsiveValue::from(String::from("16px")),
        };
        let state = ContainerState::new(tokens, BreakpointConfig::material());

        let output = render_container(&state);

        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_container_max_width"));
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_container_max_width-md"));
        assert!(output
            .inline_style()
            .contains("--rustic_ui_container_fixed"));
    }
}
