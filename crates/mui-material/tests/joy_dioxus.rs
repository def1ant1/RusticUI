#![cfg(feature = "dioxus")]

//! Joy adapter regression tests for the Dioxus renderer.
//!
//! Dioxus consumes the same shared renderers as the SSR integrations, so these
//! assertions ensure the serialized HTML continues to line up with the React
//! baseline when Joy styling tokens change.

#[path = "common/fixtures.rs"]
mod fixtures;

use fixtures::{
    button_props, button_state_default, chip_props, chip_state_focused, dialog_aria_label,
    dialog_body_markup, dialog_state_open, dialog_surface_options,
};
use mui_material::{button, chip, dialog};

#[test]
fn dioxus_button_matches_react_output() {
    let props = button_props();
    let state = button_state_default();

    let react = button::react::render(&props, &state);
    let dioxus = button::dioxus::render(&props, &state);

    assert_eq!(dioxus, react);
    assert!(react.contains("class=\""));
    assert!(react.contains("role=\"button\""));
}

#[test]
fn dioxus_chip_retains_focus_and_delete_hooks() {
    let props = chip_props();
    let state_react = chip_state_focused();
    let state_dioxus = chip_state_focused();

    let react = chip::react::render(&props, &state_react);
    let dioxus = chip::dioxus::render(&props, &state_dioxus);

    assert_eq!(dioxus, react);
    assert!(react.contains("aria-describedby=\"joy-chip-delete\""));
    assert!(react.contains("data-chip-slot=\"delete\""));
}

#[test]
fn dioxus_dialog_parity_preserves_focus_trap_metadata() {
    let state = dialog_state_open();
    let surface = dialog_surface_options();
    let body = dialog_body_markup();
    let aria_label = dialog_aria_label();

    let react_props = dialog::react::DialogProps {
        state: state.clone(),
        surface: surface.clone(),
        children: body.clone(),
        aria_label: aria_label.clone(),
    };
    let react = dialog::react::render(&react_props);

    let dioxus_props = dialog::dioxus::DialogProps {
        state,
        surface,
        children: body,
        aria_label,
    };
    let dioxus = dialog::dioxus::render(&dioxus_props);

    assert_eq!(dioxus, react);
    assert!(react.contains("data-focus-trap=\"true\""));
    assert!(react.contains("data-transition"));
}
