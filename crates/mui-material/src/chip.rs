//! Material flavored chip renderer derived from the headless [`ChipState`](mui_headless::chip::ChipState).
//!
//! Similar to [`tooltip`](crate::tooltip) and [`button`](crate::button) the
//! component centralizes markup, ARIA wiring and CSS so that every framework
//! adapter simply forwards props/state into the shared helpers.  This eliminates
//! the need for Yew/Leptos/Dioxus/Sycamore integrations to replicate styling or
//! automation hooks, ensuring SSR output remains deterministic regardless of the
//! runtime.  All visual decisions flow through [`css_with_theme!`](mui_styled_engine::css_with_theme)
//! which means palette, typography and density overrides propagate automatically
//! from the active [`Theme`](mui_styled_engine::Theme).
//!
//! The module emits a single `<div>` representing the chip root and (optionally)
//! a trailing delete button when the configuration is dismissible.  Both share
//! scoped classes generated from the styled engine so server renders and client
//! hydration always agree on class names.  Extensive `data-*` attributes are
//! included for large automation suites that need deterministic selectors across
//! frameworks and render modes.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use mui_headless::chip::{ChipConfig, ChipState};
//! use mui_material::chip::{yew as chip_yew, ChipProps};
//! use mui_styled_engine::{StyleRegistry, Theme};
//!
//! let mut theme = Theme::default();
//! theme.palette.secondary = "#D81B60".into();
//! let registry = StyleRegistry::new(theme.clone());
//!
//! let mut state = ChipState::new(ChipConfig::enterprise_defaults());
//! state.focus();
//! state.poll();
//!
//! let props = ChipProps::new("Escalated")
//!     .with_automation_id("feedback-chip")
//!     .with_delete_label("remove escalation");
//!
//! let html = chip_yew::render(&props, &state);
//! assert!(html.contains("data-component=\"mui-chip\""));
//! assert!(html.contains("data-automation-id=\"feedback-chip\""));
//!
//! // Style collection mirrors the tooltip story so SSR snapshots remain themed.
//! let _ = registry.style_manager();
//! let _ = registry.flush_styles();
//! ```
//!
//! See [`examples/feedback-chips`](../../examples/feedback-chips) for a
//! multi-framework bootstrapper that renders dismissible and read-only chips
//! with automation hooks pre-wired for analytics pipelines.

use mui_headless::chip::{ChipAttributes, ChipDeleteAttributes, ChipState};
use mui_styled_engine::{css_with_theme, Style};

/// Shared properties consumed by every chip adapter.
#[derive(Clone, Debug)]
pub struct ChipProps {
    /// Label rendered inside the chip.  Typically short text describing a filter or tag.
    pub label: String,
    /// Optional automation identifier propagated into `data-*` hooks and DOM ids.
    pub automation_id: Option<String>,
    /// Accessible label for the delete affordance.
    pub delete_label: Option<String>,
    /// Visual glyph rendered inside the delete affordance.
    pub delete_icon: String,
    /// Mirrors [`ChipConfig::dismissible`] so render output matches behaviour.
    pub dismissible: bool,
}

impl ChipProps {
    /// Construct a chip with sensible defaults aligned with Material's baseline.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            automation_id: None,
            delete_label: Some("Remove".into()),
            delete_icon: "✕".into(),
            dismissible: true,
        }
    }

    /// Override the automation identifier used for deterministic selectors.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Override the accessible label applied to the delete button.
    pub fn with_delete_label(mut self, label: impl Into<String>) -> Self {
        self.delete_label = Some(label.into());
        self
    }

    /// Override the glyph rendered inside the delete affordance.
    pub fn with_delete_icon(mut self, icon: impl Into<String>) -> Self {
        self.delete_icon = icon.into();
        self
    }

    /// Toggle whether the chip exposes a delete affordance.
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}

/// Shared rendering routine used by SSR and hydration aware adapters.
fn render_html(props: &ChipProps, state: &ChipState) -> String {
    let base_id = automation_base(props);
    let label_id = label_id(&base_id);
    let delete_id = delete_id(&base_id);

    let root_attrs = crate::style_helpers::themed_attributes_html(
        themed_root_style(),
        root_attributes(props, state, &base_id, &label_id, &delete_id),
    );
    let label_html = crate::render_helpers::render_element_html(
        "span",
        themed_label_style(),
        label_attributes(&label_id),
        &props.label,
    );

    let delete_html = if props.dismissible {
        Some(crate::render_helpers::render_element_html(
            "button",
            themed_delete_style(),
            delete_attributes(props, state, &delete_id),
            &props.delete_icon,
        ))
    } else {
        None
    };

    let mut children = String::new();
    children.push_str(&label_html);
    if let Some(html) = delete_html {
        children.push_str(&html);
    }

    format!("<div {root_attrs}>{children}</div>")
}

/// Resolve the automation identifier base.
fn automation_base(props: &ChipProps) -> String {
    props
        .automation_id
        .clone()
        .unwrap_or_else(|| "mui-chip".into())
}

/// DOM id for the label span.
fn label_id(base: &str) -> String {
    format!("{base}-label")
}

/// DOM id for the delete button.
fn delete_id(base: &str) -> String {
    format!("{base}-delete")
}

/// Attribute map for the chip root element.
fn root_attributes(
    props: &ChipProps,
    state: &ChipState,
    base_id: &str,
    label_id: &str,
    delete_id: &str,
) -> Vec<(String, String)> {
    let mut builder = ChipAttributes::new(state).id(base_id).labelled_by(label_id);
    if props.dismissible {
        builder = builder.described_by(delete_id);
    }

    let mut attrs = Vec::new();
    attrs.push(("role".into(), builder.role().into()));
    if let Some((key, value)) = builder.id_attr() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.labelledby() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.describedby() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((disabled_key, disabled_value)) = builder.disabled() {
        attrs.push((disabled_key.into(), disabled_value));
    }
    let (hidden_key, hidden_value) = builder.hidden();
    attrs.push((hidden_key.into(), hidden_value.into()));
    attrs.push((
        "tabindex".into(),
        if state.disabled() { "-1" } else { "0" }.into(),
    ));
    attrs.push(("data-component".into(), "mui-chip".into()));
    attrs.push(("data-visible".into(), state.is_visible().to_string()));
    attrs.push((
        "data-controls-visible".into(),
        state.controls_visible().to_string(),
    ));
    attrs.push((
        "data-deletion-pending".into(),
        state.deletion_pending().to_string(),
    ));
    if let Some((data_key, data_value)) = builder.data_disabled() {
        attrs.push((data_key.into(), data_value));
    }
    attrs.push(("data-dismissible".into(), props.dismissible.to_string()));
    attrs.push(("data-label-id".into(), label_id.to_string()));
    attrs.push(("data-delete-id".into(), delete_id.to_string()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".into(), id.clone()));
    }
    attrs
}

/// Attributes for the label span.
fn label_attributes(label_id: &str) -> Vec<(String, String)> {
    vec![
        ("id".into(), label_id.to_string()),
        ("data-chip-slot".into(), "label".into()),
    ]
}

/// Attributes for the delete button.
fn delete_attributes(
    props: &ChipProps,
    state: &ChipState,
    delete_id: &str,
) -> Vec<(String, String)> {
    let mut builder =
        ChipDeleteAttributes::new(state).label(props.delete_label.as_deref().unwrap_or("Remove"));

    let mut attrs = Vec::new();
    attrs.push(("id".into(), delete_id.to_string()));
    attrs.push(("type".into(), "button".into()));
    attrs.push(("data-chip-slot".into(), "delete".into()));
    attrs.push(("data-visible".into(), state.controls_visible().to_string()));
    attrs.push((
        "data-deletion-pending".into(),
        state.deletion_pending().to_string(),
    ));
    attrs.push(("data-disabled".into(), state.disabled().to_string()));
    attrs.push(("role".into(), builder.role().into()));
    let (hidden_key, hidden_value) = builder.hidden();
    attrs.push((hidden_key.into(), hidden_value.into()));
    if let Some((key, value)) = builder.aria_label() {
        attrs.push((key.into(), value.into()));
    }
    attrs
}

/// Root container styling.
fn themed_root_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        background: ${background};
        color: ${text_color};
        box-shadow: 0 0 0 1px ${border_color};
        font-family: ${font_family};
        font-size: ${font_size};
        line-height: ${line_height};
        cursor: pointer;
        transition: background-color 160ms ease, box-shadow 160ms ease, opacity 200ms ease;

        &[data-disabled='true'] {
            cursor: not-allowed;
            opacity: 0.6;
        }

        &[data-visible='false'] {
            opacity: 0;
            pointer-events: none;
        }

        &:focus-visible {
            outline: ${focus_width} solid ${focus_color};
            outline-offset: 2px;
        }
    "#,
        gap = format!("{}px", theme.spacing(1)),
        padding_y = format!("{}px", theme.spacing(1) / 2),
        padding_x = format!("{}px", theme.spacing(1)),
        radius = format!("{}px", theme.joy.radius),
        background = theme.palette.background_paper.clone(),
        text_color = theme.palette.text_primary.clone(),
        border_color = format!(
            "color-mix(in srgb, {} 28%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body2),
        line_height = format!("{:.2}", theme.typography.line_height),
        focus_width = format!("{:.1}px", (theme.joy.focus.thickness as f32).max(1.0)),
        focus_color = theme.palette.primary.clone(),
    )
}

/// Styling for the label span.
fn themed_label_style() -> Style {
    css_with_theme!(
        r#"
        font-weight: ${font_weight};
        letter-spacing: ${letter_spacing};
    "#,
        font_weight = theme.typography.font_weight_medium.to_string(),
        letter_spacing = format!("{:.3}rem", theme.typography.button_letter_spacing),
    )
}

/// Styling for the delete button.
fn themed_delete_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: ${size};
        height: ${size};
        border: none;
        border-radius: ${radius};
        background: transparent;
        color: ${icon_color};
        font-family: ${font_family};
        font-size: ${font_size};
        cursor: pointer;
        transition: background-color 160ms ease, color 160ms ease, opacity 200ms ease;

        &[aria-hidden='true'] {
            opacity: 0;
            pointer-events: none;
        }

        &[aria-hidden='false'] {
            opacity: 1;
        }

        &:focus-visible {
            outline: ${focus_width} solid ${focus_color};
            outline-offset: 2px;
        }
    "#,
        size = format!("{}px", theme.spacing(3)),
        radius = format!("{}px", theme.joy.radius / 2),
        icon_color = theme.palette.text_secondary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.caption),
        focus_width = format!("{:.1}px", (theme.joy.focus.thickness as f32).max(1.0)),
        focus_color = theme.palette.primary.clone(),
    )
}

// ---------------------------------------------------------------------------
// Adapter implementations
// ---------------------------------------------------------------------------

/// Adapter targeting server rendered React environments.
///
/// Server driven React stacks often diff SSR output against client renders to
/// guarantee hydration fidelity. Providing a dedicated adapter that reuses the
/// shared [`render_html`] helper keeps those comparisons simple and avoids
/// duplicating style orchestration in automation harnesses.
pub mod react {
    use super::*;

    /// Render the chip into a HTML string that mirrors the other framework
    /// adapters. Keeping the implementation as a thin wrapper over
    /// [`super::render_html`] ensures every integration emits identical markup
    /// and scoped class names.
    pub fn render(props: &ChipProps, state: &ChipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`yew`] framework.
pub mod yew {
    use super::*;

    /// Render the chip into a HTML string using the shared renderer.
    pub fn render(props: &ChipProps, state: &ChipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    /// Render the chip into a HTML string using the shared renderer.
    pub fn render(props: &ChipProps, state: &ChipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    /// Render the chip into a HTML string using the shared renderer.
    pub fn render(props: &ChipProps, state: &ChipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    /// Render the chip into a HTML string using the shared renderer.
    pub fn render(props: &ChipProps, state: &ChipState) -> String {
        super::render_html(props, state)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::chip::ChipConfig;

    #[test]
    fn render_html_includes_delete_affordance() {
        let props = ChipProps::new("Filters");
        let state = ChipState::new(ChipConfig::default());
        let html = super::render_html(&props, &state);

        assert!(html.contains("data-component=\"mui-chip\""));
        assert!(html.contains("data-chip-slot=\"delete\""));
        assert!(html.contains("aria-hidden"));
    }

    #[test]
    fn non_dismissible_chips_omit_delete_button() {
        let props = ChipProps::new("Static").with_dismissible(false);
        let mut config = ChipConfig::default();
        config.dismissible = false;
        let state = ChipState::new(config);
        let html = super::render_html(&props, &state);

        assert!(!html.contains("data-chip-slot=\"delete\""));
    }
}
