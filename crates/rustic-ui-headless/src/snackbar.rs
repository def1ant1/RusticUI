//! Queue based snackbar state machine shared between Material and Joy layers.
//!
//! The goal is to provide deterministic automation hooks for transient feedback
//! components.  The state machine exposes precise control over queuing,
//! auto-hide timing, pausing, and manual dismissal so large applications can run
//! integration tests without brittle timeouts sprinkled throughout UI code.

use crate::timing::{Clock, SystemClock, Timer};
use std::collections::VecDeque;
use std::time::Duration;

/// Configuration describing how the snackbar behaves.
#[derive(Debug, Clone)]
pub struct SnackbarConfig {
    /// Duration each message should remain visible.
    pub auto_hide: Duration,
    /// Maximum number of queued messages (excluding the currently visible one).
    pub max_queue: usize,
}

impl SnackbarConfig {
    /// Enterprise defaults mirroring the Material/Joy design language.
    pub fn enterprise_defaults() -> Self {
        Self {
            auto_hide: Duration::from_millis(4000),
            max_queue: 3,
        }
    }
}

impl Default for SnackbarConfig {
    fn default() -> Self {
        Self::enterprise_defaults()
    }
}

/// Message entry managed by the snackbar queue.
#[derive(Debug, Clone)]
pub struct SnackbarMessage<T> {
    /// Monotonically increasing identifier useful for automation.
    pub id: u64,
    /// Custom payload forwarded to adapters.
    pub payload: T,
}

/// Change notification emitted from snackbar transitions.
#[derive(Debug, Clone)]
pub struct SnackbarChange<T> {
    /// Message that became visible after the transition (if any).
    pub shown: Option<SnackbarMessage<T>>,
    /// Message that was dismissed as part of the transition (if any).
    pub dismissed: Option<SnackbarMessage<T>>,
}

impl<T: Clone> SnackbarChange<T> {
    fn merge(mut self, other: SnackbarChange<T>) -> SnackbarChange<T> {
        if other.shown.is_some() {
            self.shown = other.shown;
        }
        if other.dismissed.is_some() {
            self.dismissed = other.dismissed;
        }
        self
    }
}

impl<T> Default for SnackbarChange<T> {
    fn default() -> Self {
        Self {
            shown: None,
            dismissed: None,
        }
    }
}

/// Headless snackbar state machine.
#[derive(Debug, Clone)]
pub struct SnackbarState<T, C: Clock = SystemClock> {
    clock: C,
    config: SnackbarConfig,
    queue: VecDeque<SnackbarMessage<T>>,
    current: Option<SnackbarMessage<T>>,
    timer: Timer<C>,
    next_id: u64,
    paused_remaining: Option<Duration>,
}

impl<T: Clone> SnackbarState<T, SystemClock> {
    /// Construct a snackbar bound to the system clock.
    pub fn new(config: SnackbarConfig) -> Self {
        Self::with_clock(SystemClock, config)
    }
}

impl<T: Clone, C: Clock> SnackbarState<T, C> {
    /// Construct a snackbar bound to an arbitrary clock (mock clocks for tests).
    pub fn with_clock(clock: C, config: SnackbarConfig) -> Self {
        Self {
            clock,
            config,
            queue: VecDeque::new(),
            current: None,
            timer: Timer::new(),
            next_id: 0,
            paused_remaining: None,
        }
    }

    /// Returns the currently visible message (if any).
    #[inline]
    pub fn current(&self) -> Option<&SnackbarMessage<T>> {
        self.current.as_ref()
    }

    /// Returns how many messages are waiting in the queue.
    #[inline]
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Returns whether the snackbar is idle.
    #[inline]
    pub fn is_idle(&self) -> bool {
        self.current.is_none() && self.queue.is_empty()
    }

    /// Enqueue a new message.
    pub fn enqueue(&mut self, payload: T) -> SnackbarChange<T> {
        let message = SnackbarMessage {
            id: self.next_id,
            payload,
        };
        self.next_id = self.next_id.wrapping_add(1);
        if self.current.is_some() {
            if self.queue.len() >= self.config.max_queue {
                self.queue.pop_front();
            }
            self.queue.push_back(message);
            SnackbarChange::default()
        } else {
            self.show_message(message)
        }
    }

    /// Manually dismiss the current message.
    pub fn dismiss_current(&mut self) -> SnackbarChange<T> {
        self.dismiss_internal()
    }

    /// Remove a specific queued message by identifier.
    pub fn remove_queued(&mut self, id: u64) {
        if let Some(position) = self.queue.iter().position(|msg| msg.id == id) {
            self.queue.remove(position);
        }
    }

    /// Pause the auto-hide timer preserving the remaining duration.
    pub fn pause(&mut self) {
        if self.current.is_none() || self.paused_remaining.is_some() {
            return;
        }
        if let Some(remaining) = self.timer.remaining(&self.clock) {
            self.paused_remaining = Some(remaining);
            self.timer.cancel();
        }
    }

    /// Resume the auto-hide timer if it was paused.
    pub fn resume(&mut self) {
        if let Some(remaining) = self.paused_remaining.take() {
            if remaining > Duration::ZERO {
                self.timer.schedule(&self.clock, remaining);
            }
        }
    }

    /// Advance the internal clock and process timeouts.
    pub fn tick(&mut self) -> SnackbarChange<T> {
        if self.timer.fire_if_due(&self.clock) {
            self.dismiss_internal()
        } else {
            SnackbarChange::default()
        }
    }

    fn show_message(&mut self, message: SnackbarMessage<T>) -> SnackbarChange<T> {
        self.current = Some(message.clone());
        self.paused_remaining = None;
        if self.config.auto_hide > Duration::ZERO {
            self.timer.schedule(&self.clock, self.config.auto_hide);
        }
        SnackbarChange {
            shown: Some(message),
            ..SnackbarChange::default()
        }
    }

    fn dismiss_internal(&mut self) -> SnackbarChange<T> {
        if let Some(current) = self.current.take() {
            self.timer.cancel();
            let mut change = SnackbarChange {
                dismissed: Some(current.clone()),
                ..SnackbarChange::default()
            };
            if let Some(next) = self.queue.pop_front() {
                change = change.merge(self.show_message(next));
            }
            change
        } else {
            SnackbarChange::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timing::MockClock;

    #[test]
    fn enqueue_immediately_shows_first_message() {
        let mut state = SnackbarState::new(SnackbarConfig::enterprise_defaults());
        let change = state.enqueue("hello".to_string());
        assert_eq!(
            change.shown.as_ref().map(|m| m.payload.clone()),
            Some("hello".to_string())
        );
        assert!(state.current().is_some());
    }

    #[test]
    fn auto_hide_advances_queue() {
        let clock = MockClock::new();
        let mut state = SnackbarState::with_clock(
            clock.clone(),
            SnackbarConfig {
                auto_hide: Duration::from_millis(100),
                max_queue: 5,
            },
        );
        state.enqueue("first");
        state.enqueue("second");
        clock.advance(Duration::from_millis(120));
        let change = state.tick();
        assert_eq!(change.dismissed.unwrap().payload, "first");
        assert_eq!(change.shown.unwrap().payload, "second");
    }

    #[test]
    fn pause_and_resume_preserves_timeout() {
        let clock = MockClock::new();
        let mut state = SnackbarState::with_clock(
            clock.clone(),
            SnackbarConfig {
                auto_hide: Duration::from_millis(200),
                max_queue: 5,
            },
        );
        state.enqueue("first");
        state.pause();
        clock.advance(Duration::from_millis(400));
        assert!(state.tick().dismissed.is_none());
        state.resume();
        clock.advance(Duration::from_millis(200));
        let change = state.tick();
        assert_eq!(change.dismissed.unwrap().payload, "first");
    }
}
