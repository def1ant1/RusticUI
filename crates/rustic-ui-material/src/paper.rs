//! Material renderer for the headless [`PaperState`](rustic_ui_headless::paper::PaperState).
//!
//! Surfaces convert the abstract elevation/variant metadata into CSS classes
//! derived from the active [`Theme`](rustic_ui_styled_engine::Theme).  Enterprise
//! adapters use the returned attributes to keep SSR and client renders in sync.

use rustic_ui_headless::paper::{PaperState, PaperVariant};
use rustic_ui_styled_engine::css_with_theme;

use crate::style_helpers;

/// Rendered output for a Paper surface.
#[derive(Debug, Clone)]
pub struct PaperRenderOutput {
    attributes: Vec<(String, String)>,
}

impl PaperRenderOutput {
    /// Attributes merged into the surface container.
    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }
}

/// Render the provided [`PaperState`] into deterministic attributes.
pub fn render_paper(state: &PaperState) -> PaperRenderOutput {
    let mut attrs: Vec<(String, String)> = state
        .accessibility_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    for (key, value) in state.tokens() {
        attrs.push((format!("data-paper-{}", key), value));
    }

    let theme_attrs = style_helpers::themed_attributes(paper_style(state.variant()), attrs);

    PaperRenderOutput {
        attributes: theme_attrs,
    }
}

fn paper_style(variant: PaperVariant) -> rustic_ui_styled_engine::Style {
    match variant {
        PaperVariant::Elevated => css_with_theme!(
            r#"
            background-color: ${background};
            color: ${text};
            border-radius: ${radius}px;
            box-shadow: ${shadow};
            transition: box-shadow 160ms ease-in-out;
        "#,
            background = theme.palette.background_paper.clone(),
            text = theme.palette.text_primary.clone(),
            radius = theme.joy.radius,
            shadow = theme.joy.shadow.surface.clone()
        ),
        PaperVariant::Outlined => css_with_theme!(
            r#"
            background-color: ${background};
            color: ${text};
            border-radius: ${radius}px;
            border: 1px solid ${outline};
        "#,
            background = theme.palette.background_paper.clone(),
            text = theme.palette.text_primary.clone(),
            radius = theme.joy.radius,
            outline = theme.palette.neutral.clone()
        ),
        PaperVariant::Plain => css_with_theme!(
            r#"
            background-color: transparent;
            color: ${text};
            border-radius: ${radius}px;
        "#,
            text = theme.palette.text_primary.clone(),
            radius = theme.joy.radius
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::paper::PaperState;

    #[test]
    fn attributes_include_data_tokens_and_class() {
        let state = PaperState::new(PaperVariant::Elevated).with_elevation(6);
        let render = render_paper(&state);
        let attrs = render.attributes();
        assert!(attrs.iter().any(|(k, _)| k == "class"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-paper-elevation" && v == "6"));
    }
}
