#![cfg(feature = "unstable")]

use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::unstable_trap_focus::{
    FocusLoopDirection, FocusLoopEvent, UnstableFocusTrapState,
};

fn trap_with_nodes() -> UnstableFocusTrapState {
    let mut trap = UnstableFocusTrapState::new(true);
    trap.set_focusables(["first", "middle", "last"]);
    trap.register_focus(Some("last"));
    trap
}

#[test]
fn forward_wrap_records_event_and_counter() {
    let mut trap = trap_with_nodes();
    trap.handle_key(ControlKey::ArrowRight);
    assert_eq!(trap.loop_event_count(), 1);
    let event = trap.last_loop_event().expect("expected a loop event");
    assert_eq!(event.direction, FocusLoopDirection::Forward);
    assert_eq!(event.from, "last");
    assert_eq!(event.to, "first");
    assert_eq!(event.total_focusables, 3);
    assert_eq!(event.occurrence, 1);
}

#[test]
fn backward_wrap_tracks_direction() {
    let mut trap = UnstableFocusTrapState::new(true);
    trap.set_focusables(["alpha", "omega"]);
    trap.register_focus(Some("alpha"));
    trap.handle_key(ControlKey::ArrowLeft);
    let events = trap.loop_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].direction.as_str(), "backward");
}

#[test]
fn observer_invoked_on_each_loop() {
    use std::sync::{Arc, Mutex};

    let mut trap = trap_with_nodes();
    let hits: Arc<Mutex<Vec<FocusLoopDirection>>> = Arc::new(Mutex::new(Vec::new()));
    let witness = Arc::clone(&hits);
    trap.set_loop_observer(Some(move |event: &FocusLoopEvent| {
        witness.lock().unwrap().push(event.direction);
    }));

    trap.handle_key(ControlKey::ArrowRight);
    trap.handle_key(ControlKey::ArrowLeft);

    let observed = hits.lock().unwrap();
    assert_eq!(
        observed.as_slice(),
        &[FocusLoopDirection::Forward, FocusLoopDirection::Backward]
    );
}

#[test]
fn take_loop_events_resets_buffer() {
    let mut trap = trap_with_nodes();
    trap.handle_key(ControlKey::ArrowRight);
    let captured = trap.take_loop_events();
    assert_eq!(captured.len(), 1);
    assert!(trap.loop_events().is_empty());
}
