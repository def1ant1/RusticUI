//! Material flavored checkbox built on the headless [`CheckboxState`].
//!
//! The module orchestrates styling through [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme)
//! and delegates markup generation to [`selection_control::render_toggle`]
//! keeping adapters tiny. Extensive inline documentation is provided to help
//! enterprise teams adapt the component to their own design systems.

use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::selection_control;

/// Props shared across all framework adapters.
#[derive(Clone, Debug)]
pub struct CheckboxProps {
    /// Visible label rendered alongside the checkbox indicator.
    pub label: String,
}

impl CheckboxProps {
    /// Convenience constructor for tests and examples.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

fn render_html(props: &CheckboxProps, state: &CheckboxState) -> String {
    let attrs = state
        .aria_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    selection_control::render_toggle(&props.label, themed_checkbox_style(), attrs)
}

/// Generates the themed style for the checkbox container. The macro pulls
/// palette colors, typography metrics and spacing tokens from the active
/// [`Theme`](rustic_ui_styled_engine::Theme) so enterprise teams can rely on global
/// design governance rather than tweaking individual components.
fn themed_checkbox_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        cursor: pointer;
        color: ${text_color};
        position: relative;
        font-family: ${font_family};
        font-size: ${font_size};

        &::before {
            content: "";
            display: inline-block;
            width: ${box_size};
            height: ${box_size};
            margin-right: ${gap};
            border-radius: ${box_radius};
            border: 2px solid ${border_color};
            background: ${box_background};
            transition: background-color 160ms ease, border-color 160ms ease;
        }

        &[data-checked='true']::before {
            background: ${checked_background};
            border-color: ${checked_background};
        }

        &[data-focus-visible='true'] {
            outline: ${focus_outline_width} solid ${focus_outline_color};
            outline-offset: 2px;
        }

        &[aria-disabled='true'] {
            cursor: not-allowed;
            opacity: 0.38;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        radius = format!("{}px", theme.joy.radius),
        text_color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body1),
        box_size = format!("{}px", theme.spacing(2)),
        box_radius = format!("{}px", theme.joy.radius),
        border_color = theme.palette.text_secondary.clone(),
        box_background = theme.palette.background_paper.clone(),
        checked_background = theme.palette.primary.clone(),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
}

/// Helper exposed for tests so we can assert the attribute map contains the
/// expected ARIA metadata. Production callers should rely on
/// [`render_html`].
#[cfg_attr(not(test), allow(dead_code))]
fn themed_checkbox_attributes(state: &CheckboxState) -> Vec<(String, String)> {
    state
        .aria_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

pub mod yew {
    use super::*;

    pub fn render(props: &CheckboxProps, state: &CheckboxState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    pub fn render(props: &CheckboxProps, state: &CheckboxState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    pub fn render(props: &CheckboxProps, state: &CheckboxState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    pub fn render(props: &CheckboxProps, state: &CheckboxState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_attributes_include_role() {
        let state = CheckboxState::uncontrolled(false, true);
        let attrs = themed_checkbox_attributes(&state);
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "checkbox"));
    }

    #[test]
    fn render_html_includes_label() {
        let props = CheckboxProps::new("Accept");
        let state = CheckboxState::uncontrolled(false, false);
        let html = render_html(&props, &state);
        assert!(html.contains(">Accept<"));
        assert!(html.contains("aria-checked"));
    }
}
