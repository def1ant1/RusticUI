//! Material renderer for [`PaginationState`](rustic_ui_headless::pagination::PaginationState).
//!
//! The adapter merges ARIA metadata from the headless state machine with Material
//! themed styling and automation identifiers.  Each framework adapter reuses the
//! same attribute collections, guaranteeing deterministic SSR markup.

use rustic_ui_headless::pagination::{
    PaginationItemKind, PaginationListAttributes, PaginationRootAttributes, PaginationState,
};
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

/// Describes a single pagination control rendered inside the list.
#[derive(Debug, Clone)]
pub struct PaginationItemDescriptor<'a> {
    /// Kind of control rendered (page, previous, next...).
    pub kind: PaginationItemKind,
    /// Optional DOM id applied to the control.
    pub id: Option<&'a str>,
    /// Optional accessible label overriding the default.
    pub aria_label: Option<&'a str>,
    /// Inner HTML rendered inside the control button.
    pub content: &'a str,
}

/// Adapter props shared across React/Yew/Leptos/Dioxus/Sycamore integrations.
#[derive(Debug, Clone)]
pub struct PaginationAdapterProps<'a> {
    /// Headless state describing focus, selection, and analytics hooks.
    pub state: &'a PaginationState,
    /// Root attribute builder returned from [`PaginationState::root_attributes`].
    pub root_attributes: PaginationRootAttributes<'a>,
    /// List attribute builder returned from [`PaginationState::list_attributes`].
    pub list_attributes: PaginationListAttributes<'a>,
    /// Controls rendered inside the pagination list.
    pub items: &'a [PaginationItemDescriptor<'a>],
    /// Optional event channel surfaced via `data-on-select`.
    pub on_select_event: Option<&'a str>,
}

/// Rendered pagination output.
#[derive(Debug, Clone)]
pub struct PaginationRenderOutput {
    root_attributes: Vec<(String, String)>,
    list_attributes: Vec<(String, String)>,
    items: Vec<PaginationRenderedItem>,
}

impl PaginationRenderOutput {
    /// Attributes applied to the `<nav>` wrapper.
    pub fn root_attributes(&self) -> &[(String, String)] {
        &self.root_attributes
    }

    /// Attributes applied to the `<ul>` list container.
    pub fn list_attributes(&self) -> &[(String, String)] {
        &self.list_attributes
    }

    /// Rendered controls.
    pub fn items(&self) -> &[PaginationRenderedItem] {
        &self.items
    }
}

/// Rendered control metadata.
#[derive(Debug, Clone)]
pub struct PaginationRenderedItem {
    container_attributes: Vec<(String, String)>,
    control_attributes: Vec<(String, String)>,
    content: String,
}

impl PaginationRenderedItem {
    /// Attributes applied to the `<li>` element.
    pub fn container_attributes(&self) -> &[(String, String)] {
        &self.container_attributes
    }

    /// Attributes applied to the `<button>` element.
    pub fn control_attributes(&self) -> &[(String, String)] {
        &self.control_attributes
    }

    /// Inner HTML rendered inside the control button.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Render the pagination controls into attribute maps.
#[must_use]
pub fn render_pagination(props: PaginationAdapterProps<'_>) -> PaginationRenderOutput {
    let root_attributes = build_root_attributes(props.root_attributes, props.on_select_event);
    let list_attributes = build_list_attributes(props.list_attributes);
    let items = props
        .items
        .iter()
        .enumerate()
        .map(|(index, descriptor)| build_item(props.state, index, descriptor))
        .collect();

    PaginationRenderOutput {
        root_attributes,
        list_attributes,
        items,
    }
}

/// Render pagination controls into HTML markup.
#[must_use]
pub fn render_pagination_html(props: PaginationAdapterProps<'_>) -> String {
    let rendered = render_pagination(props);
    let items_html = rendered
        .items()
        .iter()
        .map(|item| {
            let button_html = format!(
                "<button {attrs}>{content}</button>",
                attrs = attributes_to_html(item.control_attributes()),
                content = item.content()
            );
            format!(
                "<li {attrs}>{button}</li>",
                attrs = attributes_to_html(item.container_attributes()),
                button = button_html
            )
        })
        .collect::<String>();

    let list_html = format!(
        "<ul {attrs}>{items}</ul>",
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
    attrs: PaginationRootAttributes<'_>,
    on_select_event: Option<&str>,
) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(8);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_label_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    if let Some(event) = on_select_event {
        pairs.push(("data-on-select".into(), event.into()));
    }
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("pagination"),
    ));
    pairs.push((
        style_helpers::automation_data_attr("pagination", ["root"]),
        "true".into(),
    ));
    style_helpers::themed_attributes(pagination_root_style(), pairs)
}

fn build_list_attributes(attrs: PaginationListAttributes<'_>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(6);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push((
        style_helpers::automation_data_attr("pagination", ["list"]),
        "true".into(),
    ));
    style_helpers::themed_attributes(pagination_list_style(), pairs)
}

fn build_item(
    state: &PaginationState,
    index: usize,
    descriptor: &PaginationItemDescriptor<'_>,
) -> PaginationRenderedItem {
    let mut container_pairs: Vec<(String, String)> = Vec::with_capacity(6);
    container_pairs.push((
        style_helpers::automation_data_attr("pagination", ["item", &index.to_string()]),
        "true".into(),
    ));
    container_pairs.push((
        "data-component".into(),
        style_helpers::component_marker("pagination-item"),
    ));
    let container_attributes =
        style_helpers::themed_attributes(pagination_item_style(), container_pairs);

    let mut builder = state.item_attributes(descriptor.kind);
    if let Some(id) = descriptor.id {
        builder = builder.id(id);
    }
    if let Some(label) = descriptor.aria_label {
        builder = builder.aria_label(label);
    }
    let pairs: Vec<(String, String)> = builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let mut attributes = style_helpers::themed_attributes(pagination_button_style(), pairs);
    attributes.push((
        style_helpers::automation_data_attr(
            "pagination",
            [
                "control",
                descriptor.kind.as_data_value(),
                &index.to_string(),
            ],
        ),
        "true".into(),
    ));

    PaginationRenderedItem {
        container_attributes,
        control_attributes: attributes,
        content: descriptor.content.to_string(),
    }
}

fn pagination_root_style() -> Style {
    css_with_theme!(
        r#"
        display: block;
        width: 100%;
    "#
    )
}

fn pagination_list_style() -> Style {
    css_with_theme!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        gap: ${gap};
        list-style: none;
        margin: 0;
        padding: ${padding_y}px ${padding_x}px;
        background: ${background};
        border-radius: ${radius};
    "#,
        gap = theme.spacing(1),
        padding_y = theme.spacing(1),
        padding_x = theme.spacing(2),
        background = theme.palette.background_default.clone(),
        radius = format!("{}px", theme.joy.radius)
    )
}

fn pagination_item_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
    "#
    )
}

fn pagination_button_style() -> Style {
    css_with_theme!(
        r#"
        appearance: none;
        border: none;
        background: transparent;
        color: ${color};
        font-family: ${font_family};
        font-size: ${font_size};
        min-width: 2.25rem;
        height: 2.25rem;
        border-radius: ${radius};
        display: inline-flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: background 120ms ease, color 120ms ease;
        &[aria-current="page"] {
            background: ${active_bg};
            color: ${active_color};
        }
        &:hover {
            background: ${hover_bg};
        }
        &[data-disabled="true"] {
            cursor: default;
            opacity: 0.5;
        }
    "#,
        color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.button),
        radius = format!("{}px", theme.joy.radius),
        active_bg = theme.palette.primary.clone(),
        active_color = theme.palette.background_paper.clone(),
        hover_bg = format!(
            "color-mix(in srgb, {} 16%, transparent)",
            theme.palette.primary.clone()
        )
    )
}

/// React adapter rendering pagination markup.
pub mod react {
    use super::*;

    /// Render the pagination controls for React SSR.
    pub fn render_pagination(props: PaginationAdapterProps<'_>) -> String {
        super::render_pagination_html(props)
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render pagination controls for snapshot tests.
    pub fn render_pagination(props: PaginationAdapterProps<'_>) -> String {
        super::render_pagination_html(props)
    }
}

/// Leptos adapter delegating to the shared renderer.
pub mod leptos {
    use super::*;

    /// Render pagination markup for Leptos SSR.
    pub fn render_pagination(props: PaginationAdapterProps<'_>) -> String {
        super::render_pagination_html(props)
    }
}

/// Sycamore adapter reusing the shared HTML renderer.
pub mod sycamore {
    use super::*;

    /// Render pagination markup.
    pub fn render_pagination(props: PaginationAdapterProps<'_>) -> String {
        super::render_pagination_html(props)
    }
}

/// Dioxus adapter mirroring the other integrations.
pub mod dioxus {
    use super::*;

    /// Render pagination markup.
    pub fn render_pagination(props: PaginationAdapterProps<'_>) -> String {
        super::render_pagination_html(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::pagination::PaginationItemKind;
    use rustic_ui_headless::ControlStrategy;

    fn build_state() -> PaginationState {
        PaginationState::new(
            5,
            Some(2),
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        )
    }

    fn build_props<'a>(state: &'a PaginationState) -> PaginationAdapterProps<'a> {
        PaginationAdapterProps {
            state,
            root_attributes: state.root_attributes().id("pager"),
            list_attributes: state.list_attributes(),
            items: &[
                PaginationItemDescriptor {
                    kind: PaginationItemKind::First,
                    id: Some("first"),
                    aria_label: Some("First"),
                    content: "&laquo;",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Previous,
                    id: Some("prev"),
                    aria_label: Some("Previous"),
                    content: "&lsaquo;",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Page(0),
                    id: Some("page-1"),
                    aria_label: None,
                    content: "1",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Page(1),
                    id: Some("page-2"),
                    aria_label: None,
                    content: "2",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Page(2),
                    id: Some("page-3"),
                    aria_label: None,
                    content: "3",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Next,
                    id: Some("next"),
                    aria_label: Some("Next"),
                    content: "&rsaquo;",
                },
                PaginationItemDescriptor {
                    kind: PaginationItemKind::Last,
                    id: Some("last"),
                    aria_label: Some("Last"),
                    content: "&raquo;",
                },
            ],
            on_select_event: Some("paginate"),
        }
    }

    #[test]
    fn root_attributes_include_event_channel() {
        let state = build_state();
        let rendered = render_pagination(build_props(&state));
        assert!(rendered
            .root_attributes()
            .iter()
            .any(|(k, v)| k == "data-on-select" && v == "paginate"));
    }

    #[test]
    fn html_renderer_emits_buttons_for_each_control() {
        let state = build_state();
        let html = render_pagination_html(build_props(&state));
        assert!(html.contains("<nav"));
        assert!(html.matches("<button").count() >= 7);
    }
}
