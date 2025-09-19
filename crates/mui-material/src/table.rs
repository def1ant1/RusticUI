//! Material themed data table renderer.
//!
//! The table builds on the same headless selection machinery as [`crate::list`]
//! while layering column metadata, zebra striping, and numeric alignment.  All
//! frameworks consume the shared [`render_html`] routine which guarantees that
//! SSR output, hydration adapters, and static site generators receive identical
//! markup and deterministic automation hooks.
//!
//! Layout is derived from the [`mui-system`] theme: spacing originates from the
//! `Theme::spacing` scale, typography comes from the shared
//! [`ListTypography`](crate::list::ListTypography) enum, and focus styling honors
//! Joy tokens so enterprise overrides automatically cascade.

use crate::list::{ListDensity, ListTypography};
use mui_headless::list::{ListState, SelectionMode};
use mui_styled_engine::{css_with_theme, Style};

/// Describes a column rendered in the table header.
#[derive(Clone, Debug, PartialEq)]
pub struct TableColumn {
    /// Header label displayed to the user.
    pub header: String,
    /// Whether the column is numeric. Numeric columns are right aligned and use
    /// tabular numbers for consistent metrics.
    pub numeric: bool,
    /// Stable automation identifier appended to `data-automation-column`.
    pub automation_id: Option<String>,
}

impl TableColumn {
    /// Convenience constructor for a text column.
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            numeric: false,
            automation_id: None,
        }
    }

    /// Marks the column as numeric.
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// Overrides the automation identifier suffix.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }
}

/// Represents a single table row.
#[derive(Clone, Debug, PartialEq)]
pub struct TableRow {
    /// Individual cell values rendered in order.
    pub cells: Vec<String>,
    /// Optional automation identifier appended to `data-automation-row`.
    pub automation_id: Option<String>,
    /// Whether the row should be flagged as disabled.
    pub disabled: bool,
}

impl TableRow {
    /// Convenience constructor mirroring spreadsheet style APIs.
    pub fn new(cells: Vec<String>) -> Self {
        Self {
            cells,
            automation_id: None,
            disabled: false,
        }
    }

    /// Overrides the automation identifier suffix.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Marks the row as disabled (non-selectable).
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Shared props consumed by the table renderer across frameworks.
#[derive(Clone, Debug, PartialEq)]
pub struct TableProps {
    /// Column metadata rendered in the `<thead>`.
    pub columns: Vec<TableColumn>,
    /// Row data rendered inside `<tbody>`.
    pub rows: Vec<TableRow>,
    /// Density preset controlling padding.
    pub density: ListDensity,
    /// Typography applied to the header cells.
    pub header_typography: ListTypography,
    /// Typography applied to body cells.
    pub body_typography: ListTypography,
    /// Whether zebra striping should be applied to alternate rows.
    pub striped: bool,
    /// Selection mode forwarded to the [`ListState`].
    pub selection_mode: SelectionMode,
    /// Optional caption describing the table for assistive technology.
    pub caption: Option<String>,
    /// Optional automation identifier prefix.
    pub automation_id: Option<String>,
}

impl TableProps {
    /// Creates a new table configuration with sensible defaults.
    pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self {
            columns,
            rows,
            density: ListDensity::Comfortable,
            header_typography: ListTypography::Subtitle1,
            body_typography: ListTypography::Body2,
            striped: true,
            selection_mode: SelectionMode::None,
            caption: None,
            automation_id: None,
        }
    }

    /// Overrides the density preset.
    pub fn with_density(mut self, density: ListDensity) -> Self {
        self.density = density;
        self
    }

    /// Overrides the header typography.
    pub fn with_header_typography(mut self, variant: ListTypography) -> Self {
        self.header_typography = variant;
        self
    }

    /// Overrides the body typography.
    pub fn with_body_typography(mut self, variant: ListTypography) -> Self {
        self.body_typography = variant;
        self
    }

    /// Toggles zebra striping.
    pub fn with_striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Configures selection support.
    pub fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Adds an accessible caption.
    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets the automation identifier prefix.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }
}

/// Render the table into HTML markup shared across frameworks.
fn render_html(props: &TableProps, state: &ListState) -> String {
    let root_attrs = crate::style_helpers::themed_attributes_html(
        table_style(props),
        table_attributes(props, state),
    );

    let caption_html = props
        .caption
        .as_ref()
        .map(|caption| format!("<caption>{caption}</caption>"))
        .unwrap_or_default();

    let header_row_attrs = crate::style_helpers::themed_attributes_html(
        table_header_row_style(),
        vec![(
            String::from("data-component"),
            String::from("mui-table-header-row"),
        )],
    );

    let mut header_cells_html = String::new();
    for (index, column) in props.columns.iter().enumerate() {
        let cell_attrs = crate::style_helpers::themed_attributes_html(
            table_header_cell_style(),
            header_cell_attributes(props, column, index),
        );
        header_cells_html.push_str(&format!("<th {cell_attrs}>{}</th>", column.header));
    }

    let mut body_rows_html = String::new();
    for (index, row) in props.rows.iter().enumerate() {
        let row_attrs = crate::style_helpers::themed_attributes_html(
            table_row_style(),
            row_attributes(props, state, row, index),
        );
        body_rows_html.push_str(&format!(
            "<tr {row_attrs}>{}</tr>",
            row_markup(props, row, index)
        ));
    }

    format!(
        "<table {root_attrs}>{caption}<thead><tr {header_row_attrs}>{headers}</tr></thead><tbody>{rows}</tbody></table>",
        caption = caption_html,
        headers = header_cells_html,
        rows = body_rows_html,
    )
}

fn automation_base(props: &TableProps) -> String {
    props
        .automation_id
        .clone()
        .unwrap_or_else(|| "mui-table".into())
}

fn column_id(props: &TableProps, index: usize) -> String {
    format!("{}-column-{index}", automation_base(props))
}

fn row_id(props: &TableProps, index: usize) -> String {
    format!("{}-row-{index}", automation_base(props))
}

fn cell_automation_id(props: &TableProps, row: usize, col: usize, column: &TableColumn) -> String {
    if let Some(id) = &column.automation_id {
        format!("{}-{}-row-{row}", automation_base(props), id)
    } else {
        format!("{}-cell-{row}-{col}", automation_base(props))
    }
}

fn table_attributes(props: &TableProps, state: &ListState) -> Vec<(String, String)> {
    let mut attrs = vec![
        ("data-component".to_string(), String::from("mui-table")),
        (
            "data-density".to_string(),
            props.density.data_value().to_string(),
        ),
        ("data-striped".to_string(), props.striped.to_string()),
        ("aria-rowcount".to_string(), props.rows.len().to_string()),
        ("aria-colcount".to_string(), props.columns.len().to_string()),
    ];

    match props.selection_mode {
        SelectionMode::None => {
            attrs.push(("role".to_string(), String::from("table")));
        }
        SelectionMode::Single | SelectionMode::Multiple => {
            attrs.push(("role".to_string(), String::from("grid")));
            attrs.push(("tabindex".to_string(), String::from("0")));
            if props.selection_mode == SelectionMode::Multiple {
                attrs.push(("aria-multiselectable".to_string(), String::from("true")));
            }
            if let Some(highlight) = state.highlighted() {
                attrs.push((
                    "aria-activedescendant".to_string(),
                    row_id(props, highlight),
                ));
            }
        }
    }

    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".to_string(), id.clone()));
    }

    attrs
}

fn header_cell_attributes(
    props: &TableProps,
    column: &TableColumn,
    index: usize,
) -> Vec<(String, String)> {
    vec![
        ("id".to_string(), column_id(props, index)),
        ("scope".to_string(), String::from("col")),
        ("role".to_string(), String::from("columnheader")),
        ("data-numeric".to_string(), column.numeric.to_string()),
        (
            "data-automation-column".to_string(),
            column
                .automation_id
                .clone()
                .unwrap_or_else(|| format!("{}-column-{index}", automation_base(props))),
        ),
    ]
}

fn row_attributes(
    props: &TableProps,
    state: &ListState,
    row: &TableRow,
    index: usize,
) -> Vec<(String, String)> {
    let mut attrs = vec![
        ("id".into(), row_id(props, index)),
        ("role".into(), "row".into()),
        ("data-index".into(), index.to_string()),
        ("data-selected".into(), state.is_selected(index).to_string()),
        (
            "data-highlighted".into(),
            (state.highlighted() == Some(index)).to_string(),
        ),
        ("data-disabled".into(), row.disabled.to_string()),
        (
            "data-automation-row".into(),
            row.automation_id
                .clone()
                .unwrap_or_else(|| format!("{}-row-{index}", automation_base(props))),
        ),
    ];

    if matches!(
        props.selection_mode,
        SelectionMode::Single | SelectionMode::Multiple
    ) {
        attrs.push(("aria-selected".into(), state.is_selected(index).to_string()));
    }

    if row.disabled {
        attrs.push(("aria-disabled".into(), "true".into()));
    }

    attrs
}

fn row_markup(props: &TableProps, row: &TableRow, row_index: usize) -> String {
    let mut html = String::new();
    let column_count = props.columns.len();
    for (col_index, column) in props.columns.iter().enumerate() {
        let cell_value = row.cells.get(col_index).cloned().unwrap_or_default();
        let cell_attrs = crate::style_helpers::themed_attributes_html(
            table_body_cell_style(),
            body_cell_attributes(props, column, row_index, col_index),
        );
        html.push_str(&format!("<td {cell_attrs}>{cell_value}</td>"));
    }
    // If the row provides more cells than columns, render the extras in case
    // callers want to append hidden automation data.
    if row.cells.len() > column_count {
        for extra_index in column_count..row.cells.len() {
            let cell_attrs = crate::style_helpers::themed_attributes_html(
                table_body_cell_style(),
                vec![
                    (String::from("role"), String::from("gridcell")),
                    (
                        String::from("data-automation-cell"),
                        format!(
                            "{}-cell-{}-extra-{}",
                            automation_base(props),
                            row_index,
                            extra_index
                        ),
                    ),
                ],
            );
            html.push_str(&format!("<td {cell_attrs}>{}</td>", row.cells[extra_index]));
        }
    }
    html
}

fn body_cell_attributes(
    props: &TableProps,
    column: &TableColumn,
    row_index: usize,
    col_index: usize,
) -> Vec<(String, String)> {
    vec![
        ("role".to_string(), String::from("gridcell")),
        ("data-numeric".to_string(), column.numeric.to_string()),
        ("headers".to_string(), column_id(props, col_index)),
        (
            "data-automation-cell".to_string(),
            cell_automation_id(props, row_index, col_index, column),
        ),
    ]
}

fn table_style(props: &TableProps) -> Style {
    let density = props.density;
    css_with_theme!(
        r#"
        width: 100%;
        border-collapse: collapse;
        background: ${background};
        color: ${text_color};
        border-radius: ${radius};
        border: 1px solid ${border_color};
        overflow: hidden;
        --mui-table-padding-y: ${padding_y};
        --mui-table-padding-x: ${padding_x};
        --mui-table-header-font-size: ${header_size};
        --mui-table-header-font-weight: ${header_weight};
        --mui-table-body-font-size: ${body_size};
        --mui-table-body-font-weight: ${body_weight};

        &[data-striped='true'] tbody tr:nth-child(even) {
            background: ${striped_bg};
        }
    "#,
        background = theme.palette.background_paper.clone(),
        text_color = theme.palette.text_primary.clone(),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 18%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        padding_y = format!("{}px", theme.spacing(density.vertical_padding())),
        padding_x = format!("{}px", theme.spacing(2)),
        header_size = format!("{:.3}rem", props.header_typography.font_size(&theme)),
        header_weight = props.header_typography.font_weight(&theme).to_string(),
        body_size = format!("{:.3}rem", props.body_typography.font_size(&theme)),
        body_weight = props.body_typography.font_weight(&theme).to_string(),
        striped_bg = format!(
            "color-mix(in srgb, {} 8%, transparent)",
            theme.palette.text_secondary.clone()
        ),
    )
}

fn table_header_row_style() -> Style {
    css_with_theme!(
        r#"
        background: ${header_bg};
    "#,
        header_bg = format!(
            "color-mix(in srgb, {} 6%, transparent)",
            theme.palette.text_secondary.clone()
        ),
    )
}

fn table_header_cell_style() -> Style {
    css_with_theme!(
        r#"
        text-align: left;
        padding: var(--mui-table-padding-y) var(--mui-table-padding-x);
        font-size: var(--mui-table-header-font-size);
        font-weight: var(--mui-table-header-font-weight);
        color: ${header_color};
        border-bottom: 1px solid ${divider};
        letter-spacing: 0.01em;
        text-transform: uppercase;

        &[data-numeric='true'] {
            text-align: right;
        }
    "#,
        header_color = theme.palette.text_secondary.clone(),
        divider = format!(
            "color-mix(in srgb, {} 20%, transparent)",
            theme.palette.text_secondary.clone()
        ),
    )
}

fn table_row_style() -> Style {
    css_with_theme!(
        r#"
        transition: background 120ms ease;

        &[data-selected='true'] {
            background: ${selected_bg};
            color: ${selected_color};
        }

        &[data-highlighted='true'] {
            outline: ${focus_width} solid ${focus_color};
            outline-offset: -${focus_offset};
        }

        &[data-disabled='true'] {
            opacity: 0.64;
        }
    "#,
        selected_bg = format!(
            "color-mix(in srgb, {} 12%, transparent)",
            theme.palette.primary.clone()
        ),
        selected_color = theme.palette.primary.clone(),
        focus_color = theme.palette.primary.clone(),
        focus_width = format!("{}px", theme.joy.focus_thickness.max(1)),
        focus_offset = format!("{:.1}px", (theme.joy.focus_thickness as f32) / 2.0),
    )
}

fn table_body_cell_style() -> Style {
    css_with_theme!(
        r#"
        padding: var(--mui-table-padding-y) var(--mui-table-padding-x);
        font-size: var(--mui-table-body-font-size);
        font-weight: var(--mui-table-body-font-weight);
        color: ${body_color};
        border-bottom: 1px solid ${divider};
        font-family: ${font_family};

        &[data-numeric='true'] {
            text-align: right;
            font-variant-numeric: tabular-nums;
        }
    "#,
        body_color = theme.palette.text_primary.clone(),
        divider = format!(
            "color-mix(in srgb, {} 12%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        font_family = theme.typography.font_family.clone(),
    )
}

pub mod yew {
    use super::*;

    /// Render the table into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &TableProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    /// Render the table into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &TableProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    /// Render the table into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &TableProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    /// Render the table into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &TableProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_state(row_count: usize) -> ListState {
        ListState::uncontrolled(row_count, &[], SelectionMode::Single)
    }

    fn sample_props() -> TableProps {
        TableProps::new(
            vec![
                TableColumn::new("Name"),
                TableColumn::new("Usage").numeric(),
            ],
            vec![
                TableRow::new(vec!["Objects".into(), "12".into()]),
                TableRow::new(vec!["Functions".into(), "8".into()]),
            ],
        )
        .with_selection_mode(SelectionMode::Single)
        .with_automation_id("sample-table")
    }

    #[test]
    fn table_attributes_include_grid_contract() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let attrs = table_attributes(&props, &state);
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "grid"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-activedescendant" && v.starts_with("sample-table")));
    }

    #[test]
    fn header_cell_attributes_include_scope_and_ids() {
        let props = sample_props();
        let attrs = header_cell_attributes(&props, &props.columns[1], 1);
        assert!(attrs.iter().any(|(k, v)| k == "scope" && v == "col"));
        assert!(attrs.iter().any(|(k, _)| k == "data-automation-column"));
    }

    #[test]
    fn row_markup_renders_cells_and_automation_hooks() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let html = super::render_html(&props, &state);
        assert!(html.contains("data-automation-cell"));
        assert!(html.contains("<table"));
        assert!(html.contains("mui-table"));
    }
}
