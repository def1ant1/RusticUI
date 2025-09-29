//! Material renderer for [`SkeletonState`](rustic_ui_headless::skeleton::SkeletonState).

use rustic_ui_headless::skeleton::{SkeletonAnimation, SkeletonState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::render_helpers::render_inline_block_html;

/// Render output consumed by adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonRenderOutput {
    /// Serialized HTML markup.
    pub html: String,
}

/// Adapter props bridging headless state and renderer.
#[derive(Debug, Clone)]
pub struct SkeletonAdapterProps<'a> {
    /// Headless skeleton state.
    pub state: &'a SkeletonState,
}

impl<'a> SkeletonAdapterProps<'a> {
    /// Convenience constructor.
    pub fn new(state: &'a SkeletonState) -> Self {
        Self { state }
    }
}

/// Render the Material skeleton placeholder.
#[must_use]
pub fn render_skeleton(props: &SkeletonAdapterProps<'_>) -> SkeletonRenderOutput {
    let html = render_inline_block_html(
        "span",
        skeleton_style(props.state.animation()),
        props.state.aria_attributes(),
        "",
    );
    SkeletonRenderOutput { html }
}

fn skeleton_style(animation: SkeletonAnimation) -> Style {
    css_with_theme!(
        r#"
        display: inline-block;
        width: ${width};
        height: ${height};
        background: linear-gradient(90deg, ${start} 25%, ${middle} 37%, ${start} 63%);
        background-size: 400% 100%;
        border-radius: ${radius};
        animation: ${animation};
    "#,
        width = "var(--skeleton-width, 100%)".to_string(),
        height = "var(--skeleton-height, 1em)".to_string(),
        start = theme.palette.text_secondary.clone(),
        middle = theme.palette.background_paper.clone(),
        radius = format!("{}px", theme.joy.radius / 2),
        animation = match animation {
            SkeletonAnimation::None => String::from("none"),
            SkeletonAnimation::Pulse => String::from("skeleton-pulse 1.2s ease-in-out infinite"),
            SkeletonAnimation::Wave => String::from("skeleton-wave 1.5s linear infinite"),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_skeleton_respects_animation() {
        let state = SkeletonState::new(SkeletonAnimation::Pulse);
        let html = render_skeleton(&SkeletonAdapterProps::new(&state)).html;
        assert!(html.contains("data-skeleton-animation=\"pulse\""));
    }
}
