//! Material flavored tooltip renderer building on the headless [`TooltipState`](mui_headless::tooltip::TooltipState).
//!
//! The implementation mirrors [`button`](crate::button) by centralizing the
//! HTML + CSS assembly so that every framework adapter (Yew, Leptos, Dioxus and
//! Sycamore) simply forwards props/state into the shared helpers.  By keeping
//! the rendering logic in one place we guarantee:
//!
//! * **SSR parity** – the exact markup (including scoped classes) is emitted no
//!   matter which framework triggers the render.  This keeps automated diffing
//!   and streaming pipelines deterministic.
//! * **Theme awareness** – [`css_with_theme!`](mui_styled_engine::css_with_theme)
//!   pulls palette/typography/density tokens from the active
//!   [`Theme`](mui_styled_engine::Theme) so enterprise overrides cascade without
//!   touching this module.
//! * **Automation hooks** – consistent `data-*` attributes and ARIA metadata
//!   (`aria-describedby`, `aria-hidden`, etc.) are produced for QA suites and
//!   assistive technology.  Adapters do not need to duplicate this wiring which
//!   keeps accessibility audits centralized.
//!
//! The module intentionally contains no framework specific code.  Instead it
//! exposes lightweight adapters per framework that simply call
//! [`render_html`].  Downstream integrations wanting to hook into DOM portals or
//! generate custom layouts can re-use the smaller helper functions documented
//! below while still benefiting from the shared class/attribute logic.

use mui_headless::tooltip::{TooltipState, TooltipSurfaceAttributes, TooltipTriggerAttributes};
use mui_styled_engine::{css_with_theme, Style};
use mui_system::portal::PortalMount;

/// Shared tooltip properties consumed by every adapter.
#[derive(Clone, Debug)]
pub struct TooltipProps {
    /// Text or HTML fragment rendered inside the tooltip surface.
    pub tooltip: String,
    /// Content rendered inside the trigger element (typically a button or icon).
    pub trigger_label: String,
    /// Optional automation identifier propagated into `data-*` hooks and DOM ids.
    pub automation_id: Option<String>,
    /// Optional popup relationship announced to assistive technology.
    pub trigger_haspopup: Option<&'static str>,
    /// Optional identifier used to populate `aria-labelledby` on the surface.
    pub surface_labelled_by: Option<String>,
}

impl TooltipProps {
    /// Convenience constructor used by documentation examples and integration tests.
    pub fn new(trigger_label: impl Into<String>, tooltip: impl Into<String>) -> Self {
        Self {
            trigger_label: trigger_label.into(),
            tooltip: tooltip.into(),
            automation_id: None,
            trigger_haspopup: None,
            surface_labelled_by: None,
        }
    }

    /// Override the automation identifier.  This value flows into `data-*`
    /// attributes, DOM ids and portal containers which keeps SSR and client
    /// renders aligned for analytics and QA automation.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Attach an `aria-haspopup` relationship to the trigger (e.g. when the
    /// tooltip controls a rich popup like a menu or dialog).
    pub fn with_trigger_haspopup(mut self, value: &'static str) -> Self {
        self.trigger_haspopup = Some(value);
        self
    }

    /// Supply an identifier used for `aria-labelledby` on the tooltip surface.
    pub fn with_surface_labelled_by(mut self, id: impl Into<String>) -> Self {
        self.surface_labelled_by = Some(id.into());
        self
    }
}

/// Shared rendering routine invoked by every framework adapter.
///
/// The function wires themed classes, ARIA metadata and portal containers so
/// that SSR output exactly matches what client renderers expect.  Individual
/// adapters simply forward props/state keeping their implementations trivial and
/// guaranteeing that automation hooks stay in sync across frameworks.
fn render_html(props: &TooltipProps, state: &TooltipState) -> String {
    let base_id = automation_base(props);
    let trigger_id = trigger_id(&base_id);
    let surface_id = surface_id(&base_id);
    let portal = tooltip_portal(&base_id);

    // Attribute strings derived from themed styles + ARIA builders.  Keeping
    // them centralized ensures every adapter ships identical markup.
    let root_attrs = crate::style_helpers::themed_attributes_html(
        themed_root_style(),
        root_attributes(props, state, &portal, &base_id, &trigger_id, &surface_id),
    );
    let trigger_html = crate::render_helpers::render_element_html(
        "button",
        themed_trigger_style(),
        trigger_attributes(props, state, &portal, &trigger_id, &surface_id),
        &props.trigger_label,
    );
    let surface_html = crate::render_helpers::render_element_html(
        "div",
        themed_surface_style(),
        surface_attributes(props, state, &portal, &surface_id),
        &props.tooltip,
    );

    let anchor_html = portal.anchor_html();
    let portal_markup = portal.wrap(surface_html).into_html();

    format!(
        "<span {root_attrs}>{trigger_html}{anchor_html}</span>{portal_markup}",
        root_attrs = root_attrs,
        trigger_html = trigger_html,
        anchor_html = anchor_html,
        portal_markup = portal_markup
    )
}

/// Resolve the base automation identifier used to derive ids and data hooks.
fn automation_base(props: &TooltipProps) -> String {
    props
        .automation_id
        .clone()
        .unwrap_or_else(|| "mui-tooltip".into())
}

/// Compute the DOM id for the trigger element.
fn trigger_id(base: &str) -> String {
    format!("{base}-trigger")
}

/// Compute the DOM id for the tooltip surface.
fn surface_id(base: &str) -> String {
    format!("{base}-surface")
}

/// Construct the portal mount coordinating anchor + detached container markup.
fn tooltip_portal(base: &str) -> PortalMount {
    PortalMount::popover(base)
}

/// Attributes applied to the root span wrapping the trigger/anchor.
fn root_attributes(
    props: &TooltipProps,
    state: &TooltipState,
    portal: &PortalMount,
    base_id: &str,
    trigger_id: &str,
    surface_id: &str,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), base_id.to_string()));
    attrs.push(("data-component".into(), "mui-tooltip".into()));
    attrs.push(("data-visible".into(), state.visible().to_string()));
    attrs.push((
        "data-interactive".into(),
        state.config().interactive.to_string(),
    ));
    attrs.push((
        "data-dismissible".into(),
        state.config().dismissible.to_string(),
    ));
    attrs.push((
        "data-portal-layer".into(),
        portal.layer().as_str().to_string(),
    ));
    attrs.push(("data-trigger-id".into(), trigger_id.to_string()));
    attrs.push(("data-surface-id".into(), surface_id.to_string()));
    attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    attrs.push(("data-portal-root".into(), portal.container_id()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".into(), id.clone()));
    }
    attrs
}

/// Attributes applied to the interactive trigger.
fn trigger_attributes(
    props: &TooltipProps,
    state: &TooltipState,
    portal: &PortalMount,
    trigger_id: &str,
    surface_id: &str,
) -> Vec<(String, String)> {
    let mut builder = TooltipTriggerAttributes::new(state)
        .id(trigger_id)
        .described_by(surface_id);
    if let Some(kind) = props.trigger_haspopup {
        builder = builder.has_popup(kind);
    }

    let mut attrs = Vec::new();
    if let Some((key, value)) = builder.id_attr() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.describedby() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.haspopup() {
        attrs.push((key.into(), value.into()));
    }
    let (expanded_key, expanded_value) = builder.expanded();
    attrs.push((expanded_key.into(), expanded_value.into()));
    attrs.push(("aria-controls".into(), surface_id.to_string()));
    attrs.push(("type".into(), "button".into()));
    attrs.push(("data-component".into(), "mui-tooltip-trigger".into()));
    attrs.push(("data-visible".into(), state.visible().to_string()));
    attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    attrs.push(("data-portal-root".into(), portal.container_id()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-trigger".into(), id.clone()));
    }
    attrs
}

/// Attributes applied to the tooltip surface.
fn surface_attributes(
    props: &TooltipProps,
    state: &TooltipState,
    portal: &PortalMount,
    surface_id: &str,
) -> Vec<(String, String)> {
    let mut builder = TooltipSurfaceAttributes::new(state).id(surface_id);
    if let Some(labelled) = props.surface_labelled_by.as_deref() {
        builder = builder.labelled_by(labelled);
    }

    let mut attrs = Vec::new();
    attrs.push(("role".into(), builder.role().into()));
    if let Some((key, value)) = builder.id_attr() {
        attrs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = builder.labelledby() {
        attrs.push((key.into(), value.into()));
    }
    let (hidden_key, hidden_value) = builder.hidden();
    attrs.push((hidden_key.into(), hidden_value.into()));
    attrs.push(("data-component".into(), "mui-tooltip-surface".into()));
    attrs.push(("data-visible".into(), state.visible().to_string()));
    attrs.push((
        "data-interactive".into(),
        state.config().interactive.to_string(),
    ));
    attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    attrs.push(("data-portal-root".into(), portal.container_id()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-surface".into(), id.clone()));
    }
    attrs
}

/// Baseline wrapper style ensuring the trigger and anchor remain inline.
fn themed_root_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        position: relative;
        gap: ${gap};
    "#,
        gap = format!("{}px", theme.spacing(0)),
    )
}

/// Visual styling for the trigger element.
fn themed_trigger_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: ${gap};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        border: 1px solid ${border_color};
        background: ${background};
        color: ${text_color};
        font-family: ${font_family};
        font-size: ${font_size};
        line-height: ${line_height};
        cursor: help;
        transition: color 160ms ease, background-color 160ms ease, box-shadow 160ms ease;

        &:hover {
            color: ${hover_color};
            box-shadow: 0 0 0 ${focus_outline_width} ${focus_outline_color_transparent};
        }

        &:focus-visible {
            outline: ${focus_outline_width} solid ${focus_outline_color};
            outline-offset: 2px;
        }
    "#,
        gap = format!("{}px", theme.spacing(1) / 2),
        padding_y = format!("{}px", theme.spacing(1) / 2),
        padding_x = format!("{}px", theme.spacing(1)),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 32%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        background = theme.palette.background_paper.clone(),
        text_color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body2),
        line_height = format!("{:.2}", theme.typography.line_height),
        hover_color = theme.palette.primary.clone(),
        focus_outline_width = format!("{:.1}px", (theme.joy.focus_thickness as f32).max(1.0)),
        focus_outline_color = theme.palette.primary.clone(),
        focus_outline_color_transparent = format!(
            "color-mix(in srgb, {} 28%, transparent)",
            theme.palette.primary.clone()
        )
    )
}

/// Styling for the tooltip surface including elevation and transitions.
fn themed_surface_style() -> Style {
    css_with_theme!(
        r#"
        min-width: ${min_width};
        max-width: ${max_width};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        background: ${background};
        color: ${text_color};
        box-shadow: ${shadow};
        font-family: ${font_family};
        font-size: ${font_size};
        line-height: ${line_height};
        pointer-events: auto;
        transition: opacity 140ms ease, transform 140ms ease;
        transform-origin: top center;

        &[aria-hidden='true'] {
            opacity: 0;
            transform: translateY(-4px);
            pointer-events: none;
        }

        &[aria-hidden='false'] {
            opacity: 1;
            transform: translateY(0);
        }
    "#,
        min_width = format!("{}px", theme.spacing(16)),
        max_width = format!("{}px", theme.spacing(32)),
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(2)),
        radius = format!("{}px", theme.joy.radius),
        background = format!(
            "color-mix(in srgb, {} 92%, transparent)",
            theme.palette.neutral.clone()
        ),
        text_color = theme.palette.background_paper.clone(),
        shadow = "0px 8px 24px rgba(15, 23, 42, 0.25)".to_string(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.caption),
        line_height = format!("{:.2}", theme.typography.line_height),
    )
}

// ---------------------------------------------------------------------------
// Adapter implementations
// ---------------------------------------------------------------------------

/// Adapter targeting the [`yew`] framework.
pub mod yew {
    use super::*;

    /// Render the tooltip into a HTML string using the shared renderer.
    pub fn render(props: &TooltipProps, state: &TooltipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    /// Render the tooltip into a HTML string using the shared renderer.
    pub fn render(props: &TooltipProps, state: &TooltipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    /// Render the tooltip into a HTML string using the shared renderer.
    pub fn render(props: &TooltipProps, state: &TooltipState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    /// Render the tooltip into a HTML string using the shared renderer.
    pub fn render(props: &TooltipProps, state: &TooltipState) -> String {
        super::render_html(props, state)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::tooltip::TooltipConfig;

    #[test]
    fn render_html_includes_aria_and_portal_metadata() {
        let props = TooltipProps::new("Info", "Additional context");
        let state = TooltipState::new(TooltipConfig::default());
        let html = super::render_html(&props, &state);

        assert!(html.contains("data-component=\"mui-tooltip\""));
        assert!(html.contains("aria-describedby"));
        assert!(html.contains("role=\"tooltip\""));
        assert!(html.contains("data-portal-layer=\"popover\""));
    }

    #[test]
    fn trigger_attributes_include_expanded_state() {
        let props = TooltipProps::new("Help", "Tooltip");
        let state = TooltipState::new(TooltipConfig::default());
        let attrs = super::trigger_attributes(
            &props,
            &state,
            &tooltip_portal("mui-tooltip"),
            &trigger_id("mui-tooltip"),
            &surface_id("mui-tooltip"),
        );

        assert!(attrs.iter().any(|(k, _)| k == "aria-expanded"));
        assert!(attrs.iter().any(|(k, _)| k == "aria-describedby"));
    }
}
