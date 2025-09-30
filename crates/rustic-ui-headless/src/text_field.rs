#![deny(missing_docs)]
//! Form friendly state machine capturing value, validation, and visitation metadata.
//!
//! Text fields frequently underpin mission critical workflows.  This module keeps
//! the mutable pieces – current value, dirty/visited flags, validation errors and
//! debounce configuration – colocated with attribute helpers so adapters across
//! frameworks behave identically.  The API leans on controlled/uncontrolled
//! patterns to minimise manual bookkeeping in higher layers.  Internally the
//! implementation now composes the shared [`crate::input_base::InputState`]
//! primitives so analytics, focus tracking, and validation updates behave the
//! same way across every future input.  This keeps the headless surface area
//! small while making it trivial for other controls to bolt on text field style
//! behaviour without copying large chunks of bookkeeping logic.  The
//! [`TextFieldControlBuilder`] further wraps [`crate::input_base::InputControlBuilder`]
//! so adapters can grab the [`FormControlState`](crate::form_control::FormControlState)
//! and [`TextFieldState`] in one shot.

use crate::form_control::FormControlState;
use crate::input_base::{
    InputAnalyticsEvent, InputChange, InputChangeEvent, InputCommit, InputCommitEvent,
    InputControlBuilder, InputControlBundle, InputReset, InputResetEvent, InputSelection,
    InputState,
};
use crate::selection::ControlStrategy;
use std::time::Duration;

/// Snapshot emitted when the text field value changes.
#[derive(Debug, Clone)]
pub struct TextFieldChange<'a> {
    /// Shared [`InputChange`] snapshot capturing the base value/dirty metadata.
    pub base: InputChange<'a>,
    /// Debounce interval configured for change notifications.
    pub debounce: Option<Duration>,
}

impl<'a> TextFieldChange<'a> {
    /// Returns the latest value taking pending controlled edits into account.
    pub fn value(&self) -> &str {
        self.base.value
    }

    /// Indicates whether the value currently diverges from the initial value.
    pub fn dirty(&self) -> bool {
        self.base.dirty
    }

    /// Returns the current selection if adapters provided one during the change.
    pub fn selection(&self) -> Option<InputSelection> {
        self.base.selection
    }

    /// Returns analytics events generated alongside the value mutation.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }

    /// Returns the configured debounce interval.
    pub fn debounce(&self) -> Option<Duration> {
        self.debounce
    }
}

/// Owned variant of [`TextFieldChange`] used by UI adapters that require `'static` lifetimes.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldChangeEvent {
    /// Shared [`InputChangeEvent`] snapshot capturing the base change metadata.
    pub base: InputChangeEvent,
    /// Debounce interval configured for change notifications.
    pub debounce: Option<Duration>,
}

impl TextFieldChangeEvent {
    /// Returns the latest value provided by the user.
    pub fn value(&self) -> &str {
        &self.base.value
    }

    /// Indicates whether the value currently diverges from the initial value.
    pub fn dirty(&self) -> bool {
        self.base.dirty
    }

    /// Returns the captured selection, if any.
    pub fn selection(&self) -> Option<InputSelection> {
        self.base.selection
    }

    /// Returns analytics events emitted alongside the change.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }
}

/// Snapshot emitted when the text field commits (blur/enter).
#[derive(Debug, Clone)]
pub struct TextFieldCommit<'a> {
    /// Shared [`InputCommit`] snapshot describing the commit metadata.
    pub base: InputCommit<'a>,
}

impl<'a> TextFieldCommit<'a> {
    /// Returns the value at commit time.
    pub fn value(&self) -> &str {
        self.base.value
    }

    /// Indicates whether validation errors were present during the commit.
    pub fn has_errors(&self) -> bool {
        self.base.has_errors
    }

    /// Returns whether the field was previously visited prior to this commit.
    pub fn previously_visited(&self) -> bool {
        self.base.previously_visited
    }

    /// Returns analytics events associated with the commit.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }
}

/// Owned variant of [`TextFieldCommit`] for frameworks that require `'static` values.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldCommitEvent {
    /// Shared [`InputCommitEvent`] snapshot describing the commit metadata.
    pub base: InputCommitEvent,
}

impl TextFieldCommitEvent {
    /// Returns the value at commit time.
    pub fn value(&self) -> &str {
        &self.base.value
    }

    /// Indicates whether validation errors were present during the commit.
    pub fn has_errors(&self) -> bool {
        self.base.has_errors
    }

    /// Returns whether the field was previously visited prior to this commit.
    pub fn previously_visited(&self) -> bool {
        self.base.previously_visited
    }

    /// Returns analytics events associated with the commit.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }
}

/// Snapshot emitted when the text field resets back to its initial value.
#[derive(Debug, Clone)]
pub struct TextFieldReset<'a> {
    /// Shared [`InputReset`] snapshot describing the reset metadata.
    pub base: InputReset<'a>,
}

impl<'a> TextFieldReset<'a> {
    /// Returns the value after the reset completed.
    pub fn value(&self) -> &str {
        self.base.value
    }

    /// Flag describing whether validation errors were present before the reset.
    pub fn cleared_errors(&self) -> bool {
        self.base.cleared_errors
    }

    /// Returns analytics events generated by the reset operation.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }
}

/// Owned variant of [`TextFieldReset`] for stateful UI adapters.
#[derive(Debug, Clone, PartialEq)]
pub struct TextFieldResetEvent {
    /// Shared [`InputResetEvent`] snapshot describing the reset metadata.
    pub base: InputResetEvent,
}

impl TextFieldResetEvent {
    /// Returns the value after the reset completed.
    pub fn value(&self) -> &str {
        &self.base.value
    }

    /// Flag describing whether validation errors were present before the reset.
    pub fn cleared_errors(&self) -> bool {
        self.base.cleared_errors
    }

    /// Returns analytics events generated by the reset operation.
    pub fn analytics(&self) -> &[InputAnalyticsEvent] {
        &self.base.analytics
    }
}

/// Aggregates text field state including validation and automation metadata.
#[derive(Debug, Clone)]
pub struct TextFieldState {
    base: InputState,
    debounce: Option<Duration>,
}

impl TextFieldState {
    /// Construct an uncontrolled text field with an initial value and optional debounce window.
    pub fn uncontrolled(initial: impl Into<String>, debounce: Option<Duration>) -> Self {
        Self {
            base: InputState::uncontrolled(initial, None),
            debounce,
        }
    }

    /// Construct a controlled text field.  The parent component must call
    /// [`TextFieldState::sync_value`] after receiving change notifications.
    pub fn controlled(initial: impl Into<String>, debounce: Option<Duration>) -> Self {
        Self {
            base: InputState::controlled(initial, None),
            debounce,
        }
    }

    /// Internal constructor used by [`TextFieldControlBuilder`] to wrap an [`InputState`].
    pub(crate) fn from_input_state(base: InputState, debounce: Option<Duration>) -> Self {
        Self { base, debounce }
    }

    /// Returns the current value taking pending controlled edits into account.
    #[inline]
    pub fn value(&self) -> &str {
        self.base.value()
    }

    /// Returns the configured control strategy.
    #[inline]
    pub fn control_strategy(&self) -> ControlStrategy {
        self.base.control_strategy()
    }

    /// Returns whether the field has unsaved changes.
    #[inline]
    pub fn dirty(&self) -> bool {
        self.base.dirty()
    }

    /// Returns whether the field has been visited.
    #[inline]
    pub fn visited(&self) -> bool {
        self.base.visited()
    }

    /// Returns the configured debounce interval.
    #[inline]
    pub fn debounce(&self) -> Option<Duration> {
        self.debounce
    }

    /// Returns the currently captured validation errors.
    #[inline]
    pub fn errors(&self) -> &[String] {
        self.base.errors()
    }

    /// Drain accumulated analytics events.
    pub fn drain_analytics(&mut self) -> Vec<InputAnalyticsEvent> {
        self.base.drain_analytics()
    }

    /// Update the current value emitting a [`TextFieldChange`] snapshot.
    pub fn change<F>(&mut self, next: impl Into<String>, notify: F)
    where
        F: FnOnce(TextFieldChange<'_>),
    {
        let change = self.base.change(next, None);
        let snapshot = TextFieldChange {
            base: change,
            debounce: self.debounce,
        };
        notify(snapshot);
    }

    /// Mark the field as visited and emit a [`TextFieldCommit`] snapshot.
    pub fn commit<F>(&mut self, notify: F)
    where
        F: FnOnce(TextFieldCommit<'_>),
    {
        let commit = self.base.commit();
        let snapshot = TextFieldCommit { base: commit };
        notify(snapshot);
    }

    /// Reset the field back to its initial value clearing validation errors.
    pub fn reset<F>(&mut self, notify: F)
    where
        F: FnOnce(TextFieldReset<'_>),
    {
        self.base.set_visited(false);
        let reset = self.base.reset();
        let snapshot = TextFieldReset { base: reset };
        notify(snapshot);
    }

    /// Synchronize the value for controlled fields.  Uncontrolled fields may
    /// also call this method during hydration to align SSR renders.
    pub fn sync_value(&mut self, value: impl Into<String>) {
        let value = value.into();
        self.base.set_value_silently(value.clone());
        self.base.set_initial_value(value);
    }

    /// Replace the validation errors with a new collection.
    pub fn set_errors<I, S>(&mut self, errors: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.base.set_errors(errors);
    }

    /// Clear any captured validation errors.
    pub fn clear_errors(&mut self) {
        self.base.clear_errors();
    }

    /// Update the initial baseline used to calculate the dirty flag.
    pub fn set_initial_value(&mut self, value: impl Into<String>) {
        self.base.set_initial_value(value);
    }

    /// Returns a handle to the underlying [`InputState`].
    pub fn base(&self) -> &InputState {
        &self.base
    }

    /// Returns a mutable handle to the underlying [`InputState`].
    pub fn base_mut(&mut self) -> &mut InputState {
        &mut self.base
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
        (!self.state.errors().is_empty()).then_some(("aria-invalid", "true"))
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
            if self.state.dirty() { "true" } else { "false" },
        )
    }

    /// Returns a `data-visited` tuple describing whether the field has been touched.
    #[inline]
    pub fn data_visited(&self) -> (&'static str, &'static str) {
        (
            "data-visited",
            if self.state.visited() {
                "true"
            } else {
                "false"
            },
        )
    }

    /// Returns an analytics identifier tuple when configured.
    #[inline]
    pub fn data_analytics_id(&self) -> Option<(&'static str, &str)> {
        self.analytics_tag.map(|value| ("data-analytics-id", value))
    }

    /// Returns a condensed status message by joining validation errors.
    pub fn status_message(&self) -> Option<String> {
        if self.state.errors().is_empty() {
            None
        } else {
            Some(self.state.errors().join("\n"))
        }
    }
}

/// Tuple aligning [`TextFieldState`] with the surrounding [`FormControlState`].
#[derive(Debug)]
pub struct TextFieldControlBundle {
    /// Headless text field state machine backed by [`InputState`].
    pub text_field: TextFieldState,
    /// Form control shell providing label/description ARIA metadata.
    pub form_control: FormControlState,
}

/// Fluent builder that wraps [`InputControlBuilder`] while layering debounce
/// configuration specific to text fields.
#[derive(Debug, Clone)]
pub struct TextFieldControlBuilder {
    base: InputControlBuilder,
    debounce: Option<Duration>,
}

impl TextFieldControlBuilder {
    /// Start a new builder using the provided initial value.
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            base: InputControlBuilder::new(initial),
            debounce: None,
        }
    }

    /// Apply a debounce interval to the derived [`TextFieldState`].
    pub fn debounce(mut self, debounce: Option<Duration>) -> Self {
        self.debounce = debounce;
        self
    }

    /// Switch the builder into controlled mode.
    pub fn controlled(mut self) -> Self {
        self.base = self.base.controlled();
        self
    }

    /// Switch the builder into uncontrolled mode explicitly.
    pub fn uncontrolled(mut self) -> Self {
        self.base = self.base.uncontrolled();
        self
    }

    /// Configure an initial selection range for the underlying [`InputState`].
    pub fn selection(mut self, selection: Option<InputSelection>) -> Self {
        self.base = self.base.selection(selection);
        self
    }

    /// Assign an id for the underlying form control.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.base = self.base.id(id);
        self
    }

    /// Replace the `aria-describedby` list with the provided collection.
    pub fn described_by<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.base = self.base.described_by(ids);
        self
    }

    /// Set the label id for the control shell.
    pub fn labelled_by(mut self, id: impl Into<String>) -> Self {
        self.base = self.base.labelled_by(id);
        self
    }

    /// Mark the control as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }

    /// Mark the control as required.
    pub fn required(mut self, required: bool) -> Self {
        self.base = self.base.required(required);
        self
    }

    /// Attach an automation identifier used by analytics probes.
    pub fn automation_id(mut self, id: impl Into<String>) -> Self {
        self.base = self.base.automation_id(id);
        self
    }

    /// Finalise the builder returning the aligned bundle.
    pub fn build(self) -> TextFieldControlBundle {
        let InputControlBundle {
            input,
            form_control,
        } = self.base.build();
        TextFieldControlBundle {
            text_field: TextFieldState::from_input_state(input, self.debounce),
            form_control,
        }
    }
}

impl<'a> From<TextFieldChange<'a>> for TextFieldChangeEvent {
    fn from(snapshot: TextFieldChange<'a>) -> Self {
        Self {
            base: InputChangeEvent::from(snapshot.base),
            debounce: snapshot.debounce,
        }
    }
}

impl<'a> From<TextFieldCommit<'a>> for TextFieldCommitEvent {
    fn from(snapshot: TextFieldCommit<'a>) -> Self {
        Self {
            base: InputCommitEvent::from(snapshot.base),
        }
    }
}

impl<'a> From<TextFieldReset<'a>> for TextFieldResetEvent {
    fn from(snapshot: TextFieldReset<'a>) -> Self {
        Self {
            base: InputResetEvent::from(snapshot.base),
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
            snapshot_value = snapshot.value().to_string();
            assert!(snapshot.dirty());
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
            change_called = snapshot.value() == "world";
            assert!(snapshot.dirty());
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
        state.commit(|snapshot| visited_before = Some(snapshot.previously_visited()));
        assert_eq!(visited_before, Some(false));
        assert!(state.visited());
    }

    #[test]
    fn reset_restores_initial_state_and_clears_errors() {
        let mut state = TextFieldState::uncontrolled("baseline", None);
        state.set_errors([String::from("Required")]);
        state.change("updated", |_| {});
        state.commit(|_| {});
        let mut cleared = None;
        state.reset(|snapshot| cleared = Some(snapshot.cleared_errors()));
        assert_eq!(cleared, Some(true));
        assert_eq!(state.value(), "baseline");
        assert!(!state.dirty());
        assert!(!state.visited());
        assert!(state.errors().is_empty());
    }

    #[test]
    fn attribute_builder_emits_expected_metadata() {
        let mut state = TextFieldState::uncontrolled("", None);
        state.set_errors([String::from("Required"), String::from("Must be unique")]);
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
        assert_eq!(event.value(), "updated");
        assert!(event.dirty());
        assert_eq!(event.debounce, Some(Duration::from_millis(150)));
    }

    #[test]
    fn owned_commit_event_preserves_flags() {
        let mut state = TextFieldState::uncontrolled("value", None);
        state.set_errors([String::from("Required")]);
        let mut event = None;
        state.commit(|snapshot| {
            event = Some(TextFieldCommitEvent::from(snapshot));
        });
        let event = event.expect("commit event");
        assert_eq!(event.value(), "value");
        assert!(event.has_errors());
        assert!(!event.previously_visited());
    }

    #[test]
    fn owned_reset_event_captures_cleared_errors() {
        let mut state = TextFieldState::uncontrolled("value", None);
        state.set_errors([String::from("Required")]);
        let mut event = None;
        state.reset(|snapshot| {
            event = Some(TextFieldResetEvent::from(snapshot));
        });
        let event = event.expect("reset event");
        assert_eq!(event.value(), "value");
        assert!(event.cleared_errors());
    }
}
