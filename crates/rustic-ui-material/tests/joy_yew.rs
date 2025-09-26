#![cfg(feature = "yew")]

//! Joy adapter parity checks for the Yew integration.
//!
//! The tests render representative components through the new React SSR adapter
//! and the existing Yew hooks, asserting the serialized markup matches exactly.
//! Comparing the HTML strings guarantees that scoped classes, ARIA metadata and
//! automation hooks stay aligned across frameworks even as design tokens evolve.

#[path = "common/fixtures.rs"]
mod fixtures;

use fixtures::{button_props, button_state_default, chip_props, chip_state_focused};
use mui_material::{button, chip};

#[test]
fn yew_button_matches_react_baseline() {
    let props = button_props();
    let state = button_state_default();

    let react = button::react::render(&props, &state);
    let yew = button::yew::render(&props, &state);

    assert_eq!(yew, react);
    assert!(react.contains("class=\""), "missing scoped class: {react}");
    assert!(react.contains("role=\"button\""));
    assert!(react.contains("aria-pressed=\"false\""));
}

#[test]
fn yew_chip_includes_all_automation_hooks() {
    let props = chip_props();
    let state_react = chip_state_focused();
    let state_yew = chip_state_focused();

    let react = chip::react::render(&props, &state_react);
    let yew = chip::yew::render(&props, &state_yew);

    assert_eq!(yew, react);
    assert!(react.contains("data-component=\"mui-chip\""));
    assert!(react.contains("data-automation-id=\"joy-chip\""));
    assert!(react.contains("data-chip-slot=\"delete\""));
    assert!(react.contains("aria-label=\"Remove joy filter\""));
}
