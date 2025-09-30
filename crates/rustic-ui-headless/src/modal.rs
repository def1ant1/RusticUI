#![deny(missing_docs)]
//! Modal orchestration built on top of the reusable transition state.
//!
//! The modal state machine centralises focus loop bookkeeping, animation
//! lifecycles, and analytics identifiers.  The controller purposely mirrors the
//! architecture used by `dialog` and `drawer` to simplify migrations: consumers
//! can swap to [`ModalState`] and reuse the same notification hooks.

use crate::{
    selection::ControlStrategy,
    transition::{TransitionPhase, TransitionState},
};

/// Strategy used by modal overlays to determine how focus loops behave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTrapStrategy {
    /// Automatically derive focusable elements using DOM queries.
    Auto,
    /// Developer supplied focusable identifiers.
    Manual,
    /// Focus trap disabled – useful for non-modal disclosure patterns.
    Disabled,
}

/// Events emitted by [`ModalState`] when visibility changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalEvent {
    /// Modal opened.
    Opened,
    /// Modal closed.
    Closed,
}

/// State machine coordinating modal visibility, transition lifecycles and focus
/// behaviour.
#[derive(Debug, Clone)]
pub struct ModalState {
    control: ControlStrategy,
    focus: FocusTrapStrategy,
    transition: TransitionState,
    open: bool,
}

impl ModalState {
    /// Creates a new uncontrolled modal.
    pub fn uncontrolled(default_open: bool, focus: FocusTrapStrategy, automation_id: Option<String>) -> Self {
        let mut state = Self {
            control: ControlStrategy::Uncontrolled,
            focus,
            transition: TransitionState::new(automation_id),
            open: default_open,
        };
        if default_open {
            state.transition.begin_enter();
        }
        state
    }

    /// Creates a new controlled modal.  External controllers must call
    /// [`ModalState::sync_open`] when reacting to emitted events.
    pub fn controlled(focus: FocusTrapStrategy, automation_id: Option<String>) -> Self {
        Self {
            control: ControlStrategy::Controlled,
            focus,
            transition: TransitionState::new(automation_id),
            open: false,
        }
    }

    /// Returns whether the modal is currently open.
    #[inline]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the control strategy.
    #[inline]
    pub const fn control_strategy(&self) -> ControlStrategy {
        self.control
    }

    /// Returns the focus trap strategy for the modal surface.
    #[inline]
    pub const fn focus_strategy(&self) -> FocusTrapStrategy {
        self.focus
    }

    /// Returns the transition snapshot for DOM annotations.
    #[inline]
    pub fn transition(&self) -> crate::transition::TransitionSnapshot {
        self.transition.snapshot()
    }

    /// Reconciles external state for controlled modals.
    pub fn sync_open(&mut self, open: bool) {
        if !self.control.is_controlled() {
            return;
        }
        self.open = open;
        if open {
            self.transition.begin_enter();
        } else if self.transition.begin_exit() {
            self.transition.complete();
        }
    }

    /// Request to open the modal.
    pub fn open<F: FnOnce(ModalEvent)>(&mut self, notify: F) {
        if self.open {
            return;
        }
        if !self.control.is_controlled() {
            self.open = true;
        }
        let changed = self.transition.begin_enter();
        if changed {
            notify(ModalEvent::Opened);
        }
    }

    /// Request to close the modal.
    pub fn close<F: FnOnce(ModalEvent)>(&mut self, notify: F) {
        if !self.open {
            return;
        }
        if !self.control.is_controlled() {
            self.open = false;
        }
        let mut changed = false;
        if self.transition.begin_exit() {
            changed = true;
            self.transition.complete();
        }
        if changed {
            notify(ModalEvent::Closed);
        }
    }

    /// Returns a tuple describing the modal's `data-state` attribute.
    pub fn data_state(&self) -> (&'static str, &'static str) {
        (
            "data-state",
            if self.transition.snapshot().phase().is_visible() {
                "open"
            } else {
                "closed"
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_uncontrolled_flow() {
        let mut events = Vec::new();
        let mut modal = ModalState::uncontrolled(false, FocusTrapStrategy::Auto, None);
        modal.open(|event| events.push(event));
        assert!(modal.is_open());
        assert_eq!(modal.transition().phase(), TransitionPhase::Entering);
        modal.close(|event| events.push(event));
        assert!(!modal.is_open());
        assert_eq!(modal.transition().phase(), TransitionPhase::Completed);
        assert_eq!(events, vec![ModalEvent::Opened, ModalEvent::Closed]);
    }

    #[test]
    fn modal_controlled_flow() {
        let mut events = Vec::new();
        let mut modal = ModalState::controlled(FocusTrapStrategy::Auto, Some("modal".into()));
        modal.open(|event| events.push(event));
        assert!(!modal.is_open());
        assert_eq!(events, vec![ModalEvent::Opened]);
        modal.sync_open(true);
        assert!(modal.is_open());
        assert_eq!(modal.transition().phase(), TransitionPhase::Entering);
        modal.close(|event| events.push(event));
        modal.sync_open(false);
        assert!(!modal.is_open());
        assert_eq!(modal.transition().phase(), TransitionPhase::Completed);
        assert_eq!(events, vec![ModalEvent::Opened, ModalEvent::Closed]);
    }
}
