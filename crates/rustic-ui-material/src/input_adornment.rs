//! Material renderer for [`InputAdornmentState`](rustic_ui_headless::input_adornment::InputAdornmentState).

use rustic_ui_headless::input_adornment::{AdornmentPosition, InputAdornmentState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::render_inline_block_html;

/// Render output bridging headless adornments to adapter markup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAdornmentRenderOutput {
    /// Serialized HTML string for the adornment.
    pub html: String,
}

/// Adapter props used by framework integrations.
#[derive(Debug, Clone)]
pub struct InputAdornmentAdapterProps<'a> {
    /// Headless adornment state.
    pub state: &'a InputAdornmentState,
    /// Inner markup (icon/text) rendered inside the adornment container.
    pub content: &'a str,
}

impl<'a> InputAdornmentAdapterProps<'a> {
    /// Helper constructor.
    pub fn new(state: &'a InputAdornmentState, content: &'a str) -> Self {
        Self { state, content }
    }
}

/// Render the Material adornment container.
#[must_use]
pub fn render_input_adornment(
    props: &InputAdornmentAdapterProps<'_>,
) -> InputAdornmentRenderOutput {
    let html = render_inline_block_html(
        "span",
        adornment_style(props.state.position()),
        props.state.aria_attributes(),
        props.content,
    );
    InputAdornmentRenderOutput { html }
}

fn adornment_style(position: AdornmentPosition) -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        color: ${color};
        padding-inline: ${padding};
    "#,
        color = theme.palette.text_secondary.clone(),
        padding = match position {
            AdornmentPosition::Start => format!("0 {}px 0 0", theme.spacing(1)),
            AdornmentPosition::End => format!("0 0 0 {}px", theme.spacing(1)),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_includes_position_attribute() {
        let state = InputAdornmentState::new(AdornmentPosition::End);
        let html = render_input_adornment(&InputAdornmentAdapterProps::new(&state, "$")).html;
        assert!(html.contains("data-adornment-position=\"end\""));
    }
}
