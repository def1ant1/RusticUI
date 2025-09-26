//! Material themed select/listbox renderer built atop the headless [`SelectState`].
//!
//! The module mirrors the structure of [`button`](crate::button) by centralizing
//! HTML assembly and [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme)
//! styling in one place so framework adapters simply forward props/state into the
//! shared helpers.  This keeps enterprise surfaces consistent across SSR and
//! client runtimes while leaning on design tokens sourced from
//! [`Theme`](rustic_ui_styled_engine::Theme) for every visual decision.
//!
//! ## Why this lives in `mui-material`
//! * Rendering is centralized to prevent divergence between Yew, Leptos, Dioxus
//!   and Sycamore adapters.
//! * Styles derive from the active theme instead of literal CSS which allows
//!   brand palettes, typography ramps and spacing tokens to flow automatically
//!   through every select.
//! * Automation hooks (`data-*` attributes) are standardized so QA teams can
//!   target components reliably regardless of hosting framework.

use rustic_ui_headless::select::SelectState;
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_system::portal::PortalMount;

/// Discrete option rendered inside the Material select popover.
#[derive(Clone, Debug)]
pub struct SelectOption {
    /// Human readable label presented to end users.
    pub label: String,
    /// Machine readable value emitted when the option is selected.
    pub value: String,
}

impl SelectOption {
    /// Convenience constructor used by examples and integration tests.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// Props shared across all framework adapters.
#[derive(Clone, Debug)]
pub struct SelectProps {
    /// Text rendered inside the trigger button.
    pub label: String,
    /// Collection of options displayed in the popover.
    pub options: Vec<SelectOption>,
    /// Optional automation identifier used to stamp deterministic `data-*`
    /// attributes for end-to-end tests.
    pub automation_id: Option<String>,
}

impl SelectProps {
    /// Convenience constructor for tests and documentation snippets.
    pub fn new(label: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            label: label.into(),
            options,
            automation_id: None,
        }
    }

    /// Override the automation identifier enabling deterministic selector
    /// generation across SSR and client renders.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }
}

/// Shared rendering routine used by SSR adapters.
fn render_html(props: &SelectProps, state: &SelectState) -> String {
    let portal = popover_mount(props);
    // Compute the attribute strings for each element.  The helper automatically
    // attaches the generated class from the themed `Style` alongside ARIA and
    // data hooks.  Centralizing this ensures hydration order matches server
    // output and keeps automation selectors consistent across frameworks.
    let root_attrs = crate::style_helpers::themed_attributes_html(
        themed_root_style(),
        root_attributes(props, state, &portal),
    );
    let trigger_attrs = crate::style_helpers::themed_attributes_html(
        themed_trigger_style(),
        trigger_attributes(props, state, &portal),
    );
    let list_attrs = crate::style_helpers::themed_attributes_html(
        themed_list_style(),
        list_attributes(props, state, &portal),
    );

    // Render each option with its own themed attributes.  We intentionally keep
    // this loop declarative so adapters never need to hand-roll HTML when
    // updating or testing the component.
    let mut options_html = String::new();
    for (index, option) in props.options.iter().enumerate() {
        let option_attrs = crate::style_helpers::themed_attributes_html(
            themed_option_style(),
            option_attributes(props, state, index),
        );
        options_html.push_str(&format!("<li {option_attrs}>{}</li>", option.label));
    }

    let anchor_html = portal.anchor_html();
    let popover_markup = portal.wrap(format!("<ul {list_attrs}>{options_html}</ul>"));

    format!(
        "<div {root_attrs}><button {trigger_attrs}>{}</button>{}</div>{}",
        props.label,
        anchor_html,
        popover_markup.into_html()
    )
}

/// Resolve the automation identifier used for data hooks and DOM ids.
fn automation_base(props: &SelectProps) -> String {
    props
        .automation_id
        .clone()
        .unwrap_or_else(|| "mui-select".into())
}

/// Compute the DOM id for the option list.
fn list_id(props: &SelectProps) -> String {
    format!("{}-list", automation_base(props))
}

/// Compute the DOM id for a given option.
fn option_id(props: &SelectProps, index: usize) -> String {
    format!("{}-option-{index}", automation_base(props))
}

/// Build the attribute map for the root container.
fn root_attributes(
    props: &SelectProps,
    state: &SelectState,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("data-component".into(), "mui-select".into()));
    attrs.push(("data-open".into(), state.is_open().to_string()));
    attrs.push((
        "data-portal-layer".into(),
        portal.layer().as_str().to_string(),
    ));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".into(), id.clone()));
    }
    attrs
}

/// Build the attribute map for the trigger button.
fn trigger_attributes(
    props: &SelectProps,
    state: &SelectState,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("role".into(), state.trigger_role().into()));
    let (key, value) = state.trigger_haspopup();
    attrs.push((key.into(), value.into()));
    let (expanded_key, expanded_value) = state.trigger_expanded();
    attrs.push((expanded_key.into(), expanded_value.into()));
    attrs.push(("aria-controls".into(), list_id(props)));
    attrs.push(("data-open".into(), state.is_open().to_string()));
    attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    attrs.push(("data-portal-root".into(), portal.container_id()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-trigger".into(), id.clone()))
    }
    attrs
}

/// Build the attribute map for the listbox container.
fn list_attributes(
    props: &SelectProps,
    state: &SelectState,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), list_id(props)));
    attrs.push(("role".into(), state.list_role().into()));
    attrs.push(("aria-hidden".into(), (!state.is_open()).to_string()));
    if let Some(highlighted) = state.highlighted() {
        attrs.push((
            "aria-activedescendant".into(),
            option_id(props, highlighted),
        ));
    }
    attrs.push(("data-open".into(), state.is_open().to_string()));
    attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    attrs.push(("data-portal-root".into(), portal.container_id()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-list".into(), id.clone()));
    }
    attrs
}

/// Build the attribute map for an individual option.
fn option_attributes(
    props: &SelectProps,
    state: &SelectState,
    index: usize,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), option_id(props, index)));
    for (key, value) in state.option_accessibility_attributes(index) {
        attrs.push((key.into(), value));
    }
    let is_selected = state.selected() == Some(index);
    let is_highlighted = state.highlighted() == Some(index);
    attrs.push(("aria-selected".into(), is_selected.to_string()));
    attrs.push(("data-selected".into(), is_selected.to_string()));
    attrs.push(("data-highlighted".into(), is_highlighted.to_string()));
    attrs.push(("data-index".into(), index.to_string()));
    attrs.push(("data-value".into(), props.options[index].value.clone()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-option".into(), format!("{id}-{index}")));
    }
    attrs
}

fn popover_mount(props: &SelectProps) -> PortalMount {
    let base = format!("{}-popover", automation_base(props));
    PortalMount::popover(base)
}

/// Baseline wrapper style ensuring the select trigger and list share consistent
/// spacing.  The macro leans on global spacing tokens so enterprise teams can
/// scale density centrally without visiting every component.
fn themed_root_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        flex-direction: column;
        gap: ${gap};
        position: relative;
    "#,
        gap = format!("{}px", theme.spacing(0)),
    )
}

/// Button style responsible for rendering the trigger element.
///
/// Palette tokens flow directly from the theme ensuring brand overrides cascade
/// automatically.  Inline notes document how the generated CSS leans on design
/// tokens instead of hard coded values which keeps enterprise rollouts
/// repeatable.
fn themed_trigger_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: space-between;
        min-width: ${min_width};
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        border: 1px solid ${border_color};
        background: ${background};
        color: ${text_color};
        font-family: ${font_family};
        font-size: ${font_size};
        cursor: pointer;
        transition: border-color 160ms ease, box-shadow 160ms ease;

        &[data-open='true'] {
            border-color: ${focus_outline_color};
            box-shadow: 0 0 0 ${focus_outline_width} ${focus_outline_color_transparent};
        }
    "#,
        min_width = format!("{}px", theme.spacing(18)),
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(2)),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 40%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        background = theme.palette.background_paper.clone(),
        text_color = theme.palette.text_primary.clone(),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body1),
        focus_outline_color = theme.palette.primary.clone(),
        focus_outline_width = format!("{:.1}px", (theme.joy.focus.thickness as f32).max(1.0) / 2.0),
        focus_outline_color_transparent = format!(
            "color-mix(in srgb, {} 24%, transparent)",
            theme.palette.primary.clone()
        )
    )
}

/// Listbox styling controlling elevation, padding and scroll behavior.
///
/// We prefer to express the shadow and radius in terms of Joy tokens so the
/// dropdown visually aligns with menus, dialogs and other popovers.
fn themed_list_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        top: calc(100% + ${offset});
        left: 0;
        right: 0;
        max-height: ${max_height};
        overflow-y: auto;
        margin: 0;
        padding: ${padding};
        list-style: none;
        border-radius: ${radius};
        border: 1px solid ${border_color};
        background: ${background};
        box-shadow: ${shadow};
        z-index: 10;
        display: none;

        &[data-open='true'] {
            display: block;
        }
    "#,
        offset = format!("{}px", theme.spacing(1)),
        max_height = format!("{}px", theme.spacing(24)),
        padding = format!("{}px", theme.spacing(1)),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 40%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        background = theme.palette.background_paper.clone(),
        shadow = format!(
            "0 12px 24px color-mix(in srgb, {} 16%, transparent)",
            theme.palette.text_primary.clone()
        )
    )
}

/// Style applied to individual list options.
///
/// The macro leans on palette surface/hover tokens, while `data-highlighted`
/// drives hover/keyboard focus affordances so automation hooks can assert the
/// same state used for styling.
fn themed_option_style() -> Style {
    css_with_theme!(
        r#"
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        cursor: pointer;
        font-family: ${font_family};
        font-size: ${font_size};
        color: ${text_color};

        &[data-highlighted='true'],
        &:hover {
            background: ${hover_background};
        }

        &[data-selected='true'] {
            font-weight: ${font_weight};
        }
    "#,
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(2)),
        radius = format!("{:.1}px", (theme.joy.radius as f32) / 2.0),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body2),
        text_color = theme.palette.text_primary.clone(),
        hover_background = format!(
            "color-mix(in srgb, {} 12%, {})",
            theme.palette.primary.clone(),
            theme.palette.background_paper.clone()
        ),
        font_weight = theme.typography.font_weight_medium.to_string()
    )
}

/// Adapter targeting the [`yew`] framework.
pub mod yew {
    use super::*;

    /// Render the select into a HTML string using the shared renderer.
    pub fn render(props: &SelectProps, state: &SelectState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    /// Render the select into a HTML string using the shared renderer.
    pub fn render(props: &SelectProps, state: &SelectState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    /// Render the select into a HTML string using the shared renderer.
    pub fn render(props: &SelectProps, state: &SelectState) -> String {
        super::render_html(props, state)
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    /// Render the select into a HTML string using the shared renderer.
    pub fn render(props: &SelectProps, state: &SelectState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_state(option_count: usize) -> SelectState {
        SelectState::new(
            option_count,
            None,
            false,
            // `ControlStrategy` lives in a private module inside `mui-headless`.
            // The discriminant order is stable (documented within that crate),
            // so the test recreates the `Uncontrolled` variant via transmute to
            // keep the public surface lean while still exercising integration.
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        )
    }

    fn sample_props() -> SelectProps {
        SelectProps::new(
            "Choose",
            vec![SelectOption::new("One", "1"), SelectOption::new("Two", "2")],
        )
        .with_automation_id("sample")
    }

    #[test]
    fn trigger_attributes_include_aria_contract() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let portal = popover_mount(&props);
        let attrs = trigger_attributes(&props, &state, &portal);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-haspopup" && v == "listbox"));
        assert!(attrs.iter().any(|(k, _)| k == "aria-controls"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-root" && v.ends_with("-portal")));
    }

    #[test]
    fn list_attributes_link_to_highlight() {
        let mut state = build_state(2);
        state.set_highlighted(Some(1));
        let props = sample_props();
        let portal = popover_mount(&props);
        let attrs = list_attributes(&props, &state, &portal);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-activedescendant" && v.ends_with("-option-1")));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-anchor" && v.ends_with("-anchor")));
    }

    #[test]
    fn render_html_emits_data_hooks() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = render_html(&props, &state);
        assert!(html.contains("data-automation-id=\"sample\""));
        assert!(html.contains("data-value=\"1\""));
        assert!(html.contains("data-portal-root"));
        assert!(html.contains("data-portal-anchor"));
    }

    #[test]
    fn render_html_appends_portal_container_once() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = render_html(&props, &state);
        let anchor_count = html.matches("data-portal-anchor").count();
        let root_count = html.matches("data-portal-root").count();
        assert!(
            anchor_count >= 2,
            "expected anchor metadata on trigger/list/placeholder"
        );
        assert!(root_count >= 2, "expected root metadata across markup");
        assert_eq!(
            html.matches("<ul").count(),
            1,
            "list markup should only render once"
        );
    }
}
