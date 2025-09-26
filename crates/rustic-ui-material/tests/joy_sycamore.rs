#![cfg(feature = "sycamore")]

//! Joy adapter parity for the Sycamore SSR renderer.
//!
//! These tests lean on the shared fixtures to validate that Sycamore continues
//! to emit the same markup as the React baseline for Joy-influenced Material
//! components.

#[path = "common/fixtures.rs"]
mod fixtures;

use fixtures::{
    button_props, button_state_default, chip_props, chip_state_focused, dialog_aria_label,
    dialog_body_markup, dialog_state_open, dialog_surface_options,
};
use rustic_ui_material::{button, chip, dialog};

#[test]
fn sycamore_button_parity() {
    let props = button_props();
    let state = button_state_default();

    let react = button::react::render(&props, &state);
    let sycamore = button::sycamore::render(&props, &state);

    assert_eq!(sycamore, react);
    assert!(react.contains("role=\"button\""));
}

#[test]
fn sycamore_chip_snapshot_matches_react() {
    let props = chip_props();
    let state_react = chip_state_focused();
    let state_sycamore = chip_state_focused();

    let react = chip::react::render(&props, &state_react);
    let sycamore = chip::sycamore::render(&props, &state_sycamore);

    assert_eq!(sycamore, react);
    assert!(react.contains("data-rustic-chip-id=\"rustic-chip-joy-chip\""));
    assert!(react.contains("data-rustic-chip-root=\"rustic-chip-joy-chip-root\""));
}

#[test]
fn sycamore_dialog_preserves_focus_and_analytics_tags() {
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

    let sycamore_props = dialog::sycamore::DialogProps {
        state,
        surface,
        children: body,
        aria_label,
    };
    let sycamore = dialog::sycamore::render(&sycamore_props);

    assert_eq!(sycamore, react);
    assert!(react.contains("role=\"dialog\""));
    assert!(react.contains("data-analytics-id=\"joy-modal\""));
}
