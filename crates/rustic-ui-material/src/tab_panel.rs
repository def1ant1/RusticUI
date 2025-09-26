//! Material themed tab panel helpers that enrich the headless
//! [`TabPanelAttributes`](mui_headless::tab_panel::TabPanelAttributes).
//!
//! Panels inherit typography, spacing and color tokens from the active
//! [`Theme`](mui_styled_engine::Theme) via [`css_with_theme!`](mui_styled_engine::css_with_theme).
//! Automation-focused data attributes are emitted alongside ARIA metadata so
//! downstream adapters can orchestrate transitions or analytics without custom
//! bookkeeping.

use mui_headless::tab_panel::TabPanelAttributes;
use mui_headless::tabs::{TabsOrientation, TabsState};
use mui_styled_engine::{css_with_theme, Style};

/// Collect attributes for a tab panel including automation-friendly flags.
#[must_use]
pub fn tab_panel_attributes(
    state: &TabsState,
    index: usize,
    attrs: TabPanelAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(7);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.hidden() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push(("data-selected".into(), state.is_selected(index).to_string()));
    pairs.push((
        "data-orientation".into(),
        state.orientation().as_aria().to_string(),
    ));
    pairs
}

/// Render a panel element into HTML for SSR or testing scenarios.
#[must_use]
pub fn render_tab_panel_html(
    state: &TabsState,
    index: usize,
    attrs: TabPanelAttributes<'_>,
    body: &str,
) -> String {
    crate::render_helpers::render_element_html(
        "div",
        tab_panel_style(state.orientation()),
        tab_panel_attributes(state, index, attrs),
        body,
    )
}

/// Generates the themed style used by panels across orientations.
fn tab_panel_style(_orientation: TabsOrientation) -> Style {
    css_with_theme!(
        r#"
        padding: ${padding};
        background: ${background};
        color: ${text_color};
        border-radius: ${radius};
        box-shadow: 0 18px 45px -30px color-mix(in srgb, ${shadow_base} 48%, transparent);
        &[hidden] {
            display: none;
        }
        &[data-orientation="vertical"] {
            border-left: 1px solid ${divider};
        }
        @media (min-width: ${sm}px) {
            padding: ${padding_large};
        }
        @media (min-width: ${lg}px) {
            padding: ${padding_xl};
        }
    "#,
        padding = format!("{}px", theme.spacing(2)),
        padding_large = format!("{}px", theme.spacing(3)),
        padding_xl = format!("{}px", theme.spacing(4)),
        background = theme.palette.background_paper.clone(),
        text_color = theme.palette.text_primary.clone(),
        radius = format!("{}px", theme.joy.radius),
        shadow_base = theme.palette.neutral.clone(),
        divider = format!(
            "1px solid color-mix(in srgb, {} 18%, transparent)",
            theme.palette.neutral.clone()
        ),
        sm = theme.breakpoints.sm,
        lg = theme.breakpoints.lg,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::tabs::{ActivationMode, TabsOrientation};

    fn sample_state(selected: usize) -> TabsState {
        TabsState::new(
            2,
            Some(selected),
            ActivationMode::Automatic,
            TabsOrientation::Horizontal,
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        )
    }

    #[test]
    fn tab_panel_attributes_include_hidden_flag() {
        let state = sample_state(0);
        let attrs = tab_panel_attributes(&state, 1, state.panel(1));
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "tabpanel"));
        assert!(attrs.iter().any(|(k, v)| k == "hidden" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-selected" && v == "false"));
    }

    #[test]
    fn render_tab_panel_html_includes_body_content() {
        let state = sample_state(1);
        let html = render_tab_panel_html(&state, 1, state.panel(1), "<p>Content</p>");
        assert!(html.contains("class=\""));
        assert!(html.contains("role=\"tabpanel\""));
        assert!(html.contains("<p>Content</p>"));
    }
}
