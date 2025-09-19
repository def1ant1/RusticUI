//! Core state machine shared by checkbox and switch implementations.
//!
//! The struct intentionally focuses on behavior and omits any rendering
//! concerns so the logic can be reused by multiple adapters without leaking
//! framework specific concepts into the headless crate.

use crate::interaction::ControlKey;

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
    checked: bool,
}

impl ToggleState {
    /// Create a new toggle state.
    pub(crate) fn new(mode: ToggleMode, disabled: bool, initial_checked: bool) -> Self {
        Self {
            mode,
            disabled,
            focus_visible: false,
            checked: initial_checked,
        }
    }

    /// Returns whether the toggle is currently checked.
    pub(crate) fn checked(&self) -> bool {
        self.checked
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
    pub(crate) fn sync(&mut self, checked: bool) {
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
    pub(crate) fn toggle<F: FnOnce(bool)>(&mut self, callback: F) {
        if self.disabled {
            return;
        }
        let next = !self.checked;
        if self.mode == ToggleMode::Uncontrolled {
            self.checked = next;
        }
        callback(next);
    }

    /// Handle keyboard input.
    pub(crate) fn on_key<F: FnOnce(bool)>(&mut self, key: ControlKey, callback: F) {
        if matches!(key, ControlKey::Space | ControlKey::Enter) {
            self.toggle(callback);
        }
    }
}
