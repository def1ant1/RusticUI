//! Adapter-level coverage for the tooltip renderer.
//!
//! The Material tooltip centralises HTML + CSS assembly inside the crate so
//! every framework adapter (Yew, Leptos, Dioxus, Sycamore) should emit identical
//! markup. These tests assert that each adapter exposes the same automation
//! hooks, ARIA wiring and scoped class names which keeps SSR and client renders
//! aligned for enterprise automation suites.

use rustic_ui_headless::tooltip::{TooltipConfig, TooltipState};
use rustic_ui_material::tooltip::{self, TooltipProps};

/// Construct tooltip props with deterministic automation identifiers so the
/// generated IDs and `data-*` attributes can be asserted without brittle
/// snapshots.
fn sample_props() -> TooltipProps {
    TooltipProps::new("Inspect", "Detailed analytics")
        .with_automation_id("adapter-tooltip")
        .with_trigger_haspopup("dialog")
        .with_surface_labelled_by("adapter-tooltip-heading")
}

/// Build a tooltip state using the documented enterprise defaults. Keeping the
/// state hidden exercises the initial SSR payload that hydration bootstraps
/// against while still exposing the ARIA contract.
fn build_state() -> TooltipState {
    TooltipState::new(TooltipConfig::default())
}

/// Snapshot-style assertion shared across adapters.
fn assert_tooltip_markup(html: &str) {
    assert!(
        html.starts_with("<span"),
        "tooltip root should be a span: {}",
        html
    );
    assert!(
        html.contains("class=\""),
        "scoped class missing from root: {}",
        html
    );
    assert!(html.contains("data-component=\"rustic-tooltip\""));
    assert!(html.contains("data-rustic-tooltip-id=\"rustic-tooltip-adapter-tooltip\"",));
    assert!(html.contains("data-rustic-tooltip-root=\"rustic-tooltip-adapter-tooltip-root\"",));
    assert!(html.contains("data-trigger-id=\"rustic-tooltip-adapter-tooltip-trigger\"",));
    assert!(html.contains("data-surface-id=\"rustic-tooltip-adapter-tooltip-surface\"",));
    assert!(html.contains("data-portal-layer=\"popover\""));
    assert!(html.contains("data-portal-root=\"rustic-tooltip-adapter-tooltip-popover\"",));
    assert!(html.contains("aria-describedby=\"adapter-tooltip-surface\""));
    assert!(html.contains("aria-controls=\"adapter-tooltip-surface\""));
    assert!(html.contains("aria-haspopup=\"dialog\""));
    assert!(html.contains("role=\"tooltip\""));
    assert!(html.contains("data-component=\"rustic-tooltip-trigger\""));
    assert!(html.contains("data-component=\"rustic-tooltip-surface\""));
    assert!(
        html.contains("data-rustic-tooltip-trigger=\"rustic-tooltip-adapter-tooltip-trigger\"",)
    );
    assert!(
        html.contains("data-rustic-tooltip-surface=\"rustic-tooltip-adapter-tooltip-surface\"",)
    );
    assert!(
        html.matches("class=\"").count() >= 3,
        "expected scoped classes on root, trigger and surface"
    );
    assert!(
        html.contains(">Detailed analytics</div>") || html.contains(">Detailed analytics</span>"),
        "tooltip content should render"
    );
    assert!(html.contains("data-rustic-tooltip-portal=\"rustic-tooltip-adapter-tooltip-popover\"",));
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_adapter_exposes_portal_and_automation_hooks() {
        let props = sample_props();
        let state = build_state();
        let html = tooltip::yew::render(&props, &state);

        assert_tooltip_markup(&html);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_adapter_matches_yew_markup() {
        let props = sample_props();
        let state = build_state();
        let html = tooltip::leptos::render(&props, &state);

        assert_tooltip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let baseline = tooltip::yew::render(&props, &state);
            assert_eq!(html, baseline, "leptos output should mirror yew");
        }
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_adapter_emits_shared_markup() {
        let props = sample_props();
        let state = build_state();
        let html = tooltip::dioxus::render(&props, &state);

        assert_tooltip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let baseline = tooltip::yew::render(&props, &state);
            assert_eq!(html, baseline, "dioxus output should mirror yew");
        }
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_adapter_emits_shared_markup() {
        let props = sample_props();
        let state = build_state();
        let html = tooltip::sycamore::render(&props, &state);

        assert_tooltip_markup(&html);

        #[cfg(feature = "yew")]
        {
            let baseline = tooltip::yew::render(&props, &state);
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
mod cross_framework_parity {
    use super::*;

    #[test]
    fn all_adapters_share_identical_markup() {
        let props = sample_props();
        let state = build_state();
        let yew = tooltip::yew::render(&props, &state);
        let leptos = tooltip::leptos::render(&props, &state);
        let dioxus = tooltip::dioxus::render(&props, &state);
        let sycamore = tooltip::sycamore::render(&props, &state);

        assert_eq!(yew, leptos);
        assert_eq!(yew, dioxus);
        assert_eq!(yew, sycamore);
    }
}
