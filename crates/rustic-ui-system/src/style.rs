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
    /// Sets the CSS `margin` shorthand. Accepts absolute units (`px`, `rem`),
    /// percentages or keywords such as `auto` for centred layouts.
    margin => "margin",
    /// Applies padding to every edge using the CSS shorthand. Supports length
    /// units (e.g. `px`, `rem`) and percentages.
    padding => "padding",
    /// Sets the top margin. Useful for vertical stacking with units like `px`
    /// or `rem`.
    margin_top => "margin-top",
    /// Sets the bottom margin in any CSS length unit or percentage.
    margin_bottom => "margin-bottom",
    /// Sets the left margin. Accepts CSS length units and `auto`.
    margin_left => "margin-left",
    /// Sets the right margin. Accepts CSS length units and `auto`.
    margin_right => "margin-right",
    /// Applies padding on the top edge (`px`, `rem`, `%`, etc.).
    padding_top => "padding-top",
    /// Applies padding on the bottom edge (`px`, `rem`, `%`, etc.).
    padding_bottom => "padding-bottom",
    /// Applies padding on the left edge (`px`, `rem`, `%`, etc.).
    padding_left => "padding-left",
    /// Applies padding on the right edge (`px`, `rem`, `%`, etc.).
    padding_right => "padding-right",
    /// Controls the uniform gap between grid or flex items. Accepts any CSS
    /// length unit and cascades to both rows and columns.
    gap => "gap",
    /// Controls the vertical spacing between grid or flex rows.
    row_gap => "row-gap",
    /// Controls the horizontal spacing between grid or flex columns.
    column_gap => "column-gap",

    // Layout ----------------------------------------------------------------
    /// Toggles the element's `display` mode (e.g. `flex`, `grid`, `block`).
    display => "display",
    /// Sets the main axis direction for flex containers (`row`, `column`, ...).
    flex_direction => "flex-direction",
    /// Enables or disables wrapping for flex items (`wrap`, `nowrap`).
    flex_wrap => "flex-wrap",
    /// Adjusts how flex items stretch on the cross axis.
    align_items => "align-items",
    /// Aligns the content along the cross axis for multi-line flex containers.
    align_content => "align-content",
    /// Controls the self-alignment of a single flex or grid item.
    align_self => "align-self",
    /// Aligns items on the main axis for flex or grid containers (`center`,
    /// `space-between`, etc.).
    justify_content => "justify-content",
    /// Specifies the default alignment for grid items (`start`, `center`, ...).
    justify_items => "justify-items",
    /// Overrides alignment for a single grid item along the inline axis.
    justify_self => "justify-self",
    /// Sets both `align-items` and `justify-items` in one declaration.
    place_items => "place-items",
    /// Sets both `align-content` and `justify-content` for grid layouts.
    place_content => "place-content",
    /// Sets both `align-self` and `justify-self` for individual grid items.
    place_self => "place-self",
    /// Specifies the growth factor for flex items.
    flex_grow => "flex-grow",
    /// Specifies the shrink factor for flex items.
    flex_shrink => "flex-shrink",
    /// Sets the initial main size of a flex item (`px`, `%`, `auto`).
    flex_basis => "flex-basis",
    /// Controls the order in which flex items appear.
    order => "order",
    /// Defines how auto-placed items flow into the grid (`row`, `column`, ...).
    grid_auto_flow => "grid-auto-flow",
    /// Declares implicit column sizing for auto-placed grid tracks.
    grid_auto_columns => "grid-auto-columns",
    /// Declares implicit row sizing for auto-placed grid tracks.
    grid_auto_rows => "grid-auto-rows",
    /// Declares the explicit grid column track template (e.g. `repeat(12, 1fr)`).
    grid_template_columns => "grid-template-columns",
    /// Declares the explicit grid row track template.
    grid_template_rows => "grid-template-rows",
    /// Declares named grid areas used by `grid-area` assignments.
    grid_template_areas => "grid-template-areas",
    /// Shorthand for specifying a grid item's column start/end (e.g. `1 / span 2`).
    grid_column => "grid-column",
    /// Sets the starting grid line for a grid item column.
    grid_column_start => "grid-column-start",
    /// Sets the ending grid line for a grid item column.
    grid_column_end => "grid-column-end",
    /// Shorthand for specifying a grid item's row start/end (e.g. `auto / span 3`).
    grid_row => "grid-row",
    /// Sets the starting grid line for a grid item row.
    grid_row_start => "grid-row-start",
    /// Sets the ending grid line for a grid item row.
    grid_row_end => "grid-row-end",
    /// Assigns a grid item to a named area or explicit coordinates.
    grid_area => "grid-area",

    // Typography -------------------------------------------------------------
    /// Controls the font size (`px`, `rem`, `em`, `%`, etc.).
    font_size => "font-size",
    /// Sets the font weight (numeric or keywords like `bold`).
    font_weight => "font-weight",
    /// Adjusts the line height. Accepts unitless numbers or CSS lengths.
    line_height => "line-height",
    /// Sets additional tracking between letters. Use CSS length units.
    letter_spacing => "letter-spacing",

    // Sizing -----------------------------------------------------------------
    /// Sets the width (`px`, `%`, `vw`, etc.).
    width => "width",
    /// Sets the height (`px`, `%`, `vh`, etc.).
    height => "height",
    /// Sets the minimum width to prevent shrinkage.
    min_width => "min-width",
    /// Sets the minimum height to prevent shrinkage.
    min_height => "min-height",
    /// Sets the maximum width constraint.
    max_width => "max-width",
    /// Sets the maximum height constraint.
    max_height => "max-height",

    // Color & visual treatments ---------------------------------------------
    /// Sets the text colour. Accepts any valid CSS colour (hex, rgb, token).
    color => "color",
    /// Sets the background fill colour.
    background_color => "background-color",
    /// Rounds the element corners. Accepts any CSS length unit or percentages.
    border_radius => "border-radius",
    /// Applies a box shadow (`offset blur spread colour`).
    box_shadow => "box-shadow",
    /// Controls the element's overall opacity (0.0 - 1.0).
    opacity => "opacity",

    // Positioning ------------------------------------------------------------
    /// Chooses the positioning scheme (`static`, `relative`, `absolute`, ...).
    position => "position",
    /// Sets the top offset when using positioned layouts (`px`, `%`).
    top => "top",
    /// Sets the right offset when using positioned layouts (`px`, `%`).
    right => "right",
    /// Sets the bottom offset when using positioned layouts (`px`, `%`).
    bottom => "bottom",
    /// Sets the left offset when using positioned layouts (`px`, `%`).
    left => "left",
    /// Controls stacking order for positioned elements.
    z_index => "z-index",
    /// Enables scroll clipping behaviour (`visible`, `hidden`, `auto`).
    overflow => "overflow",
    /// Controls horizontal overflow (`visible`, `scroll`, `auto`).
    overflow_x => "overflow-x",
    /// Controls vertical overflow (`visible`, `scroll`, `auto`).
    overflow_y => "overflow-y",

    // Transforms -------------------------------------------------------------
    /// Applies CSS transforms like `translate`, `scale`, `rotate`.
    transform => "transform",
    /// Sets the origin for transform operations (e.g. `center`, `50% 0`).
    transform_origin => "transform-origin",

    // Transitions ------------------------------------------------------------
    /// Full transition shorthand combining property, duration and timing.
    transition => "transition",
    /// Names the CSS properties that animate.
    transition_property => "transition-property",
    /// Specifies how long the transition runs. Use `s` or `ms` units.
    transition_duration => "transition-duration",
    /// Specifies delay before the transition runs. Use `s` or `ms` units.
    transition_delay => "transition-delay",
    /// Specifies the easing function for the transition.
    transition_timing_function => "transition-timing-function",

    // Animations -------------------------------------------------------------
    /// Full animation shorthand with keyframes, duration and modifiers.
    animation => "animation",
    /// Names the keyframe animation to run.
    animation_name => "animation-name",
    /// Controls animation duration (`s` or `ms`).
    animation_duration => "animation-duration",
    /// Controls delay before animation starts (`s` or `ms`).
    animation_delay => "animation-delay",
    /// Sets the easing curve for the animation.
    animation_timing_function => "animation-timing-function",
    /// Declares how many times the animation repeats (number or `infinite`).
    animation_iteration_count => "animation-iteration-count",
    /// Controls animation direction (`normal`, `alternate`, ...).
    animation_direction => "animation-direction",
    /// Controls how the animation applies styles before/after running.
    animation_fill_mode => "animation-fill-mode",
    /// Pauses or resumes the animation (`running`, `paused`).
    animation_play_state => "animation-play-state",
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
// `use rustic_ui_system::style::*` or simply `use rustic_ui_system::*` when `lib.rs`
// re-exports this module.
