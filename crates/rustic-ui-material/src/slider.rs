//! Material renderer for the headless [`SliderState`](rustic_ui_headless::slider::SliderState).

use rustic_ui_headless::slider::{SliderOrientation, SliderState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::{render_element_html, render_inline_block_html};

/// Render output consumed by adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderRenderOutput {
    /// Serialized HTML string used by SSR integrations.
    pub html: String,
}

/// Adapter props bridging the headless slider state and renderer.
#[derive(Debug, Clone)]
pub struct SliderAdapterProps<'a> {
    /// Headless slider state machine.
    pub state: &'a SliderState,
}

impl<'a> SliderAdapterProps<'a> {
    /// Helper constructor.
    pub fn new(state: &'a SliderState) -> Self {
        Self { state }
    }
}

/// Render the Material slider into deterministic markup.
#[must_use]
pub fn render_slider(props: &SliderAdapterProps<'_>) -> SliderRenderOutput {
    let track = render_inline_block_html(
        "div",
        track_style(props.state.orientation()),
        [("data-role", "track")],
        "",
    );
    let thumb = render_inline_block_html(
        "div",
        thumb_style(),
        props
            .state
            .thumb_accessibility_attributes()
            .into_iter()
            .chain([("style", thumb_position_style(props.state))]),
        "",
    );
    let body = format!("{}{}", track, thumb);
    let html = render_element_html(
        "div",
        slider_root_style(props.state.orientation()),
        slider_root_attributes(props.state),
        &body,
    );
    SliderRenderOutput { html }
}

fn slider_root_attributes(state: &SliderState) -> Vec<(&'static str, String)> {
    let mut attrs = Vec::with_capacity(4);
    attrs.push((
        "data-orientation",
        match state.orientation() {
            SliderOrientation::Horizontal => "horizontal".into(),
            SliderOrientation::Vertical => "vertical".into(),
        },
    ));
    attrs.push(("data-value", format!("{:.3}", state.value())));
    if state.is_disabled() {
        attrs.push(("data-disabled", "true".into()));
    }
    attrs
}

fn slider_root_style(orientation: SliderOrientation) -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-flex;
        align-items: center;
        width: ${width};
        height: ${height};
        touch-action: none;
    "#,
        width = match orientation {
            SliderOrientation::Horizontal => "100%".to_string(),
            SliderOrientation::Vertical => format!("{}px", theme.spacing(3)),
        },
        height = match orientation {
            SliderOrientation::Horizontal => format!("{}px", theme.spacing(3)),
            SliderOrientation::Vertical => "120px".to_string(),
        }
    )
}

fn track_style(orientation: SliderOrientation) -> Style {
    match orientation {
        SliderOrientation::Horizontal => css_with_theme!(
            r#"
            background: ${track};
            border-radius: ${radius};
            position: absolute;
            left: 0;
            right: 0;
            height: ${height};
            top: 50%;
            transform: translateY(-50%);
        "#,
            track = theme.palette.text_secondary.clone(),
            radius = format!("{}px", theme.joy.radius / 2),
            height = format!("{}px", theme.spacing(1) / 2)
        ),
        SliderOrientation::Vertical => css_with_theme!(
            r#"
            background: ${track};
            border-radius: ${radius};
            position: absolute;
            top: 0;
            bottom: 0;
            width: ${width};
            left: 50%;
            transform: translateX(-50%);
        "#,
            track = theme.palette.text_secondary.clone(),
            radius = format!("{}px", theme.joy.radius / 2),
            width = format!("{}px", theme.spacing(1) / 2)
        ),
    }
}

fn thumb_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        width: ${size};
        height: ${size};
        border-radius: 50%;
        background: ${fill};
        box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.12);
        transform: translate(-50%, -50%);
        top: 50%;
        left: 0%;
    "#,
        size = format!("{}px", theme.spacing(3)),
        fill = theme.palette.primary.clone()
    )
}

fn thumb_position_style(state: &SliderState) -> String {
    let pct = state.percent();
    match state.orientation() {
        SliderOrientation::Horizontal => format!("left: {pct:.3}%; top: 50%;"),
        SliderOrientation::Vertical => format!("top: {rev:.3}%; left: 50%;", rev = 100.0 - pct),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::slider::{SliderConfig, SliderOrientation};

    #[test]
    fn render_slider_emits_orientation_attribute() {
        let state = SliderState::new(SliderConfig {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            page: 10.0,
            default_value: 50.0,
            disabled: false,
            orientation: SliderOrientation::Horizontal,
        });
        let html = render_slider(&SliderAdapterProps::new(&state)).html;
        assert!(html.contains("data-orientation=\"horizontal\""));
    }
}
