//! Material renderer for [`BackdropState`](rustic_ui_headless::backdrop::BackdropState).

use rustic_ui_headless::backdrop::BackdropState;
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::render_backdrop_html;

/// Render output surfaced to adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackdropRenderOutput {
    /// Serialized `<div>` markup representing the backdrop surface.
    pub html: String,
}

/// Adapter props shared across frameworks.
#[derive(Debug, Clone)]
pub struct BackdropAdapterProps<'a> {
    /// Headless state describing visibility and transition metadata.
    pub state: &'a BackdropState,
}

impl<'a> BackdropAdapterProps<'a> {
    /// Helper constructor for ergonomics.
    pub fn new(state: &'a BackdropState) -> Self {
        Self { state }
    }
}

/// Render the Material backdrop to markup for SSR adapters.
#[must_use]
pub fn render_backdrop(props: &BackdropAdapterProps<'_>) -> BackdropRenderOutput {
    let html = render_backdrop_html(backdrop_style(), props.state.aria_attributes());
    BackdropRenderOutput { html }
}

fn backdrop_style() -> Style {
    css_with_theme!(
        r#"
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.32);
        transition: opacity 180ms ease-in-out;
        opacity: 1;
        &[aria-hidden="true"] {
            opacity: 0;
        }
    "#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_backdrop_injects_animation_frame() {
        let state = BackdropState::new(true);
        let html = render_backdrop(&BackdropAdapterProps::new(&state)).html;
        assert!(html.contains("data-animation-frame"));
    }
}
