//! Focus trap regression tests covering the looping navigation contract and
//! thread-safety guarantees relied upon by the new overlay orchestration layer.

use std::sync::{Arc, Mutex};
use std::thread;

use proptest::prelude::*;
use rustic_ui_headless::focus_trap::{FocusDisposition, FocusTrapState};
use rustic_ui_headless::interaction::ControlKey;

fn assert_send_sync<T: Send + Sync>() {}

fn labelled_focusables(len: usize) -> Vec<String> {
    (0..len).map(|index| format!("node-{index}")).collect()
}

#[test]
fn focus_trap_state_is_send_sync() {
    assert_send_sync::<FocusTrapState>();
}

#[test]
fn concurrent_navigation_remains_deterministic() {
    let mut state = FocusTrapState::new(true);
    state.set_focusables(labelled_focusables(3));
    state.register_focus(Some("node-0"));

    let state = Arc::new(Mutex::new(state));
    let mut handles = Vec::new();
    let keys = [ControlKey::ArrowRight, ControlKey::ArrowRight];

    for (index, key) in keys.into_iter().enumerate() {
        let state = Arc::clone(&state);
        handles.push(thread::spawn(move || {
            let mut guard = state.lock().expect("state mutex poisoned");
            let label = match guard.handle_key(key) {
                FocusDisposition::Focus(id) => Some(id.to_string()),
                FocusDisposition::NoChange => None,
            };
            (index, label)
        }));
    }

    let mut observed = Vec::new();
    for handle in handles {
        let disposition = handle.join().expect("navigation thread panicked");
        observed.push(disposition);
    }
    observed.sort_by_key(|(index, _)| *index);

    // Both navigation events should advance focus deterministically even though
    // they executed on different threads. The first call focuses node-1, the
    // second wraps around to node-2 because the trap loops.
    assert_eq!(observed[0].1.as_deref(), Some("node-1"));
    assert_eq!(observed[1].1.as_deref(), Some("node-2"));
}

proptest! {
    /// Looping focus traps should always wrap forward navigation to the start of
    /// the list. This property exercises arbitrary list lengths, start indices
    /// and navigation steps to guarantee the wrap logic remains associative.
    #[test]
    fn looping_traps_wrap_forward_navigation(len in 1usize..8, start in 0usize..16, steps in 1usize..16) {
        let focusables = labelled_focusables(len);
        let mut trap = FocusTrapState::new(true);
        trap.set_focusables(focusables.clone());
        let start_index = start % len;
        trap.register_focus(Some(&focusables[start_index]));

        let mut last = None;
        for _ in 0..steps {
            if let FocusDisposition::Focus(id) = trap.handle_key(ControlKey::ArrowRight) {
                last = Some(id.to_string());
            }
        }

        let expected_index = (start_index + steps) % len;
        let expected_target = focusables[expected_index].clone();
        prop_assert_eq!(last, Some(expected_target));
    }
}

proptest! {
    /// Non-looping traps should clamp navigation at the end of the list so users
    /// do not escape modals unintentionally. The property mirrors a subset of the
    /// business rules enforced by the adapters and analytics instrumentation.
    #[test]
    fn non_looping_traps_clamp_navigation(len in 1usize..8, steps in 1usize..16) {
        let focusables = labelled_focusables(len);
        let mut trap = FocusTrapState::new(false);
        trap.set_focusables(focusables.clone());
        trap.register_focus(Some(&focusables[0]));

        let mut last = Some(focusables[0].clone());
        for _ in 0..steps {
            last = match trap.handle_key(ControlKey::ArrowRight) {
                FocusDisposition::Focus(id) => Some(id.to_string()),
                FocusDisposition::NoChange => last,
            };
        }

        let expected_index = std::cmp::min(steps, len.saturating_sub(1));
        let expected_target = focusables[expected_index].clone();
        prop_assert_eq!(last, Some(expected_target));
    }
}

#[test]
fn previous_navigation_wraps_when_loop_enabled() {
    let mut trap = FocusTrapState::new(true);
    trap.set_focusables(labelled_focusables(3));
    trap.register_focus(Some("node-0"));

    let disposition = trap.handle_key(ControlKey::ArrowLeft);
    assert!(matches!(disposition, FocusDisposition::Focus("node-2")));
}
