//! Material renderer for the headless [`BadgeState`](rustic_ui_headless::badge::BadgeState).
//!
//! The renderer projects badge tokens to CSS utility classes describing the
//! layering contract used across frameworks.  This keeps automation selectors
//! stable while surfacing the evaluated count to assistive technologies.

use rustic_ui_headless::badge::BadgeState;
use rustic_ui_styled_engine::css_with_theme;

use crate::style_helpers;

/// Render output describing the badge container and badge element attributes.
#[derive(Debug, Clone)]
pub struct BadgeRenderOutput {
    container_class: String,
    badge_attributes: Vec<(String, String)>,
}

impl BadgeRenderOutput {
    /// Class name attached to the relative container used for layering.
    pub fn container_class(&self) -> &str {
        &self.container_class
    }

    /// Attributes applied to the badge element.
    pub fn badge_attributes(&self) -> &[(String, String)] {
        &self.badge_attributes
    }
}

/// Render the badge state using the active theme.
pub fn render_badge(state: &BadgeState) -> BadgeRenderOutput {
    let container_class = style_helpers::themed_class(badge_container_style());

    let mut attrs: Vec<(String, String)> = state
        .accessibility_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    if let Some(display) = state.display_value() {
        attrs.push(("data-badge-content".into(), display));
    }
    attrs.push((
        style_helpers::automation_data_attr("badge", ["layer"]),
        "surface".into(),
    ));

    let badge_attributes =
        style_helpers::themed_attributes(badge_pill_style(state.is_dot()), attrs);

    BadgeRenderOutput {
        container_class,
        badge_attributes,
    }
}

fn badge_container_style() -> rustic_ui_styled_engine::Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-flex;
        align-items: center;
    "#
    )
}

fn badge_pill_style(is_dot: bool) -> rustic_ui_styled_engine::Style {
    if is_dot {
        css_with_theme!(
            r#"
            position: absolute;
            top: -4px;
            right: -4px;
            width: 10px;
            height: 10px;
            border-radius: 50%;
            background-color: ${background};
            border: 2px solid ${border};
        "#,
            background = theme.palette.secondary.clone(),
            border = theme.palette.background_paper.clone()
        )
    } else {
        css_with_theme!(
            r#"
            position: absolute;
            top: -6px;
            right: -6px;
            min-width: 22px;
            height: 22px;
            border-radius: 11px;
            padding: 0 ${padding}px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            background-color: ${background};
            color: ${color};
            font-family: ${font_family};
            font-weight: ${font_weight};
            font-size: 0.75rem;
        "#,
            padding = theme.spacing(1),
            background = theme.palette.secondary.clone(),
            color = theme.palette.background_paper.clone(),
            font_family = theme.typography.font_family.clone(),
            font_weight = theme.typography.font_weight_medium
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::badge::BadgeState;

    #[test]
    fn badge_attributes_surface_display_value() {
        let state = BadgeState::new(Some(5));
        let render = render_badge(&state);
        assert!(render.badge_attributes().iter().any(|(k, _)| k == "class"));
        assert!(render
            .badge_attributes()
            .iter()
            .any(|(k, v)| k == "data-badge-content" && v == "5"));
    }
}
