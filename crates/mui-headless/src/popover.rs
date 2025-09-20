#![deny(missing_docs)]
//! State management for floating surfaces such as popovers and menus.
//!
//! The popover primitive keeps anchor metadata, placement preferences, and
//! collision responses in a single location so SSR generated markup matches the
//! hydrated client behaviour.  Framework adapters can therefore focus on
//! rendering the floating layer while relying on this module for deterministic
//! bookkeeping.

use crate::selection::ControlStrategy;

/// Describes the preferred placement of the floating surface relative to the anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverPlacement {
    /// Position the surface above the anchor.
    Top,
    /// Position the surface below the anchor.
    Bottom,
    /// Position the surface before the anchor (typically left in LTR layouts).
    Start,
    /// Position the surface after the anchor (typically right in LTR layouts).
    End,
    /// Center the surface over the anchor.
    Center,
}

impl PopoverPlacement {
    /// String representation used by analytics hooks and CSS attribute selectors.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
        }
    }
}

impl Default for PopoverPlacement {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Geometry describing the anchor element.  Coordinates are stored in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorGeometry {
    /// X coordinate of the anchor's top-left corner.
    pub x: f64,
    /// Y coordinate of the anchor's top-left corner.
    pub y: f64,
    /// Width of the anchor box.
    pub width: f64,
    /// Height of the anchor box.
    pub height: f64,
}

impl AnchorGeometry {
    /// Returns the center point of the anchor.
    #[inline]
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// Describes the last placement decision after running collision detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionOutcome {
    /// The preferred placement was retained.
    Preferred,
    /// The placement changed due to a collision.
    Repositioned,
}

/// State machine orchestrating popover visibility and positioning metadata.
#[derive(Debug, Clone)]
pub struct PopoverState {
    control_mode: ControlStrategy,
    open: bool,
    preferred: PopoverPlacement,
    resolved: PopoverPlacement,
    anchor_id: Option<String>,
    anchor_geometry: Option<AnchorGeometry>,
    last_outcome: CollisionOutcome,
}

impl PopoverState {
    /// Construct an uncontrolled popover with an optional default open state.
    pub fn uncontrolled(default_open: bool, preferred: PopoverPlacement) -> Self {
        Self {
            control_mode: ControlStrategy::Uncontrolled,
            open: default_open,
            preferred,
            resolved: preferred,
            anchor_id: None,
            anchor_geometry: None,
            last_outcome: CollisionOutcome::Preferred,
        }
    }

    /// Construct a controlled popover.  External controllers must call
    /// [`PopoverState::sync_open`] when reacting to emitted intents.
    pub fn controlled(preferred: PopoverPlacement) -> Self {
        Self {
            control_mode: ControlStrategy::Controlled,
            open: false,
            preferred,
            resolved: preferred,
            anchor_id: None,
            anchor_geometry: None,
            last_outcome: CollisionOutcome::Preferred,
        }
    }

    /// Returns the configured control strategy.
    #[inline]
    pub const fn control_strategy(&self) -> ControlStrategy {
        self.control_mode
    }

    /// Returns whether the popover is currently open.
    #[inline]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the preferred placement.
    #[inline]
    pub const fn preferred_placement(&self) -> PopoverPlacement {
        self.preferred
    }

    /// Returns the resolved placement after the last collision check.
    #[inline]
    pub const fn resolved_placement(&self) -> PopoverPlacement {
        self.resolved
    }

    /// Returns the last collision outcome.
    #[inline]
    pub const fn last_outcome(&self) -> CollisionOutcome {
        self.last_outcome
    }

    /// Update the anchor metadata.  Passing `None` clears any previously stored
    /// geometry which is useful when the anchor unmounts during responsive
    /// layout shifts.
    pub fn set_anchor_metadata(
        &mut self,
        id: Option<impl Into<String>>,
        geometry: Option<AnchorGeometry>,
    ) {
        self.anchor_id = id.map(Into::into);
        self.anchor_geometry = geometry;
    }

    /// Returns the current anchor identifier if any.
    #[inline]
    pub fn anchor_id(&self) -> Option<&str> {
        self.anchor_id.as_deref()
    }

    /// Returns the current anchor geometry if any.
    #[inline]
    pub const fn anchor_geometry(&self) -> Option<AnchorGeometry> {
        self.anchor_geometry
    }

    /// Request the popover to open.
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) {
        if self.open {
            return;
        }
        if !self.control_mode.is_controlled() {
            self.open = true;
        }
        notify(true);
    }

    /// Request the popover to close.
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) {
        if !self.open {
            return;
        }
        if !self.control_mode.is_controlled() {
            self.open = false;
        }
        notify(false);
    }

    /// Toggle the open state.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) {
        if self.open {
            self.close(notify);
        } else {
            self.open(notify);
        }
    }

    /// Synchronize the open flag when controlled externally.
    pub fn sync_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Run collision detection using the provided resolver.  The resolver
    /// receives the anchor geometry (if available) and the preferred placement
    /// and returns the final placement decision.  When no geometry is stored the
    /// resolver is skipped and the preferred placement is returned unchanged.
    pub fn resolve_with<F>(&mut self, resolver: F) -> PopoverPlacement
    where
        F: FnOnce(AnchorGeometry, PopoverPlacement) -> PopoverPlacement,
    {
        if let Some(geometry) = self.anchor_geometry {
            let placement = resolver(geometry, self.preferred);
            self.last_outcome = if placement == self.preferred {
                CollisionOutcome::Preferred
            } else {
                CollisionOutcome::Repositioned
            };
            self.resolved = placement;
        } else {
            self.last_outcome = CollisionOutcome::Preferred;
            self.resolved = self.preferred;
        }
        self.resolved
    }

    /// Returns an attribute helper for the anchor element.
    pub fn anchor_attributes(&self) -> PopoverAnchorAttributes<'_> {
        PopoverAnchorAttributes::new(self)
    }

    /// Returns an attribute helper for the surface element.
    pub fn surface_attributes(&self) -> PopoverSurfaceAttributes<'_> {
        PopoverSurfaceAttributes::new(self)
    }
}

/// Attribute helper for anchor nodes.
#[derive(Debug, Clone)]
pub struct PopoverAnchorAttributes<'a> {
    state: &'a PopoverState,
}

impl<'a> PopoverAnchorAttributes<'a> {
    fn new(state: &'a PopoverState) -> Self {
        Self { state }
    }

    /// Returns the `id` tuple for the anchor if available.
    #[inline]
    pub fn id(&self) -> Option<(&'static str, &str)> {
        self.state.anchor_id().map(|id| ("id", id))
    }

    /// Returns a data attribute exposing the resolved placement.  Anchors use
    /// this to align caret/arrow styling without recomputing placement logic.
    #[inline]
    pub fn data_placement(&self) -> (&'static str, &'static str) {
        (
            "data-popover-placement",
            self.state.resolved_placement().as_str(),
        )
    }
}

/// Attribute helper for the floating surface.
#[derive(Debug, Clone)]
pub struct PopoverSurfaceAttributes<'a> {
    state: &'a PopoverState,
    analytics_tag: Option<&'a str>,
}

impl<'a> PopoverSurfaceAttributes<'a> {
    fn new(state: &'a PopoverState) -> Self {
        Self {
            state,
            analytics_tag: None,
        }
    }

    /// Attach an analytics identifier for instrumentation teams.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns a `data-open` tuple representing the current visibility.
    #[inline]
    pub fn data_open(&self) -> (&'static str, &'static str) {
        (
            "data-open",
            if self.state.is_open() {
                "true"
            } else {
                "false"
            },
        )
    }

    /// Returns a `data-preferred-placement` tuple for SSR hydration alignment.
    #[inline]
    pub fn data_preferred(&self) -> (&'static str, &'static str) {
        (
            "data-preferred-placement",
            self.state.preferred_placement().as_str(),
        )
    }

    /// Returns a `data-resolved-placement` tuple mirroring the latest collision
    /// result.
    #[inline]
    pub fn data_resolved(&self) -> (&'static str, &'static str) {
        (
            "data-resolved-placement",
            self.state.resolved_placement().as_str(),
        )
    }

    /// Returns analytics metadata when configured.
    #[inline]
    pub fn data_analytics_id(&self) -> Option<(&'static str, &str)> {
        self.analytics_tag.map(|value| ("data-analytics-id", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_state_mutates_immediately() {
        let mut state = PopoverState::uncontrolled(false, PopoverPlacement::Top);
        state.open(|_| {});
        assert!(state.is_open());
        state.close(|_| {});
        assert!(!state.is_open());
    }

    #[test]
    fn controlled_state_requires_sync() {
        let mut state = PopoverState::controlled(PopoverPlacement::End);
        let mut last = None;
        state.open(|open| last = Some(open));
        assert_eq!(last, Some(true));
        assert!(!state.is_open());
        state.sync_open(true);
        assert!(state.is_open());
    }

    #[test]
    fn collision_resolver_updates_placement() {
        let mut state = PopoverState::uncontrolled(true, PopoverPlacement::Bottom);
        state.set_anchor_metadata(
            Some("anchor"),
            Some(AnchorGeometry {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            }),
        );
        let placement = state.resolve_with(|_, _| PopoverPlacement::Top);
        assert_eq!(placement, PopoverPlacement::Top);
        assert_eq!(state.resolved_placement(), PopoverPlacement::Top);
        assert_eq!(state.last_outcome(), CollisionOutcome::Repositioned);
    }

    #[test]
    fn attribute_helpers_emit_expected_metadata() {
        let mut state = PopoverState::uncontrolled(true, PopoverPlacement::Start);
        state.set_anchor_metadata(Some("trigger"), None);
        state.resolve_with(|_, preferred| preferred);
        let surface = state.surface_attributes().analytics_id("filters-popover");
        assert_eq!(surface.data_open(), ("data-open", "true"));
        assert_eq!(
            surface.data_preferred(),
            ("data-preferred-placement", "start")
        );
        assert_eq!(
            surface.data_resolved(),
            ("data-resolved-placement", "start")
        );
        assert_eq!(
            surface.data_analytics_id(),
            Some(("data-analytics-id", "filters-popover"))
        );
        let anchor = state.anchor_attributes();
        assert_eq!(anchor.id(), Some(("id", "trigger")));
        assert_eq!(anchor.data_placement(), ("data-popover-placement", "start"));
    }
}
