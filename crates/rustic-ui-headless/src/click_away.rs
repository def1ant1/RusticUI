#![deny(missing_docs)]
//! Deterministic click-away detection shared by overlays and disclosure widgets.
//!
//! The state machine is intentionally side-effect free and therefore `Send` +
//! `Sync` friendly.  Rendering adapters clone the state when crossing thread
//! boundaries (for example when issuing async close animations) without
//! sacrificing determinism.  The machine coordinates pointer/focus intents so
//! that a single "click away" notification is emitted regardless of how many
//! concurrent pointers are active.  Each pointer sequence is tracked in a
//! [`BTreeSet`] to maintain a stable iteration order which makes analytics hooks
//! and snapshot tests reproducible.
//!
//! The builder returned by [`ClickAwayState::root_attributes`] emits the
//! analytics and accessibility affordances expected by QA automation.  The
//! `data-rustic-click-away` marker is shared across the Material and Joy
//! adapters so downstream frameworks can opt into a centralized event listener
//! rather than wiring bespoke per-component handlers.

use std::collections::BTreeSet;

/// Result emitted after processing a pointer or focus event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAwayDisposition {
    /// No click-away action should be taken.
    NoChange,
    /// The consumer should close the controlled region.
    TriggerClose,
}

impl ClickAwayDisposition {
    /// Returns `true` when a close should be triggered.  Convenience helper
    /// used throughout the tests to keep assertions expressive.
    #[inline]
    pub const fn should_close(self) -> bool {
        matches!(self, Self::TriggerClose)
    }
}

/// Tracks pointer/focus interactions to detect when the user leaves a boundary.
#[derive(Debug, Clone, Default)]
pub struct ClickAwayState {
    root_id: Option<String>,
    armed: bool,
    focus_within: bool,
    active_sequences: BTreeSet<u64>,
}

impl ClickAwayState {
    /// Construct a new state machine.  Widgets typically call [`engage`] right
    /// before showing an overlay and [`disengage`] once the close animation
    /// completes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the identifier that will be exposed on the root node.  Passing
    /// `None` clears the identifier.
    pub fn set_root_id(&mut self, id: Option<impl Into<String>>) {
        self.root_id = id.map(Into::into);
    }

    /// Arms the detection logic.  When armed, pointer-up or focus-out events
    /// occurring outside the boundary emit [`ClickAwayDisposition::TriggerClose`].
    pub fn engage(&mut self) {
        self.armed = true;
        self.focus_within = true;
    }

    /// Disarms the detection logic and clears any tracked pointer sequences.
    pub fn disengage(&mut self) {
        self.armed = false;
        self.focus_within = false;
        self.active_sequences.clear();
    }

    /// Returns whether the detector is currently armed.
    #[inline]
    pub const fn is_engaged(&self) -> bool {
        self.armed
    }

    /// Record a pointer down event.  When `inside_root` is `true` the pointer
    /// sequence is tracked to avoid firing a spurious click-away notification
    /// when the pointer is released outside the element (for example after a drag).
    pub fn process_pointer_down(&mut self, sequence: u64, inside_root: bool) {
        if !self.armed {
            return;
        }
        if inside_root {
            self.active_sequences.insert(sequence);
            self.focus_within = true;
        }
    }

    /// Record a pointer up event.  When the pointer started outside and remains
    /// outside we return [`ClickAwayDisposition::TriggerClose`].
    pub fn process_pointer_up(&mut self, sequence: u64, inside_root: bool) -> ClickAwayDisposition {
        if !self.armed {
            return ClickAwayDisposition::NoChange;
        }
        let originated_inside = self.active_sequences.remove(&sequence);
        if inside_root {
            self.focus_within = true;
            return ClickAwayDisposition::NoChange;
        }
        if originated_inside {
            // Dragged pointer left the boundary; treat as an internal interaction.
            return ClickAwayDisposition::NoChange;
        }
        self.armed = false;
        ClickAwayDisposition::TriggerClose
    }

    /// Update whether focus currently resides within the boundary.  When focus
    /// leaves without an active pointer we treat it as a click-away.
    pub fn update_focus_within(&mut self, focus_within: bool) -> ClickAwayDisposition {
        if !self.armed {
            self.focus_within = focus_within;
            return ClickAwayDisposition::NoChange;
        }
        self.focus_within = focus_within;
        if !focus_within && self.active_sequences.is_empty() {
            self.armed = false;
            return ClickAwayDisposition::TriggerClose;
        }
        ClickAwayDisposition::NoChange
    }

    /// Returns an attribute builder exposing automation and accessibility hooks.
    pub fn root_attributes(&self) -> ClickAwayRootAttributes<'_> {
        ClickAwayRootAttributes {
            id: self.root_id.as_deref(),
            analytics_tag: None,
        }
    }
}

/// Attribute builder for the click-away boundary.
#[derive(Debug, Clone)]
pub struct ClickAwayRootAttributes<'a> {
    id: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> ClickAwayRootAttributes<'a> {
    /// Assign an identifier to the boundary element.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Attach an analytics identifier to the boundary element.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns the `id` attribute when configured.
    #[inline]
    pub fn id_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the analytics marker used by QA hooks.
    #[inline]
    pub fn analytics_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.analytics_tag
            .map(|value| ("data-rustic-analytics-id", value))
    }

    /// Returns the stable controller marker shared across adapters.
    #[inline]
    pub fn controller_attribute(&self) -> (&'static str, &'static str) {
        ("data-rustic-click-away", "root")
    }

    /// Collects the configured attributes into a vector suitable for spreading
    /// onto an element in virtual DOM frameworks.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(3);
        pairs.push(("data-rustic-click-away", "root".to_string()));
        if let Some((key, value)) = self.id_attribute() {
            pairs.push((key, value.to_string()));
        }
        if let Some((key, value)) = self.analytics_attribute() {
            pairs.push((key, value.to_string()));
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_outside_triggers_close() {
        let mut state = ClickAwayState::new();
        state.engage();
        assert!(state.process_pointer_up(42, false).should_close());
    }

    #[test]
    fn drag_inside_does_not_trigger() {
        let mut state = ClickAwayState::new();
        state.engage();
        state.process_pointer_down(1, true);
        assert!(!state.process_pointer_up(1, false).should_close());
    }

    #[test]
    fn focus_exit_without_pointer_triggers() {
        let mut state = ClickAwayState::new();
        state.engage();
        assert!(state.update_focus_within(false).should_close());
    }

    #[test]
    fn attribute_builder_emits_expected_pairs() {
        let mut state = ClickAwayState::new();
        state.set_root_id(Some("dialog-1"));
        let attrs = state
            .root_attributes()
            .analytics_id("analytics-click-away")
            .as_pairs();
        assert_eq!(attrs.len(), 3);
    }
}
