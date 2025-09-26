//! Minimal in-memory data grid.
//!
//! The goal of this experimental API is to provide a small, easily testable
//! abstraction for manipulating tabular data. Rendering and virtualization are
//! intentionally left to higher level crates so that this module can be reused
//! across different UI frameworks or even server-side processing tools.
//!
//! The component is feature gated behind `data-grid` to avoid pulling it into
//! applications that don't need it.

/// Generic grid storing rows of data.
#[derive(Debug, Clone)]
pub struct DataGrid<T> {
    /// Rows backing the grid. In a real widget this would likely be a more
    /// complex structure supporting pagination or virtualization.
    pub rows: Vec<T>,
}

impl<T> DataGrid<T> {
    /// Creates a new grid from a set of rows.
    pub fn new(rows: Vec<T>) -> Self {
        Self { rows }
    }

    /// Sorts the rows in place using the provided comparator.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.rows.sort_by(compare);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_by_orders_rows() {
        let mut grid = DataGrid::new(vec![3, 1, 2]);
        grid.sort_by(|a, b| a.cmp(b));
        assert_eq!(grid.rows, vec![1, 2, 3]);
    }
}
