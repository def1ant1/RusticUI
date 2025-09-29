//! Renderer for [`GridState`](rustic_ui_headless::grid::GridState).
//!
//! Grids orchestrate both responsive track counts and spacing tokens.  Rather
//! than require each framework adapter to replicate the formatting logic we keep
//! a single Rust implementation that translates evaluated tokens into CSS
//! variables and inline strings.  This mirrors the strategy used for other
//! layout primitives and ensures SSR output is byte-for-byte identical with the
//! hydration pass.

use std::collections::BTreeMap;

use rustic_ui_headless::grid::{GridEvaluation, GridState};

use crate::render_helpers::{
    collect_responsive_variables, css_variables_to_style, grid_template, normalise_spacing_token,
    CssVariableMap,
};

/// Result returned by [`render_grid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl GridRenderOutput {
    /// Access the generated CSS variable map.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Inline style attribute derived from [`css_variables`].
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Produce CSS variables and inline styles for a responsive grid.
#[must_use]
pub fn render_grid(state: &GridState) -> GridRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "grid",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            grid_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    GridRenderOutput {
        css_variables,
        inline_style,
    }
}

fn grid_tokens_for_breakpoint(evaluation: &GridEvaluation<'_>) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(5);

    tokens.push(("columns", evaluation.columns.to_string()));
    tokens.push(("template", grid_template(evaluation.columns)));
    tokens.push((
        "column_gap",
        normalise_spacing_token(evaluation.column_gap.as_str()),
    ));
    tokens.push((
        "row_gap",
        normalise_spacing_token(evaluation.row_gap.as_str()),
    ));
    tokens.push(("role", evaluation.role.as_str().to_string()));

    if evaluation.dense {
        tokens.push(("auto_flow", String::from("row dense")));
    } else {
        tokens.push(("auto_flow", String::from("row")));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

    #[test]
    fn grid_renderer_populates_template_and_gaps() {
        let tokens = rustic_ui_headless::grid::GridTokens {
            columns: ResponsiveValue::new(2).with_override(Breakpoint::Lg, 4),
            column_gap: ResponsiveValue::from(String::from("1rem")),
            row_gap: ResponsiveValue::from(String::from("2rem")),
        };
        let state = GridState::new(tokens, BreakpointConfig::material());

        let output = render_grid(&state);

        assert_eq!(
            output
                .css_variables()
                .get("--rustic_ui_grid_template")
                .map(String::as_str),
            Some("repeat(2, minmax(0, 1fr))")
        );
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_grid_template-lg"));
        assert!(output.inline_style().contains("--rustic_ui_grid_auto_flow"));
    }
}
