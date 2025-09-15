//! State machine managing the interactive behavior of a generic button.
//!
//! The goal is to keep framework specific code free from business logic.
//! Each adapter simply renders the current state produced here.

use crate::aria;
use std::time::{Duration, Instant};

/// Finite state machine capturing disabled and pressed state while also
/// throttling rapid clicks to prevent accidental double submits.
#[derive(Debug, Default)]
pub struct ButtonState {
    disabled: bool,
    throttle: Option<Duration>,
    last_click: Option<Instant>,
    pressed: bool,
}

impl ButtonState {
    /// Construct a new state machine.  `throttle` limits how frequently the
    /// `press` method will invoke its callback.
    pub fn new(disabled: bool, throttle: Option<Duration>) -> Self {
        Self {
            disabled,
            throttle,
            last_click: None,
            pressed: false,
        }
    }

    /// Returns whether the button is currently disabled.
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Mutably toggle the disabled flag.
    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }

    /// Execute `f` if the button is enabled and not throttled.
    ///
    /// The `pressed` flag is set for the duration of the callback allowing
    /// adapters to reflect the active state in their rendering.
    pub fn press<F: FnOnce(&mut Self)>(&mut self, f: F) {
        if self.disabled {
            return;
        }
        if let Some(limit) = self.throttle {
            if let Some(last) = self.last_click {
                if last.elapsed() < limit {
                    return;
                }
            }
            self.last_click = Some(Instant::now());
        }
        // Toggle the pressed flag and invoke the callback so external
        // observers can react to the state change.
        self.pressed = !self.pressed;
        f(self);
    }

    /// Access the `aria-pressed` tuple alongside the `role` attribute.
    pub fn aria_attributes(&self) -> [(&'static str, &'static str); 2] {
        [
            ("role", aria::role_button()),
            aria::aria_pressed(self.pressed),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn respects_disabled_flag() {
        let mut state = ButtonState::new(true, None);
        let mut called = false;
        state.press(|_| called = true);
        assert!(!called, "callback should not run when disabled");
    }

    #[test]
    fn throttles_rapid_presses() {
        let throttle = Duration::from_millis(50);
        let mut state = ButtonState::new(false, Some(throttle));
        let mut count = 0;
        state.press(|_| count += 1);
        state.press(|_| count += 1);
        assert_eq!(count, 1, "second press should be ignored due to throttle");
    }

    #[test]
    fn toggles_pressed_flag() {
        let mut state = ButtonState::new(false, None);
        assert!(!state.pressed, "initial state should be unpressed");
        state.press(|_| {});
        assert!(state.pressed, "press toggles to pressed");
        state.press(|_| {});
        assert!(!state.pressed, "press again toggles back to unpressed");
    }
}
