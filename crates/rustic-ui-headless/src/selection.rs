//! Shared building blocks for list based controls.
//!
//! The select and menu state machines both require typeahead handling and a
//! consistent approach to controlled/uncontrolled state management.  Keeping
//! the primitives centralized avoids duplicating the bookkeeping logic and
//! provides a single location for future components such as autocomplete to
//! reuse.

use std::time::{Duration, Instant};

/// Describes whether a piece of state is owned by the component or by an
/// external controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlStrategy {
    /// Controlled widgets only emit intents through callbacks and expect their
    /// parent to call a synchronization method with the latest value.
    Controlled,
    /// Uncontrolled widgets mutate the internal field immediately and still
    /// emit callbacks so observers remain informed.
    Uncontrolled,
}

impl ControlStrategy {
    #[inline]
    pub(crate) fn is_controlled(self) -> bool {
        matches!(self, Self::Controlled)
    }
}

/// Rolling buffer used to implement typeahead navigation in list based
/// controls.
#[derive(Debug, Clone)]
pub(crate) struct TypeaheadBuffer {
    timeout: Duration,
    last_key: Option<Instant>,
    value: String,
}

impl TypeaheadBuffer {
    /// Construct a new buffer with a configurable timeout.  WAI-ARIA authoring
    /// practices recommend clearing the query after roughly one second to keep
    /// the interaction feeling responsive even when users pause between key
    /// strokes.
    pub(crate) fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            last_key: None,
            value: String::new(),
        }
    }

    /// Clears the buffer immediately.  This is typically called whenever the
    /// popover closes so the next open interaction starts from a clean slate.
    pub(crate) fn reset(&mut self) {
        self.value.clear();
        self.last_key = None;
    }

    /// Push a new character into the rolling buffer and return the updated
    /// query slice.  Whenever the elapsed time exceeds the configured timeout
    /// the buffer is cleared before appending the next character.
    pub(crate) fn push(&mut self, ch: char) -> &str {
        let now = Instant::now();
        if let Some(previous) = self.last_key {
            if now.duration_since(previous) > self.timeout {
                self.value.clear();
            }
        }
        self.last_key = Some(now);
        self.value.push(ch);
        &self.value
    }
}

/// Wraps an index change inside an option count, returning the new highlighted
/// index.  When the list is empty the function returns `None`.
#[inline]
pub(crate) fn wrap_index(current: Option<usize>, delta: isize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let base = current.unwrap_or(0);
    let len_isize = len as isize;
    let mut next = (base as isize + delta) % len_isize;
    if next < 0 {
        next += len_isize;
    }
    Some(next as usize)
}

/// Clamp a provided index to the current list bounds returning `None` whenever
/// the index falls outside the range.
#[inline]
pub(crate) fn clamp_index(index: Option<usize>, len: usize) -> Option<usize> {
    index.filter(|value| *value < len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn typeahead_rolls_over_after_timeout() {
        let mut buffer = TypeaheadBuffer::new(Duration::from_millis(1));
        buffer.push('a');
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(buffer.push('b'), "b");
    }

    #[test]
    fn wrap_index_handles_negative_deltas() {
        assert_eq!(wrap_index(Some(0), -1, 5), Some(4));
    }

    #[test]
    fn clamp_index_filters_out_of_range_values() {
        assert_eq!(clamp_index(Some(10), 3), None);
    }
}
