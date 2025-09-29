//! Renderer for the responsive [`StackState`](rustic_ui_headless::stack::StackState).
//!
//! Stacks compose padding and direction metadata to align children along a
//! vertical or horizontal axis.  Enterprise adapters often need the evaluated
//! tokens as CSS variables so they can plug the values into their preferred
//! templating engine.  This module mirrors the approach used by the other layout
//! primitives and exposes a single [`render_stack`] function that handles the
//! formatting in a deterministic fashion.

use std::collections::BTreeMap;

use rustic_ui_headless::stack::{StackEvaluation, StackState};

use crate::render_helpers::{
    collect_responsive_variables, css_variables_to_style, normalise_css_token,
    normalise_spacing_token, CssVariableMap,
};

/// Result returned by [`render_stack`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl StackRenderOutput {
    /// Return the generated CSS variables keyed under the `--rustic_ui_stack_*` namespace.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Inline style attribute built from [`css_variables`].
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Render a [`StackState`] into deterministic CSS variables and inline styles.
#[must_use]
pub fn render_stack(state: &StackState) -> StackRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "stack",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            stack_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    StackRenderOutput {
        css_variables,
        inline_style,
    }
}

fn stack_tokens_for_breakpoint(evaluation: &StackEvaluation<'_>) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(5);

    tokens.push(("direction", evaluation.direction.as_str().to_string()));
    tokens.push(("gap", normalise_spacing_token(evaluation.gap.as_str())));
    tokens.push(("role", evaluation.role.as_str().to_string()));

    match evaluation.divider.as_ref() {
        Some(token) if !token.trim().is_empty() => {
            tokens.push(("divider", normalise_css_token(token, "none")));
        }
        _ => tokens.push(("divider", String::from("none"))),
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
    use rustic_ui_headless::stack::StackTokens;

    #[test]
    fn stack_renderer_emits_direction_tokens() {
        let tokens = StackTokens {
            direction: ResponsiveValue::new(rustic_ui_headless::stack::StackDirection::Vertical)
                .with_override(
                    Breakpoint::Md,
                    rustic_ui_headless::stack::StackDirection::Horizontal,
                ),
            gap: ResponsiveValue::from(String::from("8px")),
            divider: ResponsiveValue::from(Some(String::from("1px solid var(--border)"))),
        };
        let state = StackState::new(tokens, BreakpointConfig::material());

        let output = render_stack(&state);

        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_stack_direction"));
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_stack_direction-md"));
        assert!(output.inline_style().contains("--rustic_ui_stack_gap"));
    }
}
