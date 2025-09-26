//! Material flavored tab list utilities that layer presentation on top of the
//! headless [`TabsState`](rustic_ui_headless::tabs::TabsState).
//!
//! The functions here intentionally avoid framework specifics. Instead, they
//! expose reusable attribute collections and HTML renderers that adapters can
//! forward directly into Yew, Leptos, Dioxus, Sycamore or any server-side
//! runtime. Styling is centralized through [`css_with_theme!`]
//! (rustic_ui_styled_engine::css_with_theme) so palette updates, typography tweaks and
//! responsive breakpoints propagate automatically across all integrations.
//!
//! Downstream teams can lean on the documented helpers to assemble enterprise
//! grade tab experiences without re-implementing indicator logic, orientation
//! handling or accessibility wiring. This minimizes repetitive engineering work
//! and enables automated scaffolding tools to stitch together consistent tab
//! shells for large product portfolios.

use rustic_ui_headless::tabs::{TabListAttributes, TabsOrientation, TabsState};
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_system::{
    responsive::{viewport_width, Responsive},
    stack::{StackDirection, StackStyleInputs},
    theme::Theme,
};

/// Configuration shared across framework adapters describing how the tab list
/// should respond to viewport changes.
///
/// Enterprise teams frequently need horizontal tab bars on compact screens yet
/// prefer vertical navigation rails on larger breakpoints.  Modeling that
/// behaviour with [`Responsive`] keeps the decision declarative and allows the
/// adapters below to lean on the centralized resolution logic provided by
/// `mui-system`.
#[derive(Clone, Debug)]
pub struct TabListLayoutOptions {
    /// Responsive orientation strategy. Defaults to horizontal tabs on small
    /// screens and vertical rails from the medium breakpoint upwards.
    pub orientation: Responsive<TabsOrientation>,
    /// Responsive spacing factors expressed as `Theme::spacing` multipliers.
    /// The values are converted into pixel based gap declarations for the
    /// layout container. Keeping this in spacing units ensures global design
    /// token tweaks automatically cascade through the adapters.
    pub spacing: Responsive<u16>,
}

impl Default for TabListLayoutOptions {
    fn default() -> Self {
        Self {
            orientation: Responsive {
                xs: TabsOrientation::Horizontal,
                sm: None,
                md: Some(TabsOrientation::Vertical),
                lg: Some(TabsOrientation::Vertical),
                xl: Some(TabsOrientation::Vertical),
            },
            spacing: Responsive::constant(2),
        }
    }
}

impl TabListLayoutOptions {
    /// Resolve the orientation for the provided viewport width using the
    /// configured [`Responsive`] declaration.
    #[must_use]
    pub fn resolve_orientation(&self, theme: &Theme, viewport: u32) -> TabsOrientation {
        self.orientation.resolve(viewport, &theme.breakpoints)
    }

    fn resolve_spacing(&self, theme: &Theme) -> Responsive<String> {
        Responsive {
            xs: format!("{}px", theme.spacing(self.spacing.xs)),
            sm: self
                .spacing
                .sm
                .map(|value| format!("{}px", theme.spacing(value))),
            md: self
                .spacing
                .md
                .map(|value| format!("{}px", theme.spacing(value))),
            lg: self
                .spacing
                .lg
                .map(|value| format!("{}px", theme.spacing(value))),
            xl: self
                .spacing
                .xl
                .map(|value| format!("{}px", theme.spacing(value))),
        }
    }
}

/// Properties consumed by the framework adapters when rendering a responsive
/// tab list.
///
/// The struct intentionally mirrors the shape of adapters across every
/// supported framework so internal automation can generate integration tests
/// and documentation snippets without juggling bespoke prop names.
#[derive(Clone, Debug)]
pub struct TabListProps<'a> {
    /// Resolved tab state produced by `mui-headless`.
    pub state: &'a TabsState,
    /// Attribute builder describing identifiers and labelling metadata.
    pub attributes: TabListAttributes<'a>,
    /// Pre-rendered markup for the tab buttons.
    pub children: &'a str,
    /// Layout directives shared across all adapters.
    pub layout: &'a TabListLayoutOptions,
    /// Active theme driving spacing and breakpoint resolution.
    pub theme: &'a Theme,
    /// Optional explicit viewport width used during SSR driven tests. When not
    /// supplied we fall back to [`viewport_width`] which inspects the runtime
    /// environment.
    pub viewport: Option<u32>,
    /// Optional event channel identifier allowing frameworks to wire custom
    /// activation hooks. The adapters surface the value through a
    /// `data-on-activate` attribute so orchestration layers can bind to the
    /// event without stringly-typed duplication.
    pub on_activate_event: Option<&'a str>,
}

/// Convert a `mui-headless` attribute builder into a stable vector of HTML
/// attributes ready for SSR pipelines or client side adapters.
///
/// The function augments the ARIA metadata emitted by `mui-headless` with
/// automation friendly data attributes so design systems can target the
/// orientation and state without hardcoding selectors in every application.
#[must_use]
pub fn tab_list_attributes(
    state: &TabsState,
    attrs: TabListAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(6);
    pairs.push(("role".into(), attrs.role().into()));
    let (orientation_attr, orientation_value) = attrs.orientation();
    pairs.push((orientation_attr.into(), orientation_value.into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.labelledby() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push((
        "data-orientation".into(),
        state.orientation().as_aria().to_string(),
    ));
    pairs.push((
        "data-activation".into(),
        match state.activation_mode() {
            rustic_ui_headless::tabs::ActivationMode::Automatic => "automatic",
            rustic_ui_headless::tabs::ActivationMode::Manual => "manual",
        }
        .to_string(),
    ));
    pairs
}

/// Render the tab list into serialized HTML using theme aware styling.
///
/// Adapters can call this helper directly for SSR scenarios or inspect the
/// resulting string in snapshot tests. Client side renderers typically re-use
/// [`tab_list_style`] alongside [`tab_list_attributes`] for the same output.
#[must_use]
pub fn render_tab_list_html(
    state: &TabsState,
    attrs: TabListAttributes<'_>,
    children: &str,
) -> String {
    crate::render_helpers::render_element_html(
        "div",
        tab_list_style(state.orientation()),
        tab_list_attributes(state, attrs),
        children,
    )
}

/// Generates the themed style used by the tab list container.
fn tab_list_style(_orientation: TabsOrientation) -> Style {
    css_with_theme!(
        r#"
        display: flex;
        flex-wrap: nowrap;
        align-items: center;
        gap: ${gap_small};
        padding: ${padding_small};
        margin: 0;
        list-style: none;
        background: ${background};
        border-radius: ${radius};
        border: 1px solid ${border_color};
        overflow-x: auto;
        scrollbar-width: none;
        &::-webkit-scrollbar { display: none; }
        &[data-orientation="horizontal"] {
            flex-direction: row;
        }
        &[data-orientation="vertical"] {
            flex-direction: column;
            align-items: stretch;
            min-width: ${vertical_min_width};
            max-height: 100%;
        }
        @media (min-width: ${sm}px) {
            gap: ${gap_large};
            padding: ${padding_large};
        }
        @media (min-width: ${md}px) {
            border-color: transparent;
            background: transparent;
        }
    "#,
        gap_small = format!("{}px", theme.spacing(1)),
        gap_large = format!("{}px", theme.spacing(2)),
        padding_small = format!("{}px", theme.spacing(1)),
        padding_large = format!("{}px", theme.spacing(2)),
        background = theme.palette.background_paper.clone(),
        border_color = format!(
            "color-mix(in srgb, {} 24%, transparent)",
            theme.palette.neutral.clone()
        ),
        radius = format!("{}px", theme.joy.radius),
        vertical_min_width = format!("{}px", theme.spacing(22)),
        sm = theme.breakpoints.sm,
        md = theme.breakpoints.md,
    )
}

struct TabListRenderParams<'a> {
    state: &'a TabsState,
    attributes: TabListAttributes<'a>,
    children: &'a str,
    layout: &'a TabListLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
    on_activate_event: Option<&'a str>,
}

impl<'a> From<TabListProps<'a>> for TabListRenderParams<'a> {
    fn from(props: TabListProps<'a>) -> Self {
        Self {
            state: props.state,
            attributes: props.attributes,
            children: props.children,
            layout: props.layout,
            theme: props.theme,
            viewport: props.viewport.unwrap_or_else(viewport_width),
            on_activate_event: props.on_activate_event,
        }
    }
}

fn orientation_to_direction(orientation: TabsOrientation) -> StackDirection {
    match orientation {
        TabsOrientation::Horizontal => StackDirection::Row,
        TabsOrientation::Vertical => StackDirection::Column,
    }
}

fn render_tab_list_with_layout(params: TabListRenderParams<'_>) -> String {
    let orientation = params
        .layout
        .resolve_orientation(params.theme, params.viewport);
    debug_assert_eq!(
        params.state.orientation(),
        orientation,
        "TabsState orientation should match responsive layout configuration",
    );

    let spacing = params.layout.resolve_spacing(params.theme);
    let stack_css = rustic_ui_system::stack::build_stack_style(
        params.viewport,
        &params.theme.breakpoints,
        StackStyleInputs {
            direction: Some(orientation_to_direction(orientation)),
            spacing: Some(&spacing),
            align_items: Some("stretch"),
            justify_content: Some("flex-start"),
            sx: None,
        },
    );
    let stack_style =
        Style::new(stack_css).expect("mui-system stack builder should emit valid CSS");
    let inner_html = render_tab_list_html(params.state, params.attributes, params.children);

    let mut outer_attrs = Vec::with_capacity(1);
    if let Some(event) = params.on_activate_event {
        outer_attrs.push(("data-on-activate".to_string(), event.to_string()));
    }

    crate::render_helpers::render_element_html("div", stack_style, outer_attrs, &inner_html)
}

/// Adapter targeting server-rendered React integrations.  The adapter returns a
/// HTML string so Node driven pipelines can stitch the markup into templates
/// before hydration occurs on the client.  React specific behaviour is minimal
/// because the shared helpers already emit declarative attributes and scoped
/// classes.
pub mod react {
    use super::*;

    /// Render the responsive tab list into HTML markup.
    pub fn render_tab_list(props: TabListProps<'_>) -> String {
        super::render_tab_list_with_layout(props.into())
    }
}

/// Adapter targeting the [`yew`] framework.  Rendering is performed entirely by
/// the shared helper which keeps parity with the React/Leptos variants and
/// ensures automation tools can diff the serialized HTML across all
/// integrations.
pub mod yew {
    use super::*;

    /// Render the tab list into HTML markup for snapshot testing or static
    /// generation pipelines.
    pub fn render_tab_list(props: TabListProps<'_>) -> String {
        super::render_tab_list_with_layout(props.into())
    }
}

/// Adapter targeting the [`leptos`] framework.  Mirrors the Yew/React
/// implementation so teams can swap frameworks without rewriting orchestration
/// logic.
pub mod leptos {
    use super::*;

    /// Render the tab list into HTML markup for SSR scenarios.
    pub fn render_tab_list(props: TabListProps<'_>) -> String {
        super::render_tab_list_with_layout(props.into())
    }
}

/// Adapter targeting the [`sycamore`] framework.  Delegates straight to the
/// shared renderer keeping the markup deterministic across integrations.
pub mod sycamore {
    use super::*;

    /// Render the tab list into serialized HTML.
    pub fn render_tab_list(props: TabListProps<'_>) -> String {
        super::render_tab_list_with_layout(props.into())
    }
}

/// Adapter targeting the [`dioxus`] framework.  The implementation mirrors all
/// other adapters to guarantee consistent semantics and layout behaviour.
pub mod dioxus {
    use super::*;

    /// Render the tab list into HTML markup.
    pub fn render_tab_list(props: TabListProps<'_>) -> String {
        super::render_tab_list_with_layout(props.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::tabs::{ActivationMode, TabsOrientation};

    fn sample_state(orientation: TabsOrientation) -> TabsState {
        TabsState::new(
            3,
            Some(1),
            ActivationMode::Automatic,
            orientation,
            // `ControlStrategy` lives in a private module. The discriminant
            // order is documented so we transmute the `Uncontrolled` variant
            // for testing just like other integration suites in this crate.
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        )
    }

    #[test]
    fn tab_list_attributes_include_orientation_and_activation() {
        let state = sample_state(TabsOrientation::Horizontal);
        let attrs = tab_list_attributes(&state, state.list_attributes().id("tabs"));
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "tablist"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-orientation" && v == "horizontal"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-orientation" && v == "horizontal"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-activation" && v == "automatic"));
    }

    #[test]
    fn render_tab_list_html_includes_scoped_class_and_children() {
        let state = sample_state(TabsOrientation::Vertical);
        let html = render_tab_list_html(&state, state.list_attributes(), "<button>One</button>");
        assert!(html.contains("class=\""));
        assert!(html.contains("data-orientation=\"vertical\""));
        assert!(html.contains("<button>One</button>"));
    }
}
