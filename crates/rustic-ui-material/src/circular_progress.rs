//! Material renderer for [`CircularProgressState`](rustic_ui_headless::circular_progress::CircularProgressState).

use rustic_ui_headless::circular_progress::{CircularProgressState, ProgressMode};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::{render_inline_block_html, render_progress_shell_html};

/// Render output provided to framework adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircularProgressRenderOutput {
    /// Serialized HTML representation.
    pub html: String,
}

/// Adapter props bridging the headless state with the renderer.
#[derive(Debug, Clone)]
pub struct CircularProgressAdapterProps<'a> {
    /// Headless state describing progress and automation metadata.
    pub state: &'a CircularProgressState,
}

impl<'a> CircularProgressAdapterProps<'a> {
    /// Helper constructor for adapters.
    pub fn new(state: &'a CircularProgressState) -> Self {
        Self { state }
    }
}

/// Render a themed circular progress indicator.
#[must_use]
pub fn render_circular_progress(
    props: &CircularProgressAdapterProps<'_>,
) -> CircularProgressRenderOutput {
    let indicator = render_inline_block_html(
        "svg",
        indicator_style(),
        [("viewBox", "0 0 48 48"), ("aria-hidden", "true")],
        &format!(
            "<circle class=\"indicator-track\" cx=\"24\" cy=\"24\" r=\"20\" />\
<circle class=\"indicator-thumb\" cx=\"24\" cy=\"24\" r=\"20\" style=\"stroke-dashoffset: {dash};\" />",
            dash = determinate_dash_offset(props.state)
        ),
    );

    let html = render_progress_shell_html(
        circular_container_style(),
        props.state.aria_attributes(),
        &indicator,
    );
    CircularProgressRenderOutput { html }
}

fn determinate_dash_offset(state: &CircularProgressState) -> String {
    match state.mode() {
        ProgressMode::Determinate { value } => {
            let radius = 20.0;
            let circumference = 2.0 * std::f32::consts::PI * radius;
            let progress = value.clamp(0.0, 1.0);
            let dash = circumference * (1.0 - progress);
            format!("{dash:.3}")
        }
        ProgressMode::Indeterminate => String::from("0"),
    }
}

fn circular_container_style() -> Style {
    css_with_theme!(
        r#"
        width: ${size};
        height: ${size};
        display: inline-flex;
        align-items: center;
        justify-content: center;
    "#,
        size = format!("{}px", theme.spacing(6))
    )
}

fn indicator_style() -> Style {
    css_with_theme!(
        r#"
        width: 100%;
        height: 100%;
        .indicator-track {
            fill: none;
            stroke: ${track};
            stroke-width: 4;
            opacity: 0.25;
        }
        .indicator-thumb {
            fill: none;
            stroke: ${thumb};
            stroke-width: 4;
            stroke-linecap: round;
            stroke-dasharray: ${dasharray};
            transition: stroke-dashoffset 120ms linear;
        }
    "#,
        track = theme.palette.text_secondary.clone(),
        thumb = theme.palette.primary.clone(),
        dasharray = format!("{:.3}", 2.0 * std::f32::consts::PI * 20.0)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::circular_progress::ProgressMode;

    #[test]
    fn determinate_mode_updates_dash_offset() {
        let state = CircularProgressState::new(ProgressMode::Determinate { value: 0.5 });
        let html = render_circular_progress(&CircularProgressAdapterProps::new(&state)).html;
        assert!(html.contains("stroke-dashoffset"));
    }
}
