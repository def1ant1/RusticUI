//! Typed helper functions for common CSS properties.
//!
//! These helpers are generated via macros to keep the source lightweight while
//! still exposing strongly typed builders. Each function returns a CSS style
//! string of the form `name:value;` and can be freely composed using the
//! [`style_props!`](crate::style_props) macro.
//!
//! The goal is to minimise repetitive manual mapping of Rust fields to CSS
//! strings and instead centralise the mapping in a declarative list. New style
//! properties can be added by appending a line to the macro invocation below.

use crate::define_style_props;
use serde_json::Value;

// Generate a suite of style property builders. These mirror a subset of the
// most commonly used system props from MUI. The list can easily be extended as
// additional style shortcuts are required.
define_style_props! {
    // Spacing ---------------------------------------------------------------
    margin => "margin",
    padding => "padding",
    margin_top => "margin-top",
    margin_bottom => "margin-bottom",
    margin_left => "margin-left",
    margin_right => "margin-right",
    padding_top => "padding-top",
    padding_bottom => "padding-bottom",
    padding_left => "padding-left",
    padding_right => "padding-right",

    // Layout ----------------------------------------------------------------
    display => "display",
    flex_direction => "flex-direction",
    flex_wrap => "flex-wrap",
    align_items => "align-items",
    justify_content => "justify-content",
    max_width => "max-width",
    max_height => "max-height",
    min_height => "min-height",

    // Typography -------------------------------------------------------------
    font_size => "font-size",
    font_weight => "font-weight",
    line_height => "line-height",
    letter_spacing => "letter-spacing",

    // Sizing -----------------------------------------------------------------
    width => "width",
    height => "height",
    min_width => "min-width",

    // Color & visual treatments ---------------------------------------------
    color => "color",
    background_color => "background-color",
    border_radius => "border-radius",
    box_shadow => "box-shadow",

    // Positioning ------------------------------------------------------------
    position => "position",
    top => "top",
    right => "right",
    bottom => "bottom",
    left => "left",
}

/// Convert a JSON object representing CSS declarations into a `prop:value;` string.
///
/// The helper keeps component code focused on the style *data* rather than the
/// textual representation, making it trivial to bolt on new properties or merge
/// JSON driven overrides without sprinkling string concatenation everywhere.
pub fn json_to_style_string(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let mut css = String::new();
            for (key, value) in entries {
                if let Some(nested) = value.as_object() {
                    // Nested objects are serialised as `selector{...}` which can be
                    // extended in future iterations. For now we flatten them into
                    // scoped blocks to keep inline styles predictable.
                    let mut nested_entries: Vec<_> = nested.iter().collect();
                    nested_entries.sort_by(|a, b| a.0.cmp(b.0));
                    css.push_str(key);
                    css.push('{');
                    for (nested_key, nested_value) in nested_entries {
                        css.push_str(nested_key);
                        css.push(':');
                        css.push_str(&value_to_string(nested_value));
                        css.push(';');
                    }
                    css.push('}');
                } else if !value.is_null() {
                    css.push_str(key);
                    css.push(':');
                    css.push_str(&value_to_string(value));
                    css.push(';');
                }
            }
            css
        }
        other => value_to_string(other),
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

// The module intentionally re-exports all generated helpers so callers can do
// `use mui_system::style::*` or simply `use mui_system::*` when `lib.rs`
// re-exports this module.
