//! Material radio group built atop the headless [`RadioGroupState`].
//!
//! Rendering logic is intentionally centralized so Yew, Leptos, Dioxus and
//! Sycamore integrations share identical markup while also giving adapters
//! access to structured descriptors for hydration.

use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::selection_control::{self, RadioGroupDescriptor, RadioOptionDescriptor};

#[derive(Clone, Debug)]
pub struct RadioGroupProps {
    /// Optional custom labels for each option. When omitted the state's option
    /// names are reused.
    pub option_labels: Vec<String>,
}

impl RadioGroupProps {
    pub fn new(option_labels: impl Into<Vec<String>>) -> Self {
        Self {
            option_labels: option_labels.into(),
        }
    }

    pub fn from_state(state: &RadioGroupState) -> Self {
        Self {
            option_labels: state.options().to_vec(),
        }
    }
}

fn build_descriptor(props: &RadioGroupProps, state: &RadioGroupState) -> RadioGroupDescriptor {
    let orientation_value = match state.orientation() {
        RadioOrientation::Horizontal => "horizontal",
        RadioOrientation::Vertical => "vertical",
    };

    let labels = if props.option_labels.is_empty() {
        state.options().to_vec()
    } else {
        props.option_labels.clone()
    };

    let mut descriptor = RadioGroupDescriptor::new(themed_radio_group_style())
        .with_group_attributes(state.group_aria_attributes())
        .group_attribute("data-orientation", orientation_value);

    for (index, option) in state.options().iter().enumerate() {
        let label = labels.get(index).cloned().unwrap_or_else(|| option.clone());
        let option_descriptor = RadioOptionDescriptor::new(label, themed_radio_option_style())
            .with_attributes(state.option_aria_attributes(index))
            .attribute("data-index", index.to_string());
        descriptor = descriptor.option(option_descriptor);
    }

    descriptor
}

fn render_html(props: &RadioGroupProps, state: &RadioGroupState) -> String {
    let descriptor = build_descriptor(props, state);
    selection_control::render_radio_group_html(&descriptor)
}

/// Generates layout styling for the radio group container, including
/// orientation-aware flex direction toggles.
fn themed_radio_group_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        flex-direction: column;
        gap: ${gap};

        &[data-orientation='horizontal'] {
            flex-direction: row;
        }

        &[aria-disabled='true'] {
            opacity: 0.38;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
    )
}

/// Visual styling for individual radio options including the faux dot used to
/// communicate selection.
fn themed_radio_option_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        cursor: pointer;
        font-family: ${font_family};
        font-size: ${font_size};
        color: ${text_color};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};

        &::before {
            content: "";
            width: ${dot_size};
            height: ${dot_size};
            border-radius: 9999px;
            border: 2px solid ${border_color};
            margin-right: ${gap};
            box-sizing: border-box;
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
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body1),
        text_color = theme.palette.text_primary.clone(),
        padding_y = format!("{}px", theme.spacing(0)),
        padding_x = format!("{}px", theme.spacing(0)),
        radius = format!("{}px", theme.joy.radius),
        dot_size = format!("{}px", theme.spacing(1)),
        border_color = theme.palette.text_secondary.clone(),
        checked_background = theme.palette.primary.clone(),
        focus_outline_width = format!("{}px", theme.joy.focus.thickness),
        focus_outline_color = theme.palette.primary.clone()
    )
}

pub mod yew {
    use super::*;

    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    pub fn render(props: &RadioGroupProps, state: &RadioGroupState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_html_includes_all_options() {
        let props = RadioGroupProps::new(vec!["A".to_string(), "B".to_string()]);
        let state = RadioGroupState::uncontrolled(
            vec!["A".into(), "B".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let html = render_html(&props, &state);
        assert!(html.contains("data-index=\"0\""));
        assert!(html.contains("data-index=\"1\""));
    }

    #[test]
    fn descriptor_exposes_aria_metadata() {
        let props = RadioGroupProps::new(vec!["A".to_string(), "B".to_string()]);
        let state = RadioGroupState::uncontrolled(
            vec!["A".into(), "B".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let descriptor = build_descriptor(&props, &state);
        assert!(descriptor.aria_attributes().any(|(k, _)| k == "role"));
        assert!(descriptor.options().iter().any(|option| option
            .aria_attributes()
            .any(|(k, v)| k == "aria-checked" && v == "true")));
    }
}
