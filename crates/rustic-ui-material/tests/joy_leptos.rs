#![cfg(feature = "leptos")]

//! Joy adapter parity checks for the Leptos SSR renderer.
//!
//! Each test compares the canonical React HTML against the Leptos adapter to
//! guarantee hydration-friendly markup parity for Material components that lean
//! on Joy design tokens.

#[path = "common/fixtures.rs"]
mod fixtures;

use fixtures::{
    button_props, button_state_default, chip_props, chip_state_focused, dialog_aria_label,
    dialog_body_markup, dialog_state_open, dialog_surface_options,
};
use mui_material::{button, chip, dialog};

#[test]
fn leptos_button_matches_react_markup() {
    let props = button_props();
    let state = button_state_default();

    let react = button::react::render(&props, &state);
    let leptos = button::leptos::render(&props, &state);

    assert_eq!(leptos, react);
    assert!(react.contains("class=\""));
    assert!(react.contains("role=\"button\""));
}

#[test]
fn leptos_chip_mirrors_react_snapshot() {
    let props = chip_props();
    let state_react = chip_state_focused();
    let state_leptos = chip_state_focused();

    let react = chip::react::render(&props, &state_react);
    let leptos = chip::leptos::render(&props, &state_leptos);

    assert_eq!(leptos, react);
    assert!(react.contains("data-component=\"mui-chip\""));
    assert!(react.contains("aria-describedby=\"joy-chip-delete\""));
}

#[test]
fn leptos_dialog_ssr_matches_react_renderer() {
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

    let leptos_props = dialog::leptos::DialogProps {
        state,
        surface,
        children: body,
        aria_label,
    };
    let leptos = dialog::leptos::render(&leptos_props);

    assert_eq!(leptos, react);
    assert!(react.contains("role=\"dialog\""));
    assert!(react.contains("data-focus-trap=\"true\""));
    assert!(react.contains("data-analytics-id=\"joy-modal\""));
}
