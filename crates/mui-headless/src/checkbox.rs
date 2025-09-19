//! State machine powering Material style checkboxes.
//!
//! The implementation focuses purely on behavior so rendering layers can
//! consume the same logic across web frameworks.  It models both controlled and
//! uncontrolled usage patterns, tracks focus visibility and exposes ARIA
//! metadata consistent with the WAI-ARIA Authoring Practices.

use crate::aria;
use crate::interaction::ControlKey;
use crate::toggle::{ToggleMode, ToggleState};

/// Represents a Material checkbox.
#[derive(Debug, Clone)]
pub struct CheckboxState {
    inner: ToggleState,
    mode: ToggleMode,
}

impl CheckboxState {
    /// Create a checkbox whose checked value is owned by the caller.
    pub fn controlled(disabled: bool, checked: bool) -> Self {
        let mode = ToggleMode::Controlled;
        Self {
            inner: ToggleState::new(mode, disabled, checked),
            mode,
        }
    }

    /// Create an uncontrolled checkbox that manages its own state.
    pub fn uncontrolled(disabled: bool, default_checked: bool) -> Self {
        let mode = ToggleMode::Uncontrolled;
        Self {
            inner: ToggleState::new(mode, disabled, default_checked),
            mode,
        }
    }

    /// Returns whether the checkbox currently owns its value.
    pub fn is_controlled(&self) -> bool {
        matches!(self.mode, ToggleMode::Controlled)
    }

    /// Returns whether the checkbox is checked.
    pub fn checked(&self) -> bool {
        self.inner.checked()
    }

    /// Update the stored checked state.  Controlled owners should call this in
    /// response to the callback passed to [`toggle`].  Uncontrolled callers can
    /// use it to imperatively set the value when syncing with server data.
    pub fn sync_checked(&mut self, checked: bool) {
        self.inner.sync(checked);
    }

    /// Returns whether the checkbox is disabled.
    pub fn disabled(&self) -> bool {
        self.inner.disabled()
    }

    /// Update the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.inner.set_disabled(disabled);
    }

    /// Marks the checkbox as focused.
    pub fn focus(&mut self) {
        self.inner.focus();
    }

    /// Clears focus.
    pub fn blur(&mut self) {
        self.inner.blur();
    }

    /// Whether focus styling should be applied.
    pub fn focus_visible(&self) -> bool {
        self.inner.focus_visible()
    }

    /// Toggle the checkbox if enabled.  The callback is invoked with the
    /// desired value so controlled parents can update their copy.
    pub fn toggle<F: FnOnce(bool)>(&mut self, callback: F) {
        self.inner.toggle(callback);
    }

    /// Handle keyboard interaction. Space and Enter toggle the checkbox in line
    /// with the ARIA authoring practices.
    pub fn on_key<F: FnOnce(bool)>(&mut self, key: ControlKey, callback: F) {
        self.inner.on_key(key, callback);
    }

    /// Return ARIA metadata and data attributes describing the checkbox.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", aria::role_checkbox().into()));
        let (k, v) = aria::aria_checked(self.checked());
        attrs.push((k, v.into()));
        let (k, v) = aria::aria_disabled(self.disabled());
        attrs.push((k, v.into()));
        attrs.push((
            "tabindex",
            if self.disabled() { "-1" } else { "0" }.to_string(),
        ));
        attrs.push((
            "data-checked",
            if self.checked() { "true" } else { "false" }.to_string(),
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
    fn uncontrolled_toggle_updates_state() {
        let mut state = CheckboxState::uncontrolled(false, false);
        state.toggle(|_| {});
        assert!(state.checked());
    }

    #[test]
    fn controlled_toggle_only_notifies() {
        let mut state = CheckboxState::controlled(false, false);
        let mut received = None;
        state.toggle(|checked| received = Some(checked));
        assert_eq!(state.checked(), false);
        assert_eq!(received, Some(true));
    }

    #[test]
    fn keyboard_space_invokes_toggle() {
        let mut state = CheckboxState::uncontrolled(false, false);
        state.on_key(ControlKey::Space, |_| {});
        assert!(state.checked());
    }

    #[test]
    fn focus_flags_roundtrip() {
        let mut state = CheckboxState::uncontrolled(false, false);
        assert!(!state.focus_visible());
        state.focus();
        assert!(state.focus_visible());
        state.blur();
        assert!(!state.focus_visible());
    }

    #[test]
    fn aria_attributes_reflect_state() {
        let mut state = CheckboxState::uncontrolled(false, true);
        state.focus();
        let attrs = state.aria_attributes();
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "checkbox"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-checked" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-focus-visible" && v == "true"));
    }
}
