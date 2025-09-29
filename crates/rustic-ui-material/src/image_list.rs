//! Renderer for [`ImageListState`](rustic_ui_headless::image_list::ImageListState).
//!
//! Image lists combine responsive column counts, spacing and row height tokens.
//! Centralising the renderer keeps CSS variable naming stable across adapters
//! which is critical when enterprises diff SSR output across frameworks.

use std::collections::BTreeMap;

use rustic_ui_headless::image_list::{ImageListEvaluation, ImageListState};

use crate::render_helpers::{
    collect_responsive_variables, css_variables_to_style, grid_template, normalise_spacing_token,
    CssVariableMap,
};

/// Result returned by [`render_image_list`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageListRenderOutput {
    css_variables: CssVariableMap,
    inline_style: String,
}

impl ImageListRenderOutput {
    /// Ordered CSS variables keyed by `--rustic_ui_image_list_*`.
    #[inline]
    pub fn css_variables(&self) -> &BTreeMap<String, String> {
        &self.css_variables
    }

    /// Inline `style` string derived from [`css_variables`].
    #[inline]
    pub fn inline_style(&self) -> &str {
        &self.inline_style
    }
}

/// Render the provided [`ImageListState`] into CSS variables that work across
/// SSR and CSR pipelines.
#[must_use]
pub fn render_image_list(state: &ImageListState) -> ImageListRenderOutput {
    let mut css_variables = CssVariableMap::new();

    collect_responsive_variables(
        &mut css_variables,
        "image_list",
        state.breakpoints(),
        |breakpoint| {
            let evaluation = state.evaluate_for(breakpoint);
            image_list_tokens_for_breakpoint(&evaluation)
        },
    );

    let inline_style = css_variables_to_style(&css_variables);

    ImageListRenderOutput {
        css_variables,
        inline_style,
    }
}

fn image_list_tokens_for_breakpoint(
    evaluation: &ImageListEvaluation<'_>,
) -> Vec<(&'static str, String)> {
    let mut tokens = Vec::with_capacity(6);

    tokens.push(("columns", evaluation.columns.to_string()));
    tokens.push(("template", grid_template(evaluation.columns)));
    tokens.push(("gap", normalise_spacing_token(evaluation.gap.as_str())));
    tokens.push(("row_height", format!("{}px", evaluation.row_height)));
    tokens.push(("role", evaluation.role.as_str().to_string()));
    tokens.push(("variant", evaluation.variant.as_str().to_string()));

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

/// Adapter props shared across framework integrations.
#[derive(Clone, Copy, Debug)]
pub struct ImageListAdapterProps<'a> {
    /// Headless image list state controlling responsive layout metadata.
    pub state: &'a ImageListState,
}

impl<'a> ImageListAdapterProps<'a> {
    /// Convenience constructor for integration code.
    #[inline]
    pub fn new(state: &'a ImageListState) -> Self {
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
fn render_image_list_with_props(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
    render_image_list(props.state)
}

/// React adapter invoking the shared renderer.
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Execute the canonical renderer during React SSR so hydration sees the
    /// same CSS variable payload.
    pub fn render(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
        super::render_image_list_with_props(props)
    }
}

/// Yew adapter forwarding to [`render_image_list`].
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Generate CSS variables from within Yew components without duplicating
    /// layout logic.
    pub fn render(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
        super::render_image_list_with_props(props)
    }
}

/// Leptos adapter bridging signals to the renderer.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Feed the [`ImageListState`] stored in signals through the shared
    /// renderer.  SSR snapshots and client recomputation will remain identical.
    pub fn render(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
        super::render_image_list_with_props(props)
    }
}

/// Dioxus adapter mirroring other frameworks.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Invoke the shared renderer inside Dioxus' lifecycle to obtain
    /// deterministic CSS variables for streaming or hydration.
    pub fn render(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
        super::render_image_list_with_props(props)
    }
}

/// Sycamore adapter hooking into the canonical renderer.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore orchestrations simply borrow the headless state and delegate to
    /// the renderer, keeping SSR deterministic across ecosystems.
    pub fn render(props: ImageListAdapterProps<'_>) -> ImageListRenderOutput {
        super::render_image_list_with_props(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::image_list::{ImageListState, ImageListTokens, ImageListVariant};
    use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

    #[test]
    fn image_list_renderer_emits_columns_and_gap() {
        let tokens = ImageListTokens {
            columns: ResponsiveValue::new(2).with_override(Breakpoint::Lg, 6),
            gap: ResponsiveValue::from(String::from("24px")),
            row_height: ResponsiveValue::from(320),
        };
        let state = ImageListState::new(tokens, BreakpointConfig::material())
            .variant(ImageListVariant::Masonry);

        let output = render_image_list(&state);

        assert_eq!(
            output
                .css_variables()
                .get("--rustic_ui_image_list_columns")
                .map(String::as_str),
            Some("2"),
        );
        assert!(output
            .css_variables()
            .contains_key("--rustic_ui_image_list_template-lg"));
        assert!(output
            .inline_style()
            .contains("--rustic_ui_image_list_row_height"));
    }

    #[test]
    fn adapter_delegates_to_renderer() {
        let tokens = ImageListTokens::uniform(ResponsiveValue::new(3), "8px", 240);
        let state = ImageListState::new(tokens, BreakpointConfig::material());

        let base = render_image_list(&state);
        let adapter = super::render_image_list_with_props(ImageListAdapterProps::new(&state));

        assert_eq!(base.inline_style(), adapter.inline_style());
    }
}
