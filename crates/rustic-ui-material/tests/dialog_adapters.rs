#![cfg(any(feature = "dioxus", feature = "leptos", feature = "sycamore"))]

use rustic_ui_headless::dialog::DialogState;
use rustic_ui_material::dialog::DialogSurfaceOptions;

/// Validate that dialog adapters attach the generated class and ARIA metadata
/// when open while emitting no markup when closed.

fn uncontrolled_open_state() -> DialogState {
    let mut state = DialogState::uncontrolled(false);
    state.open(|_| {});
    state.finish_open();
    state
}

fn controlled_closing_state() -> DialogState {
    let mut state = DialogState::controlled();
    state.open(|_| {});
    state.sync_open(true);
    let _ = state.handle_escape(|_| {});
    state
}

fn non_modal_open_state() -> DialogState {
    let mut state = DialogState::uncontrolled(true);
    state.set_modal(false);
    state.finish_open();
    state
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use rustic_ui_material::dialog::dioxus;

    fn build_props(state: DialogState) -> dioxus::DialogProps {
        dioxus::DialogProps {
            state,
            surface: DialogSurfaceOptions {
                id: Some("dialog-surface".into()),
                analytics_id: Some("checkout-flow".into()),
                ..Default::default()
            },
            children: "body".into(),
            aria_label: Some("Settings".into()),
        }
    }

    #[test]
    fn open_state_exposes_focus_and_analytics_metadata() {
        let html = dioxus::render(&build_props(uncontrolled_open_state()));
        assert!(html.starts_with("<div"));
        assert!(html.contains("role=\"dialog\""));
        assert!(html.contains("aria-modal=\"true\""));
        assert!(html.contains("aria-label=\"Settings\""));
        assert!(html.contains("data-state=\"open\""));
        assert!(html.contains("data-transition=\"open\""));
        assert!(html.contains("data-focus-trap=\"active\""));
        assert!(html.contains("data-analytics-id=\"checkout-flow\""));
    }

    #[test]
    fn escape_close_emits_transition_marker() {
        let html = dioxus::render(&build_props(controlled_closing_state()));
        assert!(html.contains("data-state=\"closing\""));
        assert!(html.contains("data-transition=\"close\""));
    }

    #[test]
    fn non_modal_dialog_reports_inactive_focus_trap() {
        let mut props = build_props(non_modal_open_state());
        props.aria_label = None;
        let html = dioxus::render(&props);
        assert!(html.contains("data-focus-trap=\"inactive\""));
        assert!(!html.contains("aria-label"));
    }

    #[test]
    fn closed_dialog_emits_no_markup() {
        let html = dioxus::render(&dioxus::DialogProps::default());
        assert!(html.is_empty());
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;
    use rustic_ui_material::dialog::leptos;

    fn build_props(state: DialogState) -> leptos::DialogProps {
        leptos::DialogProps {
            state,
            aria_label: Some("Settings".into()),
            surface: Some(DialogSurfaceOptions {
                id: Some("dialog-surface".into()),
                analytics_id: Some("checkout-flow".into()),
                ..Default::default()
            }),
            children: (|| "body".into()).into(),
        }
    }

    #[test]
    fn open_state_exposes_focus_and_analytics_metadata() {
        let html = leptos::render(&build_props(uncontrolled_open_state()));
        assert!(html.starts_with("<div"));
        assert!(html.contains("role=\"dialog\""));
        assert!(html.contains("data-state=\"open\""));
        assert!(html.contains("data-focus-trap=\"active\""));
        assert!(html.contains("data-analytics-id=\"checkout-flow\""));
    }

    #[test]
    fn escape_close_emits_transition_marker() {
        let html = leptos::render(&build_props(controlled_closing_state()));
        assert!(html.contains("data-transition=\"close\""));
    }

    #[test]
    fn non_modal_dialog_reports_inactive_focus_trap() {
        let mut props = build_props(non_modal_open_state());
        props.aria_label = None;
        let html = leptos::render(&props);
        assert!(html.contains("data-focus-trap=\"inactive\""));
        assert!(!html.contains("aria-label"));
    }

    #[test]
    fn closed_dialog_emits_no_markup() {
        let html = leptos::render(&leptos::DialogProps::default());
        assert!(html.is_empty());
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use rustic_ui_material::dialog::sycamore;

    fn build_props(state: DialogState) -> sycamore::DialogProps {
        sycamore::DialogProps {
            state,
            surface: DialogSurfaceOptions {
                id: Some("dialog-surface".into()),
                analytics_id: Some("checkout-flow".into()),
                ..Default::default()
            },
            children: "body".into(),
            aria_label: Some("Settings".into()),
        }
    }

    #[test]
    fn open_state_exposes_focus_and_analytics_metadata() {
        let html = sycamore::render(&build_props(uncontrolled_open_state()));
        assert!(html.starts_with("<div"));
        assert!(html.contains("data-state=\"open\""));
        assert!(html.contains("data-focus-trap=\"active\""));
        assert!(html.contains("data-analytics-id=\"checkout-flow\""));
    }

    #[test]
    fn escape_close_emits_transition_marker() {
        let html = sycamore::render(&build_props(controlled_closing_state()));
        assert!(html.contains("data-transition=\"close\""));
    }

    #[test]
    fn non_modal_dialog_reports_inactive_focus_trap() {
        let mut props = build_props(non_modal_open_state());
        props.aria_label = None;
        let html = sycamore::render(&props);
        assert!(html.contains("data-focus-trap=\"inactive\""));
        assert!(!html.contains("aria-label"));
    }

    #[test]
    fn closed_dialog_emits_no_markup() {
        let html = sycamore::render(&sycamore::DialogProps::default());
        assert!(html.is_empty());
    }
}
