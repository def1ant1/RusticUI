//! Presentation helpers for individual tabs layered on top of the
//! headless [`TabAttributes`](mui_headless::tab::TabAttributes).
//!
//! Each helper returns automation-friendly attribute collections and HTML
//! snippets so adapters can stay declarative. The styling leans on
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) to translate design
//! tokens (typography ramp, Joy radius, palette colors) into scoped CSS without
//! duplicating literals across applications.  Responsive breakpoints are baked
//! into the style so padding and typography scale seamlessly from mobile to
//! desktop layouts.

use mui_headless::tab::TabAttributes;
use mui_headless::tabs::{TabsOrientation, TabsState};
use mui_styled_engine::{css_with_theme, Style};

/// Collect the attributes required to render a tab element.
///
/// The function merges the ARIA metadata from `mui-headless` with data attributes
/// that downstream CSS (or analytics tooling) can leverage without needing to
/// re-implement state derivation.
#[must_use]
pub fn tab_attributes(state: &TabsState, attrs: TabAttributes<'_>) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(8);
    pairs.push(("role".into(), attrs.role().into()));
    let (aria_selected_key, aria_selected_value) = attrs.aria_selected();
    pairs.push((aria_selected_key.into(), aria_selected_value.into()));
    let (tabindex_key, tabindex_value) = attrs.tabindex();
    pairs.push((tabindex_key.into(), tabindex_value.into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_controls() {
        pairs.push((key.into(), value.into()));
    }
    pairs.push(("data-selected".into(), attrs.is_selected().to_string()));
    pairs.push(("data-focused".into(), attrs.is_focused().to_string()));
    pairs.push((
        "data-orientation".into(),
        state.orientation().as_aria().to_string(),
    ));
    pairs
}

/// Render a tab element into serialized HTML using the shared renderer.
#[must_use]
pub fn render_tab_html(state: &TabsState, attrs: TabAttributes<'_>, label: &str) -> String {
    crate::render_helpers::render_element_html(
        "button",
        tab_style(state.orientation()),
        tab_attributes(state, attrs),
        label,
    )
}

/// Generates the theme-driven style shared by every tab.
fn tab_style(_orientation: TabsOrientation) -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: ${gap};
        padding: ${padding_y} ${padding_x};
        min-width: ${min_width};
        background: transparent;
        color: ${color_inactive};
        font-family: ${font_family};
        font-size: ${font_size_small};
        font-weight: ${font_weight_medium};
        line-height: ${line_height};
        border: none;
        border-radius: ${radius};
        cursor: pointer;
        transition: color 160ms ease, background 160ms ease;
        text-transform: none;
        &[data-selected="true"] {
            color: ${color_active};
            font-weight: ${font_weight_bold};
        }
        &[data-selected="true"]::after {
            content: "";
            position: absolute;
            left: 16%;
            right: 16%;
            bottom: 0;
            height: ${indicator_thickness};
            background: ${indicator_color};
            border-radius: ${indicator_radius};
        }
        &[data-orientation="vertical"][data-selected="true"]::after {
            left: 0;
            right: auto;
            top: 18%;
            bottom: 18%;
            width: ${indicator_thickness};
            height: auto;
        }
        &[data-focused="true"] {
            outline: ${focus_width} solid ${focus_color};
            outline-offset: 2px;
        }
        &:hover {
            background: ${hover_background};
        }
        @media (min-width: ${sm}px) {
            padding: ${padding_y_large} ${padding_x_large};
            font-size: ${font_size_large};
        }
    "#,
        gap = format!("{}px", theme.spacing(1) / 2),
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(2)),
        padding_y_large = format!("{}px", theme.spacing(2)),
        padding_x_large = format!("{}px", theme.spacing(3)),
        min_width = format!("{}px", theme.spacing(10)),
        color_inactive = theme.palette.text_secondary.clone(),
        color_active = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size_small = format!("{:.3}rem", theme.typography.button),
        font_size_large = format!("{:.3}rem", theme.typography.subtitle1),
        font_weight_medium = theme.typography.font_weight_medium.to_string(),
        font_weight_bold = theme.typography.font_weight_bold.to_string(),
        line_height = format!("{:.3}", theme.typography.line_height),
        radius = format!("{}px", theme.joy.radius),
        indicator_thickness = format!("{}px", theme.joy.focus_thickness.max(2)),
        indicator_color = theme.palette.primary.clone(),
        indicator_radius = format!("{}px", theme.joy.focus_thickness.max(2)),
        focus_width = format!("{}px", theme.joy.focus_thickness),
        focus_color = theme.palette.primary.clone(),
        hover_background = format!(
            "color-mix(in srgb, {} 12%, transparent)",
            theme.palette.primary.clone()
        ),
        sm = theme.breakpoints.sm,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::tabs::{ActivationMode, TabsOrientation};

    fn sample_state(selected: usize) -> TabsState {
        TabsState::new(
            3,
            Some(selected),
            ActivationMode::Manual,
            TabsOrientation::Horizontal,
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        )
    }

    #[test]
    fn tab_attributes_include_data_flags() {
        let state = sample_state(1);
        let attrs = tab_attributes(&state, state.tab(1));
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "tab"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-selected" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-orientation" && v == "horizontal"));
    }

    #[test]
    fn render_tab_html_emits_class_and_label() {
        let state = sample_state(0);
        let html = render_tab_html(&state, state.tab(0), "Overview");
        assert!(html.contains("class=\""));
        assert!(html.contains(">Overview<"));
        assert!(html.contains("role=\"tab\""));
    }
}
