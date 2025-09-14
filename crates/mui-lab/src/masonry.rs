//! Simple Masonry layout algorithm.
//!
//! **Unstable:** This module is an early preview.  It implements a minimal
//! column-based layout to illustrate how the algorithm might be structured.
//! Real world use cases will likely require virtualized rendering and more
//! configuration options.

/// Masonry layout that distributes items into a fixed number of columns in a
/// round-robin fashion.  The generic `T` must implement [`Clone`] so the
/// layout can return owned values without lifetime juggling.
#[derive(Debug, Default)]
pub struct Masonry<T: Clone> {
    columns: usize,
    items: Vec<T>,
}

impl<T: Clone> Masonry<T> {
    /// Creates a new layout with `columns` columns.
    pub fn new(columns: usize) -> Self {
        Self {
            columns: columns.max(1),
            items: Vec::new(),
        }
    }

    /// Adds an item to the layout.  Items are stored in insertion order and
    /// later distributed across columns when [`layout`](Self::layout) is
    /// called.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Computes the columnar layout returning a vector of columns where each
    /// column contains the items assigned to it.  The algorithm is intentionally
    /// simple and therefore predictable which aids in testing and future
    /// optimizations.
    pub fn layout(&self) -> Vec<Vec<T>> {
        let mut cols: Vec<Vec<T>> = vec![Vec::new(); self.columns];
        for (idx, item) in self.items.iter().cloned().enumerate() {
            cols[idx % self.columns].push(item);
        }
        cols
    }
}
