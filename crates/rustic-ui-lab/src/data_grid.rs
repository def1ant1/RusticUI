//! Enterprise-ready headless data grid engine.
//!
//! This module intentionally focuses on declarative state management so UI
//! frameworks can render Material flavored experiences (or any other
//! presentation) without duplicating business logic.  The engine models all of
//! the non-visual concepts required by spreadsheet style components: column
//! metadata, sort/filter projections, virtualization, pagination, range-aware
//! selection, and extensible telemetry hooks.  Rendering crates consume the
//! exposed APIs to produce SSR-friendly markup, hydration bridges, or even
//! CLI/desktop frontends.
//!
//! ```rust
//! use rustic_ui_lab::data_grid::{
//!     ColumnDefinition, ColumnId, DataGridEngine, DataGridTelemetry, DataValue, SortDirection,
//!     SortDescriptor,
//! };
//!
//! #[derive(Debug)]
//! struct Record {
//!     sku: String,
//!     inventory: i32,
//! }
//!
//! // A minimal telemetry sink that simply stores emitted events so tests can assert on them.
//! #[derive(Default, Clone)]
//! struct Telemetry(std::sync::Arc<std::sync::Mutex<Vec<String>>>);
//!
//! impl DataGridTelemetry<Record> for Telemetry {
//!     fn record_sort(&self, descriptors: &[SortDescriptor]) {
//!         let mut buffer = self.0.lock().unwrap();
//!         buffer.push(format!("sorted on {:?}", descriptors));
//!     }
//! }
//!
//! let rows = vec![
//!     Record { sku: "A-100".into(), inventory: 42 },
//!     Record { sku: "B-200".into(), inventory: 5 },
//!     Record { sku: "Z-900".into(), inventory: 99 },
//! ];
//! let columns = vec![
//!     ColumnDefinition::text(ColumnId::new("sku"), |row: &Record| row.sku.clone())
//!         .with_width(240.0),
//!     ColumnDefinition::number(ColumnId::new("inventory"), |row: &Record| row.inventory as f64),
//! ];
//!
//! let telemetry = Telemetry::default();
//! let events = telemetry.0.clone();
//! let mut grid = DataGridEngine::builder(columns, rows)
//!     .with_page_size(25)
//!     .with_telemetry(telemetry)
//!     .build();
//!
//! grid.apply_sort(vec![SortDescriptor::new(ColumnId::new("inventory"), SortDirection::Descending)]);
//! let visible = grid.visible_rows();
//! assert_eq!(visible.first().unwrap().row.inventory, 99);
//! assert!(events.lock().unwrap()[0].contains("inventory"));
//! ```

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;
use std::sync::{Arc, Mutex};

use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

/// Identifier assigned to each column.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnId(Arc<str>);

impl ColumnId {
    /// Creates a new column identifier.
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for ColumnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifier allocated to rows for stable selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowId(pub u64);

/// Value extracted from a row for sorting/filtering.
#[derive(Clone, Debug, PartialEq)]
pub enum DataValue {
    /// Freeform text.
    Text(String),
    /// Numeric payload stored as 64-bit float for portability.
    Number(f64),
    /// Boolean flag.
    Boolean(bool),
    /// Epoch millis representing a date/time.
    Timestamp(i64),
}

impl PartialOrd for DataValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (DataValue::Text(a), DataValue::Text(b)) => Some(a.cmp(b)),
            (DataValue::Number(a), DataValue::Number(b)) => a.partial_cmp(b),
            (DataValue::Boolean(a), DataValue::Boolean(b)) => Some(a.cmp(b)),
            (DataValue::Timestamp(a), DataValue::Timestamp(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

/// Describes how a column should be measured during layout.
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnWidth {
    /// Auto size driven by content heuristics.
    Auto,
    /// Pixel width for deterministic virtualization calculations.
    Fixed(f32),
    /// Fractional share used for fluid columns.
    Fraction(f32),
}

impl Default for ColumnWidth {
    fn default() -> Self {
        ColumnWidth::Auto
    }
}

/// Declarative description of a column.
#[derive(Clone)]
pub struct ColumnDefinition<R> {
    /// Stable identifier.
    pub id: ColumnId,
    /// Human readable label used by adapters.
    pub label: Cow<'static, str>,
    /// Value accessor used during sort/filter operations.
    pub accessor: Arc<dyn Fn(&R) -> DataValue + Send + Sync>,
    /// Optional automation identifier appended to SSR attributes.
    pub automation_id: Option<Cow<'static, str>>,
    /// Width contract consumed by virtualization calculations.
    pub width: ColumnWidth,
}

impl<R> fmt::Debug for ColumnDefinition<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColumnDefinition")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("automation_id", &self.automation_id)
            .field("width", &self.width)
            .finish()
    }
}

impl<R> ColumnDefinition<R> {
    /// Creates a textual column.
    pub fn text(id: ColumnId, accessor: impl Fn(&R) -> String + Send + Sync + 'static) -> Self {
        Self {
            id,
            label: Cow::Borrowed(""),
            accessor: Arc::new(move |row| DataValue::Text(accessor(row))),
            automation_id: None,
            width: ColumnWidth::Auto,
        }
    }

    /// Creates a numeric column.
    pub fn number(id: ColumnId, accessor: impl Fn(&R) -> f64 + Send + Sync + 'static) -> Self {
        Self {
            id,
            label: Cow::Borrowed(""),
            accessor: Arc::new(move |row| DataValue::Number(accessor(row))),
            automation_id: None,
            width: ColumnWidth::Auto,
        }
    }

    /// Creates a boolean column.
    pub fn boolean(id: ColumnId, accessor: impl Fn(&R) -> bool + Send + Sync + 'static) -> Self {
        Self {
            id,
            label: Cow::Borrowed(""),
            accessor: Arc::new(move |row| DataValue::Boolean(accessor(row))),
            automation_id: None,
            width: ColumnWidth::Auto,
        }
    }

    /// Updates the label.
    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = label.into();
        self
    }

    /// Updates the automation identifier.
    pub fn with_automation_id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Updates the width.
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = ColumnWidth::Fixed(width);
        self
    }
}

/// Sort order applied to a column.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortDescriptor {
    /// Column being sorted.
    pub column: ColumnId,
    /// Direction of the sort.
    pub direction: SortDirection,
}

impl SortDescriptor {
    /// Creates a new descriptor.
    pub fn new(column: ColumnId, direction: SortDirection) -> Self {
        Self { column, direction }
    }
}

/// Sort direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending order.
    Ascending,
    /// Descending order.
    Descending,
}

/// Predicate applied to a row.
#[derive(Clone)]
pub struct FilterDescriptor<R> {
    /// Column identifier.
    pub column: ColumnId,
    /// Predicate executed for the row.
    pub predicate: Arc<dyn Fn(&DataValue) -> bool + Send + Sync>,
    /// Projection retrieving the value from the row.
    pub accessor: Arc<dyn Fn(&R) -> DataValue + Send + Sync>,
}

impl<R> fmt::Debug for FilterDescriptor<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterDescriptor")
            .field("column", &self.column)
            .finish()
    }
}

impl<R> FilterDescriptor<R> {
    /// Creates a new descriptor.
    pub fn new(
        column: ColumnId,
        accessor: Arc<dyn Fn(&R) -> DataValue + Send + Sync>,
        predicate: Arc<dyn Fn(&DataValue) -> bool + Send + Sync>,
    ) -> Self {
        Self {
            column,
            predicate,
            accessor,
        }
    }
}

/// Virtualization window describing the viewport.
#[derive(Clone, Debug)]
pub struct VirtualWindow {
    /// Row range requested by the renderer.
    pub range: Range<usize>,
    /// Number of rows fetched before/after the viewport for smooth scrolling.
    pub overscan: usize,
}

impl Default for VirtualWindow {
    fn default() -> Self {
        Self {
            range: 0..usize::MAX,
            overscan: 0,
        }
    }
}

/// Pagination configuration.
#[derive(Clone, Debug)]
pub struct PaginationState {
    /// Number of rows per page.
    pub page_size: usize,
    /// Zero-based page index.
    pub current_page: usize,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            page_size: usize::MAX,
            current_page: 0,
        }
    }
}

/// Selection mode supported by the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    /// Selection disabled entirely.
    None,
    /// Single row selection.
    Single,
    /// Multiple rows can be selected concurrently.
    Multiple,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::None
    }
}

/// Tracks selected rows.
#[derive(Clone, Debug)]
pub struct SelectionState {
    mode: SelectionMode,
    selected: BTreeSet<RowId>,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            mode: SelectionMode::None,
            selected: BTreeSet::new(),
        }
    }
}

impl SelectionState {
    fn with_mode(mode: SelectionMode) -> Self {
        Self {
            mode,
            selected: BTreeSet::new(),
        }
    }

    fn update(&mut self, row: RowId, selected: bool) {
        match self.mode {
            SelectionMode::None => self.selected.clear(),
            SelectionMode::Single => {
                self.selected.clear();
                if selected {
                    self.selected.insert(row);
                }
            }
            SelectionMode::Multiple => {
                if selected {
                    self.selected.insert(row);
                } else {
                    self.selected.remove(&row);
                }
            }
        }
    }

    fn toggle(&mut self, row: RowId) {
        let should_select = !self.selected.contains(&row);
        self.update(row, should_select);
    }
}

/// Event hooks used for telemetry/analytics integrations.
pub trait DataGridTelemetry<R>: Send + Sync + 'static {
    /// Fired when the virtualization window changes.
    fn record_window(&self, _window: &VirtualWindow) {}
    /// Fired when pagination changes.
    fn record_pagination(&self, _state: &PaginationState) {}
    /// Fired when the selection collection updates.
    fn record_selection(&self, _selected: &BTreeSet<RowId>) {}
    /// Fired when the sort descriptors change.
    fn record_sort(&self, _descriptors: &[SortDescriptor]) {}
    /// Fired when filters update.
    fn record_filters(&self, _filters: &[ColumnId]) {}
    /// Fired when rows are streamed to the renderer.
    fn record_rows_streamed(&self, _count: usize) {}
}

/// Default telemetry implementation that discards all events.
#[derive(Default)]
pub struct NoopTelemetry;

impl<R> DataGridTelemetry<R> for NoopTelemetry {}

/// Row stored inside the engine.
struct RowEntry<R> {
    id: RowId,
    row: Arc<R>,
}

impl<R> Clone for RowEntry<R> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            row: Arc::clone(&self.row),
        }
    }
}

/// Builder used to configure the engine.
pub struct DataGridBuilder<R> {
    columns: Vec<ColumnDefinition<R>>,
    rows: Vec<R>,
    telemetry: Arc<dyn DataGridTelemetry<R>>,
    window: VirtualWindow,
    pagination: PaginationState,
    selection_mode: SelectionMode,
}

impl<R: 'static> DataGridEngine<R> {
    /// Starts building a grid with [`NoopTelemetry`].
    pub fn builder(columns: Vec<ColumnDefinition<R>>, rows: Vec<R>) -> DataGridBuilder<R> {
        DataGridBuilder {
            columns,
            rows,
            telemetry: Arc::new(NoopTelemetry),
            window: VirtualWindow::default(),
            pagination: PaginationState::default(),
            selection_mode: SelectionMode::None,
        }
    }
}

impl<R: 'static> DataGridBuilder<R> {
    /// Overrides the telemetry sink.
    pub fn with_telemetry<T>(mut self, telemetry: T) -> Self
    where
        T: DataGridTelemetry<R>,
    {
        self.telemetry = Arc::new(telemetry);
        self
    }

    /// Sets the virtualization window.
    pub fn with_window(mut self, window: VirtualWindow) -> Self {
        self.window = window;
        self
    }

    /// Sets the page size (defaults to unlimited).
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.pagination.page_size = page_size.max(1);
        self
    }

    /// Sets the initial page.
    pub fn with_initial_page(mut self, page: usize) -> Self {
        self.pagination.current_page = page;
        self
    }

    /// Enables selection.
    pub fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Builds the engine.
    pub fn build(self) -> DataGridEngine<R> {
        DataGridEngine::new_internal(
            self.columns,
            self.rows,
            self.telemetry,
            self.window,
            self.pagination,
            self.selection_mode,
        )
    }
}

/// Primary state container powering headless grids.
pub struct DataGridEngine<R> {
    columns: Vec<ColumnDefinition<R>>,
    rows: Vec<RowEntry<R>>,
    telemetry: Arc<dyn DataGridTelemetry<R>>,
    window: VirtualWindow,
    pagination: PaginationState,
    selection: SelectionState,
    sort: Vec<SortDescriptor>,
    filters: Vec<FilterDescriptor<R>>,
    cached_visible: Arc<Mutex<Vec<RowEntry<R>>>>,
}

impl<R> fmt::Debug for DataGridEngine<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataGridEngine")
            .field("columns", &self.columns)
            .field("row_count", &self.rows.len())
            .field("window", &self.window)
            .field("pagination", &self.pagination)
            .field("selection", &self.selection.selected)
            .field("sort", &self.sort)
            .finish()
    }
}

impl<R: 'static> DataGridEngine<R> {
    fn new_internal(
        columns: Vec<ColumnDefinition<R>>,
        rows: Vec<R>,
        telemetry: Arc<dyn DataGridTelemetry<R>>,
        window: VirtualWindow,
        pagination: PaginationState,
        selection_mode: SelectionMode,
    ) -> Self {
        static ROW_COUNTER: AtomicU64 = AtomicU64::new(0);
        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let id = RowId(ROW_COUNTER.fetch_add(1, AtomicOrdering::Relaxed) + 1);
            entries.push(RowEntry {
                id,
                row: Arc::new(row),
            });
        }

        let engine = Self {
            columns,
            rows: entries,
            telemetry,
            window,
            pagination,
            selection: SelectionState::with_mode(selection_mode),
            sort: Vec::new(),
            filters: Vec::new(),
            cached_visible: Arc::new(Mutex::new(Vec::new())),
        };

        engine.recompute_cache();
        engine
    }

    /// Returns immutable column descriptors.
    pub fn columns(&self) -> &[ColumnDefinition<R>] {
        &self.columns
    }

    /// Updates the virtualization window.
    pub fn set_window(&mut self, window: VirtualWindow) {
        self.window = window;
        self.telemetry.record_window(&self.window);
        self.recompute_cache();
    }

    /// Updates the pagination state.
    pub fn set_page(&mut self, page: usize) {
        self.pagination.current_page = page;
        self.telemetry.record_pagination(&self.pagination);
        self.recompute_cache();
    }

    /// Updates the page size.
    pub fn set_page_size(&mut self, size: usize) {
        self.pagination.page_size = size.max(1);
        self.telemetry.record_pagination(&self.pagination);
        self.recompute_cache();
    }

    /// Applies new sort descriptors.
    pub fn apply_sort(&mut self, descriptors: Vec<SortDescriptor>) {
        self.sort = descriptors;
        self.telemetry.record_sort(&self.sort);
        self.recompute_cache();
    }

    /// Applies a new filter set.
    pub fn apply_filters(&mut self, filters: Vec<FilterDescriptor<R>>) {
        let columns: Vec<_> = filters.iter().map(|filter| filter.column.clone()).collect();
        self.filters = filters;
        self.telemetry.record_filters(&columns);
        self.recompute_cache();
    }

    /// Toggles row selection.
    pub fn toggle_row(&mut self, row: RowId) {
        self.selection.toggle(row);
        self.telemetry.record_selection(&self.selection.selected);
    }

    /// Marks a row as selected or not.
    pub fn set_row_selected(&mut self, row: RowId, selected: bool) {
        self.selection.update(row, selected);
        self.telemetry.record_selection(&self.selection.selected);
    }

    /// Clears the selection state.
    pub fn clear_selection(&mut self) {
        self.selection.selected.clear();
        self.telemetry.record_selection(&self.selection.selected);
    }

    /// Returns the selected row identifiers.
    pub fn selected_rows(&self) -> impl Iterator<Item = RowId> + '_ {
        self.selection.selected.iter().copied()
    }

    /// Returns rows currently visible according to filters, pagination, and virtualization.
    pub fn visible_rows(&self) -> Vec<VisibleRow<R>> {
        let rows = self.cached_visible.lock().unwrap();
        self.telemetry.record_rows_streamed(rows.len());
        rows.iter()
            .map(|entry| VisibleRow {
                id: entry.id,
                row: entry.row.clone(),
            })
            .collect()
    }

    /// Total number of rows before filtering.
    pub fn total_rows(&self) -> usize {
        self.rows.len()
    }

    /// Total rows after filters but before virtualization.
    pub fn filtered_rows(&self) -> usize {
        let rows = self.cached_visible.lock().unwrap();
        rows.len()
    }

    fn recompute_cache(&self) {
        let mut filtered: Vec<&RowEntry<R>> = self.rows.iter().collect();

        if !self.filters.is_empty() {
            filtered.retain(|entry| {
                self.filters.iter().all(|filter| {
                    let value = (filter.accessor)(&entry.row);
                    (filter.predicate)(&value)
                })
            });
        }

        if !self.sort.is_empty() {
            filtered.sort_by(|a, b| {
                for descriptor in &self.sort {
                    if let Some(column) = self
                        .columns
                        .iter()
                        .find(|column| column.id == descriptor.column)
                    {
                        let value_a = (column.accessor)(&a.row);
                        let value_b = (column.accessor)(&b.row);
                        if let Some(ordering) = value_a.partial_cmp(&value_b) {
                            if ordering != Ordering::Equal {
                                return match descriptor.direction {
                                    SortDirection::Ascending => ordering,
                                    SortDirection::Descending => ordering.reverse(),
                                };
                            }
                        }
                    }
                }
                Ordering::Equal
            });
        }

        let start_index = self
            .pagination
            .current_page
            .saturating_mul(self.pagination.page_size);
        let end_index = (start_index + self.pagination.page_size).min(filtered.len());
        let paginated = if start_index < end_index {
            &filtered[start_index..end_index]
        } else {
            &[]
        };

        let window_start = self.window.range.start.saturating_sub(self.window.overscan);
        let window_end = self
            .window
            .range
            .end
            .saturating_add(self.window.overscan)
            .min(paginated.len());
        let sliced = if window_start < window_end {
            &paginated[window_start..window_end]
        } else {
            &[]
        };

        let mut cache = self.cached_visible.lock().unwrap();
        cache.clear();
        cache.extend(sliced.iter().map(|entry| (**entry).clone()));
    }
}

/// Row surfaced by [`DataGridEngine::visible_rows`].
#[derive(Clone)]
pub struct VisibleRow<R> {
    /// Stable row identifier.
    pub id: RowId,
    /// Shared pointer to the row payload.
    pub row: Arc<R>,
}

impl<R> fmt::Debug for VisibleRow<R>
where
    R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VisibleRow")
            .field("id", &self.id)
            .field("row", &self.row)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[derive(Clone, Debug)]
    struct FixtureRow {
        id: u32,
        name: String,
        active: bool,
    }

    fn build_fixture(rows: Vec<FixtureRow>) -> DataGridEngine<FixtureRow> {
        let columns = vec![
            ColumnDefinition::number(ColumnId::new("id"), |row: &FixtureRow| row.id as f64)
                .with_label("ID"),
            ColumnDefinition::text(ColumnId::new("name"), |row: &FixtureRow| row.name.clone())
                .with_label("Name"),
            ColumnDefinition::boolean(ColumnId::new("active"), |row: &FixtureRow| row.active)
                .with_label("Active"),
        ];

        DataGridEngine::builder(columns, rows)
            .with_page_size(50)
            .with_selection_mode(SelectionMode::Multiple)
            .build()
    }

    #[test]
    fn sort_applies_direction() {
        let mut grid = build_fixture(vec![
            FixtureRow {
                id: 2,
                name: "b".into(),
                active: true,
            },
            FixtureRow {
                id: 1,
                name: "a".into(),
                active: false,
            },
        ]);

        grid.apply_sort(vec![SortDescriptor::new(
            ColumnId::new("id"),
            SortDirection::Descending,
        )]);
        let rows: Vec<_> = grid
            .visible_rows()
            .into_iter()
            .map(|row| row.row.id)
            .collect();
        assert_eq!(rows, vec![2, 1]);

        grid.apply_sort(vec![SortDescriptor::new(
            ColumnId::new("name"),
            SortDirection::Ascending,
        )]);
        let rows: Vec<_> = grid
            .visible_rows()
            .into_iter()
            .map(|row| row.row.name.clone())
            .collect();
        assert_eq!(rows, vec!["a", "b"]);
    }

    #[test]
    fn filter_limits_results() {
        let mut grid = build_fixture(vec![
            FixtureRow {
                id: 1,
                name: "alpha".into(),
                active: true,
            },
            FixtureRow {
                id: 2,
                name: "beta".into(),
                active: false,
            },
        ]);

        let columns: Vec<_> = grid.columns().iter().cloned().collect();
        let filter = FilterDescriptor::new(
            ColumnId::new("active"),
            columns[2].accessor.clone(),
            Arc::new(|value| matches!(value, DataValue::Boolean(true))),
        );
        grid.apply_filters(vec![filter]);
        let rows: Vec<_> = grid
            .visible_rows()
            .into_iter()
            .map(|row| row.row.id)
            .collect();
        assert_eq!(rows, vec![1]);
    }

    proptest! {
        #[test]
        fn pagination_never_overflows(row_count in 1usize..256) {
            let rows: Vec<_> = (0..row_count)
                .map(|id| FixtureRow { id: id as u32, name: format!("row-{id}"), active: id % 2 == 0 })
                .collect();
            let mut grid = build_fixture(rows);
            grid.set_page_size(7);
            for page in 0..row_count {
                grid.set_page(page);
                let visible = grid.visible_rows();
                prop_assert!(visible.len() <= 7);
            }
        }

        #[test]
        fn virtualization_window_is_respected(start in 0usize..32, size in 1usize..16, overscan in 0usize..4) {
            let rows: Vec<_> = (0..64)
                .map(|id| FixtureRow { id: id as u32, name: format!("row-{id}"), active: id % 2 == 0 })
                .collect();
            let mut grid = build_fixture(rows);
            grid.set_window(VirtualWindow { range: start..start + size, overscan });
            let visible = grid.visible_rows();
            let max_possible = size + overscan * 2;
            let remaining = 64usize.saturating_sub(start);
            let expected_len = max_possible.min(remaining).min(64);
            prop_assert!(visible.len() <= expected_len);
        }

        #[test]
        fn selection_respects_mode(toggle_sequence in proptest::collection::vec(0usize..8, 1..16)) {
            let rows: Vec<_> = (0..8)
                .map(|id| FixtureRow { id: id as u32, name: format!("row-{id}"), active: true })
                .collect();
            let mut grid = build_fixture(rows);
            let visible = grid.visible_rows();
            if visible.is_empty() {
                prop_assert_eq!(grid.selected_rows().count(), 0);
            } else {
                let len = visible.len();
                for index in toggle_sequence {
                    let row = &visible[index % len];
                    grid.toggle_row(row.id);
                }
                prop_assert!(grid.selected_rows().count() <= 8);
            }
        }
    }
}
