use rustic_ui_headless::dialog::{DialogPhase, DialogState, DialogTransition};
use proptest::prelude::*;

fn bool_actions() -> impl Strategy<Value = Vec<bool>> {
    prop::collection::vec(any::<bool>(), 1..24)
}

proptest! {
    /// Exercising a variety of open/close intents against the uncontrolled
    /// dialog ensures the internal phase bookkeeping mirrors the requested
    /// visibility while keeping the focus trap aligned with the modal flag.
    #[test]
    fn uncontrolled_sequences_track_focus_trap(
        default_open in any::<bool>(),
        open_requests in bool_actions(),
    ) {
        let mut state = DialogState::uncontrolled(default_open);

        for desired_open in open_requests {
            let was_open = state.is_open();
            let previous_transition = state.last_transition();
            if desired_open {
                state.open(|_| {});
            } else {
                state.close(|_| {});
            }

            prop_assert_eq!(state.is_open(), desired_open);
            prop_assert_eq!(state.focus_trap_engaged(), desired_open && state.is_modal());
            let expected_transition = if desired_open != was_open {
                if desired_open {
                    Some(DialogTransition::OpenRequested)
                } else {
                    Some(DialogTransition::CloseRequested)
                }
            } else {
                previous_transition
            };
            prop_assert_eq!(state.last_transition(), expected_transition);
            prop_assert_eq!(state.phase().is_visible(), desired_open);
        }
    }
}

proptest! {
    /// Controlled dialogs rely on the host application calling `sync_open`.
    /// This property runs through arbitrary hydration sequences to guarantee the
    /// derived data attributes (phase and focus trap markers) stay in sync with
    /// the synchronized flag regardless of modal configuration.
    #[test]
    fn controlled_sync_sequences_drive_modal_flags(
        start_open in any::<bool>(),
        modal in any::<bool>(),
        sync_values in bool_actions(),
    ) {
        let mut state = DialogState::controlled();
        state.set_modal(modal);
        state.sync_open(start_open);

        for desired_open in sync_values {
            state.sync_open(desired_open);

            prop_assert_eq!(state.is_open(), desired_open);
            let expected_phase = if desired_open {
                DialogPhase::Open
            } else {
                DialogPhase::Closed
            };
            prop_assert_eq!(state.phase(), expected_phase);
            prop_assert_eq!(state.focus_trap_engaged(), desired_open && modal);
            let expected_transition = if desired_open {
                Some(DialogTransition::OpenRequested)
            } else {
                Some(DialogTransition::CloseRequested)
            };
            prop_assert_eq!(state.last_transition(), expected_transition);
        }
    }
}

proptest! {
    /// Pressing escape should only dismiss the dialog when the consumer opts in
    /// and the surface is currently visible.  This property verifies the escape
    /// affordance across both control strategies while ensuring we surface the
    /// correct transition metadata for analytics pipelines.
    #[test]
    fn escape_dismissal_respects_configuration(
        start_open in any::<bool>(),
        escape_enabled in any::<bool>(),
    ) {
        // Uncontrolled dialog checks
        let mut uncontrolled = DialogState::uncontrolled(start_open);
        uncontrolled.set_escape_closes(escape_enabled);
        let mut uncontrolled_notified = None;
        let consumed = uncontrolled.handle_escape(|value| uncontrolled_notified = Some(value));
        if start_open && escape_enabled {
            prop_assert!(consumed);
            prop_assert_eq!(uncontrolled_notified, Some(false));
            prop_assert!(!uncontrolled.is_open());
            prop_assert_eq!(
                uncontrolled.last_transition(),
                Some(DialogTransition::CloseRequested)
            );
        } else {
            prop_assert!(!consumed);
            prop_assert_eq!(uncontrolled_notified, None);
            prop_assert_eq!(uncontrolled.is_open(), start_open);
        }

        // Controlled dialog checks
        let mut controlled = DialogState::controlled();
        controlled.set_escape_closes(escape_enabled);
        controlled.sync_open(start_open);
        let mut controlled_notified = None;
        let consumed = controlled.handle_escape(|value| controlled_notified = Some(value));
        if start_open && escape_enabled {
            prop_assert!(consumed);
            prop_assert_eq!(controlled_notified, Some(false));
            prop_assert_eq!(
                controlled.last_transition(),
                Some(DialogTransition::CloseRequested)
            );
        } else {
            prop_assert!(!consumed);
            prop_assert_eq!(controlled_notified, None);
            prop_assert_eq!(controlled.is_open(), start_open);
        }
    }
}
