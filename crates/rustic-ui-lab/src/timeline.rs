//! Lightweight event timeline.
//!
//! `Timeline` stores events in chronological order. It is intentionally kept
//! simple so that applications can build richer visualizations or persistence
//! layers on top. The component is hidden behind the `timeline` feature gate.

/// Event with an associated timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEvent<T> {
    /// Milliseconds since UNIX epoch.
    pub at: u64,
    /// Arbitrary payload for the event.
    pub data: T,
}

/// In-memory ordered collection of [`TimelineEvent`]s.
#[derive(Debug, Default, Clone)]
pub struct Timeline<T> {
    events: Vec<TimelineEvent<T>>,
}

impl<T> Timeline<T> {
    /// Creates an empty timeline.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Inserts an event and keeps the internal vector ordered by timestamp.
    pub fn push(&mut self, event: TimelineEvent<T>) {
        self.events.push(event);
        self.events.sort_by_key(|e| e.at);
    }

    /// Returns the events in chronological order.
    pub fn events(&self) -> &[TimelineEvent<T>] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_keeps_events_sorted() {
        let mut tl = Timeline::new();
        tl.push(TimelineEvent { at: 2, data: "b" });
        tl.push(TimelineEvent { at: 1, data: "a" });
        let events: Vec<_> = tl.events().iter().map(|e| e.data).collect();
        assert_eq!(events, vec!["a", "b"]);
    }
}
