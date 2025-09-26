//! Material switch built from the headless [`SwitchState`].
//!
//! Switches reuse the shared selection control renderer ensuring identical
//! markup across frameworks while leveraging theme tokens for styling.

use rustic_ui_headless::switch::SwitchState;
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::selection_control;

#[derive(Clone, Debug)]
pub struct SwitchProps {
    /// Human friendly label rendered adjacent to the switch track.
    pub label: String,
}

impl SwitchProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

fn render_html(props: &SwitchProps, state: &SwitchState) -> String {
    let attrs = state
        .aria_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    selection_control::render_toggle(&props.label, themed_switch_style(), attrs)
}

/// Builds the switch track and thumb styling from the active theme tokens. By
/// leaning on `css_with_theme!` we avoid scattering literal values and keep the
/// component responsive to palette or spacing overrides.
fn themed_switch_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        cursor: pointer;
        font-family: ${font_family};
        color: ${text_color};
        position: relative;
        padding: ${padding_y} ${padding_x};

        &::before {
            content: "";
            width: ${track_width};
            height: ${track_height};
            background: ${track_off};
            border-radius: ${track_radius};
            transition: background-color 160ms ease;
            display: inline-block;
            margin-right: ${gap};
        }

        &::after {
            content: "";
            position: absolute;
            left: ${thumb_offset};
            top: 50%;
            transform: translateY(-50%);
            width: ${thumb_size};
            height: ${thumb_size};
            background: ${thumb_color};
            border-radius: 9999px;
            transition: transform 160ms ease;
        }

        &[data-on='true']::before {
            background: ${track_on};
        }

        &[data-on='true']::after {
            transform: translate(${thumb_translate}, -50%);
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
        font_family = theme.typography.font_family.clone(),
        text_color = theme.palette.text_primary.clone(),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        track_width = format!("{}px", theme.spacing(4)),
        track_height = format!("{}px", theme.spacing(1)),
        track_radius = format!("{}px", theme.spacing(1)),
        track_off = theme.palette.text_secondary.clone(),
        track_on = theme.palette.primary.clone(),
        thumb_size = format!("{}px", theme.spacing(2)),
        thumb_color = theme.palette.background_paper.clone(),
        thumb_offset = format!("{}px", theme.spacing(0)),
        thumb_translate = format!("{}px", theme.spacing(2)),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
}

/// Testing hook mirroring the one provided in [`checkbox`].
#[cfg_attr(not(test), allow(dead_code))]
fn themed_switch_attributes(state: &SwitchState) -> Vec<(String, String)> {
    state
        .aria_attributes()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

pub mod yew {
    use super::*;

    pub fn render(props: &SwitchProps, state: &SwitchState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    pub fn render(props: &SwitchProps, state: &SwitchState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    pub fn render(props: &SwitchProps, state: &SwitchState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    pub fn render(props: &SwitchProps, state: &SwitchState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_attributes_include_role() {
        let state = SwitchState::uncontrolled(false, true);
        let attrs = themed_switch_attributes(&state);
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "switch"));
    }

    #[test]
    fn render_html_contains_label_and_data_state() {
        let props = SwitchProps::new("Notifications");
        let state = SwitchState::uncontrolled(false, false);
        let html = render_html(&props, &state);
        assert!(html.contains(">Notifications<"));
        assert!(html.contains("data-on"));
    }
}
