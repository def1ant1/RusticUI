//! Material renderer for the headless [`RatingState`](rustic_ui_headless::rating::RatingState).
//!
//! The adapter turns the deterministic state machine into themed star icons,
//! automation-friendly attributes, and SSR-friendly HTML for React, Yew,
//! Leptos, Dioxus, and Sycamore integrations.  Controlled and uncontrolled
//! scenarios are differentiated via explicit adapter props so hydration mirrors
//! the server snapshot exactly.

use rustic_ui_headless::rating::RatingState;
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

/// Describes the ownership model of the rating value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingControlMode {
    /// External orchestrator owns the value.
    Controlled,
    /// Rating manages its own value.
    Uncontrolled,
}

/// Adapter props shared across frameworks.
#[derive(Debug, Clone)]
pub struct RatingAdapterProps<'a> {
    /// Headless rating state machine providing descriptors and analytics
    /// metadata.
    pub state: &'a RatingState,
    /// Optional DOM id applied to the root container.
    pub id: Option<&'a str>,
    /// Optional accessible label applied to the radiogroup. When omitted callers
    /// can rely on external labelling via `aria-labelledby`.
    pub label: Option<&'a str>,
    /// Optional automation identifier appended to all selectors.
    pub automation_id: Option<&'a str>,
    /// Ownership model applied to `data-controlled` attributes for hydration.
    pub control: RatingControlMode,
}

/// Rendered rating output.
#[derive(Debug, Clone)]
pub struct RatingRenderOutput {
    root_attributes: Vec<(String, String)>,
    html: String,
}

impl RatingRenderOutput {
    /// Returns the attributes applied to the root container.
    pub fn root_attributes(&self) -> &[(String, String)] {
        &self.root_attributes
    }

    /// Returns the serialized HTML fragment.
    pub fn html(&self) -> &str {
        &self.html
    }
}

/// Render the rating into deterministic markup.
#[must_use]
pub fn render_rating(props: RatingAdapterProps<'_>) -> RatingRenderOutput {
    let root_attributes = build_root_attributes(&props);
    let mut items_html = String::new();
    for index in 0..props.state.max() as usize {
        items_html.push_str(&render_item(&props, index));
    }
    let html = format!(
        "<div {attrs}>{items}</div>",
        attrs = attributes_to_html(&root_attributes),
        items = items_html
    );
    RatingRenderOutput {
        root_attributes,
        html,
    }
}

/// Render helper returning just the HTML string.
#[must_use]
pub fn render_rating_html(props: RatingAdapterProps<'_>) -> String {
    render_rating(props).html
}

fn build_root_attributes(props: &RatingAdapterProps<'_>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = props
        .state
        .root_attributes()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    if let Some(id) = props.id {
        pairs.push(("id".into(), id.into()));
    }
    if let Some(label) = props.label {
        pairs.push(("aria-label".into(), label.into()));
    }
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("rating"),
    ));
    pairs.push((
        "data-controlled".into(),
        match props.control {
            RatingControlMode::Controlled => "true".into(),
            RatingControlMode::Uncontrolled => "false".into(),
        },
    ));
    pairs.push((
        style_helpers::automation_data_attr("rating", ["root"]),
        "true".into(),
    ));
    pairs.push(("data-max".into(), props.state.max().to_string()));
    pairs.push((
        "data-precision".into(),
        format!("{:.2}", props.state.precision()),
    ));
    if let Some(id) = props.automation_id {
        pairs.push((
            "data-automation-id".into(),
            style_helpers::automation_id("rating", Some(id), style_helpers::EMPTY_SEGMENTS),
        ));
    }
    style_helpers::themed_attributes(rating_root_style(), pairs)
}

fn render_item(props: &RatingAdapterProps<'_>, index: usize) -> String {
    let descriptor = props.state.item_descriptor(index);
    let mut button_pairs = props
        .state
        .item_attributes(index)
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<Vec<_>>();
    button_pairs.push(("type".into(), "button".into()));
    button_pairs.push((
        "aria-label".into(),
        format!(
            "{} star{}",
            descriptor.value,
            if (descriptor.value - 1.0).abs() < f32::EPSILON {
                ""
            } else {
                "s"
            }
        ),
    ));
    button_pairs.push((
        style_helpers::automation_data_attr(
            "rating",
            ["item", &(descriptor.index + 1).to_string()],
        ),
        "true".into(),
    ));
    button_pairs.push((
        "data-component".into(),
        style_helpers::component_marker("rating-item"),
    ));
    if let Some(id) = props.automation_id {
        button_pairs.push((
            "data-automation-id".into(),
            style_helpers::automation_id(
                "rating",
                Some(id),
                ["item", &(descriptor.index + 1).to_string()],
            ),
        ));
    }
    let button_attrs = style_helpers::themed_attributes(rating_item_style(), button_pairs);
    let icon_pairs = vec![
        ("aria-hidden".to_string(), "true".to_string()),
        (
            "style".to_string(),
            format!("--rustic-rating-fill: {:.0}%;", descriptor.fill * 100.0),
        ),
    ];
    let icon_attrs = style_helpers::themed_attributes(rating_icon_style(), icon_pairs);
    format!(
        "<button {button_attrs}><span {icon_attrs}></span></button>",
        button_attrs = attributes_to_html(&button_attrs),
        icon_attrs = attributes_to_html(&icon_attrs)
    )
}

fn rating_root_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        color: ${color};
        --rustic-rating-fill: 0%;
    "#,
        gap = format!("{}px", theme.spacing(1)),
        color = theme.palette.text_primary.clone()
    )
}

fn rating_item_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: ${size}px;
        height: ${size}px;
        border: none;
        background: transparent;
        padding: 0;
        cursor: pointer;
        color: inherit;
        &[data-hovered="true"]::after {
            content: '';
            position: absolute;
            inset: 0;
            border-radius: ${radius};
            background: ${hover};
            opacity: 0.12;
        }
        &[aria-disabled="true"],
        &[data-disabled="true"] {
            cursor: not-allowed;
            opacity: 0.64;
        }
    "#,
        size = theme.spacing(5),
        radius = format!("{}px", theme.spacing(1)),
        hover = theme.palette.primary.clone()
    )
}

fn rating_icon_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-block;
        width: 100%;
        height: 100%;
        font-size: ${font};
        line-height: 1;
        color: ${inactive};
        &::before {
            content: '★';
            color: ${inactive};
            position: absolute;
            inset: 0;
        }
        &::after {
            content: '★';
            color: ${active};
            position: absolute;
            inset: 0;
            width: var(--rustic-rating-fill, 0%);
            overflow: hidden;
        }
    "#,
        font = format!("{}px", theme.typography.font_size * 1.75),
        inactive = format!(
            "color-mix(in srgb, {} 40%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        active = theme.palette.warning.clone()
    )
}

/// React adapter exposing both controlled and uncontrolled helpers.
pub mod react {
    use super::*;

    /// Render a controlled rating for React SSR.
    pub fn render_controlled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Controlled,
            ..props
        })
    }

    /// Render an uncontrolled rating for React SSR.
    pub fn render_uncontrolled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Uncontrolled,
            ..props
        })
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render a controlled rating for Yew SSR/snapshots.
    pub fn render_controlled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Controlled,
            ..props
        })
    }

    /// Render an uncontrolled rating for Yew SSR/snapshots.
    pub fn render_uncontrolled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Uncontrolled,
            ..props
        })
    }
}

/// Leptos adapter keeping parity with React/Yew.
pub mod leptos {
    use super::*;

    /// Render a controlled rating for Leptos SSR.
    pub fn render_controlled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Controlled,
            ..props
        })
    }

    /// Render an uncontrolled rating for Leptos SSR.
    pub fn render_uncontrolled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Uncontrolled,
            ..props
        })
    }
}

/// Dioxus adapter.
pub mod dioxus {
    use super::*;

    /// Render a controlled rating for Dioxus SSR.
    pub fn render_controlled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Controlled,
            ..props
        })
    }

    /// Render an uncontrolled rating for Dioxus SSR.
    pub fn render_uncontrolled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Uncontrolled,
            ..props
        })
    }
}

/// Sycamore adapter.
pub mod sycamore {
    use super::*;

    /// Render a controlled rating for Sycamore SSR.
    pub fn render_controlled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Controlled,
            ..props
        })
    }

    /// Render an uncontrolled rating for Sycamore SSR.
    pub fn render_uncontrolled(props: RatingAdapterProps<'_>) -> String {
        super::render_rating_html(RatingAdapterProps {
            control: RatingControlMode::Uncontrolled,
            ..props
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::rating::{RatingConfig, RatingState};

    fn build_state() -> RatingState {
        RatingState::uncontrolled(RatingConfig::enterprise_defaults())
    }

    #[test]
    fn root_attributes_include_component_marker() {
        let state = build_state();
        let rendered = render_rating(RatingAdapterProps {
            state: &state,
            id: Some("rating"),
            label: Some("Rate experience"),
            automation_id: Some("feedback"),
            control: RatingControlMode::Uncontrolled,
        });
        assert!(
            rendered
                .root_attributes()
                .iter()
                .any(|(k, v)| k == "data-component"
                    && v == &style_helpers::component_marker("rating"))
        );
        assert!(rendered.html().contains("<button"));
        assert!(rendered.html().contains("aria-label=\"1 star\""));
    }
}
