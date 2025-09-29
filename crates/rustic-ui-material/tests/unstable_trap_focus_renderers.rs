#![cfg(feature = "unstable")]

use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::unstable_trap_focus::UnstableFocusTrapState;
use rustic_ui_material::focus_trap::{FocusTrapSentinelKind, FocusTrapSentinelOptions};
use rustic_ui_material::unstable_trap_focus::{
    render_unstable_focus_trap_sentinel_html, unstable_focus_trap_sentinel_attributes,
};

fn seeded_state() -> UnstableFocusTrapState {
    let mut state = UnstableFocusTrapState::new(true);
    state.set_focusables(["trigger", "close"]);
    state.set_analytics_tag(Some("dialog-focus"));
    state.register_focus(Some("close"));
    state.handle_key(ControlKey::ArrowRight);
    state
}

fn sentinel_options() -> FocusTrapSentinelOptions {
    FocusTrapSentinelOptions {
        automation_prefix: Some("dialog::checkout".into()),
    }
}

#[test]
fn material_unstable_focus_trap_includes_loop_metadata() {
    let state = seeded_state();
    let options = sentinel_options();
    let html = render_unstable_focus_trap_sentinel_html(
        &state,
        FocusTrapSentinelKind::Start,
        &options,
        "dialog",
    );
    assert!(html.contains("data-rustic-focus-trap=\"sentinel-start\""));
    assert!(html.contains("data-automation-id=\"dialog::checkout::focus-trap-start\""));
    assert!(html.contains("data-rustic-focus-loop-count=\"1\""));
    assert!(html.contains("data-rustic-focus-loop-last-direction=\"forward\""));
    assert!(html.contains("aria-roledescription=\"focus trap instrumentation sentinel\""));
}

#[test]
fn attribute_builder_reports_loop_statistics() {
    let state = seeded_state();
    let options = sentinel_options();
    let pairs = unstable_focus_trap_sentinel_attributes(
        &state,
        FocusTrapSentinelKind::End,
        &options,
        "dialog",
    );
    assert!(pairs
        .iter()
        .any(|(key, value)| key == "data-rustic-focus-loop-count" && value == "1"));
    assert!(pairs.iter().any(|(key, value)| {
        key == "data-rustic-focus-loop-last-direction" && value == "forward"
    }));
}

#[cfg(all(feature = "dioxus"))]
#[test]
fn dioxus_renderer_matches_html_output() {
    use rustic_ui_material::unstable_focus_trap_dioxus as dioxus_renderer;

    let state = seeded_state();
    let options = sentinel_options();
    let fallback = "dialog".to_string();
    let baseline = render_unstable_focus_trap_sentinel_html(
        &state,
        FocusTrapSentinelKind::End,
        &options,
        &fallback,
    );
    let html = dioxus_renderer::render(&dioxus_renderer::UnstableFocusTrapSentinelProps {
        state,
        kind: FocusTrapSentinelKind::End,
        options,
        fallback_prefix: fallback.clone(),
    });
    assert_eq!(html, baseline);
}
