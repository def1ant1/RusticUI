use std::time::Duration;

use rustic_ui_headless::timing::MockClock;
use rustic_ui_headless::tooltip::{
    TooltipConfig, TooltipState, TooltipSurfaceAttributes, TooltipTriggerAttributes,
};

fn bootstrap_state(clock: MockClock) -> TooltipState<MockClock> {
    TooltipState::with_clock(clock, TooltipConfig::default())
}

#[test]
fn show_timer_is_respected() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    let change = state.focus_anchor();
    assert_eq!(
        change.visibility_changed, None,
        "change should be timer driven"
    );

    clock.advance(Duration::from_millis(149));
    assert_eq!(state.poll().visibility_changed, None);

    clock.advance(Duration::from_millis(1));
    let change = state.poll();
    assert_eq!(change.visibility_changed, Some(true));
    assert!(state.visible());
}

#[test]
fn hide_timer_cancels_when_surface_hovered() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    state.pointer_enter_anchor();
    clock.advance(Duration::from_millis(150));
    state.poll();
    assert!(state.visible());

    state.pointer_leave_anchor();
    clock.advance(Duration::from_millis(50));
    state.pointer_enter_tooltip();
    assert_eq!(state.poll().visibility_changed, None);

    clock.advance(Duration::from_millis(200));
    assert_eq!(
        state.poll().visibility_changed,
        None,
        "tooltip should stay visible"
    );

    state.pointer_leave_tooltip();
    clock.advance(Duration::from_millis(100));
    assert_eq!(state.poll().visibility_changed, Some(false));
    assert!(!state.visible());
}

#[test]
fn dismiss_respects_configuration() {
    let clock = MockClock::new();
    let mut state = TooltipState::with_clock(
        clock.clone(),
        TooltipConfig {
            dismissible: false,
            ..TooltipConfig::default()
        },
    );

    state.focus_anchor();
    clock.advance(Duration::from_millis(150));
    state.poll();
    assert!(state.visible());

    assert_eq!(state.dismiss().visibility_changed, None);
    assert!(state.visible());
}

#[test]
fn blur_does_not_hide_while_hovered() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    state.pointer_enter_anchor();
    clock.advance(Duration::from_millis(150));
    state.poll();
    assert!(state.visible());

    state.focus_anchor();
    state.blur_anchor();
    clock.advance(Duration::from_millis(200));
    assert_eq!(state.poll().visibility_changed, None);

    state.pointer_leave_anchor();
    clock.advance(Duration::from_millis(100));
    assert_eq!(state.poll().visibility_changed, Some(false));
}

#[test]
fn aria_builders_reflect_visibility() {
    let clock = MockClock::new();
    let mut state = bootstrap_state(clock.clone());

    let trigger = TooltipTriggerAttributes::new(&state)
        .id("trigger")
        .described_by("tip");
    assert_eq!(trigger.id_attr(), Some(("id", "trigger")));
    assert_eq!(trigger.describedby(), Some(("aria-describedby", "tip")));
    assert_eq!(trigger.expanded(), ("aria-expanded", "false"));

    let surface = TooltipSurfaceAttributes::new(&state).id("tip");
    assert_eq!(surface.role(), "tooltip");
    assert_eq!(surface.id_attr(), Some(("id", "tip")));
    assert_eq!(surface.hidden(), ("aria-hidden", "true"));

    state.focus_anchor();
    clock.advance(Duration::from_millis(150));
    state.poll();

    let trigger = TooltipTriggerAttributes::new(&state).described_by("tip");
    assert_eq!(trigger.expanded(), ("aria-expanded", "true"));

    let surface = TooltipSurfaceAttributes::new(&state).id("tip");
    assert_eq!(surface.hidden(), ("aria-hidden", "false"));
}
