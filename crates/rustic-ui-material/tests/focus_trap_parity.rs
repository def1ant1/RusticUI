use rustic_ui_headless::focus_trap::FocusTrapState;
use rustic_ui_material::focus_trap::{
    render_focus_trap_sentinel_html, FocusTrapSentinelKind, FocusTrapSentinelOptions,
};

fn seeded_trap() -> FocusTrapState {
    let mut trap = FocusTrapState::new(true);
    trap.set_analytics_tag(Some("dialog-focus"));
    trap.set_focusables(["trigger", "close"]);
    trap
}

fn sentinel_options() -> FocusTrapSentinelOptions {
    FocusTrapSentinelOptions {
        automation_prefix: Some("dialog::checkout".into()),
    }
}

#[test]
fn material_focus_trap_sentinel_contains_expected_attributes() {
    let trap = seeded_trap();
    let options = sentinel_options();
    let html =
        render_focus_trap_sentinel_html(&trap, FocusTrapSentinelKind::Start, &options, "dialog");
    assert!(html.contains("data-rustic-focus-trap=\"sentinel-start\""));
    assert!(html.contains("data-automation-id=\"dialog::checkout::focus-trap-start\""));
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_focus_trap_matches_material_baseline() {
    use rustic_ui_material::focus_trap::dioxus;

    let trap = seeded_trap();
    let options = sentinel_options();
    let fallback = "dialog".to_string();

    let baseline =
        render_focus_trap_sentinel_html(&trap, FocusTrapSentinelKind::End, &options, &fallback);
    let html = dioxus::render(&dioxus::FocusTrapSentinelProps {
        state: trap,
        kind: FocusTrapSentinelKind::End,
        options,
        fallback_prefix: fallback.clone(),
    });

    assert_eq!(html, baseline);
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_focus_trap_matches_material_baseline() {
    use rustic_ui_material::focus_trap::sycamore;

    let trap = seeded_trap();
    let options = sentinel_options();
    let fallback = "dialog".to_string();

    let baseline =
        render_focus_trap_sentinel_html(&trap, FocusTrapSentinelKind::End, &options, &fallback);
    let html = sycamore::render(&sycamore::FocusTrapSentinelProps {
        state: trap,
        kind: FocusTrapSentinelKind::End,
        options,
        fallback_prefix: fallback.clone(),
    });

    assert_eq!(html, baseline);
}
