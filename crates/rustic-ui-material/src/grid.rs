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

/// Props passed to the framework specific adapters.
///
/// [`GridState`] encapsulates responsive evaluation along with the control
/// strategy, so adapters merely borrow it.  Controlled integrations keep the
/// state in a signal or hook while uncontrolled flows construct it on demand;
/// both converge on the same renderer which reads the most recent evaluation.
#[derive(Clone, Copy, Debug)]
pub struct GridAdapterProps<'a> {
    /// Headless grid state driving track and spacing values.
    pub state: &'a GridState,
}

impl<'a> GridAdapterProps<'a> {
    /// Helper for ergonomic construction in integration code.
    #[inline]
    pub fn new(state: &'a GridState) -> Self {
        Self { state }
    }
}

/// Internal helper shared by all adapter modules.
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
fn render_grid_with_props(props: GridAdapterProps<'_>) -> GridRenderOutput {
    // Delegate to the canonical renderer so SSR and hydration are byte-identical
    // regardless of which framework hosts the layout component.
    render_grid(props.state)
}

/// React adapter for grid rendering.
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Run the shared renderer during React's server-side render lifecycle.
    /// Enterprises often stash the CSS variable map inside orchestration layers
    /// so design systems can compose layouts without bespoke glue code.
    pub fn render(props: GridAdapterProps<'_>) -> GridRenderOutput {
        super::render_grid_with_props(props)
    }
}

/// Yew adapter bridging to [`render_grid`].
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Produce deterministic grid CSS variables for Yew components.  By routing
    /// through the shared renderer, controlled props (state stored in hooks) and
    /// uncontrolled props (state created per render) both respect the same
    /// headless evaluation.
    pub fn render(props: GridAdapterProps<'_>) -> GridRenderOutput {
        super::render_grid_with_props(props)
    }
}

/// Leptos adapter mirroring other frameworks.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Leptos signals trigger this renderer automatically when inputs change.
    /// The inline style string can be injected into SSR templates to guarantee
    /// hydration parity.
    pub fn render(props: GridAdapterProps<'_>) -> GridRenderOutput {
        super::render_grid_with_props(props)
    }
}

/// Dioxus adapter for SSR/hydration pipelines.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Dioxus orchestrators should call this during render to obtain the CSS
    /// variables before streaming markup.  The shared helper keeps controlled and
    /// uncontrolled ownership models aligned without extra bookkeeping.
    pub fn render(props: GridAdapterProps<'_>) -> GridRenderOutput {
        super::render_grid_with_props(props)
    }
}

/// Sycamore adapter forwarding to the canonical renderer.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore's signals or stored state can both drive this helper which then
    /// formats the CSS variables for SSR snapshots.  Enterprise automation suites
    /// can diff the resulting markup across frameworks with confidence.
    pub fn render(props: GridAdapterProps<'_>) -> GridRenderOutput {
        super::render_grid_with_props(props)
    }
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

    #[test]
    fn adapter_mirrors_shared_renderer() {
        let tokens = rustic_ui_headless::grid::GridTokens {
            columns: ResponsiveValue::new(3),
            column_gap: ResponsiveValue::from(String::from("8px")),
            row_gap: ResponsiveValue::from(String::from("12px")),
        };
        let state = GridState::new(tokens, BreakpointConfig::material());

        let base = render_grid(&state);
        let adapter = super::render_grid_with_props(GridAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
