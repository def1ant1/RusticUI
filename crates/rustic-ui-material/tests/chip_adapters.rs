//! Adapter parity checks for the Material chip renderer.
//!
//! Similar to the tooltip tests we validate that each framework specific
//! adapter forwards props/state into the shared renderer without dropping
//! automation hooks, scoped styles or ARIA wiring. Keeping these invariants
//! stable is critical for enterprise QA suites that diff SSR and hydrated output
//! across frameworks.

use rustic_ui_headless::chip::{ChipConfig, ChipState};
use rustic_ui_material::chip::{self, ChipProps};
use std::time::Duration;

/// Construct representative chip props mirroring the automated selectors used in
/// browser tests. The automation id flows into DOM ids, data hooks and delete
/// button wiring.
fn sample_props() -> ChipProps {
    ChipProps::new("Search filter")
        .with_automation_id("adapter-chip")
        .with_delete_label("Remove filter")
        .with_delete_icon("✕")
}

/// Build a chip state with zeroed delays to make the trailing delete button
/// visible immediately. This mimics client side focus/hover activation and gives
/// us deterministic markup to assert against.
fn build_state() -> ChipState {
    let mut config = ChipConfig::default();
    config.show_delay = Duration::from_millis(0);
    config.hide_delay = Duration::from_millis(0);
    config.delete_delay = Duration::from_millis(0);
    ChipState::new(config)
}

/// Shared assertion that inspects the rendered HTML for automation and
/// accessibility metadata.
fn assert_chip_markup(html: &str) {
    assert!(
        html.starts_with("<div"),
        "chip root should be a div: {}",
        html
    );
    assert!(
        html.contains("class=\""),
        "missing scoped class on chip root"
    );
    assert!(html.contains("role=\"button\""));
    assert!(html.contains("tabindex=\"0\""));
    assert!(html.contains("data-component=\"rustic_ui_chip\""));
    assert!(html.contains("data-automation-id=\"adapter-chip\""));
    assert!(html.contains("data-label-id=\"adapter-chip-label\""));
    assert!(html.contains("data-delete-id=\"adapter-chip-delete\""));
    assert!(html.contains("aria-labelledby=\"adapter-chip-label\""));
    assert!(html.contains("aria-describedby=\"adapter-chip-delete\""));
    assert!(html.contains("data-chip-slot=\"label\""));
    assert!(html.contains("data-chip-slot=\"delete\""));
    assert!(html.contains("aria-label=\"Remove filter\""));
    assert!(
        html.matches("class=\"").count() >= 2,
        "expected scoped classes on root + delete affordance"
    );
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_adapter_includes_delete_affordance() {
        let props = sample_props();
        let mut state = build_state();
        state.focus();
        let html = chip::yew::render(&props, &state);

        assert_chip_markup(&html);
        assert!(html.contains("data-controls-visible=\"true\""));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_adapter_matches_yew_markup() {
        let props = sample_props();
        let mut state = build_state();
        state.focus();
        let html = chip::leptos::render(&props, &state);

        assert_chip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let mut yew_state = state.clone();
            let baseline = chip::yew::render(&props, &yew_state);
            assert_eq!(html, baseline, "leptos output should mirror yew");
        }
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_adapter_emits_scoped_markup() {
        let props = sample_props();
        let mut state = build_state();
        state.focus();
        let html = chip::dioxus::render(&props, &state);

        assert_chip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let mut yew_state = state.clone();
            let baseline = chip::yew::render(&props, &yew_state);
            assert_eq!(html, baseline, "dioxus output should mirror yew");
        }
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_adapter_emits_scoped_markup() {
        let props = sample_props();
        let mut state = build_state();
        state.focus();
        let html = chip::sycamore::render(&props, &state);

        assert_chip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let mut yew_state = state.clone();
            let baseline = chip::yew::render(&props, &yew_state);
            assert_eq!(html, baseline, "sycamore output should mirror yew");
        }
    }
}

#[cfg(all(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
mod parity_suite {
    use super::*;

    #[test]
    fn adapters_share_identical_markup() {
        let props = sample_props();
        let mut state = build_state();
        state.focus();
        let yew = chip::yew::render(&props, &state);
        let leptos = chip::leptos::render(&props, &state);
        let dioxus = chip::dioxus::render(&props, &state);
        let sycamore = chip::sycamore::render(&props, &state);

        assert_eq!(yew, leptos);
        assert_eq!(yew, dioxus);
        assert_eq!(yew, sycamore);
    }
}
