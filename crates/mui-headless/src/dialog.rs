#![deny(missing_docs)]
//! Declarative state machine powering modal and non-modal dialog surfaces.
//!
//! The dialog primitive is intentionally opinionated: it centralises open/close
//! transitions, escape key handling, and focus trap bookkeeping so adapters can
//! remain rendering-centric.  By funnelling all state through a single struct we
//! can keep SSR and CSR behaviour aligned while exposing analytics-friendly data
//! hooks that large applications rely on for automation and observability.

use crate::aria;
use crate::selection::ControlStrategy;

/// High level lifecycle states the dialog can occupy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogPhase {
    /// Dialog is hidden and idle.
    Closed,
    /// Dialog has been asked to open and is typically running an entry
    /// animation.  Consumers should call [`DialogState::finish_open`] once the
    /// transition completes.
    Opening,
    /// Dialog is fully visible with focus trapped inside.
    Open,
    /// Dialog has been asked to close and is typically running an exit
    /// animation.  Consumers should call [`DialogState::finish_close`] when the
    /// animation settles.
    Closing,
}

impl DialogPhase {
    /// Returns whether the surface should be rendered.
    #[inline]
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Opening | Self::Open | Self::Closing)
    }

    /// Returns a string representation suitable for analytics hooks.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Opening => "opening",
            Self::Open => "open",
            Self::Closing => "closing",
        }
    }
}

impl Default for DialogPhase {
    fn default() -> Self {
        Self::Closed
    }
}

/// Describes the last transition intent emitted by the dialog state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTransition {
    /// The dialog has requested to open.
    OpenRequested,
    /// The dialog has requested to close.
    CloseRequested,
}

impl DialogTransition {
    /// Returns a string representation ideal for `data-*` attributes.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRequested => "open",
            Self::CloseRequested => "close",
        }
    }
}

/// Aggregates dialog state including transition bookkeeping and accessibility
/// metadata.
#[derive(Debug, Clone)]
pub struct DialogState {
    phase: DialogPhase,
    control_mode: ControlStrategy,
    escape_closes: bool,
    focus_trap_engaged: bool,
    last_transition: Option<DialogTransition>,
    modal: bool,
}

impl DialogState {
    /// Construct an uncontrolled dialog.  The dialog immediately mutates its
    /// internal phase whenever [`DialogState::open`] or [`DialogState::close`]
    /// is invoked, making it ideal for simple applications and storybooks.
    pub fn uncontrolled(default_open: bool) -> Self {
        let mut state = Self {
            phase: DialogPhase::Closed,
            control_mode: ControlStrategy::Uncontrolled,
            escape_closes: true,
            focus_trap_engaged: false,
            last_transition: None,
            modal: true,
        };
        if default_open {
            state.phase = DialogPhase::Open;
            state.focus_trap_engaged = state.modal;
        }
        state
    }

    /// Construct a controlled dialog.  Controlled dialogs emit intents through
    /// callbacks and expect the parent component to synchronize the phase via
    /// [`DialogState::sync_open`].
    pub fn controlled() -> Self {
        Self {
            phase: DialogPhase::Closed,
            control_mode: ControlStrategy::Controlled,
            escape_closes: true,
            focus_trap_engaged: false,
            last_transition: None,
            modal: true,
        }
    }

    /// Returns the current phase of the dialog.
    #[inline]
    pub const fn phase(&self) -> DialogPhase {
        self.phase
    }

    /// Returns whether the dialog should be rendered.
    #[inline]
    pub const fn is_open(&self) -> bool {
        self.phase.is_visible()
    }

    /// Returns whether the internal focus trap should be considered active.
    #[inline]
    pub const fn focus_trap_engaged(&self) -> bool {
        self.focus_trap_engaged
    }

    /// Returns whether the dialog is currently considered modal.
    #[inline]
    pub const fn is_modal(&self) -> bool {
        self.modal
    }

    /// Returns the strategy used to control the open flag.
    #[inline]
    pub const fn control_strategy(&self) -> ControlStrategy {
        self.control_mode
    }

    /// Configure whether pressing `Escape` should close the dialog.  Enterprise
    /// experiences occasionally disable this behaviour for destructive flows
    /// that require explicit acknowledgement.
    pub fn set_escape_closes(&mut self, escape_closes: bool) {
        self.escape_closes = escape_closes;
    }

    /// Configure whether the dialog should expose modal semantics.  When set to
    /// `false` the focus trap helpers remain disabled so layout engines can keep
    /// background content interactive.
    pub fn set_modal(&mut self, modal: bool) {
        self.modal = modal;
        if !modal {
            self.focus_trap_engaged = false;
        } else if matches!(self.phase, DialogPhase::Open) {
            self.focus_trap_engaged = true;
        }
    }

    /// Returns the last transition intent if any.  Analytics systems often
    /// inspect this to correlate open/close rates with feature flags.
    #[inline]
    pub const fn last_transition(&self) -> Option<DialogTransition> {
        self.last_transition
    }

    /// Request the dialog to open.  The provided callback receives the desired
    /// visibility flag (`true`).
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) {
        if matches!(self.phase, DialogPhase::Opening | DialogPhase::Open) {
            return;
        }
        self.phase = DialogPhase::Opening;
        self.focus_trap_engaged = false;
        self.last_transition = Some(DialogTransition::OpenRequested);
        if !self.control_mode.is_controlled() {
            self.finish_open();
        }
        notify(true);
    }

    /// Request the dialog to close.  The provided callback receives the desired
    /// visibility flag (`false`).
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) {
        if matches!(self.phase, DialogPhase::Closing | DialogPhase::Closed) {
            return;
        }
        self.phase = DialogPhase::Closing;
        self.last_transition = Some(DialogTransition::CloseRequested);
        if !self.control_mode.is_controlled() {
            self.finish_close();
        }
        notify(false);
    }

    /// Toggle the dialog visibility.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) {
        if self.is_open() {
            self.close(notify);
        } else {
            self.open(notify);
        }
    }

    /// Handle an escape key press returning whether the event was consumed.  If
    /// escape handling is enabled the dialog issues a close intent.
    pub fn handle_escape<F: FnOnce(bool)>(&mut self, notify: F) -> bool {
        if !self.escape_closes || !self.is_open() {
            return false;
        }
        self.close(notify);
        true
    }

    /// Synchronize the visible state for controlled dialogs.  Uncontrolled
    /// dialogs may also call this during hydration to ensure SSR parity.
    pub fn sync_open(&mut self, open: bool) {
        self.phase = if open {
            DialogPhase::Open
        } else {
            DialogPhase::Closed
        };
        self.focus_trap_engaged = open && self.modal;
        if !open {
            self.last_transition = Some(DialogTransition::CloseRequested);
        } else {
            self.last_transition = Some(DialogTransition::OpenRequested);
        }
    }

    /// Mark the end of the open transition, enabling the focus trap.
    pub fn finish_open(&mut self) {
        self.phase = DialogPhase::Open;
        self.focus_trap_engaged = self.modal;
    }

    /// Mark the end of the close transition, releasing the focus trap.
    pub fn finish_close(&mut self) {
        self.phase = DialogPhase::Closed;
        self.focus_trap_engaged = false;
    }

    /// Returns a helper used to build ARIA/data attributes for the dialog
    /// surface.
    pub fn surface_attributes(&self) -> DialogSurfaceAttributes<'_> {
        DialogSurfaceAttributes::new(self)
    }

    /// Returns a helper used to expose analytics friendly hooks for the
    /// backdrop element.
    pub fn backdrop_attributes(&self) -> DialogBackdropAttributes<'_> {
        DialogBackdropAttributes::new(self)
    }
}

/// Attribute builder for dialog surfaces.
#[derive(Debug, Clone)]
pub struct DialogSurfaceAttributes<'a> {
    state: &'a DialogState,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
    described_by: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> DialogSurfaceAttributes<'a> {
    fn new(state: &'a DialogState) -> Self {
        Self {
            state,
            id: None,
            labelled_by: None,
            described_by: None,
            analytics_tag: None,
        }
    }

    /// Attach an element identifier to the dialog surface.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the labelling element for screen readers.
    pub fn labelled_by(mut self, id: &'a str) -> Self {
        self.labelled_by = Some(id);
        self
    }

    /// Set the describing element for screen readers.
    pub fn described_by(mut self, id: &'a str) -> Self {
        self.described_by = Some(id);
        self
    }

    /// Attach an analytics identifier.  The returned tuple can be spread onto a
    /// JSX/Sycamore node without additional ceremony.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns the semantic role for the surface.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_dialog()
    }

    /// Returns the `aria-modal` tuple signalling focus should remain trapped.
    #[inline]
    pub fn aria_modal(&self) -> (&'static str, &'static str) {
        aria::aria_modal(self.state.modal)
    }

    /// Returns the `aria-labelledby` tuple if configured.
    #[inline]
    pub fn aria_labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Returns the `aria-describedby` tuple if configured.
    #[inline]
    pub fn aria_describedby(&self) -> Option<(&'static str, &str)> {
        self.described_by.map(aria::aria_describedby)
    }

    /// Returns the `id` attribute tuple if configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns a `data-state` tuple reflecting the current phase.
    #[inline]
    pub fn data_state(&self) -> (&'static str, &'static str) {
        ("data-state", self.state.phase.as_str())
    }

    /// Returns a `data-transition` tuple exposing the last transition intent.
    #[inline]
    pub fn data_transition(&self) -> Option<(&'static str, &'static str)> {
        self.state
            .last_transition
            .map(|transition| ("data-transition", transition.as_str()))
    }

    /// Returns a `data-focus-trap` tuple describing whether focus is currently
    /// trapped within the dialog.
    #[inline]
    pub fn data_focus_trap(&self) -> (&'static str, &'static str) {
        (
            "data-focus-trap",
            if self.state.focus_trap_engaged {
                "active"
            } else {
                "inactive"
            },
        )
    }

    /// Returns an analytics identifier tuple if configured.
    #[inline]
    pub fn data_analytics_id(&self) -> Option<(&'static str, &str)> {
        self.analytics_tag.map(|value| ("data-analytics-id", value))
    }
}

/// Attribute builder for dialog backdrops.
#[derive(Debug, Clone)]
pub struct DialogBackdropAttributes<'a> {
    state: &'a DialogState,
}

impl<'a> DialogBackdropAttributes<'a> {
    fn new(state: &'a DialogState) -> Self {
        Self { state }
    }

    /// Returns whether the backdrop should be visible.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.state.is_open()
    }

    /// Returns analytics metadata for the backdrop, mirroring the dialog
    /// surface.  Automation suites use this to measure close interactions.
    #[inline]
    pub fn data_state(&self) -> (&'static str, &'static str) {
        ("data-backdrop-state", self.state.phase.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_dialog_transitions_and_focus_trap() {
        let mut state = DialogState::uncontrolled(false);
        assert!(!state.is_open());
        state.open(|_| {});
        assert!(state.is_open());
        assert!(state.focus_trap_engaged());
        state.close(|_| {});
        assert!(!state.is_open());
        assert!(!state.focus_trap_engaged());
    }

    #[test]
    fn controlled_dialog_requires_sync() {
        let mut state = DialogState::controlled();
        let mut last = None;
        state.open(|open| last = Some(open));
        assert_eq!(last, Some(true));
        assert_eq!(state.phase(), DialogPhase::Opening);
        assert!(!state.focus_trap_engaged());
        state.sync_open(true);
        assert!(state.is_open());
        assert!(state.focus_trap_engaged());
    }

    #[test]
    fn escape_key_closes_when_enabled() {
        let mut state = DialogState::uncontrolled(true);
        let mut closed = false;
        let consumed = state.handle_escape(|open| closed = !open);
        assert!(consumed);
        assert!(closed);
        assert_eq!(
            state.last_transition(),
            Some(DialogTransition::CloseRequested)
        );
    }

    #[test]
    fn surface_builder_emits_expected_attributes() {
        let mut state = DialogState::uncontrolled(false);
        state.open(|_| {});
        let attrs = state
            .surface_attributes()
            .id("dialog")
            .labelled_by("dialog-title")
            .described_by("dialog-description")
            .analytics_id("checkout-flow");
        assert_eq!(attrs.role(), "dialog");
        assert_eq!(attrs.aria_modal(), ("aria-modal", "true"));
        assert_eq!(
            attrs.aria_labelledby(),
            Some(("aria-labelledby", "dialog-title"))
        );
        assert_eq!(
            attrs.aria_describedby(),
            Some(("aria-describedby", "dialog-description"))
        );
        assert_eq!(attrs.id_attr(), Some(("id", "dialog")));
        assert_eq!(attrs.data_state(), ("data-state", "open"));
        assert_eq!(attrs.data_transition(), Some(("data-transition", "open")));
        assert_eq!(attrs.data_focus_trap(), ("data-focus-trap", "active"));
        assert_eq!(
            attrs.data_analytics_id(),
            Some(("data-analytics-id", "checkout-flow"))
        );
    }

    #[test]
    fn non_modal_dialog_reports_non_modal_attributes() {
        let mut state = DialogState::uncontrolled(true);
        state.set_modal(false);
        state.finish_open();
        assert!(!state.focus_trap_engaged());
        let attrs = state.surface_attributes();
        assert_eq!(attrs.aria_modal(), ("aria-modal", "false"));
    }
}
