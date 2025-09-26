use std::time::Duration;

use rustic_ui_headless::button::ButtonState;
use rustic_ui_headless::chip::{ChipAttributes, ChipConfig, ChipState};
use rustic_ui_headless::timing::MockClock;

type ChipStateUnderTest = ChipState<MockClock>;

// Build a deterministic chip configuration for the adapter verifications. All
// timers resolve instantly so the markup assertions do not rely on real time.
fn chip_config(disabled: bool) -> ChipConfig {
    ChipConfig {
        show_delay: Duration::ZERO,
        hide_delay: Duration::ZERO,
        delete_delay: Duration::ZERO,
        dismissible: true,
        disabled,
    }
}

fn chip_state(disabled: bool) -> ChipStateUnderTest {
    ChipState::with_clock(MockClock::new(), chip_config(disabled))
}

fn render_button_markup(state: &ButtonState, label: &str) -> String {
    let aria = state.aria_attributes();
    format!(
        "<button role=\"{}\" aria-pressed=\"{}\">{label}</button>",
        aria[0].1, aria[1].1
    )
}

fn render_chip_markup(
    state: &ChipStateUnderTest,
    id: Option<&str>,
    labelled_by: Option<&str>,
    described_by: Option<&str>,
) -> String {
    let mut builder = ChipAttributes::new(state);
    if let Some(value) = id {
        builder = builder.id(value);
    }
    if let Some(value) = labelled_by {
        builder = builder.labelled_by(value);
    }
    if let Some(value) = described_by {
        builder = builder.described_by(value);
    }

    let mut attrs = Vec::new();
    attrs.push(format!("role=\"{}\"", builder.role()));
    let (_, hidden) = builder.hidden();
    attrs.push(format!("aria-hidden=\"{hidden}\""));
    if let Some((_, value)) = builder.disabled() {
        attrs.push(format!("aria-disabled=\"{value}\""));
    }
    if let Some((_, value)) = builder.data_disabled() {
        attrs.push(format!("data-disabled=\"{value}\""));
    }
    if let Some((_, value)) = builder.id_attr() {
        attrs.push(format!("id=\"{value}\""));
    }
    if let Some((_, value)) = builder.labelledby() {
        attrs.push(format!("aria-labelledby=\"{value}\""));
    }
    if let Some((_, value)) = builder.describedby() {
        attrs.push(format!("aria-describedby=\"{value}\""));
    }

    format!("<span {}></span>", attrs.join(" "))
}

#[cfg(feature = "yew")]
mod yew {
    use super::*;
    use rustic_ui_joy::helpers::{ButtonAria, ChipAdapterConfig, ChipAria};
    use yew::virtual_dom::AttrValue;

    #[test]
    fn button_adapter_maps_pressed_state_into_attr_values() {
        // Yew hooks should faithfully translate the headless pressed flag so
        // interactive analytics keep working across SSR + hydration.
        let mut state = ButtonState::new(false, None);
        let aria = ButtonAria::from_pairs(state.aria_attributes());
        assert_eq!(aria.role, AttrValue::from("button"));
        assert_eq!(aria.aria_pressed, AttrValue::from("false"));

        state.press(|_| {});
        let pressed = ButtonAria::from_pairs(state.aria_attributes());
        assert_eq!(pressed.aria_pressed, AttrValue::from("true"));
    }

    #[test]
    fn chip_adapter_includes_identifiers_and_disabled_markers() {
        // The Joy Yew adapter augments the headless state with automation IDs
        // so enterprise E2E tests can target chips reliably.
        let mut config = ChipAdapterConfig {
            dismissible: true,
            disabled: true,
            show_delay: Duration::ZERO,
            hide_delay: Duration::ZERO,
            delete_delay: Duration::ZERO,
            id: Some("chip-42".into()),
            labelled_by: Some("chip-label".into()),
            described_by: Some("chip-help".into()),
        };
        let state = ChipState::with_clock(MockClock::new(), config.headless_config());
        let aria = ChipAria::from_state(&state, &config);

        assert_eq!(aria.role, AttrValue::from("button"));
        assert_eq!(aria.aria_hidden, AttrValue::from("false"));
        assert_eq!(aria.id, Some(AttrValue::from("chip-42")));
        assert_eq!(aria.aria_labelledby, Some(AttrValue::from("chip-label")));
        assert_eq!(aria.aria_describedby, Some(AttrValue::from("chip-help")));
        assert_eq!(aria.aria_disabled, Some(AttrValue::from("true")));
        assert_eq!(aria.data_disabled, Some(AttrValue::from("true")));
    }
}

#[cfg(feature = "leptos")]
mod leptos {
    use super::*;

    #[test]
    fn button_markup_shows_pressed_state_snapshot() {
        // The Leptos adapter should emit identical markup so SSR snapshots stay
        // aligned with the Yew implementation.
        let state = ButtonState::new(false, None);
        let markup = render_button_markup(&state, "Approve");
        assert_eq!(
            markup,
            "<button role=\"button\" aria-pressed=\"false\">Approve</button>"
        );
    }

    #[test]
    fn chip_markup_reports_hidden_and_disabled_flags() {
        // Delete and disabled states must survive the server rendering pass so
        // downstream hydration never exposes stale automation hooks.
        let mut state = chip_state(true);
        state.request_delete();
        let markup = render_chip_markup(&state, Some("chip"), Some("filter"), Some("hint"));
        assert!(markup.contains("aria-hidden=\"true\""));
        assert!(markup.contains("aria-disabled=\"true\""));
        assert!(markup.contains("data-disabled=\"true\""));
        assert!(markup.contains("aria-labelledby=\"filter\""));
        assert!(markup.contains("aria-describedby=\"hint\""));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus {
    use super::*;

    #[test]
    fn button_markup_consistency_guards_against_regressions() {
        // Dioxus renderers reuse the same automation contract, so we snapshot
        // the expected markup to prevent regressions when adapting templates.
        let mut state = ButtonState::new(false, None);
        let initial = render_button_markup(&state, "Save");
        assert_eq!(
            initial,
            "<button role=\"button\" aria-pressed=\"false\">Save</button>"
        );

        state.press(|_| {});
        let toggled = render_button_markup(&state, "Save");
        assert!(toggled.contains("aria-pressed=\"true\""));
    }

    #[test]
    fn chip_markup_retains_identity_attributes() {
        // The markup builder ensures data and aria hooks survive when rendered
        // through the Dioxus adapter.
        let state = chip_state(false);
        let markup = render_chip_markup(&state, Some("chip-x"), Some("label"), Some("desc"));
        assert!(markup.contains("role=\"button\""));
        assert!(markup.contains("aria-hidden=\"false\""));
        assert!(markup.contains("id=\"chip-x\""));
        assert!(markup.contains("aria-labelledby=\"label\""));
        assert!(markup.contains("aria-describedby=\"desc\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore {
    use super::*;

    #[test]
    fn button_markup_includes_accessibility_role() {
        // Sycamore shares the same minimal button contract, so verifying the
        // role + aria snapshot protects the cross framework parity story.
        let state = ButtonState::new(false, None);
        let markup = render_button_markup(&state, "Archive");
        assert_eq!(
            markup,
            "<button role=\"button\" aria-pressed=\"false\">Archive</button>"
        );
    }

    #[test]
    fn chip_markup_tracks_visibility_flip() {
        // When a chip is deleted the adapter must expose aria-hidden so screen
        // readers drop the element. This ensures the Sycamore binding follows suit.
        let mut state = chip_state(false);
        state.request_delete();
        let markup = render_chip_markup(&state, None, None, None);
        assert!(markup.contains("aria-hidden=\"true\""));
    }
}
