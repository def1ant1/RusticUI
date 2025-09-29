//! Integration tests for the click-away detector state machine.
//!
//! The scenarios focus on concurrency and pointer/focus edge cases that surfaced
//! while hardening the new overlay orchestration pipelines.  Capturing the
//! behaviour here keeps the automation contract stable for every adapter without
//! requiring manual QA sweeps.

use std::sync::{Arc, Mutex};
use std::thread;

use proptest::prelude::*;
use rustic_ui_headless::click_away::{ClickAwayDisposition, ClickAwayState};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn click_away_state_is_send_sync() {
    assert_send_sync::<ClickAwayState>();
}

#[test]
fn concurrent_pointer_sequences_trigger_close_once() {
    let mut seed = ClickAwayState::new();
    seed.set_root_id(Some("dialog-surface"));
    seed.engage();
    seed.process_pointer_down(11, true);
    seed.process_pointer_down(24, false);

    let state = Arc::new(Mutex::new(seed));
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for sequence in [24_u64, 11_u64] {
        let state = Arc::clone(&state);
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            let mut guard = state.lock().expect("state mutex poisoned");
            let disposition = guard.process_pointer_up(sequence, false);
            drop(guard);
            results
                .lock()
                .expect("results mutex poisoned")
                .push((sequence, disposition));
        }));
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    let captured = results.lock().expect("results mutex poisoned");
    let close_events = captured
        .iter()
        .filter(|(_, disposition)| disposition.should_close())
        .count();
    assert_eq!(close_events, 1, "only the external pointer should close");
    assert!(captured
        .iter()
        .any(|(sequence, disposition)| *sequence == 24 && disposition.should_close()));
    assert!(captured
        .iter()
        .any(|(sequence, disposition)| *sequence == 11 && !disposition.should_close()));
}

#[test]
fn focus_exit_with_active_pointer_defers_close() {
    let mut state = ClickAwayState::new();
    state.engage();
    state.process_pointer_down(7, true);

    // Focus exiting while the pointer is still tracked must not emit a close
    // event; drag interactions should stay inside the overlay until the
    // pointer sequence finishes.
    let focus_disposition = state.update_focus_within(false);
    assert_eq!(focus_disposition, ClickAwayDisposition::NoChange);
    assert!(
        state.is_engaged(),
        "detector should remain armed until pointer ends"
    );

    // Once the pointer completes we remain armed so subsequent interactions can
    // continue without re-engaging the detector. Consumers explicitly call
    // `disengage` after the overlay closes.
    let pointer_disposition = state.process_pointer_up(7, false);
    assert_eq!(pointer_disposition, ClickAwayDisposition::NoChange);
    assert!(state.is_engaged());
    state.disengage();
    assert!(!state.is_engaged());
}

proptest! {
    /// Pointer sequences originating outside the boundary must be the only
    /// source of click-away closures. Internal drags should keep the detector
    /// armed so overlays do not close while a user selects text or drags the
    /// surface.
    #[test]
    fn external_sequences_are_the_only_close_trigger(inside_on_down in prop::collection::vec(any::<bool>(), 1..8)) {
        let mut state = ClickAwayState::new();

        for (index, started_inside) in inside_on_down.into_iter().enumerate() {
            state.disengage();
            state.engage();
            state.process_pointer_down(index as u64, started_inside);
            let disposition = state.process_pointer_up(index as u64, false);

            if started_inside {
                prop_assert_eq!(disposition, ClickAwayDisposition::NoChange);
                prop_assert!(state.is_engaged(), "dragging outside should keep detector armed");
            } else {
                prop_assert_eq!(disposition, ClickAwayDisposition::TriggerClose);
                prop_assert!(!state.is_engaged(), "external pointer must disarm detector");
            }
        }
    }
}
