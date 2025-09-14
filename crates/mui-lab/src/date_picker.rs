//! Minimal date picker used for tests and as a reference implementation.
//! Real applications are expected to build richer UIs on top of the core
//! logic showcased here.

use crate::adapters::DateAdapter;

/// Keyboard keys handled by the picker.  Limited to left/right for
/// demonstration purposes.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Left,
    Right,
}

/// Extremely small date picker that only tracks the selected date.
/// In a real widget this would manage focus, overlays and rendering but
/// here we keep it intentionally tiny so tests run fast and contributors
/// can understand the code quickly.
pub struct DatePicker<A: DateAdapter> {
    /// Adapter powering all date math and formatting.
    pub adapter: A,
    /// Currently selected date.
    pub selected: A::Date,
}

impl<A: DateAdapter> DatePicker<A> {
    /// Creates a new picker starting at today's date.
    pub fn new(adapter: A) -> Self {
        let today = adapter.today();
        Self {
            adapter,
            selected: today,
        }
    }

    /// Handles a keyboard event by moving the selection.
    pub fn handle_key(&mut self, key: Key) {
        let delta = match key {
            Key::Left => -1,
            Key::Right => 1,
        };
        self.selected = self.adapter.add_days(&self.selected, delta);
    }
}

