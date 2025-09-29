//! Material renderer for [`BottomNavigationState`](rustic_ui_headless::bottom_navigation::BottomNavigationState).
//!
//! The module converts the headless state machine into deterministic attribute
//! maps and SSR friendly HTML helpers.  Every rendered element exposes
//! automation-first `data-rustic-*` markers so QA pipelines and analytics hooks
//! can bind without resorting to brittle CSS selectors.

use rustic_ui_headless::bottom_navigation::{BottomNavigationAttributes, BottomNavigationState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::style_helpers;
use rustic_ui_utils::attributes_to_html;

/// Describes an individual navigation destination.
#[derive(Debug, Clone)]
pub struct BottomNavigationItemDescriptor<'a> {
    /// Optional DOM id applied to the rendered button.
    pub id: Option<&'a str>,
    /// Optional identifier of the panel controlled by this destination.
    pub controls: Option<&'a str>,
    /// Pre-rendered inner HTML representing the destination content.
    pub content: &'a str,
}

/// Adapter props shared across React/Yew/Leptos/Dioxus/Sycamore integrations.
#[derive(Debug, Clone)]
pub struct BottomNavigationAdapterProps<'a> {
    /// Headless state describing selection and focus semantics.
    pub state: &'a BottomNavigationState,
    /// Attribute builder returned from the headless state for the root element.
    pub attributes: BottomNavigationAttributes<'a>,
    /// Logical destinations rendered within the navigation bar.
    pub items: &'a [BottomNavigationItemDescriptor<'a>],
    /// Optional event channel surfaced via `data-on-select` for analytics.
    pub on_select_event: Option<&'a str>,
}

/// Rendered attributes for a single destination.
#[derive(Debug, Clone)]
pub struct BottomNavigationItemRender {
    attributes: Vec<(String, String)>,
    content: String,
}

impl BottomNavigationItemRender {
    /// Returns the attribute pairs describing the destination button.
    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }

    /// Returns the serialized inner HTML for the destination.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Rendered attributes for the navigation container.
#[derive(Debug, Clone)]
pub struct BottomNavigationRenderOutput {
    root_attributes: Vec<(String, String)>,
    items: Vec<BottomNavigationItemRender>,
}

impl BottomNavigationRenderOutput {
    /// Attribute pairs attached to the `<nav>` wrapper.
    pub fn root_attributes(&self) -> &[(String, String)] {
        &self.root_attributes
    }

    /// Rendered destinations.
    pub fn items(&self) -> &[BottomNavigationItemRender] {
        &self.items
    }
}

/// Render the bottom navigation into attribute maps.
#[must_use]
pub fn render_bottom_navigation(
    props: BottomNavigationAdapterProps<'_>,
) -> BottomNavigationRenderOutput {
    let root_attributes =
        build_root_attributes(props.state, props.attributes, props.on_select_event);
    let items = props
        .items
        .iter()
        .enumerate()
        .map(|(index, descriptor)| BottomNavigationItemRender {
            attributes: build_item_attributes(props.state, index, descriptor),
            content: descriptor.content.to_string(),
        })
        .collect();

    BottomNavigationRenderOutput {
        root_attributes,
        items,
    }
}

/// Render the bottom navigation into serialized HTML markup.
#[must_use]
pub fn render_bottom_navigation_html(props: BottomNavigationAdapterProps<'_>) -> String {
    let rendered = render_bottom_navigation(props);
    let items_html = rendered
        .items()
        .iter()
        .map(|item| {
            format!(
                "<button {attrs}>{content}</button>",
                attrs = attributes_to_html(item.attributes()),
                content = item.content()
            )
        })
        .collect::<String>();

    format!(
        "<nav {attrs}>{items}</nav>",
        attrs = attributes_to_html(rendered.root_attributes()),
        items = items_html
    )
}

fn build_root_attributes(
    state: &BottomNavigationState,
    attrs: BottomNavigationAttributes<'_>,
    on_select_event: Option<&str>,
) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(8);
    pairs.push(("role".into(), attrs.role().into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.analytics_attribute() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("bottom-navigation"),
    ));
    pairs.push((
        style_helpers::automation_data_attr("bottom-navigation", ["root"]),
        "true".into(),
    ));
    pairs.push((
        "data-selected-index".into(),
        state
            .selected()
            .map(|index| index.to_string())
            .unwrap_or_else(|| "".into()),
    ));
    if let Some(event) = on_select_event {
        pairs.push(("data-on-select".into(), event.into()));
    }
    style_helpers::themed_attributes(bottom_navigation_root_style(), pairs)
}

fn build_item_attributes(
    state: &BottomNavigationState,
    index: usize,
    descriptor: &BottomNavigationItemDescriptor<'_>,
) -> Vec<(String, String)> {
    let mut builder = state.item_attributes(index);
    if let Some(id) = descriptor.id {
        builder = builder.id(id);
    }
    if let Some(controls) = descriptor.controls {
        builder = builder.controls(controls);
    }
    let mut pairs: Vec<(String, String)> = builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    pairs.push((
        style_helpers::automation_data_attr("bottom-navigation", ["item", &index.to_string()]),
        "true".into(),
    ));
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("bottom-navigation-item"),
    ));
    style_helpers::themed_attributes(bottom_navigation_item_style(), pairs)
}

fn bottom_navigation_root_style() -> Style {
    css_with_theme!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-around;
        gap: ${gap};
        padding: ${padding_y}px ${padding_x}px;
        background: ${background};
        border-radius: ${radius};
        box-shadow: ${shadow};
        min-height: ${min_height}px;
    "#,
        gap = theme.spacing(2),
        padding_y = theme.spacing(1),
        padding_x = theme.spacing(2),
        background = theme.palette.background_paper.clone(),
        radius = format!("{}px", theme.joy.radius),
        shadow = theme.joy.shadow.surface.clone(),
        min_height = theme.spacing(8)
    )
}

fn bottom_navigation_item_style() -> Style {
    css_with_theme!(
        r#"
        appearance: none;
        background: transparent;
        border: none;
        padding: ${padding_y}px ${padding_x}px;
        display: inline-flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: ${color};
        font-family: ${font_family};
        font-size: ${font_size};
        line-height: 1.2;
        cursor: pointer;
        position: relative;
        transition: color 120ms ease;
        &[aria-selected="true"] {
            color: ${active_color};
        }
        &[data-disabled="true"] {
            opacity: 0.38;
            cursor: default;
        }
    "#,
        padding_y = theme.spacing(1),
        padding_x = theme.spacing(2),
        color = theme.palette.text_secondary.clone(),
        active_color = theme.palette.primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.button)
    )
}

/// React adapter that renders the bottom navigation into HTML.
pub mod react {
    use super::*;

    /// Render the bottom navigation for React SSR pipelines.
    pub fn render_bottom_navigation(props: BottomNavigationAdapterProps<'_>) -> String {
        super::render_bottom_navigation_html(props)
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render the bottom navigation for snapshot testing.
    pub fn render_bottom_navigation(props: BottomNavigationAdapterProps<'_>) -> String {
        super::render_bottom_navigation_html(props)
    }
}

/// Leptos adapter keeping parity with React/Yew output.
pub mod leptos {
    use super::*;

    /// Render the bottom navigation into deterministic HTML.
    pub fn render_bottom_navigation(props: BottomNavigationAdapterProps<'_>) -> String {
        super::render_bottom_navigation_html(props)
    }
}

/// Sycamore adapter delegating to the shared renderer.
pub mod sycamore {
    use super::*;

    /// Render the bottom navigation for Sycamore SSR flows.
    pub fn render_bottom_navigation(props: BottomNavigationAdapterProps<'_>) -> String {
        super::render_bottom_navigation_html(props)
    }
}

/// Dioxus adapter mirroring every other integration.
pub mod dioxus {
    use super::*;

    /// Render the bottom navigation into HTML markup.
    pub fn render_bottom_navigation(props: BottomNavigationAdapterProps<'_>) -> String {
        super::render_bottom_navigation_html(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::bottom_navigation::{
        BottomNavigationActivationMode, BottomNavigationState,
    };
    use rustic_ui_headless::ControlStrategy;

    fn build_state() -> BottomNavigationState {
        BottomNavigationState::new(
            3,
            Some(1),
            BottomNavigationActivationMode::Automatic,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        )
    }

    fn build_props<'a>(state: &'a BottomNavigationState) -> BottomNavigationAdapterProps<'a> {
        BottomNavigationAdapterProps {
            state,
            attributes: state.root_attributes().id("nav"),
            items: &[
                BottomNavigationItemDescriptor {
                    id: Some("home"),
                    controls: None,
                    content: "<span>Home</span>",
                },
                BottomNavigationItemDescriptor {
                    id: Some("files"),
                    controls: None,
                    content: "<span>Files</span>",
                },
                BottomNavigationItemDescriptor {
                    id: Some("settings"),
                    controls: None,
                    content: "<span>Settings</span>",
                },
            ],
            on_select_event: Some("nav-select"),
        }
    }

    #[test]
    fn root_attributes_include_automation_hooks() {
        let state = build_state();
        let rendered = render_bottom_navigation(build_props(&state));
        assert!(rendered
            .root_attributes()
            .iter()
            .any(|(k, v)| k == "data-component"
                && *v == style_helpers::component_marker("bottom-navigation")));
        assert!(rendered
            .root_attributes()
            .iter()
            .any(|(k, v)| k == "data-on-select" && v == "nav-select"));
    }

    #[test]
    fn html_renderer_emits_buttons_for_each_item() {
        let state = build_state();
        let html = render_bottom_navigation_html(build_props(&state));
        assert!(html.contains("<nav"));
        assert!(html.contains("role=\"tablist\""));
        assert!(html.matches("<button").count() >= 3);
    }
}
