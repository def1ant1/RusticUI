//! Shared rendering helpers for Material selection controls.
//!
//! The helpers convert state machine metadata into HTML strings augmented with
//! themed styles.  Centralizing the rendering logic keeps the individual
//! component modules focused on data flow rather than DOM string assembly.

use rustic_ui_styled_engine::Style;

/// Render a single toggle style control such as a checkbox or switch.
pub(crate) fn render_toggle(label: &str, style: Style, attrs: Vec<(String, String)>) -> String {
    let attr_string = crate::style_helpers::themed_attributes_html(style, attrs);
    format!("<span {attr_string}>{label}</span>")
}

/// Render a radio group with styled options.
pub(crate) fn render_radio_group<F>(
    group_style: Style,
    group_attrs: Vec<(String, String)>,
    option_style_factory: F,
    options: &[(String, Vec<(String, String)>)],
) -> String
where
    F: Fn() -> Style,
{
    let group_attr_string = crate::style_helpers::themed_attributes_html(group_style, group_attrs);
    let mut options_html = String::new();
    for (label, attrs) in options {
        let option_attr =
            crate::style_helpers::themed_attributes_html(option_style_factory(), attrs.clone());
        options_html.push_str(&format!("<span {option_attr}>{label}</span>"));
    }
    format!("<div {group_attr_string}>{options_html}</div>")
}
