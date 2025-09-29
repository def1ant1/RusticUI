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

/// Props consumed by every framework adapter.
///
/// [`ContainerState`] embeds its own control strategy so adapters simply borrow
/// the state.  Controlled flows keep the state in a reactive signal and pass a
/// reference while uncontrolled usage can let the adapter own the state.  The
/// renderer always evaluates the most recent snapshot which keeps the mapping to
/// the headless API identical across frameworks.
#[derive(Clone, Copy, Debug)]
pub struct ContainerAdapterProps<'a> {
    /// Headless state describing the container's responsive geometry.
    pub state: &'a ContainerState,
}

impl<'a> ContainerAdapterProps<'a> {
    /// Helper so integrators can construct props fluently.
    #[inline]
    pub fn new(state: &'a ContainerState) -> Self {
        Self { state }
    }
}

/// Shared implementation invoked by every adapter module.
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
fn render_container_with_props(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
    // Delegating to the canonical renderer keeps SSR output deterministic and
    // ensures hydration diffing remains stable regardless of framework.
    render_container(props.state)
}

/// React focused adapter exposing [`render_container`].
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Run the container renderer during React's server render lifecycle.  The
    /// returned CSS variable map can be fed into CSS-in-JS systems while the
    /// inline style string is ideal for streaming HTML responses.
    pub fn render(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
        super::render_container_with_props(props)
    }
}

/// Yew adapter wrapping [`render_container`] and gated behind `yew`.
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Produce SSR identical CSS variables for Yew components.  Consumers can
    /// keep the [`ContainerState`] in `UseStateHandle`s for controlled updates or
    /// instantiate it on the fly for uncontrolled flows; both strategies surface
    /// the same snapshot to the shared renderer.
    pub fn render(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
        super::render_container_with_props(props)
    }
}

/// Leptos adapter bridging signals to the shared renderer.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Invoke the renderer each time Leptos re-computes the container state.
    /// The output feeds nicely into `view!` macros via inline styles or dynamic
    /// `<style>` tags for enterprise grade SSR pipelines.
    pub fn render(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
        super::render_container_with_props(props)
    }
}

/// Dioxus adapter maintaining SSR parity.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Dioxus orchestrators can call this during the `render` phase to derive
    /// CSS variables for streaming SSR.  Because the adapter never mutates the
    /// state it respects both controlled and uncontrolled ownership models.
    pub fn render(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
        super::render_container_with_props(props)
    }
}

/// Sycamore adapter mirroring the other frameworks.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Compute the CSS variables using the canonical renderer so Sycamore's
    /// reactive scopes can hydrate without diff noise.  Passing the state by
    /// reference makes it trivial to bridge controlled signals into the headless
    /// API.
    pub fn render(props: ContainerAdapterProps<'_>) -> ContainerRenderOutput {
        super::render_container_with_props(props)
    }
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

    #[test]
    fn adapter_delegates_to_shared_renderer() {
        let tokens = rustic_ui_headless::container::ContainerTokens {
            max_width: ResponsiveValue::new("1200px".to_string()),
            padding_inline: ResponsiveValue::from(String::from("24px")),
        };
        let state = ContainerState::new(tokens, BreakpointConfig::material());

        let base = render_container(&state);
        let adapter = super::render_container_with_props(ContainerAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
