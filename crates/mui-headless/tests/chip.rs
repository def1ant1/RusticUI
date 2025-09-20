use std::time::Duration;

use mui_headless::chip::{ChipAttributes, ChipConfig, ChipDeleteAttributes, ChipState};
use mui_headless::timing::MockClock;

fn bootstrap_state(clock: MockClock) -> ChipState<MockClock> {
    ChipState::with_clock(clock, ChipConfig::default())
}

#[test]
fn controls_follow_hover_timing() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    assert_eq!(state.pointer_enter().controls_visible, None);
    clock.advance(Duration::from_millis(119));
    assert_eq!(state.poll().controls_visible, None);
    clock.advance(Duration::from_millis(1));
    assert_eq!(state.poll().controls_visible, Some(true));
    assert!(state.controls_visible());

    assert_eq!(state.pointer_leave().controls_visible, None);
    clock.advance(Duration::from_millis(159));
    assert_eq!(state.poll().controls_visible, None);
    clock.advance(Duration::from_millis(1));
    assert_eq!(state.poll().controls_visible, Some(false));
    assert!(!state.controls_visible());
}

#[test]
fn deletion_commits_after_delay() {
    let clock = MockClock::new();
    let mut state = ChipState::with_clock(
        clock.clone(),
        ChipConfig {
            delete_delay: Duration::from_millis(80),
            ..ChipConfig::default()
        },
    );

    state.pointer_enter();
    clock.advance(Duration::from_millis(120));
    state.poll();
    assert!(state.controls_visible());

    state.request_delete();
    assert!(state.deletion_pending());
    clock.advance(Duration::from_millis(79));
    assert_eq!(state.poll().deleted, false);
    clock.advance(Duration::from_millis(1));
    let change = state.poll();
    assert!(change.deleted);
    assert!(!state.is_visible());
    assert!(!state.controls_visible());
}

#[test]
fn escape_cancels_pending_delete() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    state.pointer_enter();
    clock.advance(Duration::from_millis(120));
    state.poll();

    state.request_delete();
    let change = state.escape();
    assert!(change.deletion_cancelled);
    assert!(!state.deletion_pending());

    clock.advance(Duration::from_millis(500));
    assert_eq!(state.poll().deleted, false);
    assert!(state.is_visible());
}

#[test]
fn disabled_chips_ignore_interaction() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());
    state.set_disabled(true);

    let change = state.pointer_enter();
    assert_eq!(change.controls_visible, None);
    assert!(!state.controls_visible());
}

#[test]
fn aria_builders_reflect_state() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    let attrs = ChipAttributes::new(&state)
        .id("chip")
        .labelled_by("label")
        .described_by("hint");
    assert_eq!(attrs.role(), "button");
    assert_eq!(attrs.id_attr(), Some(("id", "chip")));
    assert_eq!(attrs.labelledby(), Some(("aria-labelledby", "label")));
    assert_eq!(attrs.describedby(), Some(("aria-describedby", "hint")));
    assert_eq!(attrs.hidden(), ("aria-hidden", "false"));
    assert_eq!(ChipAttributes::new(&state).disabled(), None);
    assert_eq!(ChipAttributes::new(&state).data_disabled(), None);

    state.set_disabled(true);
    assert_eq!(
        ChipAttributes::new(&state).disabled(),
        Some(("aria-disabled", "true".into()))
    );
    assert_eq!(
        ChipAttributes::new(&state).data_disabled(),
        Some(("data-disabled", "true".into()))
    );

    let delete_attrs = ChipDeleteAttributes::new(&state).label("Remove filter");
    assert_eq!(delete_attrs.role(), "button");
    assert_eq!(delete_attrs.hidden(), ("aria-hidden", "true"));
    assert_eq!(
        delete_attrs.aria_label(),
        Some(("aria-label", "Remove filter"))
    );
}
