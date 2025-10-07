//! Property-based coverage for the collapsible region state machine.
//!
//! Each property in this suite is heavily annotated so new contributors can
//! trace the expectations back to the architectural state diagrams and the
//! enterprise-grade focus orchestration guarantees the component promises.
//! Cross-check the expectations with the diagrams in
//! [`docs/architecture/headless-state-machines.md#collapsible-region-state-machine`](../../../docs/architecture/headless-state-machines.md#collapsible-region-state-machine)
//! and the token lifecycle walkthrough under
//! [`#token-lifecycle-orchestration`](../../../docs/architecture/headless-state-machines.md#token-lifecycle-orchestration).
//! The goal is to document *why* each invariant matters for large scale
//! surfaces so future refactors have guard rails baked into CI instead of
//! relying on tribal knowledge.

use std::collections::BTreeSet;

use proptest::prelude::*;
use rustic_ui_headless::collapsible_region::{CollapsibleRegionState, RegionTransition};

/// Actions a consumer can trigger against the region state machine.
#[derive(Debug, Clone)]
enum RegionAction {
    Expand,
    Collapse,
    Toggle,
}

fn region_action_strategy() -> impl Strategy<Value = RegionAction> {
    prop_oneof![
        Just(RegionAction::Expand),
        Just(RegionAction::Collapse),
        Just(RegionAction::Toggle),
    ]
}

/// Steps used to exercise the controlled mode.  `sync_after` tells the test to
/// mirror a framework adapter calling [`CollapsibleRegionState::sync`] after the
/// host application has rendered the new expansion state.
#[derive(Debug, Clone)]
struct ControlledStep {
    action: RegionAction,
    sync_after: bool,
}

fn controlled_step_strategy() -> impl Strategy<Value = ControlledStep> {
    (region_action_strategy(), any::<bool>())
        .prop_map(|(action, sync_after)| ControlledStep { action, sync_after })
}

/// Commands exercised against the focus-return plumbing.  These mirror the
/// possible adapter behaviours: leave the configuration alone, clear it (when
/// the trigger moves out of the DOM), or swap in a new identifier as the UI
/// re-parents elements during virtualization.
#[derive(Debug, Clone)]
enum FocusCommand {
    Keep,
    Clear,
    Set(String),
}

fn focus_command_strategy() -> impl Strategy<Value = FocusCommand> {
    let id_strategy = proptest::string::string_regex("[A-Za-z0-9_-]{1,8}")
        .expect("regex validated for id generation");
    prop_oneof![
        Just(FocusCommand::Keep),
        Just(FocusCommand::Clear),
        id_strategy.prop_map(FocusCommand::Set),
    ]
}

/// Bundles focus commands with region actions so the properties can simulate how
/// downstream renderers shuffle focus targets while the disclosure opens and
/// closes.
#[derive(Debug, Clone)]
struct FocusStep {
    command: FocusCommand,
    action: RegionAction,
}

fn focus_step_strategy() -> impl Strategy<Value = FocusStep> {
    (focus_command_strategy(), region_action_strategy())
        .prop_map(|(command, action)| FocusStep { command, action })
}

/// Operations over the transition token API.  The caller can either reserve a
/// token (`Begin`) or mark it finished (`Finish`).
#[derive(Debug, Clone)]
enum TokenOperation {
    Begin(u64),
    Finish(u64),
}

fn token_operation_strategy() -> impl Strategy<Value = TokenOperation> {
    prop_oneof![
        (0u64..64).prop_map(TokenOperation::Begin),
        (0u64..64).prop_map(TokenOperation::Finish),
    ]
}

proptest! {
    /// Uncontrolled regions should behave like a simple boolean latch.  The
    /// invariant we care about is **notification parity**: whenever the state
    /// reports that the region expanded/collapsed we must observe (1) the
    /// `RegionTransition` aligning with the boolean model and (2) the callback
    /// receiving the new canonical state.  This prevents adapters from observing
    /// stale values and keeps automation runs deterministic when the disclosure
    /// participates in complex layouts.
    #[test]
    fn uncontrolled_regions_follow_boolean_model(
        default_expanded in any::<bool>(),
        actions in prop::collection::vec(region_action_strategy(), 1..8),
    ) {
        let mut state = CollapsibleRegionState::uncontrolled(default_expanded);
        let mut expected = default_expanded;

        for (index, action) in actions.into_iter().enumerate() {
            let mut observed = None;
            let transition = match action {
                RegionAction::Expand => state.expand(|expanded| observed = Some(expanded)),
                RegionAction::Collapse => state.collapse(|expanded| observed = Some(expanded)),
                RegionAction::Toggle => state.toggle(|expanded| observed = Some(expanded)),
            };

            match action {
                RegionAction::Expand => {
                    if expected {
                        // Expanding an already-open disclosure should be a no-op so
                        // analytics snapshots and focus flows stay idempotent.
                        prop_assert_eq!(transition, RegionTransition::NoChange, "step {}", index);
                        prop_assert_eq!(observed, None, "no notification emitted for redundant expand");
                    } else {
                        prop_assert_eq!(transition, RegionTransition::Expanded, "step {}", index);
                        prop_assert_eq!(observed, Some(true), "notify receives the new expanded value");
                        expected = true;
                    }
                }
                RegionAction::Collapse => {
                    if expected {
                        prop_assert_eq!(transition, RegionTransition::Collapsed, "step {}", index);
                        prop_assert_eq!(observed, Some(false), "notify receives the collapsed state");
                        expected = false;
                    } else {
                        prop_assert_eq!(transition, RegionTransition::NoChange, "step {}", index);
                        prop_assert_eq!(observed, None, "no notification emitted when already collapsed");
                    }
                }
                RegionAction::Toggle => {
                    if expected {
                        prop_assert_eq!(transition, RegionTransition::Collapsed, "step {}", index);
                        prop_assert_eq!(observed, Some(false), "toggle collapsing must report false");
                    } else {
                        prop_assert_eq!(transition, RegionTransition::Expanded, "step {}", index);
                        prop_assert_eq!(observed, Some(true), "toggle expanding must report true");
                    }
                    expected = !expected;
                }
            }

            // The internal state should mirror our boolean model after every step
            // to guarantee deterministic hydration between server/client renders.
            prop_assert_eq!(state.is_expanded(), expected, "state diverged at step {}", index);
        }
    }
}

proptest! {
    /// Controlled integrations keep ownership of the canonical `expanded` flag
    /// and rely on the `sync` callback to mirror framework renders.  This
    /// property enforces two guarantees:
    /// 1. The state machine never mutates `expanded` until `sync` executes so
    ///    React/Vue/Leptos adapters avoid feedback loops.
    /// 2. The emitted notifications encode the exact value the UI should render,
    ///    ensuring the host can round-trip the state without guessing.
    #[test]
    fn controlled_regions_only_commit_after_sync(
        steps in prop::collection::vec(controlled_step_strategy(), 1..8),
    ) {
        let mut state = CollapsibleRegionState::controlled();
        let mut synced = false;
        let mut pending: Option<bool> = None;

        for (index, step) in steps.into_iter().enumerate() {
            let mut observed = None;
            let transition = match step.action {
                RegionAction::Expand => state.expand(|expanded| observed = Some(expanded)),
                RegionAction::Collapse => state.collapse(|expanded| observed = Some(expanded)),
                RegionAction::Toggle => state.toggle(|expanded| observed = Some(expanded)),
            };

            match step.action {
                RegionAction::Expand => match transition {
                    RegionTransition::Expanded => {
                        prop_assert!(!synced, "expand should only fire when synced=false at step {}", index);
                        prop_assert_eq!(observed, Some(true), "expand must request sync(true)");
                        pending = Some(true);
                    }
                    RegionTransition::NoChange => {
                        prop_assert!(synced, "expand should no-op when already synced open at step {}", index);
                        prop_assert_eq!(observed, None, "no redundant notification when already open");
                    }
                    RegionTransition::Collapsed => prop_assert!(false, "expand cannot report collapsed"),
                },
                RegionAction::Collapse => match transition {
                    RegionTransition::Collapsed => {
                        prop_assert!(synced, "collapse should only fire when synced=true at step {}", index);
                        prop_assert_eq!(observed, Some(false), "collapse must request sync(false)");
                        pending = Some(false);
                    }
                    RegionTransition::NoChange => {
                        prop_assert!(!synced, "collapse should no-op when already synced closed at step {}", index);
                        prop_assert_eq!(observed, None, "no redundant notification when already closed");
                    }
                    RegionTransition::Expanded => prop_assert!(false, "collapse cannot report expanded"),
                },
                RegionAction::Toggle => {
                    if synced {
                        prop_assert_eq!(transition, RegionTransition::Collapsed, "toggle must collapse when synced open at step {}", index);
                        prop_assert_eq!(observed, Some(false), "toggle collapse should request sync(false)");
                        pending = Some(false);
                    } else {
                        prop_assert_eq!(transition, RegionTransition::Expanded, "toggle must expand when synced closed at step {}", index);
                        prop_assert_eq!(observed, Some(true), "toggle expand should request sync(true)");
                        pending = Some(true);
                    }
                }
            }

            // Until `sync` fires the headless state must stay at the last synced
            // value so framework render cycles never loop.
            prop_assert_eq!(state.is_expanded(), synced, "controlled state mutated before sync at step {}", index);

            if step.sync_after {
                let target = pending.unwrap_or(synced);
                state.sync(target);
                synced = target;
                pending = None;
            }

            // After syncing (or intentionally skipping it) the observed state must
            // track `synced`, giving adapters deterministic hydration behaviour.
            prop_assert_eq!(state.is_expanded(), synced, "controlled state diverged post-sync at step {}", index);
        }
    }
}

proptest! {
    /// Transition tokens serialize concurrent animations, analytics events, and
    /// logging hooks.  The mission-critical invariant is that **each token is
    /// unique until released** so orchestrators never double-complete and our
    /// ordering remains stable.  This property mirrors a `BTreeSet` shadow copy
    /// to assert that `begin_transition` rejects duplicates and `finish_transition`
    /// keeps `is_transitioning` accurate even when finishes arrive out of order.
    #[test]
    fn transition_tokens_never_duplicate(
        ops in prop::collection::vec(token_operation_strategy(), 1..16),
    ) {
        let mut state = CollapsibleRegionState::uncontrolled(false);
        let mut shadow = BTreeSet::new();

        for (index, op) in ops.into_iter().enumerate() {
            match op {
                TokenOperation::Begin(token) => {
                    let inserted = state.begin_transition(token);
                    let expected = shadow.insert(token);
                    prop_assert_eq!(inserted, expected, "duplicate token admitted at step {}", index);
                }
                TokenOperation::Finish(token) => {
                    state.finish_transition(token);
                    shadow.remove(&token);
                }
            }

            // The observable transitioning flag should line up with the set's
            // emptiness.  If it ever desynchronizes we risk prematurely enabling
            // focus or analytics flows while animations are still running.
            prop_assert_eq!(state.is_transitioning(), !shadow.is_empty(), "transitioning flag out of sync at step {}", index);
        }
    }
}

proptest! {
    /// When a collapsible hides it should direct focus back to the configured
    /// trigger.  Enterprise surfaces depend on this to avoid trapping keyboard
    /// users inside virtualized accordions.  The invariant enforced here is that
    /// **focus return targets survive arbitrary region mutations** so adapters can
    /// trust the state machine to return the last configured identifier even when
    /// the region collapses multiple times in a row.
    #[test]
    fn focus_return_configuration_survives_transitions(
        default_expanded in any::<bool>(),
        steps in prop::collection::vec(focus_step_strategy(), 1..8),
    ) {
        let mut state = CollapsibleRegionState::uncontrolled(default_expanded);
        let mut expanded = default_expanded;
        let mut expected_focus: Option<String> = None;

        for (index, step) in steps.into_iter().enumerate() {
            match step.command {
                FocusCommand::Keep => {}
                FocusCommand::Clear => {
                    state.set_focus_return(None::<String>);
                    expected_focus = None;
                }
                FocusCommand::Set(id) => {
                    state.set_focus_return(Some(id.clone()));
                    expected_focus = Some(id);
                }
            }

            let transition = match step.action {
                RegionAction::Expand => state.expand(|_| {}),
                RegionAction::Collapse => state.collapse(|_| {}),
                RegionAction::Toggle => state.toggle(|_| {}),
            };

            match step.action {
                RegionAction::Expand => {
                    if expanded {
                        prop_assert_eq!(transition, RegionTransition::NoChange, "redundant expand should be ignored at step {}", index);
                    } else {
                        prop_assert_eq!(transition, RegionTransition::Expanded, "expand should succeed at step {}", index);
                        expanded = true;
                    }
                }
                RegionAction::Collapse => {
                    if expanded {
                        prop_assert_eq!(transition, RegionTransition::Collapsed, "collapse should succeed at step {}", index);
                        expanded = false;
                        prop_assert_eq!(state.focus_return_target(), expected_focus.as_deref(), "focus target lost on collapse at step {}", index);
                    } else {
                        prop_assert_eq!(transition, RegionTransition::NoChange, "redundant collapse should be ignored at step {}", index);
                    }
                }
                RegionAction::Toggle => {
                    if expanded {
                        prop_assert_eq!(transition, RegionTransition::Collapsed, "toggle collapse should succeed at step {}", index);
                        expanded = false;
                        prop_assert_eq!(state.focus_return_target(), expected_focus.as_deref(), "focus target lost on toggle collapse at step {}", index);
                    } else {
                        prop_assert_eq!(transition, RegionTransition::Expanded, "toggle expand should succeed at step {}", index);
                        expanded = true;
                    }
                }
            }

            // Regardless of how we mutated the region, the stored focus return
            // identifier must equal our shadow copy so adapters can immediately
            // focus the trigger without conditional logic.
            prop_assert_eq!(state.focus_return_target(), expected_focus.as_deref(), "focus target drifted at step {}", index);
        }
    }
}
