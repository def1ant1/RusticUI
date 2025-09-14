//! Minimal time picker demonstrating the [`TimeAdapter`] abstraction.
//!
//! **Unstable:** This API is highly experimental and may change with little
//! notice.  It focuses purely on state management to keep compile times
//! down while we gather community feedback.

use crate::adapters::TimeAdapter;

/// Very small time picker that tracks a single selected time value.
/// In a full implementation this would handle user input, validation and
/// edge cases like timezone management.  Keeping the struct lean allows
/// tests to run quickly and shows how the adapter can be leveraged.
pub struct TimePicker<A: TimeAdapter> {
    /// Adapter powering all time math and formatting.
    pub adapter: A,
    /// Currently selected time.
    pub selected: A::Time,
}

impl<A: TimeAdapter> TimePicker<A> {
    /// Creates a new picker starting at the current time.
    pub fn new(adapter: A) -> Self {
        let now = adapter.now();
        Self {
            adapter,
            selected: now,
        }
    }

    /// Advances the currently selected time by the specified number of
    /// minutes.  Negative values move backwards.
    pub fn increment(&mut self, minutes: i32) {
        self.selected = self.adapter.add_minutes(&self.selected, minutes);
    }
}
