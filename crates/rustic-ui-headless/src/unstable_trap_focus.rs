#![deny(missing_docs)]
//! Experimental focus-loop instrumentation utilities.
//!
//! This module wraps [`FocusTrapState`](crate::focus_trap::FocusTrapState) with
//! additional analytics so enterprise overlays can evaluate alternative
//! looping strategies before the API stabilizes.  The state machine mirrors the
//! deterministic behaviour of [`crate::focus_trap`] while recording when
//! keyboard navigation wraps from the last tabbable element back to the first
//! (and vice versa).  The extra metadata is intentionally verbose so downstream
//! automation can centralize reporting without scattering bespoke event hooks
//! throughout every renderer.
//!
//! # Risk profile
//!
//! The types in this module are gated behind the `unstable` feature flag and
//! may change _without_ a semver bump.  Consumers should treat the API surface
//! as experimental scaffolding: wire it into analytics pipelines to observe how
//! focus wrapping behaves in production, but keep integration boundaries thin
//! so migrating back to [`crate::focus_trap`] requires minimal work.  Once the
//! telemetry validates the design the instrumentation hooks will either merge
//! into the stable focus trap or be retired in favour of a different
//! abstraction.

use crate::focus_trap::{FocusDisposition, FocusTrapSentinelAttributes, FocusTrapState};
use crate::interaction::ControlKey;
use std::fmt;
use std::sync::Arc;

/// Direction keyboard input travelled when a loop event was observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusLoopDirection {
    /// Focus advanced forward (e.g. `Tab`, `ArrowRight`).
    Forward,
    /// Focus moved backward (e.g. `Shift+Tab`, `ArrowLeft`).
    Backward,
}

impl FocusLoopDirection {
    /// Returns a string representation suitable for telemetry attributes.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forward => "forward",
            Self::Backward => "backward",
        }
    }
}

/// Captures a single focus loop event emitted by [`UnstableFocusTrapState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusLoopEvent {
    /// Direction navigation travelled when the loop occurred.
    pub direction: FocusLoopDirection,
    /// Identifier that held focus before wrapping.
    pub from: String,
    /// Identifier that received focus after wrapping.
    pub to: String,
    /// Total tabbable nodes registered with the trap at the time of the loop.
    pub total_focusables: usize,
    /// Analytics tag mirrored from the underlying [`FocusTrapState`].
    pub analytics_tag: Option<String>,
    /// Monotonic counter describing the occurrence index of this loop event.
    pub occurrence: usize,
}

/// Convenience alias used by loop observers.
type LoopObserver = dyn Fn(&FocusLoopEvent) + Send + Sync + 'static;

/// Focus trap wrapper that records when navigation wraps around.
#[derive(Clone)]
pub struct UnstableFocusTrapState {
    inner: FocusTrapState,
    focusables: Vec<String>,
    last_focus: Option<String>,
    loop_events: Vec<FocusLoopEvent>,
    observer: Option<Arc<LoopObserver>>,
}

impl UnstableFocusTrapState {
    /// Construct a new experimental focus trap.
    pub fn new(loop_focus: bool) -> Self {
        Self {
            inner: FocusTrapState::new(loop_focus),
            focusables: Vec::new(),
            last_focus: None,
            loop_events: Vec::new(),
            observer: None,
        }
    }

    /// Returns whether the trap loops focus when reaching either edge.
    #[inline]
    pub fn loop_focus(&self) -> bool {
        self.inner.loop_focus()
    }

    /// Replace the list of focusables and reset instrumentation counters.
    pub fn set_focusables<I, S>(&mut self, focusables: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values: Vec<String> = focusables.into_iter().map(Into::into).collect();
        self.inner.set_focusables(values.clone());
        self.focusables = values;
        self.last_focus = None;
        self.loop_events.clear();
    }

    /// Returns the number of focusable elements tracked by the trap.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the trap currently manages no focusable nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Register which element currently holds focus.
    pub fn register_focus(&mut self, id: Option<&str>) {
        self.inner.register_focus(id);
        self.last_focus = id.map(|value| value.to_string());
    }

    /// Update the analytics tag mirrored onto sentinel attributes.
    pub fn set_analytics_tag(&mut self, tag: Option<impl Into<String>>) {
        self.inner.set_analytics_tag(tag);
    }

    /// Returns the analytics tag stored on the trap.
    #[inline]
    pub fn analytics_tag(&self) -> Option<&str> {
        self.inner.analytics_tag()
    }

    /// Process a control key, returning the next focus target when applicable.
    pub fn handle_key(&mut self, key: ControlKey) -> FocusDisposition<'_> {
        let loop_enabled = self.inner.loop_focus();
        let focusables_snapshot = self.focusables.clone();
        let analytics_tag_snapshot = self.inner.analytics_tag().map(ToString::to_string);
        let next_occurrence = self.loop_events.len() + 1;
        let before = self.last_focus.clone();
        let disposition = self.inner.handle_key(key);
        let mut recorded_event: Option<FocusLoopEvent> = None;
        if let FocusDisposition::Focus(next) = &disposition {
            let detection = Self::compute_loop_event(
                loop_enabled,
                &focusables_snapshot,
                before.as_deref(),
                next,
                key,
                analytics_tag_snapshot.as_deref(),
                next_occurrence,
            );
            self.last_focus = Some((*next).to_string());
            if let Some(event) = detection {
                self.loop_events.push(event.clone());
                recorded_event = Some(event);
            }
        }
        if let Some(event) = recorded_event {
            if let Some(observer) = &self.observer {
                observer(&event);
            }
        }
        disposition
    }

    /// Returns attributes for the start sentinel.
    #[inline]
    pub fn start_sentinel_attributes(&self) -> FocusTrapSentinelAttributes<'_> {
        self.inner.start_sentinel_attributes()
    }

    /// Returns attributes for the end sentinel.
    #[inline]
    pub fn end_sentinel_attributes(&self) -> FocusTrapSentinelAttributes<'_> {
        self.inner.end_sentinel_attributes()
    }

    /// Returns the inner [`FocusTrapState`] for integrations that only consume
    /// stable behaviours.
    #[inline]
    pub fn inner(&self) -> &FocusTrapState {
        &self.inner
    }

    /// Returns the recorded loop events in chronological order.
    #[inline]
    pub fn loop_events(&self) -> &[FocusLoopEvent] {
        &self.loop_events
    }

    /// Drains the recorded loop events, returning ownership to the caller.
    pub fn take_loop_events(&mut self) -> Vec<FocusLoopEvent> {
        std::mem::take(&mut self.loop_events)
    }

    /// Returns how many loop events have been recorded so far.
    #[inline]
    pub fn loop_event_count(&self) -> usize {
        self.loop_events.len()
    }

    /// Returns the most recent loop event when present.
    #[inline]
    pub fn last_loop_event(&self) -> Option<&FocusLoopEvent> {
        self.loop_events.last()
    }

    /// Attach or clear an observer invoked whenever a loop is recorded.
    pub fn set_loop_observer<F>(&mut self, observer: Option<F>)
    where
        F: Fn(&FocusLoopEvent) + Send + Sync + 'static,
    {
        self.observer = observer.map(|callback| Arc::new(callback) as Arc<LoopObserver>);
    }

    /// Consume the wrapper and return the underlying [`FocusTrapState`].
    #[inline]
    pub fn into_inner(self) -> FocusTrapState {
        self.inner
    }

    fn compute_loop_event(
        loop_enabled: bool,
        focusables: &[String],
        before: Option<&str>,
        after: &str,
        key: ControlKey,
        analytics_tag: Option<&str>,
        occurrence: usize,
    ) -> Option<FocusLoopEvent> {
        if !loop_enabled {
            return None;
        }

        let before = before?;
        if focusables.len() < 2 {
            return None;
        }

        let before_index = focusables.iter().position(|value| value == before)?;
        let after_index = focusables.iter().position(|value| value == after)?;

        let direction = match key {
            ControlKey::ArrowDown | ControlKey::ArrowRight => FocusLoopDirection::Forward,
            ControlKey::ArrowUp | ControlKey::ArrowLeft => FocusLoopDirection::Backward,
            _ => return None,
        };

        let loop_detected = match direction {
            FocusLoopDirection::Forward => before_index == focusables.len() - 1 && after_index == 0,
            FocusLoopDirection::Backward => {
                before_index == 0 && after_index == focusables.len() - 1
            }
        };

        if !loop_detected {
            return None;
        }

        Some(FocusLoopEvent {
            direction,
            from: before.to_string(),
            to: after.to_string(),
            total_focusables: focusables.len(),
            analytics_tag: analytics_tag.map(ToString::to_string),
            occurrence,
        })
    }
}

impl fmt::Debug for UnstableFocusTrapState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnstableFocusTrapState")
            .field("inner", &self.inner)
            .field("focusables", &self.focusables)
            .field("last_focus", &self.last_focus)
            .field("loop_events", &self.loop_events)
            .finish_non_exhaustive()
    }
}

impl Default for UnstableFocusTrapState {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_forward_loop_events() {
        let mut state = UnstableFocusTrapState::new(true);
        state.set_focusables(["first", "last"]);
        state.register_focus(Some("last"));
        let disposition = state.handle_key(ControlKey::ArrowRight);
        assert!(matches!(disposition, FocusDisposition::Focus("first")));
        let events = state.loop_events();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.direction, FocusLoopDirection::Forward);
        assert_eq!(event.from, "last");
        assert_eq!(event.to, "first");
        assert_eq!(event.total_focusables, 2);
        assert_eq!(event.occurrence, 1);
    }

    #[test]
    fn observer_notified_on_loop() {
        use std::sync::{Arc, Mutex};

        let mut state = UnstableFocusTrapState::new(true);
        state.set_focusables(["a", "b"]);
        state.register_focus(Some("a"));

        let directions: Arc<Mutex<Vec<FocusLoopDirection>>> = Arc::new(Mutex::new(Vec::new()));
        let watcher = Arc::clone(&directions);
        state.set_loop_observer(Some(move |event: &FocusLoopEvent| {
            watcher.lock().unwrap().push(event.direction);
        }));

        state.handle_key(ControlKey::ArrowLeft);

        let observed = directions.lock().unwrap();
        assert_eq!(observed.as_slice(), &[FocusLoopDirection::Backward]);
    }

    #[test]
    fn resets_loop_counters_on_focusable_swap() {
        let mut state = UnstableFocusTrapState::new(true);
        state.set_focusables(["one", "two"]);
        state.register_focus(Some("two"));
        state.handle_key(ControlKey::ArrowRight);
        assert_eq!(state.loop_event_count(), 1);

        state.set_focusables(["alpha", "beta", "gamma"]);
        assert_eq!(state.loop_event_count(), 0);
        assert!(state.last_loop_event().is_none());
    }
}
