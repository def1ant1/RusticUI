//! Material aware helpers around [`rustic_ui_headless::transition`].
//!
//! The headless [`TransitionState`] delivers lifecycle semantics for overlays.
//! This module turns those semantics into themable automation hooks that remain
//! stable across frameworks.  The helpers deliberately return attribute tuples so
//! adapters can forward them directly to HTML nodes without re-encoding logic in
//! every renderer.

pub use rustic_ui_headless::transition::{TransitionPhase, TransitionSnapshot, TransitionState};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Material data attribute describing the current transition phase.
#[must_use]
pub fn data_attributes(snapshot: &TransitionSnapshot) -> Vec<(String, String)> {
    let (key, value) = snapshot.data_transition();
    vec![(key.into(), value)]
}

/// CSS class applied while the overlay is visible.  Uses the shared theme helper
/// so SSR, WASM and CSR adapters stay consistent.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_visible_class() -> String {
    crate::style_helpers::themed_class(visible_style())
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn visible_style() -> Style {
    css_with_theme!(
        r#"
        transition: opacity 120ms ease, transform 120ms ease;
        &[data-transition="entering"],
        &[data-transition="visible"] {
            opacity: 1;
            transform: translateY(0);
        }
        &[data-transition="idle"],
        &[data-transition="completed"] {
            opacity: 0;
        }
        "#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_attributes_return_expected_key() {
        let mut state = TransitionState::new(None);
        state.begin_enter();
        let snapshot = state.snapshot();
        let attrs = data_attributes(&snapshot);
        assert_eq!(attrs[0].0, "data-transition");
    }
}
