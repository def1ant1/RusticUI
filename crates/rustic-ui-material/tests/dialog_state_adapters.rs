#![cfg(any(feature = "dioxus", feature = "leptos", feature = "sycamore"))]

use mui_headless::dialog::{DialogState, DialogTransition};
use mui_material::dialog::{self, DialogSurfaceOptions};

fn opening_state() -> DialogState {
    let mut state = DialogState::controlled();
    state.open(|_| {});
    state
}

fn modal_open_state() -> DialogState {
    let mut state = DialogState::uncontrolled(true);
    state.finish_open();
    state
}

fn surface_options() -> DialogSurfaceOptions {
    DialogSurfaceOptions {
        id: Some("dialog-surface".into()),
        analytics_id: Some("analytics-checkout".into()),
        labelled_by: Some("heading".into()),
        described_by: Some("description".into()),
    }
}

fn assert_surface_contract(html: &str, state: &DialogState) {
    let attrs = state
        .surface_attributes()
        .id("dialog-surface")
        .labelled_by("heading")
        .described_by("description")
        .analytics_id("analytics-checkout");

    let state_marker = format!("data-state=\"{}\"", state.phase().as_str());
    assert!(
        html.contains(&state_marker),
        "missing phase marker: {}",
        html
    );
    if let Some((_, transition)) = attrs.data_transition() {
        let marker = format!("data-transition=\"{}\"", transition);
        assert!(html.contains(&marker), "missing transition: {}", html);
    }
    let (focus_key, focus_value) = attrs.data_focus_trap();
    let focus_marker = format!("{}=\"{}\"", focus_key, focus_value);
    assert!(
        html.contains(&focus_marker),
        "missing focus trap marker: {}",
        html
    );
    if let Some((key, value)) = attrs.data_analytics_id() {
        let analytics_marker = format!("{}=\"{}\"", key, value);
        assert!(
            html.contains(&analytics_marker),
            "missing analytics id: {}",
            html
        );
    }
    let (modal_key, modal_value) = attrs.aria_modal();
    let modal_marker = format!("{}=\"{}\"", modal_key, modal_value);
    assert!(html.contains(&modal_marker), "missing aria-modal: {}", html);
    assert!(html.contains("role=\"dialog\""));
    assert!(html.contains("aria-labelledby=\"heading\""));
    assert!(html.contains("aria-describedby=\"description\""));
}

#[cfg(feature = "leptos")]
fn leptos_props(state: DialogState) -> dialog::leptos::DialogProps {
    dialog::leptos::DialogProps {
        state,
        surface: surface_options(),
        children: "<p>Leptos SSR</p>".into(),
        aria_label: Some("Team settings".into()),
    }
}

#[cfg(feature = "dioxus")]
fn dioxus_props(state: DialogState) -> dialog::dioxus::DialogProps {
    dialog::dioxus::DialogProps {
        state,
        surface: DialogSurfaceOptions {
            id: Some("dialog-surface".into()),
            analytics_id: Some("analytics-checkout".into()),
            labelled_by: Some("heading".into()),
            described_by: Some("description".into()),
        },
        children: "<p>Dioxus SSR</p>".into(),
        aria_label: Some("Team settings".into()),
    }
}

#[cfg(feature = "sycamore")]
fn sycamore_props(state: DialogState) -> dialog::sycamore::DialogProps {
    dialog::sycamore::DialogProps {
        state,
        surface: DialogSurfaceOptions {
            id: Some("dialog-surface".into()),
            analytics_id: Some("analytics-checkout".into()),
            labelled_by: Some("heading".into()),
            described_by: Some("description".into()),
        },
        children: "<p>Sycamore SSR</p>".into(),
        aria_label: Some("Team settings".into()),
    }
}

#[cfg(feature = "leptos")]
#[test]
fn leptos_dialog_reports_transition_metadata() {
    let state = opening_state();
    let html = dialog::leptos::render(&leptos_props(state.clone()));
    assert_surface_contract(&html, &state);
    assert!(html.contains("aria-label=\"Team settings\""));
    assert!(html.contains("data-transition=\"open\""));
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_dialog_retains_focus_trap_markers() {
    let state = modal_open_state();
    let html = dialog::dioxus::render(&dioxus_props(state.clone()));
    assert_surface_contract(&html, &state);
    assert!(html.contains("data-focus-trap=\"active\""));
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_dialog_matches_state_contract() {
    let mut state = opening_state();
    state.finish_open();
    state.close(|_| {});
    let html = dialog::sycamore::render(&sycamore_props(state.clone()));
    assert!(
        html.contains(&format!(
            "data-transition=\"{}\"",
            DialogTransition::CloseRequested.as_str()
        )),
        "closing transition missing: {}",
        html
    );
    assert_surface_contract(&html, &state);
}

#[cfg(all(feature = "leptos", feature = "dioxus"))]
#[test]
fn leptos_and_dioxus_share_identical_markup() {
    let state = opening_state();
    let leptos_html = dialog::leptos::render(&leptos_props(state.clone()));
    let dioxus_html = dialog::dioxus::render(&dioxus_props(state));
    assert_eq!(leptos_html, dioxus_html);
}

#[cfg(all(feature = "leptos", feature = "sycamore"))]
#[test]
fn leptos_and_sycamore_align_for_modal_state() {
    let state = modal_open_state();
    let leptos_html = dialog::leptos::render(&leptos_props(state.clone()));
    let sycamore_html = dialog::sycamore::render(&sycamore_props(state));
    assert_eq!(leptos_html, sycamore_html);
}
