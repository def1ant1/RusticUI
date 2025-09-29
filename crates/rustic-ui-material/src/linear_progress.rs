//! Material renderer for [`LinearProgressState`](rustic_ui_headless::linear_progress::LinearProgressState).

use rustic_ui_headless::linear_progress::{LinearProgressMode, LinearProgressState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::{render_inline_block_html, render_progress_shell_html};

/// Render output provided to adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearProgressRenderOutput {
    /// Serialized HTML markup.
    pub html: String,
}

/// Adapter props shared across frameworks.
#[derive(Debug, Clone)]
pub struct LinearProgressAdapterProps<'a> {
    /// Headless state describing progress values and ARIA metadata.
    pub state: &'a LinearProgressState,
}

impl<'a> LinearProgressAdapterProps<'a> {
    /// Constructor used by adapters.
    pub fn new(state: &'a LinearProgressState) -> Self {
        Self { state }
    }
}

/// Render the Material linear progress indicator.
#[must_use]
pub fn render_linear_progress(
    props: &LinearProgressAdapterProps<'_>,
) -> LinearProgressRenderOutput {
    let primary_attrs = vec![
        ("data-track".to_string(), "primary".to_string()),
        ("style".to_string(), primary_style(props.state)),
    ];
    let buffer_attrs = vec![
        ("data-track".to_string(), "buffer".to_string()),
        ("style".to_string(), buffer_style(props.state)),
    ];
    let indicator_markup = format!(
        "{}{}",
        render_inline_block_html("span", indicator_style(), primary_attrs, ""),
        render_inline_block_html("span", buffer_indicator_style(), buffer_attrs, "")
    );

    let html = render_progress_shell_html(
        linear_container_style(),
        props.state.aria_attributes(),
        &indicator_markup,
    );
    LinearProgressRenderOutput { html }
}

fn buffer_style(state: &LinearProgressState) -> String {
    match state.mode() {
        LinearProgressMode::Buffer { buffer, .. } => {
            let pct = (buffer.clamp(0.0, 1.0) * 100.0).round();
            format!("--progress-buffer: {pct}%;")
        }
        _ => String::new(),
    }
}

fn primary_style(state: &LinearProgressState) -> String {
    let pct = match state.mode() {
        LinearProgressMode::Determinate { value } => (value.clamp(0.0, 1.0) * 100.0).round(),
        LinearProgressMode::Buffer { value, .. } => (value.clamp(0.0, 1.0) * 100.0).round(),
        LinearProgressMode::Indeterminate => 50.0,
    };
    format!("transform: scaleX({});", pct / 100.0)
}

fn linear_container_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        width: 100%;
        height: 4px;
        background: ${track};
        overflow: hidden;
        border-radius: ${radius};
    "#,
        track = theme.palette.text_secondary.clone(),
        radius = format!("{}px", theme.joy.radius / 2)
    )
}

fn indicator_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        inset: 0;
        transform-origin: left center;
        background: ${fill};
        transition: transform 160ms linear;
    "#,
        fill = theme.palette.primary.clone()
    )
}

fn buffer_indicator_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        inset: 0;
        transform-origin: left center;
        background: ${buffer};
        opacity: 0.3;
        transition: transform 160ms linear;
    "#,
        buffer = theme.palette.primary.clone()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_mode_emits_buffer_variable() {
        let state = LinearProgressState::new(LinearProgressMode::Buffer {
            value: 0.25,
            buffer: 0.75,
        });
        let html = render_linear_progress(&LinearProgressAdapterProps::new(&state)).html;
        assert!(html.contains("--progress-buffer"));
    }
}
