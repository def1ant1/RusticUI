#![deny(missing_docs)]
//! Floating element orchestration with collision awareness.
//!
//! Popper state builds upon [`TransitionState`](crate::transition::TransitionState)
//! and [`PortalState`](crate::portal::PortalState) to provide deterministic
//! placement metadata for Material adapters.  It mirrors the design of
//! [`popover`](crate::popover) but exposes additional levers commonly required by
//! tooltips, menus, and bespoke automation overlays.

use crate::{portal::PortalState, transition::TransitionState};

/// Pointer modality recorded when a popper opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerType {
    /// Mouse pointer.
    Mouse,
    /// Touch pointer.
    Touch,
    /// Keyboard interaction.
    Keyboard,
}

/// Describes how the floating element should react to collisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionStrategy {
    /// Prefer the requested placement even if it overflows.
    None,
    /// Flip the placement when an overflow is detected.
    Flip,
    /// Shift along the primary axis to keep the surface on-screen.
    Shift,
}

impl Default for CollisionStrategy {
    fn default() -> Self {
        Self::Flip
    }
}

/// Axis aligned placement options for popper surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopperPlacement {
    /// Above the anchor.
    Top,
    /// Below the anchor.
    Bottom,
    /// Before the anchor (left in LTR).
    Start,
    /// After the anchor (right in LTR).
    End,
}

impl Default for PopperPlacement {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Snapshot describing where the surface should render.
#[derive(Debug, Clone, PartialEq)]
pub struct PopperSnapshot {
    placement: PopperPlacement,
    pointer: Option<PointerType>,
    transition: crate::transition::TransitionSnapshot,
}

impl PopperSnapshot {
    /// Returns the resolved placement.
    #[inline]
    pub const fn placement(&self) -> PopperPlacement {
        self.placement
    }

    /// Returns the last pointer type used to open the popper if any.
    #[inline]
    pub const fn pointer_type(&self) -> Option<PointerType> {
        self.pointer
    }

    /// Returns the embedded transition snapshot.
    #[inline]
    pub const fn transition(&self) -> &crate::transition::TransitionSnapshot {
        &self.transition
    }
}

/// Popper controller storing placement resolution, transition state, and
/// hydration metadata.
#[derive(Debug, Clone)]
pub struct PopperState {
    preferred: PopperPlacement,
    resolved: PopperPlacement,
    collision: CollisionStrategy,
    pointer: Option<PointerType>,
    transition: TransitionState,
    portal: PortalState,
}

impl PopperState {
    /// Creates a new popper controller.
    pub fn new(
        preferred: PopperPlacement,
        collision: CollisionStrategy,
        automation_id: Option<String>,
    ) -> Self {
        let transition = TransitionState::new(automation_id.clone());
        let portal = PortalState::new(automation_id, false);
        Self {
            preferred,
            resolved: preferred,
            collision,
            pointer: None,
            transition,
            portal,
        }
    }

    /// Returns the preferred placement before collision handling.
    #[inline]
    pub const fn preferred(&self) -> PopperPlacement {
        self.preferred
    }

    /// Returns the resolved placement.
    #[inline]
    pub const fn resolved(&self) -> PopperPlacement {
        self.resolved
    }

    /// Returns the collision strategy.
    #[inline]
    pub const fn collision_strategy(&self) -> CollisionStrategy {
        self.collision
    }

    /// Returns the portal state.
    #[inline]
    pub const fn portal(&self) -> &PortalState {
        &self.portal
    }

    /// Mutable portal state for hydration aware adapters.
    #[inline]
    pub fn portal_mut(&mut self) -> &mut PortalState {
        &mut self.portal
    }

    /// Updates the resolved placement after a collision pass.
    pub fn set_resolved(&mut self, placement: PopperPlacement) {
        self.resolved = placement;
    }

    /// Records the pointer type that opened the popper.
    pub fn register_pointer(&mut self, pointer: PointerType) {
        self.pointer = Some(pointer);
        self.transition.begin_enter();
    }

    /// Returns a snapshot for rendering layers.
    pub fn snapshot(&self) -> PopperSnapshot {
        PopperSnapshot {
            placement: self.resolved,
            pointer: self.pointer,
            transition: self.transition.snapshot(),
        }
    }

    /// Requests the popper to close.
    pub fn close(&mut self) {
        if self.transition.begin_exit() {
            self.transition.complete();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transition::TransitionPhase;

    #[test]
    fn popper_tracks_pointer_and_resolved_state() {
        let mut popper = PopperState::new(PopperPlacement::Bottom, CollisionStrategy::Flip, None);
        popper.portal_mut().hydrate();
        popper.register_pointer(PointerType::Mouse);
        let snapshot = popper.snapshot();
        assert_eq!(snapshot.placement(), PopperPlacement::Bottom);
        assert_eq!(snapshot.pointer_type(), Some(PointerType::Mouse));
        assert_eq!(snapshot.transition().phase(), TransitionPhase::Entering);
        popper.set_resolved(PopperPlacement::Start);
        popper.close();
        assert_eq!(
            popper.snapshot().transition().phase(),
            TransitionPhase::Completed
        );
        assert_eq!(popper.resolved(), PopperPlacement::Start);
    }
}
