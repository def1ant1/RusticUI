#![deny(missing_docs)]
//! Shared headless state machine for input like controls.
//!
//! Enterprise grade experiences rely on deterministic bookkeeping around
//! controlled/uncontrolled value handling, selection ranges, focus transitions,
//! validation state, and analytics instrumentation.  This module extracts that
//! logic into a reusable state machine so text fields, number fields, and
//! similar inputs can compose the same semantics regardless of the rendering
//! adapter.  The builders exposed here intentionally mirror the documentation
//! style from [`crate::text_field`] to keep guidance consistent across the
//! forms surface area.
//!
//! ## Design goals
//!
//! * Centralise value/selection/focus state without leaking framework specific
//!   details into downstream crates.
//! * Emit analytics friendly markers for automation harnesses without forcing
//!   consumers to sprinkle bespoke logging throughout their components.
//! * Provide fluent builders that wrap [`crate::form_control::FormControlState`]
//!   so high level widgets can stitch together the accessibility shell and the
//!   input state machine with a single call.
//! * Retain the controlled/uncontrolled ergonomics already proven in the
//!   [`crate::text_field`] state machine while covering additional metadata such
//!   as selections and validation error collections.

use crate::form_control::{FormControlConfig, FormControlMode, FormControlState};
use crate::selection::ControlStrategy;
use std::mem;

/// Normalised selection range used by input adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSelection {
    /// Inclusive start offset within the value buffer.
    pub start: usize,
    /// Exclusive end offset within the value buffer.
    pub end: usize,
}

impl InputSelection {
    /// Construct a new selection ensuring `start <= end` by normalising the
    /// provided offsets.
    pub fn new(a: usize, b: usize) -> Self {
        let (start, end) = if a <= b { (a, b) } else { (b, a) };
        Self { start, end }
    }

    /// Build a collapsed caret style selection.
    pub fn collapsed(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    /// Returns whether the selection is collapsed (caret only).
    #[inline]
    pub fn is_collapsed(&self) -> bool {
        self.start == self.end
    }
}

/// Analytics markers emitted for automation/test instrumentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputAnalyticsEventKind {
    /// Value change was applied.
    ValueChange,
    /// Commit event fired (typically blur or enter key).
    Commit,
    /// Reset returned the input to its initial value.
    Reset,
    /// Validation errors were updated.
    Validation,
    /// Focus was gained by the control.
    FocusGained,
    /// Focus left the control.
    FocusLost,
}

/// Concrete analytics event containing optional detail payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAnalyticsEvent {
    /// Specific event kind.
    pub kind: InputAnalyticsEventKind,
    /// Optional string payload for downstream probes.
    pub detail: Option<String>,
}

impl InputAnalyticsEvent {
    fn simple(kind: InputAnalyticsEventKind) -> Self {
        Self { kind, detail: None }
    }

    fn with_detail(kind: InputAnalyticsEventKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: Some(detail.into()),
        }
    }
}

/// Snapshot emitted after mutating the value buffer.
#[derive(Debug, Clone)]
pub struct InputChange<'a> {
    /// Borrowed reference to the latest value taking pending controlled edits
    /// into account.
    pub value: &'a str,
    /// Current selection range if provided by the adapter.
    pub selection: Option<InputSelection>,
    /// Flag describing whether the input diverged from its initial value.
    pub dirty: bool,
    /// Analytics events generated alongside the change.
    pub analytics: Vec<InputAnalyticsEvent>,
}

/// Snapshot emitted when the field commits (blur/enter).
#[derive(Debug, Clone)]
pub struct InputCommit<'a> {
    /// Borrowed reference to the latest value at commit time.
    pub value: &'a str,
    /// Whether validation errors are currently registered.
    pub has_errors: bool,
    /// Whether the input was visited prior to this commit.
    pub previously_visited: bool,
    /// Analytics events generated for the commit.
    pub analytics: Vec<InputAnalyticsEvent>,
}

/// Snapshot emitted when the input resets back to its initial value.
#[derive(Debug, Clone)]
pub struct InputReset<'a> {
    /// Borrowed reference to the value after the reset completed.
    pub value: &'a str,
    /// Flag describing whether validation errors were cleared.
    pub cleared_errors: bool,
    /// Analytics events tied to the reset action.
    pub analytics: Vec<InputAnalyticsEvent>,
}

/// Owned variant for adapters that require static lifetimes.
#[derive(Debug, Clone, PartialEq)]
pub struct InputChangeEvent {
    /// Owned value at change time.
    pub value: String,
    /// Whether the new value is dirty.
    pub dirty: bool,
    /// Mirrored selection where relevant.
    pub selection: Option<InputSelection>,
    /// Analytics markers associated with the change.
    pub analytics: Vec<InputAnalyticsEvent>,
}

impl From<InputChange<'_>> for InputChangeEvent {
    fn from(value: InputChange<'_>) -> Self {
        Self {
            value: value.value.to_string(),
            dirty: value.dirty,
            selection: value.selection,
            analytics: value.analytics,
        }
    }
}

/// Owned commit event for UI layers that need to capture `'static` payloads.
#[derive(Debug, Clone, PartialEq)]
pub struct InputCommitEvent {
    /// Owned value at commit time.
    pub value: String,
    /// Whether errors were present at commit time.
    pub has_errors: bool,
    /// Whether the commit marked the first visit.
    pub previously_visited: bool,
    /// Analytics markers associated with the commit.
    pub analytics: Vec<InputAnalyticsEvent>,
}

impl From<InputCommit<'_>> for InputCommitEvent {
    fn from(value: InputCommit<'_>) -> Self {
        Self {
            value: value.value.to_string(),
            has_errors: value.has_errors,
            previously_visited: value.previously_visited,
            analytics: value.analytics,
        }
    }
}

/// Owned reset snapshot for frameworks with `'static` requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct InputResetEvent {
    /// Owned value after the reset finished.
    pub value: String,
    /// Whether validation errors were cleared.
    pub cleared_errors: bool,
    /// Analytics markers generated by the reset.
    pub analytics: Vec<InputAnalyticsEvent>,
}

impl From<InputReset<'_>> for InputResetEvent {
    fn from(value: InputReset<'_>) -> Self {
        Self {
            value: value.value.to_string(),
            cleared_errors: value.cleared_errors,
            analytics: value.analytics,
        }
    }
}

/// Core headless state machine backing input controls.
#[derive(Debug, Clone)]
pub struct InputState {
    control_mode: ControlStrategy,
    value: String,
    initial_value: String,
    pending_controlled: Option<String>,
    dirty: bool,
    visited: bool,
    focused: bool,
    selection: Option<InputSelection>,
    errors: Vec<String>,
    analytics: Vec<InputAnalyticsEvent>,
}

impl InputState {
    /// Construct an uncontrolled input with an optional initial selection.
    pub fn uncontrolled(initial: impl Into<String>, selection: Option<InputSelection>) -> Self {
        Self::new(initial, ControlStrategy::Uncontrolled, selection)
    }

    /// Construct a controlled input with an optional initial selection.
    pub fn controlled(initial: impl Into<String>, selection: Option<InputSelection>) -> Self {
        Self::new(initial, ControlStrategy::Controlled, selection)
    }

    fn new(
        initial: impl Into<String>,
        mode: ControlStrategy,
        selection: Option<InputSelection>,
    ) -> Self {
        let value = initial.into();
        Self {
            control_mode: mode,
            initial_value: value.clone(),
            value,
            pending_controlled: None,
            dirty: false,
            visited: false,
            focused: false,
            selection,
            errors: Vec::new(),
            analytics: Vec::new(),
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

    /// Returns the current selection range, if any.
    #[inline]
    pub const fn selection(&self) -> Option<InputSelection> {
        self.selection
    }

    /// Update the selection stored in the state machine.
    pub fn set_selection(&mut self, selection: Option<InputSelection>) {
        self.selection = selection.map(|sel| InputSelection::new(sel.start, sel.end));
    }

    /// Returns whether the input is dirty.
    #[inline]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether the input has been visited.
    #[inline]
    pub const fn visited(&self) -> bool {
        self.visited
    }

    /// Returns whether the control is currently focused.
    #[inline]
    pub const fn focused(&self) -> bool {
        self.focused
    }

    /// Returns the validation errors currently registered.
    #[inline]
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn push_analytics(&mut self, event: InputAnalyticsEvent) {
        self.analytics.push(event);
    }

    fn take_analytics(&mut self) -> Vec<InputAnalyticsEvent> {
        mem::take(&mut self.analytics)
    }

    /// Drain accumulated analytics events without performing any additional
    /// state transitions.  Automation harnesses call this after issuing a
    /// sequence of mutations to collect telemetry in deterministic batches.
    pub fn drain_analytics(&mut self) -> Vec<InputAnalyticsEvent> {
        self.take_analytics()
    }

    fn recompute_dirty(&mut self) {
        self.dirty = self.value() != self.initial_value;
    }

    /// Apply a value change and emit a [`InputChange`] snapshot.
    pub fn change(
        &mut self,
        next: impl Into<String>,
        selection: Option<InputSelection>,
    ) -> InputChange<'_> {
        let value = next.into();
        if self.control_mode.is_controlled() {
            self.pending_controlled = Some(value);
        } else {
            self.value = value;
        }
        if let Some(sel) = selection {
            self.selection = Some(InputSelection::new(sel.start, sel.end));
        }
        self.recompute_dirty();
        self.push_analytics(InputAnalyticsEvent::with_detail(
            InputAnalyticsEventKind::ValueChange,
            self.value().to_string(),
        ));
        let analytics = self.take_analytics();
        InputChange {
            value: self.value(),
            selection: self.selection,
            dirty: self.dirty,
            analytics,
        }
    }

    /// Synchronise the value for controlled inputs after external updates.
    pub fn sync_controlled_value(&mut self, value: impl Into<String>) {
        if self.control_mode.is_controlled() {
            let incoming = value.into();
            self.value = incoming;
            self.pending_controlled = None;
            self.recompute_dirty();
        }
    }

    /// Commit the input, toggling the visited flag and emitting analytics.
    pub fn commit(&mut self) -> InputCommit<'_> {
        let previously_visited = self.visited;
        self.visited = true;
        self.push_analytics(InputAnalyticsEvent::simple(InputAnalyticsEventKind::Commit));
        let analytics = self.take_analytics();
        InputCommit {
            value: self.value(),
            has_errors: !self.errors.is_empty(),
            previously_visited,
            analytics,
        }
    }

    /// Reset the input to its initial value.
    pub fn reset(&mut self) -> InputReset<'_> {
        let cleared_errors = !self.errors.is_empty();
        self.value = self.initial_value.clone();
        self.pending_controlled = None;
        self.dirty = false;
        self.selection = None;
        if cleared_errors {
            self.errors.clear();
        }
        self.push_analytics(InputAnalyticsEvent::simple(InputAnalyticsEventKind::Reset));
        let analytics = self.take_analytics();
        InputReset {
            value: self.value(),
            cleared_errors,
            analytics,
        }
    }

    /// Toggle the focused flag returning the analytics events emitted.
    pub fn set_focused(&mut self, focused: bool) -> Vec<InputAnalyticsEvent> {
        if self.focused == focused {
            return Vec::new();
        }
        self.focused = focused;
        if focused {
            self.push_analytics(InputAnalyticsEvent::simple(
                InputAnalyticsEventKind::FocusGained,
            ));
        } else {
            self.push_analytics(InputAnalyticsEvent::simple(
                InputAnalyticsEventKind::FocusLost,
            ));
        }
        self.take_analytics()
    }

    /// Replace the validation errors collection.
    pub fn set_errors<I, S>(&mut self, errors: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.errors = errors.into_iter().map(Into::into).collect();
        self.push_analytics(InputAnalyticsEvent::with_detail(
            InputAnalyticsEventKind::Validation,
            self.errors.len().to_string(),
        ));
    }
}

/// Tuple like bundle combining [`InputState`] with the surrounding
/// [`FormControlState`].
#[derive(Debug)]
pub struct InputControlBundle {
    /// Headless input state machine.
    pub input: InputState,
    /// Form control shell providing accessibility metadata.
    pub form_control: FormControlState,
}

/// Fluent builder that keeps the [`FormControlState`] and [`InputState`]
/// configurations aligned.
#[derive(Debug, Clone)]
pub struct InputControlBuilder {
    value: String,
    selection: Option<InputSelection>,
    control: ControlStrategy,
    config: FormControlConfig,
}

impl InputControlBuilder {
    /// Start a builder using the provided initial value.  The default mode is
    /// uncontrolled to keep parity with native input elements.
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            value: initial.into(),
            selection: None,
            control: ControlStrategy::Uncontrolled,
            config: FormControlConfig::default(),
        }
    }

    /// Switch the builder into controlled mode.
    pub fn controlled(mut self) -> Self {
        self.control = ControlStrategy::Controlled;
        self
    }

    /// Switch the builder into uncontrolled mode explicitly.
    pub fn uncontrolled(mut self) -> Self {
        self.control = ControlStrategy::Uncontrolled;
        self
    }

    /// Configure an initial selection range.
    pub fn selection(mut self, selection: Option<InputSelection>) -> Self {
        self.selection = selection.map(|sel| InputSelection::new(sel.start, sel.end));
        self
    }

    /// Assign an id for the underlying form control.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.config.id = Some(id.into());
        self
    }

    /// Replace the `aria-describedby` list with the provided collection.
    pub fn described_by<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.config.described_by = ids.into_iter().map(Into::into).collect();
        self
    }

    /// Set the label id for the control shell.
    pub fn labelled_by(mut self, id: impl Into<String>) -> Self {
        self.config.labelled_by = Some(id.into());
        self
    }

    /// Mark the control as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = disabled;
        self
    }

    /// Mark the control as required.
    pub fn required(mut self, required: bool) -> Self {
        self.config.required = required;
        self
    }

    /// Attach an automation identifier used by analytics probes.
    pub fn automation_id(mut self, id: impl Into<String>) -> Self {
        self.config.automation_id = Some(id.into());
        self
    }

    /// Finalise the builder returning the aligned bundle.
    pub fn build(self) -> InputControlBundle {
        let mode = match self.control {
            ControlStrategy::Controlled => FormControlMode::Controlled,
            ControlStrategy::Uncontrolled => FormControlMode::Uncontrolled,
        };
        let mut config = self.config.clone();
        // Ensure described_by does not contain duplicates that would hurt
        // automation harness readability.
        config.described_by.sort();
        config.described_by.dedup();
        let form_control = FormControlState::new(self.value.clone(), mode, config);
        let input = match self.control {
            ControlStrategy::Controlled => InputState::controlled(self.value, self.selection),
            ControlStrategy::Uncontrolled => InputState::uncontrolled(self.value, self.selection),
        };
        InputControlBundle {
            input,
            form_control,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_aligns_modes() {
        let mut bundle = InputControlBuilder::new("seed")
            .controlled()
            .automation_id("input.email")
            .labelled_by("email-label")
            .build();
        assert!(bundle.form_control.controlled_value().is_some());
        assert_eq!(bundle.input.control_strategy(), ControlStrategy::Controlled);
        let change = bundle
            .input
            .change("next", Some(InputSelection::collapsed(4)));
        assert!(change.dirty);
        assert_eq!(bundle.form_control.controlled_value(), Some("seed"));
    }

    #[test]
    fn focus_transitions_emit_analytics() {
        let mut state = InputState::uncontrolled("value", None);
        let gained = state.set_focused(true);
        assert_eq!(gained.len(), 1);
        assert_eq!(gained[0].kind, InputAnalyticsEventKind::FocusGained);
        let lost = state.set_focused(false);
        assert_eq!(lost.len(), 1);
        assert_eq!(lost[0].kind, InputAnalyticsEventKind::FocusLost);
    }
}
