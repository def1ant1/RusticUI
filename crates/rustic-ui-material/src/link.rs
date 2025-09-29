//! Material renderer for the headless [`LinkState`](rustic_ui_headless::link::LinkState).
//!
//! The adapter layers deterministic styling and automation hooks on top of the
//! headless state so enterprise teams can share the same navigation primitives
//! across frameworks.

use rustic_ui_headless::link::{LinkAttributes, LinkState};
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

/// Adapter props shared across frameworks.
#[derive(Debug, Clone)]
pub struct LinkAdapterProps<'a> {
    /// Headless link state describing analytics and disabled semantics.
    pub state: &'a LinkState,
    /// Attribute builder returned from [`LinkState::attributes`].
    pub attributes: LinkAttributes<'a>,
    /// Inner HTML rendered inside the anchor.
    pub content: &'a str,
}

/// Render output describing the link attributes.
#[derive(Debug, Clone)]
pub struct LinkRenderOutput {
    attributes: Vec<(String, String)>,
    content: String,
}

impl LinkRenderOutput {
    /// Returns the attribute pairs applied to the anchor element.
    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }

    /// Returns the inner HTML rendered inside the anchor.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Render the link into attribute pairs.
#[must_use]
pub fn render_link(props: LinkAdapterProps<'_>) -> LinkRenderOutput {
    let mut pairs: Vec<(String, String)> = props
        .attributes
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("link"),
    ));
    pairs.push((
        style_helpers::automation_data_attr("link", style_helpers::EMPTY_SEGMENTS),
        "true".into(),
    ));
    let attributes = style_helpers::themed_attributes(link_style(), pairs);

    LinkRenderOutput {
        attributes,
        content: props.content.to_string(),
    }
}

/// Render the link into serialized HTML markup.
#[must_use]
pub fn render_link_html(props: LinkAdapterProps<'_>) -> String {
    let rendered = render_link(props);
    format!(
        "<a {attrs}>{content}</a>",
        attrs = attributes_to_html(rendered.attributes()),
        content = rendered.content()
    )
}

fn link_style() -> Style {
    css_with_theme!(
        r#"
        color: ${color};
        text-decoration: none;
        font-weight: ${weight};
        transition: color 120ms ease;
        &:hover {
            color: ${hover};
            text-decoration: underline;
        }
        &[data-disabled="true"] {
            pointer-events: none;
            color: ${disabled};
            text-decoration: none;
        }
    "#,
        color = theme.palette.primary.clone(),
        hover = format!(
            "color-mix(in srgb, {} 85%, black)",
            theme.palette.primary.clone()
        ),
        disabled = format!(
            "color-mix(in srgb, {} 40%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        weight = theme.typography.font_weight_medium
    )
}

/// React adapter returning deterministic HTML.
pub mod react {
    use super::*;

    /// Render the link into HTML markup for React SSR.
    pub fn render_link(props: LinkAdapterProps<'_>) -> String {
        super::render_link_html(props)
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render the link into HTML markup.
    pub fn render_link(props: LinkAdapterProps<'_>) -> String {
        super::render_link_html(props)
    }
}

/// Leptos adapter delegating to the shared renderer.
pub mod leptos {
    use super::*;

    /// Render the link into HTML markup.
    pub fn render_link(props: LinkAdapterProps<'_>) -> String {
        super::render_link_html(props)
    }
}

/// Sycamore adapter that reuses the shared HTML renderer.
pub mod sycamore {
    use super::*;

    /// Render the link into HTML markup.
    pub fn render_link(props: LinkAdapterProps<'_>) -> String {
        super::render_link_html(props)
    }
}

/// Dioxus adapter mirroring the other implementations.
pub mod dioxus {
    use super::*;

    /// Render the link into HTML markup.
    pub fn render_link(props: LinkAdapterProps<'_>) -> String {
        super::render_link_html(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_attributes_expose_automation_marker() {
        let mut state = LinkState::new(true);
        state.set_analytics_channel(Some("nav"));
        state.set_analytics_tag(Some("primary"));
        let output = render_link(LinkAdapterProps {
            state: &state,
            attributes: state
                .attributes()
                .href("/docs")
                .id("docs-link")
                .target("_blank"),
            content: "Docs",
        });
        let marker = style_helpers::automation_data_attr("link", style_helpers::EMPTY_SEGMENTS);
        assert!(output.attributes().iter().any(|(k, _)| k == &marker));
    }

    #[test]
    fn html_renderer_includes_anchor_markup() {
        let state = LinkState::new(false);
        let html = render_link_html(LinkAdapterProps {
            state: &state,
            attributes: state.attributes().href("/"),
            content: "Home",
        });
        assert!(html.contains("<a"));
        assert!(html.contains("data-component=\"rustic_ui_link\""));
    }
}
