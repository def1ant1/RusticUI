//! Material renderer for [`BreadcrumbsState`](rustic_ui_headless::breadcrumbs::BreadcrumbsState).
//!
//! The helpers emit deterministic attribute collections so adapters across React,
//! Yew, Leptos, Sycamore and Dioxus share identical markup.  Styling is provided
//! via `css_with_theme!` ensuring palette and typography adjustments propagate
//! automatically.

use rustic_ui_headless::breadcrumbs::{
    BreadcrumbsListAttributes, BreadcrumbsRootAttributes, BreadcrumbsState,
};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::style_helpers;
use rustic_ui_utils::attributes_to_html;

/// Describes a single crumb rendered inside the breadcrumb trail.
#[derive(Debug, Clone)]
pub struct BreadcrumbItemDescriptor<'a> {
    /// Optional DOM id for the interactive crumb element.
    pub id: Option<&'a str>,
    /// Optional navigation target.
    pub href: Option<&'a str>,
    /// Inner HTML rendered inside the crumb anchor.
    pub content: &'a str,
    /// Glyph displayed between this crumb and the next one.
    pub separator: &'a str,
}

/// Adapter props shared across framework integrations.
#[derive(Debug, Clone)]
pub struct BreadcrumbsAdapterProps<'a> {
    /// Headless state describing focus and analytics semantics.
    pub state: &'a BreadcrumbsState,
    /// Root attribute builder returned by the headless state.
    pub root_attributes: BreadcrumbsRootAttributes<'a>,
    /// Ordered list attribute builder.
    pub list_attributes: BreadcrumbsListAttributes<'a>,
    /// Rendered crumbs.
    pub items: &'a [BreadcrumbItemDescriptor<'a>],
    /// Optional event channel surfaced via `data-on-activate`.
    pub on_activate_event: Option<&'a str>,
}

/// Rendered output for a single breadcrumb item.
#[derive(Debug, Clone)]
pub struct BreadcrumbRenderItem {
    container_attributes: Vec<(String, String)>,
    link_attributes: Vec<(String, String)>,
    separator_attributes: Vec<(String, String)>,
    content: String,
    separator: String,
}

impl BreadcrumbRenderItem {
    /// Attributes applied to the `<li>` element.
    pub fn container_attributes(&self) -> &[(String, String)] {
        &self.container_attributes
    }

    /// Attributes applied to the `<a>` element.
    pub fn link_attributes(&self) -> &[(String, String)] {
        &self.link_attributes
    }

    /// Attributes applied to the separator element.
    pub fn separator_attributes(&self) -> &[(String, String)] {
        &self.separator_attributes
    }

    /// Returns the crumb content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the separator glyph.
    pub fn separator(&self) -> &str {
        &self.separator
    }
}

/// Rendered breadcrumb output.
#[derive(Debug, Clone)]
pub struct BreadcrumbRenderOutput {
    root_attributes: Vec<(String, String)>,
    list_attributes: Vec<(String, String)>,
    items: Vec<BreadcrumbRenderItem>,
}

impl BreadcrumbRenderOutput {
    /// Attributes for the `<nav>` element.
    pub fn root_attributes(&self) -> &[(String, String)] {
        &self.root_attributes
    }

    /// Attributes for the `<ol>` element.
    pub fn list_attributes(&self) -> &[(String, String)] {
        &self.list_attributes
    }

    /// Rendered crumbs.
    pub fn items(&self) -> &[BreadcrumbRenderItem] {
        &self.items
    }
}

/// Render breadcrumbs into attribute maps.
#[must_use]
pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> BreadcrumbRenderOutput {
    let root_attributes = build_root_attributes(props.root_attributes, props.on_activate_event);
    let list_attributes = build_list_attributes(props.list_attributes);
    let items = props
        .items
        .iter()
        .enumerate()
        .map(|(index, descriptor)| {
            build_item(
                props.state,
                index,
                descriptor,
                index + 1 == props.items.len(),
            )
        })
        .collect();

    BreadcrumbRenderOutput {
        root_attributes,
        list_attributes,
        items,
    }
}

/// Render breadcrumbs into serialized HTML.
#[must_use]
pub fn render_breadcrumbs_html(props: BreadcrumbsAdapterProps<'_>) -> String {
    let rendered = render_breadcrumbs(props);
    let items_html = rendered
        .items()
        .iter()
        .map(|item| {
            let link_html = format!(
                "<a {attrs}>{content}</a>",
                attrs = attributes_to_html(item.link_attributes()),
                content = item.content()
            );
            let separator_html = if item.separator().is_empty() {
                String::new()
            } else {
                format!(
                    "<span {attrs}>{glyph}</span>",
                    attrs = attributes_to_html(item.separator_attributes()),
                    glyph = item.separator()
                )
            };
            format!(
                "<li {attrs}>{body}</li>",
                attrs = attributes_to_html(item.container_attributes()),
                body = format!(
                    "{link}{separator}",
                    link = link_html,
                    separator = separator_html
                )
            )
        })
        .collect::<String>();

    let list_html = format!(
        "<ol {attrs}>{items}</ol>",
        attrs = attributes_to_html(rendered.list_attributes()),
        items = items_html
    );

    format!(
        "<nav {attrs}>{list}</nav>",
        attrs = attributes_to_html(rendered.root_attributes()),
        list = list_html
    )
}

fn build_root_attributes(
    attrs: BreadcrumbsRootAttributes<'_>,
    on_activate_event: Option<&str>,
) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(8);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_label_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    if let Some(event) = on_activate_event {
        pairs.push(("data-on-activate".into(), event.into()));
    }
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("breadcrumbs"),
    ));
    pairs.push((
        style_helpers::automation_data_attr("breadcrumbs", ["root"]),
        "true".into(),
    ));
    style_helpers::themed_attributes(breadcrumb_root_style(), pairs)
}

fn build_list_attributes(attrs: BreadcrumbsListAttributes<'_>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(6);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push((
        style_helpers::automation_data_attr("breadcrumbs", ["list"]),
        "true".into(),
    ));
    style_helpers::themed_attributes(breadcrumb_list_style(), pairs)
}

fn build_item(
    state: &BreadcrumbsState,
    index: usize,
    descriptor: &BreadcrumbItemDescriptor<'_>,
    is_last: bool,
) -> BreadcrumbRenderItem {
    let mut container_pairs: Vec<(String, String)> = Vec::with_capacity(6);
    container_pairs.push((
        style_helpers::automation_data_attr("breadcrumbs", ["item", &index.to_string()]),
        "true".into(),
    ));
    container_pairs.push((
        "data-component".into(),
        style_helpers::component_marker("breadcrumbs-item"),
    ));
    let container_attributes =
        style_helpers::themed_attributes(breadcrumb_item_style(), container_pairs);

    let mut item_builder = state.item_attributes(index);
    if let Some(id) = descriptor.id {
        item_builder = item_builder.id(id);
    }
    if let Some(href) = descriptor.href {
        item_builder = item_builder.href(href);
    }
    let link_pairs: Vec<(String, String)> = item_builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let link_attributes = style_helpers::themed_attributes(breadcrumb_link_style(), link_pairs);

    let separator_builder = state.separator_attributes();
    let separator_pairs: Vec<(String, String)> = separator_builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let mut separator_attributes =
        style_helpers::themed_attributes(breadcrumb_separator_style(), separator_pairs);
    if is_last {
        separator_attributes.push(("aria-hidden".into(), "true".into()));
        separator_attributes.push(("data-hidden".into(), "true".into()));
    }

    BreadcrumbRenderItem {
        container_attributes,
        link_attributes,
        separator_attributes,
        content: descriptor.content.to_string(),
        separator: if is_last {
            String::new()
        } else {
            descriptor.separator.to_string()
        },
    }
}

fn breadcrumb_root_style() -> Style {
    css_with_theme!(
        r#"
        display: block;
        width: 100%;
        color: ${color};
        font-family: ${font_family};
        font-size: ${font_size};
    "#,
        color = theme.palette.text_secondary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body2)
    )
}

fn breadcrumb_list_style() -> Style {
    css_with_theme!(
        r#"
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: ${gap};
        list-style: none;
        margin: 0;
        padding: 0;
    "#,
        gap = theme.spacing(1)
    )
}

fn breadcrumb_item_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        color: inherit;
    "#,
        gap = theme.spacing(1)
    )
}

fn breadcrumb_link_style() -> Style {
    css_with_theme!(
        r#"
        color: inherit;
        text-decoration: none;
        padding: ${padding_y}px ${padding_x}px;
        border-radius: ${radius};
        transition: background 120ms ease, color 120ms ease;
        &[aria-current] {
            font-weight: ${font_weight};
            color: ${active_color};
        }
        &:hover {
            background: ${hover};
        }
        &[data-disabled="true"] {
            pointer-events: none;
            opacity: 0.6;
        }
    "#,
        padding_y = 0u16,
        padding_x = theme.spacing(1),
        radius = format!("{}px", theme.joy.radius / 2),
        font_weight = theme.typography.font_weight_medium,
        active_color = theme.palette.primary.clone(),
        hover = format!(
            "color-mix(in srgb, {} 12%, transparent)",
            theme.palette.primary.clone()
        )
    )
}

fn breadcrumb_separator_style() -> Style {
    css_with_theme!(
        r#"
        color: ${color};
        display: inline-flex;
        align-items: center;
        justify-content: center;
        min-width: 0.5em;
    "#,
        color = theme.palette.text_secondary.clone()
    )
}

/// React adapter returning deterministic HTML.
pub mod react {
    use super::*;

    /// Render breadcrumbs for React SSR pipelines.
    pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> String {
        super::render_breadcrumbs_html(props)
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render breadcrumbs for snapshot tests.
    pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> String {
        super::render_breadcrumbs_html(props)
    }
}

/// Leptos adapter ensuring parity with other frameworks.
pub mod leptos {
    use super::*;

    /// Render breadcrumbs into HTML markup.
    pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> String {
        super::render_breadcrumbs_html(props)
    }
}

/// Sycamore adapter delegating to the shared renderer.
pub mod sycamore {
    use super::*;

    /// Render breadcrumbs into deterministic HTML.
    pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> String {
        super::render_breadcrumbs_html(props)
    }
}

/// Dioxus adapter keeping SSR output deterministic.
pub mod dioxus {
    use super::*;

    /// Render breadcrumbs for Dioxus SSR pipelines.
    pub fn render_breadcrumbs(props: BreadcrumbsAdapterProps<'_>) -> String {
        super::render_breadcrumbs_html(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::ControlStrategy;

    fn build_state() -> BreadcrumbsState {
        BreadcrumbsState::new(
            3,
            Some(2),
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        )
    }

    fn build_props<'a>(state: &'a BreadcrumbsState) -> BreadcrumbsAdapterProps<'a> {
        BreadcrumbsAdapterProps {
            state,
            root_attributes: state.root_attributes().id("crumbs"),
            list_attributes: state.list_attributes(),
            items: &[
                BreadcrumbItemDescriptor {
                    id: Some("home"),
                    href: Some("/"),
                    content: "<span>Home</span>",
                    separator: "/",
                },
                BreadcrumbItemDescriptor {
                    id: Some("library"),
                    href: Some("/library"),
                    content: "<span>Library</span>",
                    separator: "/",
                },
                BreadcrumbItemDescriptor {
                    id: Some("current"),
                    href: None,
                    content: "<span>Current</span>",
                    separator: "",
                },
            ],
            on_activate_event: Some("breadcrumb-activate"),
        }
    }

    #[test]
    fn root_attributes_include_event_channel() {
        let state = build_state();
        let rendered = render_breadcrumbs(build_props(&state));
        assert!(rendered
            .root_attributes()
            .iter()
            .any(|(k, v)| k == "data-on-activate" && v == "breadcrumb-activate"));
        assert!(rendered
            .list_attributes()
            .iter()
            .any(|(k, _)| k == &style_helpers::automation_data_attr("breadcrumbs", ["list"])));
    }

    #[test]
    fn html_renderer_includes_ordered_list() {
        let state = build_state();
        let html = render_breadcrumbs_html(build_props(&state));
        assert!(html.contains("<nav"));
        assert!(html.contains("<ol"));
        assert!(html.contains(&format!(
            "data-component=\"{}\"",
            style_helpers::component_marker("breadcrumbs-item")
        )));
    }
}
