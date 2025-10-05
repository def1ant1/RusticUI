//! Material themed facade for [`rustic_ui_headless::modal`].
//!
//! Applications often compose modals, drawers and dialogs using the same state
//! machine.  This module bridges the headless [`ModalState`] with Material design
//! tokens by exposing helpers that convert snapshots into themed attribute maps
//! and deterministic automation identifiers.  Each helper relies on
//! [`crate::style_helpers`] so QA instrumentation remains centralised.

pub use rustic_ui_headless::modal::{FocusTrapStrategy, ModalEvent, ModalState};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Convenience constructor mirroring [`ModalState::uncontrolled`] while
/// normalising automation identifiers.
#[must_use]
pub fn uncontrolled(
    default_open: bool,
    focus: FocusTrapStrategy,
    component: &str,
    automation_id: Option<&str>,
) -> ModalState {
    let id = crate::style_helpers::automation_id(
        component,
        automation_id,
        crate::style_helpers::EMPTY_SEGMENTS,
    );
    ModalState::uncontrolled(default_open, focus, Some(id))
}

/// Convenience constructor mirroring [`ModalState::controlled`] while applying
/// Material automation naming.
#[must_use]
pub fn controlled(
    focus: FocusTrapStrategy,
    component: &str,
    automation_id: Option<&str>,
) -> ModalState {
    let id = crate::style_helpers::automation_id(
        component,
        automation_id,
        crate::style_helpers::EMPTY_SEGMENTS,
    );
    ModalState::controlled(focus, Some(id))
}

/// Applies Material classes to the modal surface attributes.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_surface_attributes(
    state: &ModalState,
    extra: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> Vec<(String, String)> {
    let transition = state.transition();
    let mut attrs = crate::transition::data_attributes(&transition)
        .into_iter()
        .collect::<Vec<_>>();
    let mut extra_vec = Vec::new();
    for (key, value) in extra {
        extra_vec.push((key.into(), value.into()));
    }
    attrs.extend(extra_vec); // merge deterministic ordering for automation
    crate::style_helpers::themed_attributes(surface_style(), attrs)
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
        "#,
        surface = theme.palette.surface.clone(),
        on_surface = theme.palette.on_surface.clone(),
        radius = theme.shape.border_radius.medium.clone(),
        shadow = theme.shadows[8].clone(),
        padding = format!("{}px", theme.spacing(4))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_prefix_is_applied() {
        let state = uncontrolled(false, FocusTrapStrategy::Auto, "dialog", Some("primary"));
        assert!(state
            .transition()
            .automation_id()
            .unwrap()
            .starts_with(crate::style_helpers::COMPONENT_PREFIX));
    }
}
