//! Material styling facade for [`rustic_ui_headless::popper`].
//!
//! Popper driven overlays (tooltips, menus, selects) share similar portal,
//! transition and collision logic.  This module wires the headless
//! [`PopperState`] into Material theming primitives while preserving automation
//! hooks.  Framework adapters simply forward snapshots and the helpers emit the
//! deterministic attribute maps expected by enterprise QA harnesses.

pub use rustic_ui_headless::popper::{
    CollisionStrategy, PointerType, PopperPlacement, PopperSnapshot, PopperState,
};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Creates a new [`PopperState`] using Material automation naming conventions.
#[must_use]
pub fn new_state(
    preferred: PopperPlacement,
    collision: CollisionStrategy,
    component: &str,
    automation_id: Option<&str>,
) -> PopperState {
    let id = crate::style_helpers::automation_id(
        component,
        automation_id,
        crate::style_helpers::EMPTY_SEGMENTS,
    );
    PopperState::new(preferred, collision, Some(id))
}

/// Returns themed attributes for the floating surface element.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_surface_attributes(
    snapshot: &PopperSnapshot,
    extra: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push((
        "data-placement".into(),
        placement_to_str(snapshot.placement()).into(),
    ));
    attrs.extend(crate::transition::data_attributes(snapshot.transition()).into_iter());
    for (key, value) in extra {
        attrs.push((key.into(), value.into()));
    }
    crate::style_helpers::themed_attributes(surface_style(), attrs)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn placement_to_str(placement: PopperPlacement) -> &'static str {
    match placement {
        PopperPlacement::Top => "top",
        PopperPlacement::Bottom => "bottom",
        PopperPlacement::Start => "start",
        PopperPlacement::End => "end",
    }
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
        background: ${surface};
        color: ${on_surface};
        border-radius: ${radius};
        box-shadow: ${shadow};
        padding: ${padding};
        &[data-placement="top"] {
            transform-origin: center bottom;
        }
        &[data-placement="bottom"] {
            transform-origin: center top;
        }
        "#,
        surface = theme.palette.surface.clone(),
        on_surface = theme.palette.on_surface.clone(),
        radius = theme.shape.border_radius.small.clone(),
        shadow = theme.shadows[4].clone(),
        padding = format!("{}px", theme.spacing(2))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_id_is_namespaced() {
        let state = new_state(
            PopperPlacement::Bottom,
            CollisionStrategy::Flip,
            "tooltip",
            Some("primary"),
        );
        assert!(state
            .portal()
            .transition()
            .automation_id()
            .unwrap()
            .starts_with(crate::style_helpers::COMPONENT_PREFIX));
    }
}
