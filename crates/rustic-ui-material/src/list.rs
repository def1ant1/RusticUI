//! Material themed list renderer built on top of [`rustic_ui_headless`] selection
//! primitives and the shared [`rustic_ui_system`] theme tokens.
//!
//! The module purposely centralizes HTML generation for every supported
//! framework so that Yew, Leptos, Dioxus and Sycamore adapters can simply call
//! [`render_html`] and attach the returned markup.  Styling flows through the
//! [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme) macro which extracts
//! spacing, palette and typography values from the active [`Theme`].  The
//! resulting CSS classes mirror Material's density knobs and typography ramp,
//! ensuring parity with upstream designs without duplicating constants across
//! crates.
//!
//! # Automation & accessibility
//! * Deterministic `data-rustic-list-*` hooks are emitted for the list root and
//!   every item, keeping QA pipelines stable across SSR and hydration.
//! * ARIA roles mirror the WAI-ARIA listbox design pattern when selection is
//!   enabled, including `aria-multiselectable` and `aria-activedescendant`
//!   wiring.
//! * Keyboard focus is tracked via [`ListState`] and reflected in
//!   `data-highlighted` attributes so adapters can scroll the active row into
//!   view without reimplementing business logic.

use rustic_ui_headless::list::{ListState, SelectionMode};
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Density options mirroring Material UI's list spacing presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListDensity {
    /// Default spacing: 16px vertical padding per item.
    Comfortable,
    /// Compact spacing: 8px vertical padding per item.
    Compact,
}

impl Default for ListDensity {
    fn default() -> Self {
        Self::Comfortable
    }
}

impl ListDensity {
    #[inline]
    pub(crate) fn data_value(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
        }
    }

    #[inline]
    pub(crate) fn vertical_padding(self) -> u16 {
        match self {
            Self::Comfortable => 2,
            Self::Compact => 1,
        }
    }

    #[inline]
    pub(crate) fn row_gap(self) -> u16 {
        match self {
            Self::Comfortable => 1,
            Self::Compact => 0,
        }
    }
}

/// Typography variants exposed for list items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListTypography {
    /// Body1 typography, 16px by default.
    Body1,
    /// Body2 typography, 14px by default.
    Body2,
    /// Subtitle1 typography, 16px medium weight.
    Subtitle1,
}

impl Default for ListTypography {
    fn default() -> Self {
        Self::Body1
    }
}

impl ListTypography {
    pub(crate) fn font_size(self, theme: &rustic_ui_styled_engine::Theme) -> f32 {
        match self {
            Self::Body1 => theme.typography.body1,
            Self::Body2 => theme.typography.body2,
            Self::Subtitle1 => theme.typography.subtitle1,
        }
    }

    pub(crate) fn font_weight(self, theme: &rustic_ui_styled_engine::Theme) -> u16 {
        match self {
            Self::Subtitle1 => theme.typography.font_weight_medium,
            _ => theme.typography.font_weight_regular,
        }
    }
}

/// Individual row rendered by the list.
#[derive(Clone, Debug, PartialEq)]
pub struct ListItem {
    /// Primary text displayed with the configured typography variant.
    pub primary: String,
    /// Optional supporting text rendered below the primary label.
    pub secondary: Option<String>,
    /// Optional metadata column rendered on the trailing edge.
    pub meta: Option<String>,
    /// Stable automation identifier appended to `data-rustic-list-item`.
    pub automation_id: Option<String>,
    /// Whether the row should be marked as disabled.
    pub disabled: bool,
}

impl ListItem {
    /// Convenience constructor for tests and demos.
    pub fn new(primary: impl Into<String>) -> Self {
        Self {
            primary: primary.into(),
            secondary: None,
            meta: None,
            automation_id: None,
            disabled: false,
        }
    }

    /// Sets the secondary supporting text.
    pub fn with_secondary(mut self, secondary: impl Into<String>) -> Self {
        self.secondary = Some(secondary.into());
        self
    }

    /// Sets the trailing metadata content.
    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Overrides the automation identifier suffix for the row.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Marks the row as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Shared configuration consumed by every framework adapter.
#[derive(Clone, Debug, PartialEq)]
pub struct ListProps {
    /// Collection of rows rendered inside the list container.
    pub items: Vec<ListItem>,
    /// Visual density applied to the list.
    pub density: ListDensity,
    /// Typography variant for the primary text slot.
    pub primary_typography: ListTypography,
    /// Typography variant for the optional secondary slot.
    pub secondary_typography: ListTypography,
    /// Selection mode forwarded to the headless state machine.
    pub selection_mode: SelectionMode,
    /// Optional automation identifier used as a prefix for generated hooks.
    pub automation_id: Option<String>,
}

impl ListProps {
    /// Creates a new [`ListProps`] instance with the supplied items and
    /// sensible defaults for density and typography.
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            items,
            density: ListDensity::default(),
            primary_typography: ListTypography::default(),
            secondary_typography: ListTypography::Body2,
            selection_mode: SelectionMode::None,
            automation_id: None,
        }
    }

    /// Overrides the density preset.
    pub fn with_density(mut self, density: ListDensity) -> Self {
        self.density = density;
        self
    }

    /// Overrides the typography variant used for the primary slot.
    pub fn with_primary_typography(mut self, variant: ListTypography) -> Self {
        self.primary_typography = variant;
        self
    }

    /// Overrides the typography variant used for the secondary slot.
    pub fn with_secondary_typography(mut self, variant: ListTypography) -> Self {
        self.secondary_typography = variant;
        self
    }

    /// Configures the selection mode.
    pub fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Specifies the automation identifier used to stamp deterministic
    /// `data-*` hooks.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }
}

/// Render the list into a SSR friendly HTML string.
fn render_html(props: &ListProps, state: &ListState) -> String {
    let root_attrs = crate::style_helpers::themed_attributes_html(
        list_style(props),
        root_attributes(props, state),
    );

    let mut items_html = String::new();
    for (index, item) in props.items.iter().enumerate() {
        let item_attrs = crate::style_helpers::themed_attributes_html(
            list_item_style(),
            item_attributes(props, state, item, index),
        );
        items_html.push_str(&format!("<li {item_attrs}>{}</li>", item_markup(item)));
    }

    format!("<ul {root_attrs}>{items_html}</ul>")
}

fn automation_base(props: &ListProps) -> String {
    crate::style_helpers::automation_id("list", props.automation_id.as_deref(), [])
}

fn item_automation_id(props: &ListProps, item: &ListItem, index: usize) -> String {
    if let Some(id) = &item.automation_id {
        crate::style_helpers::automation_id("list", props.automation_id.as_deref(), [id])
    } else {
        crate::style_helpers::automation_id(
            "list",
            props.automation_id.as_deref(),
            [format!("item-{index}")],
        )
    }
}

fn item_id(props: &ListProps, index: usize) -> String {
    crate::style_helpers::automation_id(
        "list",
        props.automation_id.as_deref(),
        [format!("option-{index}")],
    )
}

fn root_attributes(props: &ListProps, state: &ListState) -> Vec<(String, String)> {
    let mut attrs = vec![
        (
            "data-component".into(),
            crate::style_helpers::automation_id("list", None, []),
        ),
        ("data-density".into(), props.density.data_value().into()),
        (
            "data-selection-mode".into(),
            match props.selection_mode {
                SelectionMode::None => "none".into(),
                SelectionMode::Single => "single".into(),
                SelectionMode::Multiple => "multiple".into(),
            },
        ),
    ];

    match props.selection_mode {
        SelectionMode::None => {
            attrs.push(("role".into(), "list".into()));
        }
        SelectionMode::Single | SelectionMode::Multiple => {
            attrs.push(("role".into(), "listbox".into()));
            attrs.push(("tabindex".into(), "0".into()));
            if props.selection_mode == SelectionMode::Multiple {
                attrs.push(("aria-multiselectable".into(), "true".into()));
            }
            if let Some(highlight) = state.highlighted() {
                attrs.push(("aria-activedescendant".into(), item_id(props, highlight)));
            }
        }
    }

    let base_id = automation_base(props);
    attrs.push((
        crate::style_helpers::automation_data_attr("list", ["id"]),
        base_id.clone(),
    ));
    attrs.push((
        crate::style_helpers::automation_data_attr("list", ["root"]),
        crate::style_helpers::automation_id("list", props.automation_id.as_deref(), ["root"]),
    ));

    attrs
}

fn item_attributes(
    props: &ListProps,
    state: &ListState,
    item: &ListItem,
    index: usize,
) -> Vec<(String, String)> {
    let mut attrs = vec![
        ("id".to_string(), item_id(props, index)),
        ("data-index".to_string(), index.to_string()),
        (
            "data-density".to_string(),
            props.density.data_value().to_string(),
        ),
        (
            "data-selected".to_string(),
            state.is_selected(index).to_string(),
        ),
        (
            "data-highlighted".to_string(),
            (state.highlighted() == Some(index)).to_string(),
        ),
        ("data-disabled".to_string(), item.disabled.to_string()),
    ];

    attrs.push((
        crate::style_helpers::automation_data_attr("list", ["item"]),
        item_automation_id(props, item, index),
    ));

    match props.selection_mode {
        SelectionMode::None => {
            attrs.push(("role".to_string(), String::from("listitem")));
        }
        _ => {
            attrs.push(("role".to_string(), String::from("option")));
            attrs.push((
                "aria-selected".to_string(),
                state.is_selected(index).to_string(),
            ));
        }
    }

    if item.disabled {
        attrs.push(("aria-disabled".to_string(), String::from("true")));
    }

    attrs
}

fn item_markup(item: &ListItem) -> String {
    let mut html = String::new();
    html.push_str(&format!(
        "<div class=\"rustic_ui_list_content\"><span class=\"rustic_ui_list_primary\">{}</span>",
        item.primary
    ));
    if let Some(secondary) = &item.secondary {
        html.push_str(&format!(
            "<span class=\"rustic_ui_list_secondary\">{secondary}</span>"
        ));
    }
    html.push_str("</div>");
    if let Some(meta) = &item.meta {
        html.push_str(&format!(
            "<span class=\"rustic_ui_list_meta\">{meta}</span>"
        ));
    }
    html
}

fn list_style(props: &ListProps) -> Style {
    let density = props.density;
    css_with_theme!(
        r#"
        list-style: none;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        gap: ${gap};
        background: ${background};
        border-radius: ${radius};
        border: 1px solid ${border_color};
        overflow: hidden;
        --rustic_ui_list_padding_y: ${padding_y};
        --rustic_ui_list_padding_x: ${padding_x};
        --rustic_ui_list_gap: ${item_gap};
        --rustic_ui_list_primary_font_size: ${primary_size};
        --rustic_ui_list_primary_font_weight: ${primary_weight};
        --rustic_ui_list_secondary_font_size: ${secondary_size};
        --rustic_ui_list_secondary_font_weight: ${secondary_weight};
    "#,
        gap = format!("{}px", theme.spacing(density.row_gap())),
        background = theme.palette.background_paper.clone(),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 18%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        padding_y = format!("{}px", theme.spacing(density.vertical_padding())),
        padding_x = format!("{}px", theme.spacing(2)),
        item_gap = format!("{}px", theme.spacing(1)),
        primary_size = format!("{:.3}rem", props.primary_typography.font_size(&theme)),
        primary_weight = props.primary_typography.font_weight(&theme).to_string(),
        secondary_size = format!("{:.3}rem", props.secondary_typography.font_size(&theme)),
        secondary_weight = props.secondary_typography.font_weight(&theme).to_string(),
    )
}

fn list_item_style() -> Style {
    css_with_theme!(
        r#"
        display: grid;
        grid-template-columns: 1fr auto;
        align-items: center;
        column-gap: var(--rustic_ui_list_gap);
        padding: var(--rustic_ui_list_padding_y) var(--rustic_ui_list_padding_x);
        background: transparent;
        color: ${text_color};
        font-family: ${font_family};
        transition: background 120ms ease, color 120ms ease;
        cursor: pointer;
        border-bottom: 1px solid ${divider};

        &:last-child {
            border-bottom: none;
        }

        &[data-selected='true'] {
            background: ${selected_bg};
            color: ${selected_color};
        }

        &[data-highlighted='true'] {
            outline: ${focus_width} solid ${focus_color};
            outline-offset: -${focus_offset};
        }

        &[data-disabled='true'] {
            color: ${disabled_color};
            cursor: not-allowed;
            opacity: 0.64;
        }

        .rustic_ui_list_content {
            display: flex;
            flex-direction: column;
            gap: ${content_gap};
        }

        .rustic_ui_list_primary {
            font-size: var(--rustic_ui_list_primary_font_size);
            font-weight: var(--rustic_ui_list_primary_font_weight);
            line-height: ${line_height};
        }

        .rustic_ui_list_secondary {
            font-size: var(--rustic_ui_list_secondary_font_size);
            font-weight: var(--rustic_ui_list_secondary_font_weight);
            line-height: ${line_height};
            color: ${secondary_color};
        }

        .rustic_ui_list_meta {
            justify-self: end;
            font-size: var(--rustic_ui_list_secondary_font_size);
            color: ${meta_color};
        }
    "#,
        text_color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        divider = format!(
            "color-mix(in srgb, {} 14%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        selected_bg = format!(
            "color-mix(in srgb, {} 12%, transparent)",
            theme.palette.primary.clone()
        ),
        selected_color = theme.palette.primary.clone(),
        focus_color = theme.palette.primary.clone(),
        focus_width = format!("{}px", theme.joy.focus.thickness.max(1)),
        focus_offset = format!("{:.1}px", (theme.joy.focus.thickness as f32) / 2.0),
        disabled_color = theme.palette.text_secondary.clone(),
        content_gap = format!("{}px", theme.spacing(0)),
        line_height = format!("{:.2}", theme.typography.line_height),
        secondary_color = theme.palette.text_secondary.clone(),
        meta_color = theme.palette.text_secondary.clone(),
    )
}

pub mod yew {
    use super::*;

    /// Render the list into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &ListProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    /// Render the list into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &ListProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    /// Render the list into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &ListProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    /// Render the list into HTML markup for SSR/hydration pipelines.
    pub fn render(props: &ListProps, state: &ListState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn build_state(item_count: usize) -> ListState {
        ListState::uncontrolled(item_count, &[], SelectionMode::Single)
    }

    fn sample_props() -> ListProps {
        ListProps::new(vec![
            ListItem::new("Inbox").with_secondary("23 new"),
            ListItem::new("Archive").with_meta("2 GB"),
        ])
        .with_selection_mode(SelectionMode::Single)
        .with_automation_id("sample-list")
    }

    #[test]
    fn root_attributes_include_aria_contract() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let attrs = root_attributes(&props, &state);
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "listbox"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-activedescendant" && v.starts_with("sample-list")));
    }

    #[test]
    fn item_attributes_toggle_selected_flags() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.toggle(1, |_| {});
        let attrs = item_attributes(&props, &state, &props.items[1], 1);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-selected" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-rustic-list-item" && v.contains("rustic-list-sample-list")));
    }

    #[test]
    fn render_html_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = super::render_html(&props, &state);
        assert!(html.contains("data-component=\"rustic-list\""));
        assert!(html.contains("class=\"rustic_ui_list_primary\""));
        assert!(html.contains("data-rustic-list-item"));
        assert!(html.contains("<ul"));
    }
}
