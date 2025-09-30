#![deny(missing_docs)]
//! Declarative transition state shared across overlay primitives.
//!
//! Enterprise teams frequently orchestrate multi-step animations, analytics
//! beacons, and focus management around overlay lifecycles.  Baking these
//! behaviours into every widget quickly becomes error prone and difficult to
//! coordinate across frameworks.  The [`TransitionState`] type below captures a
//! simple deterministic state machine that documents each phase of an overlay
//! transition.  Headless consumers emit the intents (enter/exit/cancel) while
//! material renderers decorate the lifecycle with CSS animations, telemetry, and
//! automation hooks.
//!
//! The machine intentionally separates *phase* from *visibility* to make focus
//! loops easier to audit:
//!
//! * `phase` – which semantic stage of the lifecycle we are in (`Entering`,
//!   `Visible`, `Exiting`, `Completed`).  Analytics pipelines typically record
//!   these values.
//! * `is_visible` – boolean flag describing whether the element should be in the
//!   accessibility tree.  During an exit animation we keep the element visible
//!   so focus does not jump unexpectedly, but we expose the intermediate phase so
//!   renderers can attach `aria-hidden` or `data-transition` hints.
//!
//! The transition machine is intentionally reusable: [`ModalState`],
//! [`PortalState`], and [`PopperState`](crate::popper::PopperState) all embed an
//! instance to ensure every overlay speaks the same language.  Downstream tests
//! assert on [`TransitionSnapshot::phase`] rather than brittle animation class
//! names.

/// Enumeration of semantic transition phases for overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionPhase {
    /// The overlay has not yet entered and should be hidden from assistive
    /// technology.
    Idle,
    /// Enter animation is running.  The element is visible but not yet fully
    /// interactive.
    Entering,
    /// The overlay finished entering and is fully interactive.
    Visible,
    /// Exit animation is running.  Consumers should keep the overlay in the DOM
    /// until [`TransitionPhase::Completed`] is observed.
    Exiting,
    /// Exit animation finished.  Consumers may safely remove the overlay.
    Completed,
}

impl TransitionPhase {
    /// Returns whether the phase indicates a visible surface.
    #[inline]
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Entering | Self::Visible | Self::Exiting)
    }
}

/// Lightweight snapshot returned to UI adapters so they can annotate DOM nodes
/// without mutating the state machine directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionSnapshot {
    phase: TransitionPhase,
    automation_id: Option<String>,
}

impl TransitionSnapshot {
    /// Creates a new snapshot from the internal transition state.
    #[inline]
    pub(crate) fn new(phase: TransitionPhase, automation_id: Option<String>) -> Self {
        Self {
            phase,
            automation_id,
        }
    }

    /// Returns the semantic transition phase.
    #[inline]
    pub const fn phase(&self) -> TransitionPhase {
        self.phase
    }

    /// Returns the automation identifier if present.
    #[inline]
    pub fn automation_id(&self) -> Option<&str> {
        self.automation_id.as_deref()
    }

    /// Returns the recommended `data-transition` attribute tuple.
    #[inline]
    pub fn data_transition(&self) -> (&'static str, String) {
        (
            "data-transition",
            match self.phase {
                TransitionPhase::Idle => "idle",
                TransitionPhase::Entering => "entering",
                TransitionPhase::Visible => "visible",
                TransitionPhase::Exiting => "exiting",
                TransitionPhase::Completed => "completed",
            }
            .into(),
        )
    }
}

/// Deterministic transition controller powering all overlay style hooks.
#[derive(Debug, Clone)]
pub struct TransitionState {
    phase: TransitionPhase,
    automation_id: Option<String>,
}

impl TransitionState {
    /// Creates a new transition state configured with an optional automation
    /// identifier.  The automation id is bubbled up to snapshots enabling Playwright
    /// tests and analytics collectors to assert on the transition lifecycle.
    #[must_use]
    pub fn new(automation_id: Option<String>) -> Self {
        Self {
            phase: TransitionPhase::Idle,
            automation_id,
        }
    }

    /// Returns a read only snapshot for UI layers.
    #[inline]
    pub fn snapshot(&self) -> TransitionSnapshot {
        TransitionSnapshot::new(self.phase, self.automation_id.clone())
    }

    /// Starts the enter animation.  Returns whether the call changed the phase.
    pub fn begin_enter(&mut self) -> bool {
        if matches!(
            self.phase,
            TransitionPhase::Idle | TransitionPhase::Completed
        ) {
            self.phase = TransitionPhase::Entering;
            true
        } else {
            false
        }
    }

    /// Marks the overlay as visible.  Typically invoked once the enter
    /// animation completes.
    pub fn mark_visible(&mut self) -> bool {
        if matches!(
            self.phase,
            TransitionPhase::Entering | TransitionPhase::Idle
        ) {
            self.phase = TransitionPhase::Visible;
            true
        } else {
            false
        }
    }

    /// Starts the exit animation.
    pub fn begin_exit(&mut self) -> bool {
        if matches!(
            self.phase,
            TransitionPhase::Visible | TransitionPhase::Entering
        ) {
            self.phase = TransitionPhase::Exiting;
            true
        } else {
            false
        }
    }

    /// Completes the exit animation and resets back to [`TransitionPhase::Completed`].
    pub fn complete(&mut self) -> bool {
        if matches!(self.phase, TransitionPhase::Exiting) {
            self.phase = TransitionPhase::Completed;
            true
        } else {
            false
        }
    }

    /// Resets the controller to [`TransitionPhase::Idle`].
    pub fn reset(&mut self) {
        self.phase = TransitionPhase::Idle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_happy_path() {
        let mut state = TransitionState::new(Some("modal-transition".into()));
        assert_eq!(state.snapshot().phase(), TransitionPhase::Idle);
        assert!(state.begin_enter());
        assert_eq!(state.snapshot().phase(), TransitionPhase::Entering);
        assert!(state.mark_visible());
        assert_eq!(state.snapshot().phase(), TransitionPhase::Visible);
        assert!(state.begin_exit());
        assert_eq!(state.snapshot().phase(), TransitionPhase::Exiting);
        assert!(state.complete());
        assert_eq!(state.snapshot().phase(), TransitionPhase::Completed);
        state.reset();
        assert_eq!(state.snapshot().phase(), TransitionPhase::Idle);
    }

    #[test]
    fn transition_prevents_duplicate_state_changes() {
        let mut state = TransitionState::new(None);
        assert!(state.begin_enter());
        assert!(!state.begin_enter());
        assert!(state.mark_visible());
        assert!(!state.mark_visible());
        assert!(state.begin_exit());
        assert!(!state.begin_exit());
        assert!(state.complete());
        assert!(!state.complete());
    }
}
