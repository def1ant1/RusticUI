//! Material renderer for the headless [`AlertState`](rustic_ui_headless::alert::AlertState).
//!
//! The renderer accepts the shared headless state and produces deterministic
//! HTML strings and attribute maps so adapters across Yew, Leptos, Dioxus, and
//! SSR contexts render identical markup. Extensive documentation is provided to
//! help future maintainers extend the alert blueprint without regressing
//! automation hooks.

use rustic_ui_headless::alert::{AlertSeverity, AlertState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::render_element_html;

/// Render output returned by [`render_alert`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertRenderOutput {
    /// Serialized HTML representation used by SSR adapters.
    pub html: String,
}

/// Adapter props shared across frameworks.
#[derive(Debug, Clone)]
pub struct AlertAdapterProps<'a> {
    /// Message presented inside the alert surface.
    pub message: &'a str,
    /// Headless state describing severity and visibility.
    pub state: &'a AlertState,
}

impl<'a> AlertAdapterProps<'a> {
    /// Convenience constructor for adapter implementations.
    pub fn new(message: &'a str, state: &'a AlertState) -> Self {
        Self { message, state }
    }
}

/// Render the Material flavored alert into deterministic markup.
#[must_use]
pub fn render_alert(props: &AlertAdapterProps<'_>) -> AlertRenderOutput {
    let attrs = props.state.aria_attributes();
    let html = render_element_html(
        "div",
        alert_style(props.state.severity()),
        attrs,
        props.message,
    );
    AlertRenderOutput { html }
}

fn alert_style(severity: AlertSeverity) -> Style {
    css_with_theme!(
        r#"
        background: ${background};
        color: ${text};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        display: flex;
        gap: ${gap};
        align-items: flex-start;
        font-weight: ${font_weight};
    "#,
        background = match severity {
            AlertSeverity::Info => theme.palette.info.clone(),
            AlertSeverity::Success => theme.palette.success.clone(),
            AlertSeverity::Warning => theme.palette.warning.clone(),
            AlertSeverity::Error => theme.palette.danger.clone(),
        },
        text = theme.palette.background_paper.clone(),
        padding_x = format!("{}px", theme.spacing(2)),
        padding_y = format!("{}px", theme.spacing(1)),
        radius = format!("{}px", theme.joy.radius),
        gap = format!("{}px", theme.spacing(1)),
        font_weight = theme.typography.font_weight_medium.to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_alert_includes_severity_attribute() {
        let state = AlertState::new(AlertSeverity::Success);
        let html = render_alert(&AlertAdapterProps::new("Synced successfully", &state)).html;
        assert!(html.contains("data-severity=\"success\""));
    }
}
