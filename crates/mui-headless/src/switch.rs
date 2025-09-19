//! State machine powering Material style switches.
//!
//! Switches share most logic with checkboxes but expose a dedicated type so the
//! API mirrors Material's component catalog. The machine handles controlled and
//! uncontrolled usage, tracks focus visibility and exposes ARIA metadata for
//! framework adapters.

use crate::aria;
use crate::interaction::ControlKey;
use crate::toggle::{ToggleMode, ToggleState};

/// Represents a Material switch.
#[derive(Debug, Clone)]
pub struct SwitchState {
    inner: ToggleState,
    mode: ToggleMode,
}

impl SwitchState {
    /// Construct a controlled switch.
    pub fn controlled(disabled: bool, on: bool) -> Self {
        let mode = ToggleMode::Controlled;
        Self {
            inner: ToggleState::new(mode, disabled, on),
            mode,
        }
    }

    /// Construct an uncontrolled switch.
    pub fn uncontrolled(disabled: bool, default_on: bool) -> Self {
        let mode = ToggleMode::Uncontrolled;
        Self {
            inner: ToggleState::new(mode, disabled, default_on),
            mode,
        }
    }

    /// Whether the switch is controlled.
    pub fn is_controlled(&self) -> bool {
        matches!(self.mode, ToggleMode::Controlled)
    }

    /// Current on/off state.
    pub fn on(&self) -> bool {
        self.inner.checked()
    }

    /// Synchronize the internal state with the provided value.
    pub fn sync_on(&mut self, on: bool) {
        self.inner.sync(on);
    }

    /// Returns whether the switch is disabled.
    pub fn disabled(&self) -> bool {
        self.inner.disabled()
    }

    /// Update the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.inner.set_disabled(disabled);
    }

    /// Mark the switch as focused.
    pub fn focus(&mut self) {
        self.inner.focus();
    }

    /// Remove focus styling.
    pub fn blur(&mut self) {
        self.inner.blur();
    }

    /// Whether focus styles should be applied.
    pub fn focus_visible(&self) -> bool {
        self.inner.focus_visible()
    }

    /// Toggle the switch.
    pub fn toggle<F: FnOnce(bool)>(&mut self, callback: F) {
        self.inner.toggle(callback);
    }

    /// Handle keyboard input.
    pub fn on_key<F: FnOnce(bool)>(&mut self, key: ControlKey, callback: F) {
        self.inner.on_key(key, callback);
    }

    /// Attributes describing the switch.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", aria::role_switch().into()));
        let (k, v) = aria::aria_checked(self.on());
        attrs.push((k, v.into()));
        let (k, v) = aria::aria_disabled(self.disabled());
        attrs.push((k, v.into()));
        attrs.push((
            "tabindex",
            if self.disabled() { "-1" } else { "0" }.to_string(),
        ));
        attrs.push((
            "data-on",
            if self.on() { "true" } else { "false" }.to_string(),
        ));
        attrs.push((
            "data-focus-visible",
            if self.focus_visible() {
                "true"
            } else {
                "false"
            }
            .to_string(),
        ));
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_toggle_flips_state() {
        let mut state = SwitchState::uncontrolled(false, false);
        state.toggle(|_| {});
        assert!(state.on());
    }

    #[test]
    fn controlled_toggle_reports_request() {
        let mut state = SwitchState::controlled(false, false);
        let mut received = None;
        state.toggle(|value| received = Some(value));
        assert!(!state.on());
        assert_eq!(received, Some(true));
    }

    #[test]
    fn keyboard_enter_triggers_toggle() {
        let mut state = SwitchState::uncontrolled(false, false);
        state.on_key(ControlKey::Enter, |_| {});
        assert!(state.on());
    }

    #[test]
    fn aria_describes_state() {
        let mut state = SwitchState::uncontrolled(false, true);
        state.focus();
        let attrs = state.aria_attributes();
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "switch"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-checked" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-focus-visible" && v == "true"));
    }
}
