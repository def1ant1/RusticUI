use rustic_ui_headless::text_field::TextFieldState;
use proptest::prelude::*;
use std::time::Duration;

fn string_strategy() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[A-Za-z0-9 ]{0,8}").unwrap()
}

proptest! {
    /// Validate that change notifications surface the configured debounce window
    /// while keeping the dirty flag aligned with the initial baseline for both
    /// controlled and uncontrolled text fields.
    #[test]
    fn change_events_propagate_debounce(
        initial in string_strategy(),
        next in string_strategy(),
        debounce_ms in proptest::option::of(0u64..2000u64),
        controlled in any::<bool>(),
    ) {
        let debounce = debounce_ms.map(Duration::from_millis);
        let mut state = if controlled {
            TextFieldState::controlled(initial.clone(), debounce)
        } else {
            TextFieldState::uncontrolled(initial.clone(), debounce)
        };

        let mut observed = None;
        state.change(next.clone(), |snapshot| {
            observed = Some((snapshot.value.to_string(), snapshot.dirty, snapshot.debounce));
        });

        let (value, dirty, emitted_debounce) = observed.expect("change handler not invoked");
        prop_assert_eq!(value.as_str(), next.as_str());
        prop_assert_eq!(dirty, initial != next);
        prop_assert_eq!(state.dirty(), initial != next);
        prop_assert_eq!(emitted_debounce, debounce);
        prop_assert_eq!(state.debounce(), debounce);
    }
}

proptest! {
    /// Committing and resetting the field should propagate validation state so
    /// analytics hooks and accessibility attributes stay aligned with the latest
    /// errors even when they are mutated dynamically.
    #[test]
    fn validation_errors_flow_through_commit_and_reset(
        initial in string_strategy(),
        errors in proptest::collection::vec(string_strategy(), 0..4),
        controlled in any::<bool>(),
    ) {
        let mut state = if controlled {
            TextFieldState::controlled(initial.clone(), None)
        } else {
            TextFieldState::uncontrolled(initial.clone(), None)
        };
        state.set_errors(errors.clone());

        let mut commit_snapshot = None;
        state.commit(|snapshot| commit_snapshot = Some((snapshot.has_errors, snapshot.previously_visited)));
        let (has_errors, previously_visited) = commit_snapshot.expect("commit snapshot missing");
        prop_assert_eq!(has_errors, !errors.is_empty());
        prop_assert!(!previously_visited);
        prop_assert!(state.visited());

        let mut reset_snapshot = None;
        state.reset(|snapshot| reset_snapshot = Some((snapshot.value.to_string(), snapshot.cleared_errors)));
        let (value, cleared) = reset_snapshot.expect("reset snapshot missing");
        prop_assert_eq!(value, initial);
        prop_assert_eq!(cleared, !errors.is_empty());
        prop_assert!(!state.visited());
        prop_assert!(state.errors().is_empty());
        prop_assert!(!state.dirty());
    }
}

proptest! {
    /// Syncing a controlled field should clear pending edits and recompute the
    /// dirty flag so hydration and CSR renders remain in lock step. The property
    /// also validates that subsequent edits re-trigger the dirty marker.
    #[test]
    fn sync_value_resets_pending_controlled_edits(
        initial in string_strategy(),
        next in string_strategy(),
    ) {
        let mut state = TextFieldState::controlled(initial.clone(), Some(Duration::from_millis(120)));
        state.change(next.clone(), |_| {});
        prop_assert_eq!(state.dirty(), initial != next);
        prop_assert_eq!(state.value(), next.as_str());

        state.sync_value(initial.clone());
        prop_assert_eq!(state.value(), initial.as_str());
        prop_assert!(!state.dirty());
        prop_assert!(state.errors().is_empty());

        state.change(next.clone(), |_| {});
        prop_assert_eq!(state.dirty(), initial != next);
        prop_assert_eq!(state.value(), next.as_str());
    }
}
