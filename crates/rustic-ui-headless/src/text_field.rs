#![deny(missing_docs)]
//! Form friendly state machine capturing value, validation, and visitation metadata.
//!
//! Text fields frequently underpin mission critical workflows.  This module keeps
//! the mutable pieces – current value, dirty/visited flags, validation errors and
//! debounce configuration – colocated with attribute helpers so adapters across
//! frameworks behave identically.  The API leans on controlled/uncontrolled
//! patterns to minimise manual bookkeeping in higher layers.

use crate::selection::ControlStrategy;
use std::time::Duration;

/// Snapshot emitted when the text field value changes.
#[derive(Debug, Clone, Copy)]
pub struct TextFieldChange<'a> {
    /// The latest value provided by the user.
    pub value: &'a str,
    /// Whether the new value differs from the initial value.
    pub dirty: bool,
    /// Debounce interval configured for change notifications.
    pub debounce: Option<Duration>,
}

/// Owned variant of [`TextFieldChange`] used by UI adapters that require `'static` lifetimes.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldChangeEvent {
    /// The latest value provided by the user.
    pub value: String,
    /// Whether the new value differs from the initial value.
    pub dirty: bool,
    /// Debounce interval configured for change notifications.
    pub debounce: Option<Duration>,
}

/// Snapshot emitted when the text field commits (blur/enter).
#[derive(Debug, Clone, Copy)]
pub struct TextFieldCommit<'a> {
    /// The value at the time of the commit event.
    pub value: &'a str,
    /// Whether the field currently contains validation errors.
    pub has_errors: bool,
    /// Tracks whether the field has been visited prior to this commit.
    pub previously_visited: bool,
}

/// Owned variant of [`TextFieldCommit`] for frameworks that require `'static` values.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldCommitEvent {
    /// The value at the time of the commit event.
    pub value: String,
    /// Whether the field currently contains validation errors.
    pub has_errors: bool,
    /// Tracks whether the field has been visited prior to this commit.
    pub previously_visited: bool,
}

/// Snapshot emitted when the text field resets back to its initial value.
#[derive(Debug, Clone, Copy)]
pub struct TextFieldReset<'a> {
    /// Value after the reset completed.
    pub value: &'a str,
    /// Flag describing whether validation errors were present before the reset.
    pub cleared_errors: bool,
}

/// Owned variant of [`TextFieldReset`] for stateful UI adapters.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldResetEvent {
    /// Value after the reset completed.
    pub value: String,
    /// Flag describing whether validation errors were present before the reset.
    pub cleared_errors: bool,
}

/// Aggregates text field state including validation and automation metadata.
#[derive(Debug, Clone)]
pub struct TextFieldState {
    control_mode: ControlStrategy,
    value: String,
    initial_value: String,
    pending_controlled: Option<String>,
    dirty: bool,
    visited: bool,
    errors: Vec<String>,
    debounce: Option<Duration>,
}

impl TextFieldState {
    /// Construct an uncontrolled text field with an initial value and optional debounce window.
    pub fn uncontrolled(initial: impl Into<String>, debounce: Option<Duration>) -> Self {
        let value = initial.into();
        Self {
            control_mode: ControlStrategy::Uncontrolled,
            initial_value: value.clone(),
            value,
            pending_controlled: None,
            dirty: false,
            visited: false,
            errors: Vec::new(),
            debounce,
        }
    }

    /// Construct a controlled text field.  The parent component must call
    /// [`TextFieldState::sync_value`] after receiving change notifications.
    pub fn controlled(initial: impl Into<String>, debounce: Option<Duration>) -> Self {
        let value = initial.into();
        Self {
            control_mode: ControlStrategy::Controlled,
            initial_value: value.clone(),
            value,
            pending_controlled: None,
            dirty: false,
            visited: false,
            errors: Vec::new(),
            debounce,
        }
    }

    /// Returns the current value taking pending controlled edits into account.
    #[inline]
    pub fn value(&self) -> &str {
        if let Some(ref pending) = self.pending_controlled {
            pending.as_str()
        } else {
            self.value.as_str()
        }
    }

    /// Returns the configured control strategy.
    #[inline]
    pub const fn control_strategy(&self) -> ControlStrategy {
        self.control_mode
    }

    /// Returns whether the field has unsaved changes.
    #[inline]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether the field has been visited.
    #[inline]
    pub const fn visited(&self) -> bool {
        self.visited
    }

    /// Returns the configured debounce interval.
    #[inline]
    pub const fn debounce(&self) -> Option<Duration> {
        self.debounce
    }

    /// Returns the currently captured validation errors.
    #[inline]
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Update the current value emitting a [`TextFieldChange`] snapshot.
    pub fn change<F>(&mut self, next: impl Into<String>, notify: F)
    where
        F: FnOnce(TextFieldChange<'_>),
    {
        let value = next.into();
        if self.control_mode.is_controlled() {
            self.pending_controlled = Some(value);
        } else {
            self.value = value;
        }
        self.recompute_dirty();
        let snapshot = TextFieldChange {
            value: self.value(),
            dirty: self.dirty,
            debounce: self.debounce,
        };
        notify(snapshot);
    }

    /// Mark the field as visited and emit a [`TextFieldCommit`] snapshot.
    pub fn commit<F>(&mut self, notify: F)
    where
        F: FnOnce(TextFieldCommit<'_>),
    {
        let previously_visited = self.visited;
        self.visited = true;
        let snapshot = TextFieldCommit {
            value: self.value(),
            has_errors: !self.errors.is_empty(),
            previously_visited,
        };
        notify(snapshot);
    }

    /// Reset the field back to its initial value clearing validation errors.
    pub fn reset<F>(&mut self, notify: F)
    where
        F: FnOnce(TextFieldReset<'_>),
    {
        let cleared_errors = !self.errors.is_empty();
        self.errors.clear();
        if self.control_mode.is_controlled() {
            self.pending_controlled = Some(self.initial_value.clone());
        } else {
            self.value = self.initial_value.clone();
        }
        self.dirty = false;
        self.visited = false;
        let snapshot = TextFieldReset {
            value: self.value(),
            cleared_errors,
        };
        notify(snapshot);
    }

    /// Synchronize the value for controlled fields.  Uncontrolled fields may
    /// also call this method during hydration to align SSR renders.
    pub fn sync_value(&mut self, value: impl Into<String>) {
        let value = value.into();
        self.value = value.clone();
        self.pending_controlled = None;
        self.initial_value = value;
        self.dirty = false;
    }

    /// Replace the validation errors with a new collection.
    pub fn set_errors(&mut self, errors: impl Into<Vec<String>>) {
        self.errors = errors.into();
    }

    /// Clear any captured validation errors.
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Update the initial baseline used to calculate the dirty flag.
    pub fn set_initial_value(&mut self, value: impl Into<String>) {
        self.initial_value = value.into();
        self.recompute_dirty();
    }

    fn recompute_dirty(&mut self) {
        self.dirty = self.value() != self.initial_value;
    }

    /// Returns an attribute helper that emits ARIA/data metadata.
    pub fn attributes(&self) -> TextFieldAttributes<'_> {
        TextFieldAttributes::new(self)
    }
}

/// Helper struct exposing ARIA/data metadata for text field inputs.
#[derive(Debug, Clone)]
pub struct TextFieldAttributes<'a> {
    state: &'a TextFieldState,
    status_id: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> TextFieldAttributes<'a> {
    fn new(state: &'a TextFieldState) -> Self {
        Self {
            state,
            status_id: None,
            analytics_tag: None,
        }
    }

    /// Provide the identifier of an element that surfaces validation messages.
    pub fn status_id(mut self, id: &'a str) -> Self {
        self.status_id = Some(id);
        self
    }

    /// Attach an analytics identifier used by SSR adapters to mirror hydrated output.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns an `aria-invalid` tuple when validation errors are present.
    #[inline]
    pub fn aria_invalid(&self) -> Option<(&'static str, &'static str)> {
        (!self.state.errors.is_empty()).then_some(("aria-invalid", "true"))
    }

    /// Returns an `aria-describedby` tuple linking to a validation status node.
    #[inline]
    pub fn aria_describedby(&self) -> Option<(&'static str, &str)> {
        self.status_id.map(|id| ("aria-describedby", id))
    }

    /// Returns a `data-dirty` tuple for styling/testing hooks.
    #[inline]
    pub fn data_dirty(&self) -> (&'static str, &'static str) {
        (
            "data-dirty",
            if self.state.dirty { "true" } else { "false" },
        )
    }

    /// Returns a `data-visited` tuple describing whether the field has been touched.
    #[inline]
    pub fn data_visited(&self) -> (&'static str, &'static str) {
        (
            "data-visited",
            if self.state.visited { "true" } else { "false" },
        )
    }

    /// Returns an analytics identifier tuple when configured.
    #[inline]
    pub fn data_analytics_id(&self) -> Option<(&'static str, &str)> {
        self.analytics_tag.map(|value| ("data-analytics-id", value))
    }

    /// Returns a condensed status message by joining validation errors.
    pub fn status_message(&self) -> Option<String> {
        if self.state.errors.is_empty() {
            None
        } else {
            Some(self.state.errors.join("\n"))
        }
    }
}

impl<'a> From<TextFieldChange<'a>> for TextFieldChangeEvent {
    fn from(snapshot: TextFieldChange<'a>) -> Self {
        Self {
            value: snapshot.value.to_string(),
            dirty: snapshot.dirty,
            debounce: snapshot.debounce,
        }
    }
}

impl<'a> From<TextFieldCommit<'a>> for TextFieldCommitEvent {
    fn from(snapshot: TextFieldCommit<'a>) -> Self {
        Self {
            value: snapshot.value.to_string(),
            has_errors: snapshot.has_errors,
            previously_visited: snapshot.previously_visited,
        }
    }
}

impl<'a> From<TextFieldReset<'a>> for TextFieldResetEvent {
    fn from(snapshot: TextFieldReset<'a>) -> Self {
        Self {
            value: snapshot.value.to_string(),
            cleared_errors: snapshot.cleared_errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_change_updates_value_and_dirty_flag() {
        let mut state = TextFieldState::uncontrolled("hello", None);
        let mut snapshot_value = String::new();
        state.change("world", |snapshot| {
            snapshot_value = snapshot.value.to_string();
            assert!(snapshot.dirty);
        });
        assert_eq!(snapshot_value, "world");
        assert!(state.dirty());
        assert_eq!(state.value(), "world");
    }

    #[test]
    fn controlled_field_requires_sync() {
        let mut state = TextFieldState::controlled("hello", None);
        let mut change_called = false;
        state.change("world", |snapshot| {
            change_called = snapshot.value == "world";
            assert!(snapshot.dirty);
        });
        assert!(change_called);
        assert_eq!(state.value(), "world");
        // Value should not commit until sync occurs.
        state.sync_value("world");
        assert_eq!(state.value(), "world");
        assert!(!state.dirty());
    }

    #[test]
    fn commit_marks_visited_and_reports_previous_state() {
        let mut state = TextFieldState::uncontrolled("a", None);
        let mut visited_before = None;
        state.commit(|snapshot| visited_before = Some(snapshot.previously_visited));
        assert_eq!(visited_before, Some(false));
        assert!(state.visited());
    }

    #[test]
    fn reset_restores_initial_state_and_clears_errors() {
        let mut state = TextFieldState::uncontrolled("baseline", None);
        state.set_errors(vec!["Required".to_string()]);
        state.change("updated", |_| {});
        state.commit(|_| {});
        let mut cleared = None;
        state.reset(|snapshot| cleared = Some(snapshot.cleared_errors));
        assert_eq!(cleared, Some(true));
        assert_eq!(state.value(), "baseline");
        assert!(!state.dirty());
        assert!(!state.visited());
        assert!(state.errors().is_empty());
    }

    #[test]
    fn attribute_builder_emits_expected_metadata() {
        let mut state = TextFieldState::uncontrolled("", None);
        state.set_errors(vec!["Required".to_string(), "Must be unique".to_string()]);
        state.commit(|_| {});
        let attrs = state
            .attributes()
            .status_id("field-status")
            .analytics_id("analytics-field-123");
        assert_eq!(attrs.aria_invalid(), Some(("aria-invalid", "true")));
        assert_eq!(
            attrs.aria_describedby(),
            Some(("aria-describedby", "field-status"))
        );
        assert_eq!(attrs.data_dirty(), ("data-dirty", "false"));
        assert_eq!(attrs.data_visited(), ("data-visited", "true"));
        assert_eq!(
            attrs.data_analytics_id(),
            Some(("data-analytics-id", "analytics-field-123"))
        );
        let message = attrs.status_message().expect("status message");
        assert!(message.contains("Required"));
        assert!(message.contains("Must be unique"));
    }

    #[test]
    fn owned_change_event_clones_value_and_debounce() {
        let mut state = TextFieldState::uncontrolled("base", Some(Duration::from_millis(150)));
        let mut last_event = None;
        state.change("updated", |snapshot| {
            last_event = Some(TextFieldChangeEvent::from(snapshot));
        });
        let event = last_event.expect("change event");
        assert_eq!(event.value, "updated");
        assert!(event.dirty);
        assert_eq!(event.debounce, Some(Duration::from_millis(150)));
    }

    #[test]
    fn owned_commit_event_preserves_flags() {
        let mut state = TextFieldState::uncontrolled("value", None);
        state.set_errors(vec!["Required".into()]);
        let mut event = None;
        state.commit(|snapshot| {
            event = Some(TextFieldCommitEvent::from(snapshot));
        });
        let event = event.expect("commit event");
        assert_eq!(event.value, "value");
        assert!(event.has_errors);
        assert!(!event.previously_visited);
    }

    #[test]
    fn owned_reset_event_captures_cleared_errors() {
        let mut state = TextFieldState::uncontrolled("value", None);
        state.set_errors(vec!["Required".into()]);
        let mut event = None;
        state.reset(|snapshot| {
            event = Some(TextFieldResetEvent::from(snapshot));
        });
        let event = event.expect("reset event");
        assert_eq!(event.value, "value");
        assert!(event.cleared_errors);
    }
}
