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

/// Shared props for framework adapters.
///
/// [`StackState`] carries the control strategy internally, allowing adapters to
/// borrow the state regardless of whether the calling code is controlled or
/// uncontrolled.  React/Yew/Leptos/Sycamore orchestrators simply pass the state
/// reference and let the renderer emit deterministic CSS variables.
#[derive(Clone, Copy, Debug)]
pub struct StackAdapterProps<'a> {
    /// Headless stack state that exposes responsive direction/gap metadata.
    pub state: &'a StackState,
}

impl<'a> StackAdapterProps<'a> {
    /// Convenience constructor for integration code.
    #[inline]
    pub fn new(state: &'a StackState) -> Self {
        Self { state }
    }
}

/// Internal helper leveraged by every adapter.
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
fn render_stack_with_props(props: StackAdapterProps<'_>) -> StackRenderOutput {
    // The canonical renderer centralises lifecycle handling so adapters remain
    // thin wrappers with zero drift across frameworks.
    render_stack(props.state)
}

/// React adapter bridging to the stack renderer.
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Render the stack during React SSR.  Enterprises can serialise the inline
    /// style directly into streamed markup knowing hydration will compute the
    /// same CSS variables once the [`StackState`] rehydrates on the client.
    pub fn render(props: StackAdapterProps<'_>) -> StackRenderOutput {
        super::render_stack_with_props(props)
    }
}

/// Yew adapter ensuring lifecycle parity with SSR output.
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Invoke the shared renderer from within Yew components.  Controlled
    /// `UseStateHandle` flows and uncontrolled per-render construction both map
    /// to the same headless state ensuring deterministic automation hooks.
    pub fn render(props: StackAdapterProps<'_>) -> StackRenderOutput {
        super::render_stack_with_props(props)
    }
}

/// Leptos adapter.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Feed the [`StackState`] from reactive signals into the canonical renderer
    /// so server rendered snapshots and client recomputation stay in lockstep.
    pub fn render(props: StackAdapterProps<'_>) -> StackRenderOutput {
        super::render_stack_with_props(props)
    }
}

/// Dioxus adapter.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Dioxus renderers call into this helper during the component lifecycle to
    /// compute CSS variables for streaming SSR.  The adapter keeps controlled and
    /// uncontrolled flows unified by delegating to the shared renderer.
    pub fn render(props: StackAdapterProps<'_>) -> StackRenderOutput {
        super::render_stack_with_props(props)
    }
}

/// Sycamore adapter mirroring other frameworks.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Whether the [`StackState`] lives in a `Signal` or a plain variable, this
    /// helper produces the same CSS variables so SSR automation harnesses remain
    /// consistent across environments.
    pub fn render(props: StackAdapterProps<'_>) -> StackRenderOutput {
        super::render_stack_with_props(props)
    }
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

    #[test]
    fn adapter_renders_identical_inline_styles() {
        let tokens = StackTokens {
            direction: ResponsiveValue::from(rustic_ui_headless::stack::StackDirection::Horizontal),
            gap: ResponsiveValue::from(String::from("16px")),
            divider: ResponsiveValue::from(None),
        };
        let state = StackState::new(tokens, BreakpointConfig::material());

        let base = render_stack(&state);
        let adapter = super::render_stack_with_props(StackAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
