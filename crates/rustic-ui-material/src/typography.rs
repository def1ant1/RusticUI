//! Material renderer for headless typography state.
//!
//! The renderer maps variants to theme driven font sizes and exposes the
//! semantic tag computed by the headless machine so SSR and client renders stay
//! aligned.

use rustic_ui_headless::typography::{TypographyState, TypographyVariant};
use rustic_ui_styled_engine::css_with_theme;

use crate::style_helpers;

/// Render output for a typography element.
#[derive(Debug, Clone)]
pub struct TypographyRenderOutput {
    tag: String,
    attributes: Vec<(String, String)>,
}

impl TypographyRenderOutput {
    /// HTML tag representing the semantic element.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Attributes attached to the rendered element.
    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }
}

/// Render the provided typography state.
pub fn render_typography(state: &TypographyState) -> TypographyRenderOutput {
    let mut attrs: Vec<(String, String)> = state
        .accessibility_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    attrs.push((
        "data-typography-variant".into(),
        state.variant().as_str().into(),
    ));

    let style = typography_style(state.variant());
    let attrs = style_helpers::themed_attributes(style, attrs);

    TypographyRenderOutput {
        tag: state.tag().to_string(),
        attributes: attrs,
    }
}

fn typography_style(variant: TypographyVariant) -> rustic_ui_styled_engine::Style {
    css_with_theme!(
        r#"
        font-family: ${font_family};
        font-weight: ${font_weight};
        font-size: ${font_size}rem;
        line-height: ${line_height};
        margin: 0;
    "#,
        font_family = theme.typography.font_family.clone(),
        font_weight = match variant {
            TypographyVariant::H1 | TypographyVariant::H2 | TypographyVariant::H3 => {
                theme.typography.font_weight_light
            }
            TypographyVariant::H4 | TypographyVariant::H5 | TypographyVariant::H6 => {
                theme.typography.font_weight_regular
            }
            TypographyVariant::Subtitle1
            | TypographyVariant::Subtitle2
            | TypographyVariant::Body1
            | TypographyVariant::Body2 => theme.typography.font_weight_regular,
            TypographyVariant::Button => theme.typography.font_weight_medium,
            TypographyVariant::Caption | TypographyVariant::Overline => {
                theme.typography.font_weight_medium
            }
        },
        font_size = match variant {
            TypographyVariant::H1 => theme.typography.h1,
            TypographyVariant::H2 => theme.typography.h2,
            TypographyVariant::H3 => theme.typography.h3,
            TypographyVariant::H4 => theme.typography.h4,
            TypographyVariant::H5 => theme.typography.h5,
            TypographyVariant::H6 => theme.typography.h6,
            TypographyVariant::Subtitle1 => theme.typography.subtitle1,
            TypographyVariant::Subtitle2 => theme.typography.subtitle2,
            TypographyVariant::Body1 => theme.typography.body1,
            TypographyVariant::Body2 => theme.typography.body2,
            TypographyVariant::Button => theme.typography.button,
            TypographyVariant::Caption => theme.typography.caption,
            TypographyVariant::Overline => theme.typography.overline,
        },
        line_height = match variant {
            TypographyVariant::Overline | TypographyVariant::Caption => 1.66,
            TypographyVariant::Button => 1.75,
            _ => theme.typography.line_height,
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::typography::{TypographyState, TypographyVariant};

    #[test]
    fn render_attaches_variant_attribute() {
        let state = TypographyState::new(TypographyVariant::H5).with_id("heading");
        let render = render_typography(&state);
        assert_eq!(render.tag(), "h5");
        assert!(render
            .attributes()
            .iter()
            .any(|(k, v)| k == "data-typography-variant" && v == "h5"));
        assert!(render
            .attributes()
            .iter()
            .any(|(k, v)| k == "id" && v == "heading"));
    }
}
