//! Material flavored drawer utilities that decorate the headless
//! [`DrawerState`](mui_headless::drawer::DrawerState).
//!
//! The helpers expose automation-ready attribute vectors and HTML renderers so
//! framework adapters can stay lean. Styling is centralized through
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) which pulls palette,
//! spacing, breakpoints and Joy token values from the active theme. The
//! resulting API keeps modal and persistent drawers visually aligned across
//! Yew/Leptos/Dioxus/Sycamore without forcing each team to juggle bespoke CSS.

use mui_headless::drawer::{
    DrawerBackdropAttributes, DrawerState, DrawerSurfaceAttributes, DrawerVariant,
};
use mui_styled_engine::{css_with_theme, Style};

/// Convert surface attributes into key/value pairs enriched with automation
/// hints.
#[must_use]
pub fn drawer_surface_attributes(
    state: &DrawerState,
    attrs: DrawerSurfaceAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(9);
    pairs.push(("role".into(), attrs.role().into()));
    let (aria_modal_key, aria_modal_value) = attrs.aria_modal();
    pairs.push((aria_modal_key.into(), aria_modal_value.into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_describedby() {
        pairs.push((key.into(), value.into()));
    }
    let (anchor_key, anchor_value) = attrs.data_anchor();
    pairs.push((anchor_key.into(), anchor_value.into()));
    pairs.push(("data-open".into(), state.is_open().to_string()));
    pairs.push((
        "data-variant".into(),
        match state.variant() {
            DrawerVariant::Modal => "modal",
            DrawerVariant::Persistent => "persistent",
        }
        .to_string(),
    ));
    pairs
}

/// Render the drawer surface to HTML using the shared renderer.
#[must_use]
pub fn render_drawer_surface_html(
    state: &DrawerState,
    attrs: DrawerSurfaceAttributes<'_>,
    body: &str,
) -> String {
    crate::render_helpers::render_element_html(
        "div",
        drawer_surface_style(),
        drawer_surface_attributes(state, attrs),
        body,
    )
}

/// Gather backdrop attributes and automation markers.
#[must_use]
pub fn drawer_backdrop_attributes(
    state: &DrawerState,
    attrs: DrawerBackdropAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(4);
    let (aria_hidden_key, aria_hidden_value) = attrs.aria_hidden();
    pairs.push((aria_hidden_key.into(), aria_hidden_value.into()));
    pairs.push(("data-open".into(), state.is_open().to_string()));
    pairs.push((
        "data-variant".into(),
        match state.variant() {
            DrawerVariant::Modal => "modal",
            DrawerVariant::Persistent => "persistent",
        }
        .to_string(),
    ));
    pairs
}

/// Render the backdrop if the variant requires one.
#[must_use]
pub fn render_drawer_backdrop_html(
    state: &DrawerState,
    attrs: DrawerBackdropAttributes<'_>,
) -> Option<String> {
    if !attrs.is_visible() {
        return None;
    }
    Some(crate::render_helpers::render_backdrop_html(
        drawer_backdrop_style(),
        drawer_backdrop_attributes(state, attrs),
    ))
}

/// Drawer surface styling shared across frameworks.
fn drawer_surface_style() -> Style {
    css_with_theme!(
        r#"
        position: fixed;
        top: 0;
        bottom: 0;
        background: ${background};
        color: ${text};
        display: flex;
        flex-direction: column;
        gap: ${gap};
        padding: ${padding_mobile};
        width: min(100%, ${width_mobile});
        max-width: 100%;
        box-shadow: 0 24px 54px -36px color-mix(in srgb, ${shadow_base} 65%, transparent);
        transition: transform 200ms ease, opacity 200ms ease, visibility 200ms ease;
        &[data-anchor="start"] {
            left: 0;
            transform: translateX(-100%);
        }
        &[data-anchor="end"] {
            right: 0;
            transform: translateX(100%);
        }
        &[data-anchor="top"] {
            top: 0;
            height: min(100%, ${width_mobile});
            transform: translateY(-100%);
        }
        &[data-anchor="bottom"] {
            bottom: 0;
            height: min(100%, ${width_mobile});
            transform: translateY(100%);
        }
        &[data-open="true"][data-anchor="start"],
        &[data-open="true"][data-anchor="end"] {
            transform: translateX(0);
        }
        &[data-open="true"][data-anchor="top"],
        &[data-open="true"][data-anchor="bottom"] {
            transform: translateY(0);
        }
        &[data-open="false"] {
            opacity: 0;
            visibility: hidden;
        }
        &[data-open="true"] {
            opacity: 1;
            visibility: visible;
        }
        @media (min-width: ${sm}px) {
            padding: ${padding_tablet};
            width: min(100%, ${width_tablet});
        }
        @media (min-width: ${lg}px) {
            padding: ${padding_desktop};
            width: ${width_desktop};
        }
    "#,
        background = theme.palette.background_paper.clone(),
        text = theme.palette.text_primary.clone(),
        gap = format!("{}px", theme.spacing(2)),
        padding_mobile = format!("{}px", theme.spacing(2)),
        padding_tablet = format!("{}px", theme.spacing(3)),
        padding_desktop = format!("{}px", theme.spacing(4)),
        width_mobile = format!("{}px", theme.spacing(40)),
        width_tablet = format!("{}px", theme.spacing(48)),
        width_desktop = format!("{}px", theme.spacing(56)),
        shadow_base = theme.palette.neutral.clone(),
        sm = theme.breakpoints.sm,
        lg = theme.breakpoints.lg,
    )
}

/// Backdrop styling controlling scrim opacity and transitions.
fn drawer_backdrop_style() -> Style {
    css_with_theme!(
        r#"
        position: fixed;
        inset: 0;
        background: color-mix(in srgb, ${scrim} 65%, transparent);
        transition: opacity 200ms ease;
        opacity: 0;
        &[data-open="true"] {
            opacity: 1;
        }
        &[data-open="false"] {
            pointer-events: none;
        }
    "#,
        scrim = theme.palette.text_primary.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::drawer::{DrawerAnchor, DrawerVariant};

    fn sample_state(open: bool, variant: DrawerVariant) -> DrawerState {
        DrawerState::new(
            open,
            unsafe { std::mem::transmute(1u8) },
            variant,
            DrawerAnchor::Start,
        )
    }

    #[test]
    fn surface_attributes_include_variant_and_open_flags() {
        let state = sample_state(true, DrawerVariant::Modal);
        let attrs = drawer_surface_attributes(&state, state.surface_attributes());
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "dialog"));
        assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-variant" && v == "modal"));
    }

    #[test]
    fn render_drawer_surface_html_includes_children() {
        let state = sample_state(false, DrawerVariant::Persistent);
        let html =
            render_drawer_surface_html(&state, state.surface_attributes(), "<nav>Links</nav>");
        assert!(html.contains("class=\""));
        assert!(html.contains("data-open=\"false\""));
        assert!(html.contains("<nav>Links</nav>"));
    }

    #[test]
    fn backdrop_renders_only_for_modal_variants() {
        let modal = sample_state(true, DrawerVariant::Modal);
        let backdrop = render_drawer_backdrop_html(&modal, modal.backdrop_attributes());
        assert!(backdrop.is_some());

        let persistent = sample_state(true, DrawerVariant::Persistent);
        let none = render_drawer_backdrop_html(&persistent, persistent.backdrop_attributes());
        assert!(none.is_none());
    }
}
