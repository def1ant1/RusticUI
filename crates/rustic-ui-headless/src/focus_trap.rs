#![deny(missing_docs)]
//! Focus trap coordination for modal surfaces and disclosure widgets.
//!
//! The state machine keeps a deterministic focus ring and exposes helpers that
//! map [`interaction::ControlKey`] inputs to focus intents.  The list of
//! focusable nodes is stored as owned `String`s so adapters can rebuild the trap
//! concurrently on background threads before swapping it into an event loop.
//! Since all transitions are pure data mutations the type is `Send` + `Sync` and
//! therefore safe to move between async tasks.
//!
//! Focus looping is handled entirely within the state machine: when
//! [`FocusTrapState::loop_focus`] returns `true` the helpers wrap navigation so
//! keyboard users never escape the trap.  The attribute builders expose the
//! sentinel nodes required by DOM-based adapters and emit analytics hooks for
//! centralized instrumentation.  The architecture companion note at
//! [`docs/architecture/headless-state-machines.md`](../../docs/architecture/headless-state-machines.md#focus-trap-state-machine)
//! documents loop behaviour, sentinel analytics propagation, the
//! `data-rustic-focus-trap` controller contract, and the automation harnesses
//! that enforce parity across adapters.

use crate::interaction::ControlKey;

/// Records the outcome of processing a control key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDisposition<'a> {
    /// No focus change should occur.
    NoChange,
    /// Focus should move to the provided element identifier.
    Focus(&'a str),
}

/// Deterministic focus trap state.
#[derive(Debug, Clone, Default)]
pub struct FocusTrapState {
    focusables: Vec<String>,
    current_index: Option<usize>,
    loop_focus: bool,
    analytics_tag: Option<String>,
}

impl FocusTrapState {
    /// Build a new focus trap.  When `loop_focus` is `true`, navigation wraps
    /// around the list of focusables instead of clamping at the edges.
    pub fn new(loop_focus: bool) -> Self {
        Self {
            focusables: Vec::new(),
            current_index: None,
            loop_focus,
            analytics_tag: None,
        }
    }

    /// Returns whether the trap loops focus.
    #[inline]
    pub const fn loop_focus(&self) -> bool {
        self.loop_focus
    }

    /// Replace the list of focusables and reset the current index.
    pub fn set_focusables<I, S>(&mut self, focusables: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.focusables = focusables.into_iter().map(Into::into).collect();
        self.current_index = None;
    }

    /// Returns the number of focusable elements tracked by the trap.
    #[inline]
    pub fn len(&self) -> usize {
        self.focusables.len()
    }

    /// Returns whether the trap currently manages no focusable nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.focusables.is_empty()
    }

    /// Register which element currently holds focus.
    pub fn register_focus(&mut self, id: Option<&str>) {
        self.current_index = id.and_then(|needle| {
            self.focusables
                .iter()
                .position(|candidate| candidate == needle)
        });
    }

    /// Process a control key, returning the next focus target when applicable.
    pub fn handle_key(&mut self, key: ControlKey) -> FocusDisposition<'_> {
        match key {
            ControlKey::Home => self.focus_first(),
            ControlKey::End => self.focus_last(),
            ControlKey::ArrowDown | ControlKey::ArrowRight => self.focus_next(),
            ControlKey::ArrowUp | ControlKey::ArrowLeft => self.focus_previous(),
            _ => FocusDisposition::NoChange,
        }
    }

    /// Returns the analytics tag stored on the trap.
    #[inline]
    pub fn analytics_tag(&self) -> Option<&str> {
        self.analytics_tag.as_deref()
    }

    /// Update the analytics tag that will be mirrored onto the sentinel
    /// attributes.
    pub fn set_analytics_tag(&mut self, tag: Option<impl Into<String>>) {
        self.analytics_tag = tag.map(Into::into);
    }

    fn focus_first(&mut self) -> FocusDisposition<'_> {
        if let Some(first) = self.focusables.first() {
            self.current_index = Some(0);
            FocusDisposition::Focus(first)
        } else {
            FocusDisposition::NoChange
        }
    }

    fn focus_last(&mut self) -> FocusDisposition<'_> {
        if let Some(last) = self.focusables.last() {
            self.current_index = Some(self.focusables.len() - 1);
            FocusDisposition::Focus(last)
        } else {
            FocusDisposition::NoChange
        }
    }

    fn focus_next(&mut self) -> FocusDisposition<'_> {
        if self.focusables.is_empty() {
            return FocusDisposition::NoChange;
        }
        let next_index = match self.current_index {
            Some(index) if index + 1 < self.focusables.len() => Some(index + 1),
            Some(_) if self.loop_focus => Some(0),
            Some(_) => None,
            None => Some(0),
        };
        if let Some(index) = next_index {
            self.current_index = Some(index);
            FocusDisposition::Focus(&self.focusables[index])
        } else {
            FocusDisposition::NoChange
        }
    }

    fn focus_previous(&mut self) -> FocusDisposition<'_> {
        if self.focusables.is_empty() {
            return FocusDisposition::NoChange;
        }
        let next_index = match self.current_index {
            Some(0) if self.loop_focus => Some(self.focusables.len() - 1),
            Some(index) if index > 0 => Some(index - 1),
            Some(_) => None,
            None => Some(self.focusables.len().saturating_sub(1)),
        };
        if let Some(index) = next_index {
            self.current_index = Some(index);
            FocusDisposition::Focus(&self.focusables[index])
        } else {
            FocusDisposition::NoChange
        }
    }

    /// Returns attributes for the start sentinel.
    pub fn start_sentinel_attributes(&self) -> FocusTrapSentinelAttributes<'_> {
        FocusTrapSentinelAttributes {
            analytics_tag: self.analytics_tag.as_deref(),
            position: "start",
        }
    }

    /// Returns attributes for the end sentinel.
    pub fn end_sentinel_attributes(&self) -> FocusTrapSentinelAttributes<'_> {
        FocusTrapSentinelAttributes {
            analytics_tag: self.analytics_tag.as_deref(),
            position: "end",
        }
    }
}

/// Attribute builder shared by the focus trap sentinels.
#[derive(Debug, Clone)]
pub struct FocusTrapSentinelAttributes<'a> {
    analytics_tag: Option<&'a str>,
    position: &'static str,
}

impl<'a> FocusTrapSentinelAttributes<'a> {
    /// Returns the analytics tuple when configured.
    #[inline]
    pub fn analytics_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.analytics_tag
            .map(|value| ("data-rustic-analytics-id", value))
    }

    /// Returns the controller marker used by adapters to register sentinel nodes.
    #[inline]
    pub fn controller_attribute(&self) -> (&'static str, String) {
        (
            "data-rustic-focus-trap",
            format!("sentinel-{}", self.position),
        )
    }

    /// Collects the configured pairs for DOM integrations.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(2);
        let (key, value) = self.controller_attribute();
        pairs.push((key, value));
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
    fn wraps_focus_when_enabled() {
        let mut state = FocusTrapState::new(true);
        state.set_focusables(["a", "b", "c"]);
        state.register_focus(Some("c"));
        let disposition = state.handle_key(ControlKey::ArrowRight);
        assert!(matches!(disposition, FocusDisposition::Focus("a")));
    }

    #[test]
    fn clamps_focus_when_loop_disabled() {
        let mut state = FocusTrapState::new(false);
        state.set_focusables(["a", "b"]);
        state.register_focus(Some("b"));
        let disposition = state.handle_key(ControlKey::ArrowRight);
        assert!(matches!(disposition, FocusDisposition::NoChange));
    }

    #[test]
    fn analytics_tags_mirror_to_sentinels() {
        let mut state = FocusTrapState::new(true);
        state.set_analytics_tag(Some("dialog-focus"));
        let attrs = state.start_sentinel_attributes().as_pairs();
        assert_eq!(attrs.len(), 2);
    }
}
