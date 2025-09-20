//! State machine powering Material style checkboxes.
//!
//! The implementation focuses purely on behavior so rendering layers can
//! consume the same logic across web frameworks.  It models both controlled and
//! uncontrolled usage patterns, tracks focus visibility and exposes ARIA
//! metadata consistent with the WAI-ARIA Authoring Practices.

use crate::aria;
use crate::interaction::ControlKey;
use crate::toggle::{ToggleCheckedState, ToggleMode, ToggleState};

/// Public alias allowing downstream crates to type the checkbox value in a more
/// domain-specific manner without re-exporting the internal toggle module.
pub type CheckboxValue = ToggleCheckedState;

/// Represents a Material checkbox.
#[derive(Debug, Clone)]
pub struct CheckboxState {
    inner: ToggleState,
    mode: ToggleMode,
}

impl CheckboxState {
    /// Create a checkbox whose checked value is owned by the caller.
    ///
    /// The helper accepts either a [`CheckboxValue`] or a plain `bool` so
    /// existing binary call sites do not need to change while tri-state owners
    /// can opt-in to [`CheckboxValue::Indeterminate`].
    pub fn controlled(disabled: bool, checked: impl Into<CheckboxValue>) -> Self {
        let mode = ToggleMode::Controlled;
        Self {
            inner: ToggleState::new(mode, disabled, checked.into()),
            mode,
        }
    }

    /// Create an uncontrolled checkbox that manages its own state.
    ///
    /// As with [`CheckboxState::controlled`], the initial value accepts either
    /// the enum or a `bool`, keeping the API terse for the common two-state
    /// cases.
    pub fn uncontrolled(disabled: bool, default_checked: impl Into<CheckboxValue>) -> Self {
        let mode = ToggleMode::Uncontrolled;
        Self {
            inner: ToggleState::new(mode, disabled, default_checked.into()),
            mode,
        }
    }

    /// Returns whether the checkbox currently owns its value.
    pub fn is_controlled(&self) -> bool {
        matches!(self.mode, ToggleMode::Controlled)
    }

    /// Returns the full tri-state value describing the checkbox.
    pub fn checked(&self) -> CheckboxValue {
        self.inner.checked()
    }

    /// Convenience helper mirroring the legacy two-state API while still
    /// supporting indeterminate under the hood.
    pub fn is_checked(&self) -> bool {
        self.inner.is_on()
    }

    /// Whether the checkbox is currently indeterminate.
    pub fn is_indeterminate(&self) -> bool {
        self.inner.is_indeterminate()
    }

    /// Update the stored checked state.  Controlled owners should call this in
    /// response to the callback passed to [`toggle`].  Uncontrolled callers can
    /// use it to imperatively set the value when syncing with server data. The
    /// input supports either the enum or a bool for ergonomics.
    pub fn sync_checked(&mut self, checked: impl Into<CheckboxValue>) {
        self.inner.sync(checked.into());
    }

    /// Mark the checkbox as indeterminate regardless of the current ownership
    /// mode. This makes it easy to drive partial selection UIs without
    /// recreating the state machine.
    pub fn set_indeterminate(&mut self) {
        self.inner.sync(CheckboxValue::Indeterminate);
    }

    /// Clear the indeterminate flag, defaulting the checkbox back to `Off`.
    /// This helper mirrors Material's recommended pattern of treating
    /// indeterminate as a transient visual cue.
    pub fn clear_indeterminate(&mut self) {
        self.inner.sync(CheckboxValue::Off);
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
    pub fn toggle<F: FnOnce(CheckboxValue)>(&mut self, callback: F) {
        self.inner.toggle(callback);
    }

    /// Handle keyboard interaction. Space and Enter toggle the checkbox in line
    /// with the ARIA authoring practices.
    pub fn on_key<F: FnOnce(CheckboxValue)>(&mut self, key: ControlKey, callback: F) {
        self.inner.on_key(key, callback);
    }

    /// Return ARIA metadata and data attributes describing the checkbox.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        // The allocation size intentionally matches the number of pushed
        // attributes to avoid runtime reallocations in hot paths.
        let mut attrs = Vec::with_capacity(7);
        attrs.push(("role", aria::role_checkbox().into()));
        let (k, v) = aria::aria_checked(self.checked().into());
        attrs.push((k, v.into()));
        let (k, v) = aria::aria_disabled(self.disabled());
        attrs.push((k, v.into()));
        attrs.push((
            "tabindex",
            if self.disabled() { "-1" } else { "0" }.to_string(),
        ));
        attrs.push((
            "data-checked",
            if self.is_checked() { "true" } else { "false" }.to_string(),
        ));
        attrs.push((
            "data-indeterminate",
            if self.is_indeterminate() {
                "true"
            } else {
                "false"
            }
            .to_string(),
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
    use proptest::prelude::*;

    fn expected_state_after_toggles(initial: CheckboxValue, toggles: u32) -> CheckboxValue {
        let mut state = initial;
        for _ in 0..toggles {
            state = state.toggled();
        }
        state
    }

    #[test]
    fn uncontrolled_toggle_updates_state() {
        let mut state = CheckboxState::uncontrolled(false, false);
        state.toggle(|_| {});
        assert!(state.is_checked());
    }

    #[test]
    fn controlled_toggle_only_notifies() {
        let mut state = CheckboxState::controlled(false, false);
        let mut received = None;
        state.toggle(|checked| received = Some(checked));
        assert!(!state.is_checked());
        assert_eq!(received, Some(CheckboxValue::On));
    }

    #[test]
    fn keyboard_space_invokes_toggle() {
        let mut state = CheckboxState::uncontrolled(false, false);
        state.on_key(ControlKey::Space, |_| {});
        assert!(state.is_checked());
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
    fn indeterminate_helpers_roundtrip() {
        let mut state = CheckboxState::uncontrolled(false, false);
        state.set_indeterminate();
        assert!(state.is_indeterminate());
        state.clear_indeterminate();
        assert!(!state.is_indeterminate());
        assert!(!state.is_checked());
    }

    #[test]
    fn aria_attributes_reflect_state() {
        let mut state = CheckboxState::uncontrolled(false, CheckboxValue::Indeterminate);
        state.focus();
        let attrs = state.aria_attributes();
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "checkbox"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-checked" && v == "mixed"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-indeterminate" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-focus-visible" && v == "true"));
    }

    proptest! {
        #[test]
        fn uncontrolled_toggle_matches_enum(
            start in prop_oneof![Just(CheckboxValue::Off), Just(CheckboxValue::On), Just(CheckboxValue::Indeterminate)],
            toggles in 0u32..20
        ) {
            let mut state = CheckboxState::uncontrolled(false, start);
            for _ in 0..toggles {
                state.toggle(|_| {});
            }
            prop_assert_eq!(state.checked(), expected_state_after_toggles(start, toggles));
        }

        #[test]
        fn controlled_toggle_does_not_mutate_internal_state(
            start in prop_oneof![Just(CheckboxValue::Off), Just(CheckboxValue::On), Just(CheckboxValue::Indeterminate)],
        ) {
            let mut state = CheckboxState::controlled(false, start);
            state.toggle(|_| {});
            prop_assert_eq!(state.checked(), start);
        }
    }
}
