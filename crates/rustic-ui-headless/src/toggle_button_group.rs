//! Toggle button group state machine shared across Joy and Material adapters.
//!
//! Joy exposes both exclusive (single selection) and non-exclusive (multi
//! selection) toggle groups.  This headless implementation keeps track of which
//! buttons are pressed, enforces exclusivity when requested, and exposes
//! declarative change metadata so adapters can update their DOM without
//! reimplementing the selection bookkeeping.

use crate::aria;

/// Configuration describing the toggle group.
#[derive(Debug, Clone)]
pub struct ToggleButtonGroupConfig {
    /// Number of toggle buttons in the group.
    pub button_count: usize,
    /// When `true` only a single button may be active at a time.
    pub exclusive: bool,
    /// Indices that should start pressed.
    pub initial_pressed: Vec<usize>,
}

impl ToggleButtonGroupConfig {
    /// Enterprise defaults aligned with Joy’s design guidance.
    pub fn enterprise_defaults(button_count: usize) -> Self {
        Self {
            button_count,
            exclusive: false,
            initial_pressed: Vec::new(),
        }
    }
}

impl Default for ToggleButtonGroupConfig {
    fn default() -> Self {
        Self::enterprise_defaults(0)
    }
}

/// Change notification emitted after toggle interactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleButtonGroupChange {
    /// Tuple describing which button changed and whether it is now pressed.
    pub toggled: Option<(usize, bool)>,
    /// Snapshot of pressed button indices after the transition.
    pub pressed: Vec<usize>,
}

impl ToggleButtonGroupChange {
    fn new(toggled: Option<(usize, bool)>, pressed: Vec<usize>) -> Self {
        Self { toggled, pressed }
    }
}

#[derive(Debug, Clone)]
struct ButtonState {
    pressed: bool,
    disabled: bool,
}

impl ButtonState {
    fn new() -> Self {
        Self {
            pressed: false,
            disabled: false,
        }
    }
}

/// Headless toggle group state machine.
#[derive(Debug, Clone)]
pub struct ToggleButtonGroupState {
    buttons: Vec<ButtonState>,
    exclusive: bool,
}

impl ToggleButtonGroupState {
    /// Construct a new toggle group.
    pub fn new(config: ToggleButtonGroupConfig) -> Self {
        let mut buttons = Vec::with_capacity(config.button_count);
        buttons.resize_with(config.button_count, ButtonState::new);
        let mut state = Self {
            buttons,
            exclusive: config.exclusive,
        };
        state.sync_pressed(&config.initial_pressed);
        state
    }

    /// Returns how many buttons are tracked.
    #[inline]
    pub fn button_count(&self) -> usize {
        self.buttons.len()
    }

    /// Returns whether a specific button is pressed.
    #[inline]
    pub fn is_pressed(&self, index: usize) -> bool {
        self.buttons
            .get(index)
            .map(|btn| btn.pressed)
            .unwrap_or(false)
    }

    /// Returns whether a specific button is disabled.
    #[inline]
    pub fn is_disabled(&self, index: usize) -> bool {
        self.buttons
            .get(index)
            .map(|btn| btn.disabled)
            .unwrap_or(true)
    }

    /// Update the disabled flag for a button.
    pub fn set_disabled(&mut self, index: usize, disabled: bool) {
        if let Some(button) = self.buttons.get_mut(index) {
            button.disabled = disabled;
            if disabled {
                button.pressed = false;
            }
        }
    }

    /// Toggle a button.
    pub fn toggle(&mut self, index: usize) -> ToggleButtonGroupChange {
        if index >= self.buttons.len() || self.is_disabled(index) {
            return ToggleButtonGroupChange::new(None, self.pressed_indices());
        }
        if self.exclusive {
            if self.buttons[index].pressed {
                self.buttons[index].pressed = false;
                return ToggleButtonGroupChange::new(Some((index, false)), self.pressed_indices());
            }
            for (i, button) in self.buttons.iter_mut().enumerate() {
                if i != index {
                    button.pressed = false;
                }
            }
            self.buttons[index].pressed = true;
            ToggleButtonGroupChange::new(Some((index, true)), self.pressed_indices())
        } else {
            self.buttons[index].pressed = !self.buttons[index].pressed;
            ToggleButtonGroupChange::new(
                Some((index, self.buttons[index].pressed)),
                self.pressed_indices(),
            )
        }
    }

    /// Force a button into a pressed/unpressed state.
    pub fn set_pressed(&mut self, index: usize, pressed: bool) -> ToggleButtonGroupChange {
        if index >= self.buttons.len() || self.is_disabled(index) {
            return ToggleButtonGroupChange::new(None, self.pressed_indices());
        }
        if self.exclusive && pressed {
            for (i, button) in self.buttons.iter_mut().enumerate() {
                button.pressed = i == index;
            }
        } else {
            self.buttons[index].pressed = pressed;
        }
        ToggleButtonGroupChange::new(
            Some((index, self.buttons[index].pressed)),
            self.pressed_indices(),
        )
    }

    /// Synchronise pressed buttons from an external controller.
    pub fn sync_pressed(&mut self, pressed: &[usize]) {
        for button in &mut self.buttons {
            button.pressed = false;
        }
        if self.exclusive {
            if let Some(first) = pressed
                .iter()
                .copied()
                .find(|idx| *idx < self.buttons.len())
            {
                if let Some(button) = self.buttons.get_mut(first) {
                    button.pressed = true;
                }
            }
            return;
        }
        for &index in pressed {
            if let Some(button) = self.buttons.get_mut(index) {
                button.pressed = true;
            }
        }
    }

    /// Adjust the number of tracked buttons.
    pub fn set_button_count(&mut self, count: usize) {
        self.buttons.resize_with(count, ButtonState::new);
        if self.exclusive {
            // Ensure only a single button remains pressed.
            let mut first = None;
            for (index, button) in self.buttons.iter_mut().enumerate() {
                if button.pressed {
                    if first.is_some() {
                        button.pressed = false;
                    } else {
                        first = Some(index);
                    }
                }
            }
        }
    }

    /// Returns the indices of pressed buttons.
    pub fn pressed_indices(&self) -> Vec<usize> {
        self.buttons
            .iter()
            .enumerate()
            .filter_map(|(index, button)| button.pressed.then_some(index))
            .collect()
    }

    /// Compute ARIA attributes for a toggle button element.
    pub fn button_attributes(&self, index: usize) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", aria::role_button().into()));
        let (pressed_key, pressed_value) = aria::aria_pressed(self.is_pressed(index));
        attrs.push((pressed_key, pressed_value.into()));
        aria::extend_disabled_attributes(&mut attrs, self.is_disabled(index));
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusive_group_allows_single_pressed_button() {
        let mut state = ToggleButtonGroupState::new(ToggleButtonGroupConfig {
            button_count: 3,
            exclusive: true,
            initial_pressed: vec![1],
        });
        let change = state.toggle(2);
        assert_eq!(change.toggled, Some((2, true)));
        assert_eq!(change.pressed, vec![2]);
    }

    #[test]
    fn non_exclusive_group_supports_multiple_buttons() {
        let mut state = ToggleButtonGroupState::new(ToggleButtonGroupConfig {
            button_count: 3,
            exclusive: false,
            initial_pressed: vec![],
        });
        state.toggle(0);
        let change = state.toggle(2);
        assert_eq!(change.pressed, vec![0, 2]);
    }

    #[test]
    fn disabled_buttons_ignore_toggles() {
        let mut state =
            ToggleButtonGroupState::new(ToggleButtonGroupConfig::enterprise_defaults(2));
        state.set_disabled(1, true);
        let change = state.toggle(1);
        assert_eq!(change.toggled, None);
        assert!(state.pressed_indices().is_empty());
    }
}
