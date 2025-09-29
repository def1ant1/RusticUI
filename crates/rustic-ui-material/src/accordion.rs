//! Material renderer for the headless [`AccordionGroupState`].
//!
//! The implementation emits deterministic CSS classes and accessibility
//! attributes so React, Yew, Leptos and Dioxus adapters can remain thin wrappers
//! that simply bind events and render children.  Styling is driven entirely via
//! [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme) which means
//! tokens automatically track palette overrides and typography ramps configured
//! at runtime.

use rustic_ui_headless::accordion::AccordionGroupState;
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::style_helpers;

/// Describes a single accordion item and the ids used for automation hooks.
#[derive(Debug, Clone)]
pub struct AccordionItemDescriptor<'a> {
    /// DOM id for the summary button.
    pub summary_id: &'a str,
    /// DOM id for the associated panel.
    pub panel_id: &'a str,
}

/// Adapter level props accepted by [`render_accordion`].
#[derive(Debug, Clone)]
pub struct AccordionAdapterProps<'a> {
    /// Headless state describing expansion and accessibility wiring.
    pub state: &'a AccordionGroupState,
    /// Static description of the rendered items.
    pub items: &'a [AccordionItemDescriptor<'a>],
}

/// Rendered output for a single accordion item.
#[derive(Debug, Clone)]
pub struct AccordionItemRender {
    summary_attributes: Vec<(String, String)>,
    details_attributes: Vec<(String, String)>,
}

impl AccordionItemRender {
    /// Attributes attached to the summary element.
    pub fn summary_attributes(&self) -> &[(String, String)] {
        &self.summary_attributes
    }

    /// Attributes attached to the details panel.
    pub fn details_attributes(&self) -> &[(String, String)] {
        &self.details_attributes
    }
}

/// Rendered output for an accordion group.
#[derive(Debug, Clone)]
pub struct AccordionRenderOutput {
    root_class: String,
    items: Vec<AccordionItemRender>,
}

impl AccordionRenderOutput {
    /// Class attached to the accordion root container.
    pub fn root_class(&self) -> &str {
        &self.root_class
    }

    /// Rendered items with merged accessibility data.
    pub fn items(&self) -> &[AccordionItemRender] {
        &self.items
    }
}

/// Renders the accordion into deterministic classes and ARIA attributes.
pub fn render_accordion(props: AccordionAdapterProps<'_>) -> AccordionRenderOutput {
    let root_class = style_helpers::themed_class(accordion_root_style());
    let items = props
        .items
        .iter()
        .enumerate()
        .map(|(index, descriptor)| {
            let summary_attrs = build_summary_attributes(props.state, index, descriptor);
            let details_attrs = build_details_attributes(props.state, index, descriptor);
            AccordionItemRender {
                summary_attributes: summary_attrs,
                details_attributes: details_attrs,
            }
        })
        .collect();

    AccordionRenderOutput { root_class, items }
}

fn build_summary_attributes(
    state: &AccordionGroupState,
    index: usize,
    descriptor: &AccordionItemDescriptor<'_>,
) -> Vec<(String, String)> {
    let mut attrs: Vec<(String, String)> = state
        .summary_accessibility_attributes(index, descriptor.panel_id)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    attrs.push(("id".into(), descriptor.summary_id.to_string()));
    attrs.push((
        style_helpers::automation_data_attr("accordion", ["summary", &index.to_string()]),
        "true".into(),
    ));

    style_helpers::themed_attributes(accordion_summary_style(), attrs)
}

fn build_details_attributes(
    state: &AccordionGroupState,
    index: usize,
    descriptor: &AccordionItemDescriptor<'_>,
) -> Vec<(String, String)> {
    let mut attrs: Vec<(String, String)> = state
        .details_accessibility_attributes(index, descriptor.summary_id)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    attrs.push(("id".into(), descriptor.panel_id.to_string()));
    attrs.push((
        style_helpers::automation_data_attr("accordion", ["panel", &index.to_string()]),
        "true".into(),
    ));

    style_helpers::themed_attributes(accordion_details_style(), attrs)
}

fn accordion_root_style() -> Style {
    css_with_theme!(
        r#"
        border-radius: ${radius}px;
        background: ${background};
        color: ${color};
        border: 1px solid ${outline};
        overflow: hidden;
        display: block;
    "#,
        radius = theme.joy.radius,
        background = theme.palette.background_paper.clone(),
        color = theme.palette.text_primary.clone(),
        outline = theme.palette.neutral.clone()
    )
}

fn accordion_summary_style() -> Style {
    css_with_theme!(
        r#"
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: ${padding_y}px ${padding_x}px;
        background: transparent;
        border: none;
        text-align: left;
        cursor: pointer;
        font-family: ${font_family};
        font-weight: ${font_weight};
        font-size: ${font_size}rem;
        transition: background-color 120ms ease-in-out;

        &[aria-expanded="true"] {
            background-color: ${expanded_background};
        }

        &:focus-visible {
            outline: ${focus_outline};
            outline-offset: 2px;
        }
    "#,
        padding_y = theme.spacing(2),
        padding_x = theme.spacing(3),
        font_family = theme.typography.font_family.clone(),
        font_weight = theme.typography.font_weight_medium,
        font_size = theme.typography.body1,
        expanded_background = theme.palette.background_default.clone(),
        focus_outline = theme.joy.focus_outline_for_color(&theme.palette.primary)
    )
}

fn accordion_details_style() -> Style {
    css_with_theme!(
        r#"
        padding: ${padding}px;
        border-top: 1px solid ${divider};
        font-size: ${font_size}rem;
        line-height: ${line_height};
        background-color: ${background};
    "#,
        padding = theme.spacing(3),
        divider = theme.palette.neutral.clone(),
        font_size = theme.typography.body1,
        line_height = theme.typography.line_height,
        background = theme.palette.background_paper.clone()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::accordion::AccordionGroupState;

    #[test]
    fn summary_attributes_include_theme_class_and_aria_metadata() {
        let state = AccordionGroupState::new(1, false, &[0]);
        let props = AccordionAdapterProps {
            state: &state,
            items: &[AccordionItemDescriptor {
                summary_id: "summary",
                panel_id: "panel",
            }],
        };
        let output = render_accordion(props);
        let summary_attrs = &output.items()[0].summary_attributes;
        assert!(summary_attrs.iter().any(|(k, _)| k == "class"));
        assert!(summary_attrs
            .iter()
            .any(|(k, v)| k == "aria-controls" && v == "panel"));
    }
}
