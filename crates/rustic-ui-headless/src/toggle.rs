//! Core state machine shared by checkbox and switch implementations.
//!
//! The struct intentionally focuses on behavior and omits any rendering
//! concerns so the logic can be reused by multiple adapters without leaking
//! framework specific concepts into the headless crate.

use crate::interaction::ControlKey;

/// Represents the logical value a toggle-like control can assume.
///
/// The states intentionally mirror the semantics used by checkbox style inputs
/// on the web where `mixed`/indeterminate conveys a partially selected group.
/// Keeping the type in this module lets both checkboxes and switches share the
/// behavior while documenting how each downstream consumer interprets the
/// variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleCheckedState {
    /// Explicitly unselected / `false`.
    Off,
    /// Explicitly selected / `true`.
    On,
    /// A partial selection state commonly rendered with a horizontal bar.
    Indeterminate,
}

impl ToggleCheckedState {
    /// Whether the state should be treated as logically on when exposing
    /// boolean oriented helpers (e.g. legacy data attributes).
    pub(crate) const fn is_on(self) -> bool {
        matches!(self, Self::On)
    }

    /// Whether the state conveys an indeterminate / mixed value.
    pub(crate) const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Compute the next state when the user requests a toggle.
    ///
    /// Material guidelines jump from indeterminate to `On` so users always see
    /// a clear binary answer after interaction. Subsequent toggles alternate
    /// between `Off` and `On` as expected for two-state controls.
    pub(crate) const fn toggled(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
            Self::Indeterminate => Self::On,
        }
    }
}

impl From<bool> for ToggleCheckedState {
    fn from(value: bool) -> Self {
        if value {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl From<ToggleCheckedState> for bool {
    fn from(value: ToggleCheckedState) -> Self {
        value.is_on()
    }
}

/// Describes whether the toggle is controlled by an external owner or manages
/// its own state internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToggleMode {
    /// Controlled widgets only broadcast the requested value through callbacks
    /// and depend on the parent to call [`ToggleState::sync`] with the new
    /// value.
    Controlled,
    /// Uncontrolled widgets mutate their internal state directly.
    Uncontrolled,
}

/// Internal toggle state shared by [`CheckboxState`](crate::checkbox::CheckboxState)
/// and [`SwitchState`](crate::switch::SwitchState).
#[derive(Debug, Clone)]
pub(crate) struct ToggleState {
    mode: ToggleMode,
    disabled: bool,
    focus_visible: bool,
    checked: ToggleCheckedState,
}

impl ToggleState {
    /// Create a new toggle state.
    pub(crate) fn new(
        mode: ToggleMode,
        disabled: bool,
        initial_checked: ToggleCheckedState,
    ) -> Self {
        Self {
            mode,
            disabled,
            focus_visible: false,
            checked: initial_checked,
        }
    }

    /// Returns the tri-state value describing the toggle.
    pub(crate) fn checked(&self) -> ToggleCheckedState {
        self.checked
    }

    /// Returns whether the toggle should be treated as logically on.
    pub(crate) fn is_on(&self) -> bool {
        self.checked.is_on()
    }

    /// Returns whether the toggle is currently in the indeterminate state.
    pub(crate) fn is_indeterminate(&self) -> bool {
        self.checked.is_indeterminate()
    }

    /// Returns whether the toggle is disabled.
    pub(crate) fn disabled(&self) -> bool {
        self.disabled
    }

    /// Update the disabled flag.
    pub(crate) fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Synchronize the state with an externally provided value.
    pub(crate) fn sync(&mut self, checked: ToggleCheckedState) {
        self.checked = checked;
    }

    /// Returns whether the toggle is currently in the focus-visible state.
    pub(crate) fn focus_visible(&self) -> bool {
        self.focus_visible
    }

    /// Marks the control as focused.
    pub(crate) fn focus(&mut self) {
        self.focus_visible = true;
    }

    /// Clears the focus-visible flag.
    pub(crate) fn blur(&mut self) {
        self.focus_visible = false;
    }

    /// Toggles the checked state if the widget is enabled.
    pub(crate) fn toggle<F: FnOnce(ToggleCheckedState)>(&mut self, callback: F) {
        if self.disabled {
            return;
        }
        let next = self.checked.toggled();
        if self.mode == ToggleMode::Uncontrolled {
            self.checked = next;
        }
        callback(next);
    }

    /// Handle keyboard input.
    pub(crate) fn on_key<F: FnOnce(ToggleCheckedState)>(&mut self, key: ControlKey, callback: F) {
        if matches!(key, ControlKey::Space | ControlKey::Enter) {
            self.toggle(callback);
        }
    }
}
