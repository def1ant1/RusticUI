use std::time::Duration;

use mui_headless::button::ButtonState;
use mui_headless::chip::{ChipAttributes, ChipConfig, ChipState};
use mui_headless::timing::MockClock;

// Utility helper returning a configuration where every transition fires
// immediately. This keeps the assertions deterministic without waiting for real
// timers which is critical for CI automation and local contributors alike.
fn instant_chip_config() -> ChipConfig {
    ChipConfig {
        show_delay: Duration::ZERO,
        hide_delay: Duration::ZERO,
        delete_delay: Duration::ZERO,
        dismissible: true,
        disabled: false,
    }
}

#[test]
fn button_press_throttles_follow_up_clicks_and_updates_aria() {
    // This test validates the automation contract between the Joy button and
    // the shared headless state machine. A throttle window should stop double
    // submissions while still exposing the correct aria metadata for QA bots.
    let mut state = ButtonState::new(false, Some(Duration::from_millis(250)));
    let mut invocations = 0;

    state.press(|_| invocations += 1);
    state.press(|_| invocations += 1);

    assert_eq!(
        invocations, 1,
        "throttle must suppress rapid follow up clicks"
    );

    let aria_pairs = state.aria_attributes();
    assert_eq!(aria_pairs[0], ("role", "button"));
    assert_eq!(aria_pairs[1], ("aria-pressed", "true"));
}

#[test]
fn chip_hover_and_delete_flow_emits_expected_visibility_changes() {
    // Enterprise dashboards depend on deterministic hover/delete behaviour so
    // we simulate the full flow: hover exposes controls, a delete request
    // removes the chip and subsequent ARIA output marks it hidden.
    let clock = MockClock::new();
    let mut state = ChipState::with_clock(clock, instant_chip_config());

    let change = state.pointer_enter();
    assert_eq!(change.controls_visible, Some(true));
    assert!(state.controls_visible(), "hover should reveal controls");

    let aria = ChipAttributes::new(&state);
    let (_, hidden_before) = aria.hidden();
    assert_eq!(
        hidden_before, "false",
        "visible chip should not be aria-hidden"
    );

    let delete_change = state.request_delete();
    assert!(
        delete_change.deleted,
        "delete should immediately mark the chip removed"
    );
    assert!(!state.is_visible(), "state should reflect logical deletion");

    let aria_after = ChipAttributes::new(&state);
    let (_, hidden_after) = aria_after.hidden();
    assert_eq!(
        hidden_after, "true",
        "deleted chips must flip aria-hidden for assistive tech"
    );
}

#[test]
fn chip_escape_cancels_pending_deletion_and_restores_controls() {
    // Pending deletions must be cancellable so keyboard users can recover from
    // mistakes. We schedule a delayed delete, trigger escape and ensure the
    // state machine never commits the removal even after the timer would fire.
    let clock = MockClock::new();
    let mut config = instant_chip_config();
    config.delete_delay = Duration::from_millis(300);
    let mut state = ChipState::with_clock(clock.clone(), config);

    state.pointer_enter();
    state.request_delete();
    assert!(state.deletion_pending(), "delete timer should be active");

    let change = state.escape();
    assert!(
        change.deletion_cancelled,
        "escape must broadcast cancellation to adapters"
    );
    assert!(
        !state.deletion_pending(),
        "cancellation should clear the pending flag"
    );

    clock.advance(Duration::from_millis(400));
    let poll = state.poll();
    assert!(
        !poll.deleted,
        "cancellation must prevent the delayed deletion"
    );
    assert!(
        state.is_visible(),
        "chip should remain visible after cancellation"
    );
}
