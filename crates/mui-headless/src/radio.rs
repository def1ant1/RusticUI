//! State machine powering Material radio groups.
//!
//! The group tracks both selection and roving focus so keyboard navigation
//! behaves consistently across adapters.  Controlled and uncontrolled usage
//! patterns are supported via the [`select`](RadioGroupState::select) API which
//! notifies callers without assuming how UI frameworks manage state.

use crate::aria;
use crate::interaction::ControlKey;
use crate::toggle::ToggleMode;

/// Orientation hint used to determine which arrow keys move the active option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioOrientation {
    /// Layout options horizontally. Left/Right arrows move between items.
    Horizontal,
    /// Layout options vertically. Up/Down arrows move between items.
    Vertical,
}

impl RadioOrientation {
    fn as_aria(self) -> &'static str {
        match self {
            RadioOrientation::Horizontal => "horizontal",
            RadioOrientation::Vertical => "vertical",
        }
    }
}

/// State machine owning a list of radio options.
#[derive(Debug, Clone)]
pub struct RadioGroupState {
    options: Vec<String>,
    mode: ToggleMode,
    disabled: bool,
    orientation: RadioOrientation,
    focus_visible: Option<usize>,
    selected: Option<usize>,
}

impl RadioGroupState {
    /// Create a controlled radio group.
    pub fn controlled(
        options: impl Into<Vec<String>>,
        disabled: bool,
        orientation: RadioOrientation,
        selected: Option<usize>,
    ) -> Self {
        Self {
            options: options.into(),
            mode: ToggleMode::Controlled,
            disabled,
            orientation,
            focus_visible: None,
            selected,
        }
    }

    /// Create an uncontrolled radio group where selection is managed internally.
    pub fn uncontrolled(
        options: impl Into<Vec<String>>,
        disabled: bool,
        orientation: RadioOrientation,
        default_selected: Option<usize>,
    ) -> Self {
        Self {
            options: options.into(),
            mode: ToggleMode::Uncontrolled,
            disabled,
            orientation,
            focus_visible: None,
            selected: default_selected,
        }
    }

    /// Access the configured options.
    pub fn options(&self) -> &[String] {
        &self.options
    }

    /// Number of options.
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Whether the group is disabled.
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Update the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Current orientation.
    pub fn orientation(&self) -> RadioOrientation {
        self.orientation
    }

    /// Adjust the orientation.
    pub fn set_orientation(&mut self, orientation: RadioOrientation) {
        self.orientation = orientation;
    }

    /// Currently selected index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Synchronize the selected index with an external value.
    pub fn sync_selected(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Whether the group is controlled.
    pub fn is_controlled(&self) -> bool {
        matches!(self.mode, ToggleMode::Controlled)
    }

    /// Apply focus to an option.
    pub fn focus(&mut self, index: usize) {
        if self.len() == 0 {
            return;
        }
        self.focus_visible = Some(index.min(self.len() - 1));
    }

    /// Clear focus.
    pub fn blur(&mut self) {
        self.focus_visible = None;
    }

    /// Inspect the focused index if present.
    pub fn focus_visible_index(&self) -> Option<usize> {
        self.focus_visible
    }

    /// Select the given option.
    pub fn select<F: FnOnce(usize)>(&mut self, index: usize, callback: F) {
        if self.disabled || index >= self.len() {
            return;
        }
        if self.mode == ToggleMode::Uncontrolled {
            self.selected = Some(index);
        }
        callback(index);
    }

    /// Handle keyboard input according to the ARIA radio group guidance.
    pub fn on_key<F: FnOnce(usize)>(&mut self, key: ControlKey, callback: F) {
        if self.disabled || self.len() == 0 {
            return;
        }
        let mut focus_index = self
            .focus_visible
            .or(self.selected)
            .unwrap_or(0)
            .min(self.len() - 1);

        match key {
            ControlKey::Space | ControlKey::Enter => {
                self.select(focus_index, callback);
            }
            ControlKey::Home => {
                focus_index = 0;
                self.focus(focus_index);
                self.select(focus_index, callback);
            }
            ControlKey::End => {
                focus_index = self.len() - 1;
                self.focus(focus_index);
                self.select(focus_index, callback);
            }
            key if self.should_advance(key) => {
                focus_index = (focus_index + 1) % self.len();
                self.focus(focus_index);
                self.select(focus_index, callback);
            }
            key if self.should_reverse(key) => {
                focus_index = if focus_index == 0 {
                    self.len() - 1
                } else {
                    focus_index - 1
                };
                self.focus(focus_index);
                self.select(focus_index, callback);
            }
            _ => {}
        }
    }

    fn should_advance(&self, key: ControlKey) -> bool {
        match self.orientation {
            RadioOrientation::Horizontal => {
                key == ControlKey::ArrowRight || key == ControlKey::ArrowDown
            }
            RadioOrientation::Vertical => key == ControlKey::ArrowDown,
        }
    }

    fn should_reverse(&self, key: ControlKey) -> bool {
        match self.orientation {
            RadioOrientation::Horizontal => {
                key == ControlKey::ArrowLeft || key == ControlKey::ArrowUp
            }
            RadioOrientation::Vertical => key == ControlKey::ArrowUp,
        }
    }

    /// ARIA metadata for the group container.
    pub fn group_aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(3);
        attrs.push(("role", "radiogroup".into()));
        attrs.push(("aria-orientation", self.orientation.as_aria().into()));
        let (k, v) = aria::aria_disabled(self.disabled);
        attrs.push((k, v.into()));
        attrs
    }

    /// ARIA metadata for a specific option.
    pub fn option_aria_attributes(&self, index: usize) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", aria::role_radio().into()));
        let checked = self.selected == Some(index);
        let (k, v) = aria::aria_checked(checked);
        attrs.push((k, v.into()));
        let (k, v) = aria::aria_disabled(self.disabled);
        attrs.push((k, v.into()));
        attrs.push((
            "tabindex",
            if checked || self.selected.is_none() && index == 0 {
                "0"
            } else {
                "-1"
            }
            .to_string(),
        ));
        attrs.push((
            "data-checked",
            if checked { "true" } else { "false" }.to_string(),
        ));
        attrs.push((
            "data-focus-visible",
            if self.focus_visible == Some(index) {
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
    fn uncontrolled_select_updates_state() {
        let mut state = RadioGroupState::uncontrolled(
            vec!["One".into(), "Two".into()],
            false,
            RadioOrientation::Horizontal,
            None,
        );
        state.select(1, |_| {});
        assert_eq!(state.selected_index(), Some(1));
    }

    #[test]
    fn controlled_select_notifies_only() {
        let mut state = RadioGroupState::controlled(
            vec!["One".into(), "Two".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let mut received = None;
        state.select(1, |idx| received = Some(idx));
        assert_eq!(state.selected_index(), Some(0));
        assert_eq!(received, Some(1));
    }

    #[test]
    fn keyboard_navigation_wraps() {
        let mut state = RadioGroupState::uncontrolled(
            vec!["One".into(), "Two".into(), "Three".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        state.on_key(ControlKey::ArrowRight, |_| {});
        assert_eq!(state.selected_index(), Some(1));
        state.on_key(ControlKey::ArrowRight, |_| {});
        assert_eq!(state.selected_index(), Some(2));
        state.on_key(ControlKey::ArrowRight, |_| {});
        assert_eq!(state.selected_index(), Some(0));
    }

    #[test]
    fn orientation_controls_keys() {
        let mut state = RadioGroupState::uncontrolled(
            vec!["One".into(), "Two".into()],
            false,
            RadioOrientation::Vertical,
            Some(0),
        );
        state.on_key(ControlKey::ArrowDown, |_| {});
        assert_eq!(state.selected_index(), Some(1));
        state.on_key(ControlKey::ArrowUp, |_| {});
        assert_eq!(state.selected_index(), Some(0));
    }

    #[test]
    fn group_attributes_include_orientation() {
        let state = RadioGroupState::controlled(
            vec!["One".into()],
            false,
            RadioOrientation::Vertical,
            Some(0),
        );
        let attrs = state.group_aria_attributes();
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "radiogroup"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-orientation" && v == "vertical"));
    }

    #[test]
    fn option_attributes_reflect_focus() {
        let mut state = RadioGroupState::uncontrolled(
            vec!["One".into(), "Two".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        state.focus(1);
        let attrs = state.option_aria_attributes(1);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-focus-visible" && v == "true"));
    }
}
