//! Material themed application bar powered by the headless [`AppBarState`].
//!
//! The component centralises automation identifiers, analytics hooks and
//! accessibility metadata so every framework adapter emits the same banner
//! semantics. Styling remains theme-driven via `css_with_theme!`, while the
//! headless state wires in telemetry defaults and SSR friendly attribute
//! builders.

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_headless::app_bar::{
    AppBarAnalytics as HeadlessAnalytics, AppBarColor as HeadlessColor, AppBarSize as HeadlessSize,
    AppBarState,
};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, use_theme, Style, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

pub use crate::macros::{Color as AppBarColor, Size as AppBarSize, Variant as AppBarVariant};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
const COMPONENT_NAME: &str = "app-bar";

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_style(theme: &Theme, color: AppBarColor, size: AppBarSize) -> (String, &'static str) {
    let bg = match color {
        AppBarColor::Primary => theme.palette.primary.clone(),
        AppBarColor::Secondary => theme.palette.secondary.clone(),
    };
    let height = match size {
        AppBarSize::Small => "48px",
        AppBarSize::Medium => "64px",
        AppBarSize::Large => "80px",
    };
    (bg, height)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn app_bar_style(theme: &Theme, color: AppBarColor, size: AppBarSize) -> Style {
    let (bg, height) = resolve_style(theme, color, size);
    css_with_theme!(
        theme,
        r#"
        background: ${bg};
        height: ${height};
        display: flex;
        align-items: center;
        padding: 0 16px;
    "#,
        bg = bg,
        height = height
    )
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn map_color(color: AppBarColor) -> HeadlessColor {
    match color {
        AppBarColor::Primary => HeadlessColor::Primary,
        AppBarColor::Secondary => HeadlessColor::Secondary,
    }
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn map_size(size: AppBarSize) -> HeadlessSize {
    match size {
        AppBarSize::Small => HeadlessSize::Small,
        AppBarSize::Medium => HeadlessSize::Medium,
        AppBarSize::Large => HeadlessSize::Large,
    }
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn build_headless_state(
    title: String,
    aria_label: String,
    color: AppBarColor,
    size: AppBarSize,
    automation_id: Option<&str>,
    analytics_view_id: Option<&str>,
    analytics_interaction_id: Option<&str>,
    svg_title_id: Option<&str>,
) -> (AppBarState, Option<String>) {
    let sanitized_label = if aria_label.trim().is_empty() {
        title.clone()
    } else {
        aria_label
    };

    let automation_dom_id = automation_id.map(|raw| {
        crate::style_helpers::automation_id(
            COMPONENT_NAME,
            Some(raw),
            crate::style_helpers::EMPTY_SEGMENTS,
        )
    });

    let mut state = AppBarState::new(title)
        .with_aria_label(sanitized_label)
        .with_color(map_color(color))
        .with_size(map_size(size));

    if let Some(id) = automation_dom_id.clone() {
        state = state.with_automation_id(id);
    }

    let mut analytics = HeadlessAnalytics::default();
    if let Some(view) = analytics_view_id {
        analytics = analytics.with_view_id(view);
    }
    if let Some(interaction) = analytics_interaction_id {
        analytics = analytics.with_interaction_id(interaction);
    }
    state = state.with_analytics(analytics);

    if let Some(svg_id) = svg_title_id {
        state = state.with_svg_title_id(svg_id);
    }

    (state, automation_dom_id)
}

#[cfg(any(feature = "yew", feature = "leptos"))]
crate::material_component_props!(AppBarProps {
    /// Title displayed inside the app bar.
    title: String,
    /// Accessible label announced by screen readers describing the app bar.
    aria_label: String,
    /// Optional automation identifier appended to the banner and SVG helpers.
    automation_id: Option<String>,
    /// Analytics impression identifier forwarded to `data-analytics-view-id`.
    analytics_view_id: Option<String>,
    /// Analytics interaction identifier forwarded to `data-analytics-interaction-id`.
    analytics_interaction_id: Option<String>,
    /// Optional SVG title id for inline branding elements.
    svg_title_id: Option<String>,
});

#[cfg(feature = "yew")]
mod yew_impl {
    //! Yew adapter rendering the [`AppBar`] as a semantic `<header>` element.
    //!
    //! The adapter mirrors SSR output by sourcing ARIA and telemetry attributes
    //! from the shared [`AppBarState`]. Automation identifiers are formatted via
    //! `style_helpers::automation_id` so QA selectors remain consistent across
    //! frameworks.
    use super::*;

    /// High level navigation bar rendered at the top of the application.
    #[function_component(AppBar)]
    pub fn app_bar(props: &AppBarProps) -> Html {
        let theme = use_theme();
        let style = app_bar_style(&theme, props.color, props.size);
        let class = crate::style_helpers::themed_class(style);

        let (state, automation_dom_id) = build_headless_state(
            props.title.clone(),
            props.aria_label.clone(),
            props.color,
            props.size,
            props.automation_id.as_deref(),
            props.analytics_view_id.as_deref(),
            props.analytics_interaction_id.as_deref(),
            props.svg_title_id.as_deref(),
        );

        let component_marker = crate::style_helpers::component_marker(COMPONENT_NAME);

        html! {
            <header
                class={class}
                id={automation_dom_id.clone()}
                role="banner"
                aria-label={state.aria_label().to_string()}
                data-component={component_marker}
                data-color={state.color().as_str()}
                data-size={state.size().as_str()}
                data-automation-id={state.automation_id().map(|id| id.to_string())}
                data-analytics-view-id={state.analytics().view_id().map(|id| id.to_string())}
                data-analytics-interaction-id={state.analytics().interaction_id().map(|id| id.to_string())}
            >
                { &props.title }
            </header>
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::AppBar;

#[cfg(feature = "leptos")]
mod leptos_impl {
    //! Leptos adapter rendering the [`AppBar`] as a semantic `<header>` element.
    //!
    //! Telemetry and automation identifiers flow through the headless state to
    //! keep SSR and CSR output aligned across frameworks.
    use super::*;
    use leptos::*;

    /// High level navigation bar rendered at the top of the application.
    #[component]
    pub fn AppBar(props: AppBarProps) -> impl IntoView {
        let theme = use_theme();
        let class =
            crate::style_helpers::themed_class(app_bar_style(&theme, props.color, props.size));

        let (state, automation_dom_id) = build_headless_state(
            props.title.clone(),
            props.aria_label.clone(),
            props.color,
            props.size,
            props.automation_id.as_deref(),
            props.analytics_view_id.as_deref(),
            props.analytics_interaction_id.as_deref(),
            props.svg_title_id.as_deref(),
        );

        let component_marker = crate::style_helpers::component_marker(COMPONENT_NAME);
        let automation_dom_id = automation_dom_id;

        view! {
            <header
                class=class
                id=automation_dom_id
                role="banner"
                aria-label=state.aria_label().to_string()
                data-component=component_marker
                data-color=state.color().as_str()
                data-size=state.size().as_str()
                data-automation-id={state.automation_id().map(|id| id.to_string())}
                data-analytics-view-id={state.analytics().view_id().map(|id| id.to_string())}
                data-analytics-interaction-id={state.analytics().interaction_id().map(|id| id.to_string())}
            >
                {props.title}
            </header>
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::AppBar;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use AppBarProps;

/// Adapter targeting the [`dioxus`] framework.
///
/// Generates a themed `<header>` element and wires up ARIA attributes so the
/// navigation region is announced correctly by assistive technologies. The
/// headless state keeps telemetry and automation hooks aligned with the Yew and
/// Leptos adapters.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        /// Title displayed inside the app bar.
        pub title: String,
        /// Accessible label announced by assistive technologies.
        pub aria_label: String,
        /// Themed color palette applied to the background.
        pub color: AppBarColor,
        /// Height variant influencing overall bar size.
        pub size: AppBarSize,
        /// Optional automation identifier forwarded to `data-automation-id`.
        pub automation_id: Option<String>,
        /// Optional analytics impression identifier.
        pub analytics_view_id: Option<String>,
        /// Optional analytics interaction identifier.
        pub analytics_interaction_id: Option<String>,
        /// Optional SVG title id for inline logos.
        pub svg_title_id: Option<String>,
    }

    /// Render the app bar into a `<header>` tag using a theme derived class.
    pub fn render(props: &AppBarProps) -> String {
        let theme = use_theme();
        let (state, automation_dom_id) = build_headless_state(
            props.title.clone(),
            props.aria_label.clone(),
            props.color,
            props.size,
            props.automation_id.as_deref(),
            props.analytics_view_id.as_deref(),
            props.analytics_interaction_id.as_deref(),
            props.svg_title_id.as_deref(),
        );

        let mut attrs = state
            .html_attributes()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Vec<_>>();
        attrs.push((
            "data-component".to_string(),
            crate::style_helpers::component_marker(COMPONENT_NAME),
        ));
        attrs.push(("data-color".to_string(), state.color().as_str().to_string()));
        attrs.push(("data-size".to_string(), state.size().as_str().to_string()));
        if let Some(id) = automation_dom_id {
            attrs.push(("id".to_string(), id));
        }

        let attr_string = crate::style_helpers::themed_attributes_html(
            app_bar_style(&theme, props.color, props.size),
            attrs,
        );
        format!("<header {}>{}</header>", attr_string, props.title)
    }
}

/// Adapter targeting the [`sycamore`] framework.
///
/// Produces an accessible `<header>` with classes derived from the active
/// [`Theme`] and optional telemetry metadata.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore variant of [`AppBar`] sharing identical props with other adapters.
    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        /// Title displayed inside the app bar.
        pub title: String,
        /// Accessible label describing the banner region.
        pub aria_label: String,
        /// Background color pulled from the theme palette.
        pub color: AppBarColor,
        /// Height variant controlling the overall size.
        pub size: AppBarSize,
        /// Optional automation identifier forwarded to `data-automation-id`.
        pub automation_id: Option<String>,
        /// Optional analytics impression identifier.
        pub analytics_view_id: Option<String>,
        /// Optional analytics interaction identifier.
        pub analytics_interaction_id: Option<String>,
        /// Optional SVG title id for inline logos.
        pub svg_title_id: Option<String>,
    }

    /// Render the app bar into plain HTML with themed styling and ARIA attributes.
    pub fn render(props: &AppBarProps) -> String {
        let theme = use_theme();
        let (state, automation_dom_id) = build_headless_state(
            props.title.clone(),
            props.aria_label.clone(),
            props.color,
            props.size,
            props.automation_id.as_deref(),
            props.analytics_view_id.as_deref(),
            props.analytics_interaction_id.as_deref(),
            props.svg_title_id.as_deref(),
        );

        let mut attrs = state
            .html_attributes()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Vec<_>>();
        attrs.push((
            "data-component".to_string(),
            crate::style_helpers::component_marker(COMPONENT_NAME),
        ));
        attrs.push(("data-color".to_string(), state.color().as_str().to_string()));
        attrs.push(("data-size".to_string(), state.size().as_str().to_string()));
        if let Some(id) = automation_dom_id {
            attrs.push(("id".to_string(), id));
        }

        let attr_string = crate::style_helpers::themed_attributes_html(
            app_bar_style(&theme, props.color, props.size),
            attrs,
        );
        format!("<header {}>{}</header>", attr_string, props.title)
    }
}
