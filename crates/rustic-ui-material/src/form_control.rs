//! Material renderer for [`FormControlState`](rustic_ui_headless::form_control::FormControlState).

use rustic_ui_headless::form_control::FormControlState;
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::render_element_html;

/// Render output containing markup and attribute metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormControlRenderOutput {
    /// Serialized HTML string for SSR adapters.
    pub html: String,
}

/// Adapter props bridging the headless state and renderer.
#[derive(Debug, Clone)]
pub struct FormControlAdapterProps<'a> {
    /// Headless state for the control wrapper.
    pub state: &'a FormControlState,
    /// Control body markup (input/select etc.)
    pub control_markup: &'a str,
    /// Optional helper text markup inserted below the control.
    pub helper_markup: Option<&'a str>,
}

impl<'a> FormControlAdapterProps<'a> {
    /// Helper constructor for adapters.
    pub fn new(state: &'a FormControlState, control_markup: &'a str) -> Self {
        Self {
            state,
            control_markup,
            helper_markup: None,
        }
    }

    /// Attach helper markup (typically helper/error text).
    pub fn with_helper_markup(mut self, helper_markup: &'a str) -> Self {
        self.helper_markup = Some(helper_markup);
        self
    }
}

/// Render the Material form control shell.
#[must_use]
pub fn render_form_control(props: &FormControlAdapterProps<'_>) -> FormControlRenderOutput {
    let mut body = String::from(props.control_markup);
    if let Some(helper) = props.helper_markup {
        body.push_str(helper);
    }
    let html = render_element_html(
        "div",
        form_control_style(),
        props.state.aria_attributes(),
        &body,
    );
    FormControlRenderOutput { html }
}

fn form_control_style() -> Style {
    css_with_theme!(
        r#"
        display: flex;
        flex-direction: column;
        gap: ${gap};
        width: 100%;
    "#,
        gap = format!("{}px", theme.spacing(1))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::form_control::{FormControlConfig, FormControlMode};

    #[test]
    fn render_form_control_merges_helper_markup() {
        let state = FormControlState::new(
            "",
            FormControlMode::Uncontrolled,
            FormControlConfig::default(),
        );
        let html = render_form_control(
            &FormControlAdapterProps::new(&state, "<input />")
                .with_helper_markup("<span>Helper</span>"),
        )
        .html;
        assert!(html.contains("Helper"));
    }
}
