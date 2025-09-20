//! Material flavored tab list utilities that layer presentation on top of the
//! headless [`TabsState`](mui_headless::tabs::TabsState).
//!
//! The functions here intentionally avoid framework specifics. Instead, they
//! expose reusable attribute collections and HTML renderers that adapters can
//! forward directly into Yew, Leptos, Dioxus, Sycamore or any server-side
//! runtime. Styling is centralized through [`css_with_theme!`]
//! (mui_styled_engine::css_with_theme) so palette updates, typography tweaks and
//! responsive breakpoints propagate automatically across all integrations.
//!
//! Downstream teams can lean on the documented helpers to assemble enterprise
//! grade tab experiences without re-implementing indicator logic, orientation
//! handling or accessibility wiring. This minimizes repetitive engineering work
//! and enables automated scaffolding tools to stitch together consistent tab
//! shells for large product portfolios.

use mui_headless::tabs::{TabListAttributes, TabsOrientation, TabsState};
use mui_styled_engine::{css_with_theme, Style};

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
            mui_headless::tabs::ActivationMode::Automatic => "automatic",
            mui_headless::tabs::ActivationMode::Manual => "manual",
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

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::tabs::{ActivationMode, TabsOrientation};

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
