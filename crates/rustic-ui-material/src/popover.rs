//! Material helpers for [`rustic_ui_headless::popover`].
//!
//! Floating surfaces require consistent automation hooks regardless of the
//! framework rendering them.  This module exposes thin wrappers that translate
//! the headless attribute builders into themed key/value pairs so SSR snapshots
//! and interactive clients emit identical markup.

pub use rustic_ui_headless::popover::{
    AnchorGeometry, CollisionOutcome, PopoverPlacement, PopoverState,
};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Generates themed attributes for the popover anchor element.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_anchor_attributes(state: &PopoverState) -> Vec<(String, String)> {
    let attrs = state.anchor_attributes();
    let mut pairs = Vec::new();
    if let Some((key, value)) = attrs.id() {
        pairs.push((key.into(), value.into()));
    }
    let (key, value) = attrs.data_placement();
    pairs.push((key.into(), value.into()));
    crate::style_helpers::themed_attributes(anchor_style(), pairs)
}

/// Generates themed attributes for the popover surface element with optional
/// analytics tagging.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_surface_attributes(
    state: &PopoverState,
    analytics: Option<&str>,
) -> Vec<(String, String)> {
    let attrs = match analytics {
        Some(tag) => state.surface_attributes().analytics_id(tag),
        None => state.surface_attributes(),
    };
    let mut pairs = Vec::new();
    let (open_key, open_value) = attrs.data_open();
    pairs.push((open_key.into(), open_value.into()));
    let (pref_key, pref_value) = attrs.data_preferred();
    pairs.push((pref_key.into(), pref_value.into()));
    let (resolved_key, resolved_value) = attrs.data_resolved();
    pairs.push((resolved_key.into(), resolved_value.into()));
    if let Some((key, value)) = attrs.data_analytics_id() {
        pairs.push((key.into(), value.into()));
    }
    crate::style_helpers::themed_attributes(surface_style(), pairs)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn anchor_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-block;
        "#
    )
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn surface_style() -> Style {
    css_with_theme!(
        r#"
        min-width: 120px;
        max-width: 320px;
        border-radius: ${radius};
        background: ${surface};
        color: ${on_surface};
        box-shadow: ${shadow};
        "#,
        radius = theme.shape.border_radius.small.clone(),
        surface = theme.palette.surface.clone(),
        on_surface = theme.palette.on_surface.clone(),
        shadow = theme.shadows[6].clone()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_attributes_include_open_flag() {
        #[cfg(any(
            feature = "yew",
            feature = "leptos",
            feature = "dioxus",
            feature = "sycamore"
        ))]
        {
            let state = PopoverState::uncontrolled(true, PopoverPlacement::Bottom);
            let attrs = themed_surface_attributes(&state, None);
            assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "true"));
        }
    }
}
