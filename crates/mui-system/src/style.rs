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
}

// The module intentionally re-exports all generated helpers so callers can do
// `use mui_system::style::*` or simply `use mui_system::*` when `lib.rs`
// re-exports this module.
