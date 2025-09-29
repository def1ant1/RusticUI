//! Material renderer for [`AvatarState`](rustic_ui_headless::avatar::AvatarState).
//!
//! The renderer converts the headless metadata into themed classes and
//! accessibility attributes while exposing deterministic automation hooks for
//! QA tooling.

use rustic_ui_headless::avatar::AvatarState;
use rustic_ui_styled_engine::css_with_theme;

use crate::style_helpers;

/// Render result describing the avatar container and media node.
#[derive(Debug, Clone)]
pub struct AvatarRenderOutput {
    container_attributes: Vec<(String, String)>,
    media_attributes: Vec<(String, String)>,
}

impl AvatarRenderOutput {
    /// Attributes for the wrapper element.
    pub fn container_attributes(&self) -> &[(String, String)] {
        &self.container_attributes
    }

    /// Attributes for the `<img>` or fallback `<span>` element.
    pub fn media_attributes(&self) -> &[(String, String)] {
        &self.media_attributes
    }
}

/// Render the avatar using the active theme.
pub fn render_avatar(state: &AvatarState) -> AvatarRenderOutput {
    let container_attrs = style_helpers::themed_attributes(
        avatar_container_style(),
        state
            .accessibility_attributes()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v)),
    );

    let mut media_attrs: Vec<(String, String)> = Vec::with_capacity(2);
    if let Some(alt) = state.alt() {
        media_attrs.push(("alt".into(), alt.to_string()));
    }
    if let Some(initials) = state.fallback_initials() {
        media_attrs.push(("data-avatar-fallback".into(), initials.to_string()));
    }

    let media_attrs = style_helpers::themed_attributes(avatar_media_style(), media_attrs);

    AvatarRenderOutput {
        container_attributes: container_attrs,
        media_attributes: media_attrs,
    }
}

fn avatar_container_style() -> rustic_ui_styled_engine::Style {
    css_with_theme!(
        r#"
        width: 40px;
        height: 40px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        background-color: ${background};
        color: ${color};
        position: relative;
    "#,
        background = theme.palette.primary.clone(),
        color = theme.palette.background_paper.clone()
    )
}

fn avatar_media_style() -> rustic_ui_styled_engine::Style {
    css_with_theme!(
        r#"
        width: 100%;
        height: 100%;
        border-radius: 50%;
        object-fit: cover;
        font-family: ${font_family};
        font-weight: ${font_weight};
        text-transform: uppercase;
        display: inline-flex;
        align-items: center;
        justify-content: center;
    "#,
        font_family = theme.typography.font_family.clone(),
        font_weight = theme.typography.font_weight_medium
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::avatar::AvatarState;

    #[test]
    fn render_includes_accessibility_metadata() {
        let state = AvatarState::new(Some("User".into()), Some("JD".into())).with_label("Team");
        let render = render_avatar(&state);
        assert!(render
            .container_attributes()
            .iter()
            .any(|(k, v)| k == "aria-label" && v == "Team"));
        assert!(render
            .media_attributes()
            .iter()
            .any(|(k, v)| k == "data-avatar-fallback" && v == "JD"));
    }
}
