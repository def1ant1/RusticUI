#![deny(missing_docs)]
//! Headless portal coordination shared across material renderers.
//!
//! The portal state keeps runtime focus orchestration and hydration guards in a
//! single location so overlay surfaces mount predictably on the client while
//! remaining safe to render on the server.  `rustic_ui_system::portal::PortalMount`
//! generates deterministic DOM anchor identifiers; this module captures the
//! runtime bookkeeping required to coordinate focus loops, hydration windows,
//! and analytics beacons.

use crate::transition::TransitionState;

/// Portal visibility and hydration guard.
#[derive(Debug, Clone)]
pub struct PortalState {
    transition: TransitionState,
    hydrated: bool,
    trap_focus: bool,
}

impl PortalState {
    /// Creates a portal state machine.  Portals default to `hydrated = false`
    /// which keeps SSR markup inert until the first client render confirms the
    /// environment is interactive.
    pub fn new(automation_id: Option<String>, trap_focus: bool) -> Self {
        let mut transition = TransitionState::new(automation_id);
        // Start idle until hydration toggles visibility.
        transition.reset();
        Self {
            transition,
            hydrated: false,
            trap_focus,
        }
    }

    /// Marks the portal as hydrated which allows the surface to enter.
    pub fn hydrate(&mut self) {
        if !self.hydrated {
            self.hydrated = true;
            self.transition.begin_enter();
        }
    }

    /// Returns whether the portal has been hydrated.
    #[inline]
    pub const fn is_hydrated(&self) -> bool {
        self.hydrated
    }

    /// Returns the underlying transition snapshot.
    #[inline]
    pub fn transition(&self) -> crate::transition::TransitionSnapshot {
        self.transition.snapshot()
    }

    /// Returns whether the portal should trap focus when visible.
    #[inline]
    pub const fn trap_focus(&self) -> bool {
        self.trap_focus
    }

    /// Requests the portal to close.
    pub fn close(&mut self) {
        if self.transition.begin_exit() {
            self.transition.complete();
        }
    }

    /// Immediately hides the portal.  Useful for server side rendering resets.
    pub fn reset(&mut self) {
        self.transition.reset();
        self.hydrated = false;
    }

    /// Returns a tuple suitable for DOM `data-state` attributes.
    #[inline]
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
    use crate::transition::TransitionPhase;

    #[test]
    fn portal_hydration_flow() {
        let mut portal = PortalState::new(Some("portal".into()), true);
        assert!(!portal.is_hydrated());
        assert_eq!(portal.transition().phase(), TransitionPhase::Idle);
        portal.hydrate();
        assert!(portal.is_hydrated());
        assert_eq!(portal.transition().phase(), TransitionPhase::Entering);
        portal.close();
        assert_eq!(portal.transition().phase(), TransitionPhase::Completed);
        portal.reset();
        assert!(!portal.is_hydrated());
        assert_eq!(portal.transition().phase(), TransitionPhase::Idle);
    }
}
