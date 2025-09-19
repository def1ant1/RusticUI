//! Material themed menu button renderer powered by the headless [`MenuState`].
//!
//! The design mirrors [`select`](crate::select) and [`button`](crate::button)
//! by concentrating HTML string generation and theme-aware styling within a
//! single module. Enterprise teams can therefore adopt the component across Yew,
//! Leptos, Dioxus and Sycamore without duplicating CSS or ARIA wiring. The
//! shared helpers also inject deterministic automation hooks so QA pipelines have
//! stable selectors regardless of the adapter being used.

use mui_headless::menu::MenuState;
use mui_styled_engine::{css_with_theme, Style};

/// Individual actionable item rendered within the menu surface.
#[derive(Clone, Debug)]
pub struct MenuItem {
    /// Human readable text displayed for the action.
    pub label: String,
    /// Stable identifier wired into `data-command` for automation scripts.
    pub command: String,
}

impl MenuItem {
    /// Convenience constructor for tests and demos.
    pub fn new(label: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
        }
    }
}

/// Props shared across framework adapters.
#[derive(Clone, Debug)]
pub struct MenuProps {
    /// Label displayed inside the trigger button.
    pub label: String,
    /// Collection of actionable menu items.
    pub items: Vec<MenuItem>,
    /// Optional automation identifier used to stamp deterministic `data-*`
    /// attributes.
    pub automation_id: Option<String>,
}

impl MenuProps {
    /// Convenience constructor producing a baseline menu configuration.
    pub fn new(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            label: label.into(),
            items,
            automation_id: None,
        }
    }

    /// Override the automation identifier allowing deterministic selector reuse
    /// across SSR and hydration phases.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }
}

/// Shared rendering routine that produces SSR friendly HTML strings.
fn render_html(props: &MenuProps, state: &MenuState) -> String {
    let root_attrs = crate::style_helpers::themed_attributes_html(
        themed_root_style(),
        root_attributes(props, state),
    );
    let trigger_attrs = crate::style_helpers::themed_attributes_html(
        themed_trigger_style(),
        trigger_attributes(props, state),
    );
    let surface_attrs = crate::style_helpers::themed_attributes_html(
        themed_surface_style(),
        surface_attributes(props, state),
    );

    let mut items_html = String::new();
    for (index, item) in props.items.iter().enumerate() {
        let item_attrs = crate::style_helpers::themed_attributes_html(
            themed_item_style(),
            item_attributes(props, state, index),
        );
        items_html.push_str(&format!("<li {item_attrs}>{}</li>", item.label));
    }

    format!(
        "<div {root_attrs}><button {trigger_attrs}>{}</button><ul {surface_attrs}>{items_html}</ul></div>",
        props.label
    )
}

fn automation_base(props: &MenuProps) -> String {
    props
        .automation_id
        .clone()
        .unwrap_or_else(|| "mui-menu".into())
}

fn surface_id(props: &MenuProps) -> String {
    format!("{}-surface", automation_base(props))
}

fn item_id(props: &MenuProps, index: usize) -> String {
    format!("{}-item-{index}", automation_base(props))
}

fn root_attributes(props: &MenuProps, state: &MenuState) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("data-component".into(), "mui-menu".into()));
    attrs.push(("data-open".into(), state.is_open().to_string()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".into(), id.clone()));
    }
    attrs
}

fn trigger_attributes(props: &MenuProps, state: &MenuState) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("role".into(), state.trigger_role().into()));
    let (key, value) = state.trigger_haspopup();
    attrs.push((key.into(), value.into()));
    let (expanded_key, expanded_value) = state.trigger_expanded();
    attrs.push((expanded_key.into(), expanded_value.into()));
    attrs.push(("aria-controls".into(), surface_id(props)));
    attrs.push(("data-open".into(), state.is_open().to_string()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-trigger".into(), id.clone()));
    }
    attrs
}

fn surface_attributes(props: &MenuProps, state: &MenuState) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), surface_id(props)));
    attrs.push(("role".into(), state.menu_role().into()));
    attrs.push(("aria-hidden".into(), (!state.is_open()).to_string()));
    if let Some(highlighted) = state.highlighted() {
        attrs.push(("data-highlighted".into(), highlighted.to_string()));
    }
    attrs.push(("data-open".into(), state.is_open().to_string()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-surface".into(), id.clone()));
    }
    attrs
}

fn item_attributes(props: &MenuProps, state: &MenuState, index: usize) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), item_id(props, index)));
    attrs.push(("role".into(), state.item_role().into()));
    let is_highlighted = state.highlighted() == Some(index);
    attrs.push(("data-highlighted".into(), is_highlighted.to_string()));
    attrs.push(("data-index".into(), index.to_string()));
    attrs.push(("data-command".into(), props.items[index].command.clone()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-item".into(), format!("{id}-{index}")));
    }
    attrs
}

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

fn themed_trigger_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: space-between;
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
            border-color: ${focus_color};
            box-shadow: 0 0 0 ${focus_width} ${focus_color_transparent};
        }
    "#,
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
        font_size = format!("{:.3}rem", theme.typography.button),
        focus_color = theme.palette.secondary.clone(),
        focus_width = format!("{:.1}px", (theme.joy.focus_thickness as f32).max(1.0) / 2.0),
        focus_color_transparent = format!(
            "color-mix(in srgb, {} 24%, transparent)",
            theme.palette.secondary.clone()
        )
    )
}

fn themed_surface_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        top: calc(100% + ${offset});
        right: 0;
        min-width: ${min_width};
        margin: 0;
        padding: ${padding};
        list-style: none;
        border-radius: ${radius};
        border: 1px solid ${border_color};
        background: ${background};
        box-shadow: ${shadow};
        display: none;
        z-index: 12;

        &[data-open='true'] {
            display: block;
        }
    "#,
        offset = format!("{}px", theme.spacing(1)),
        min_width = format!("{}px", theme.spacing(20)),
        padding = format!("{}px", theme.spacing(1)),
        radius = format!("{}px", theme.joy.radius),
        border_color = format!(
            "color-mix(in srgb, {} 40%, transparent)",
            theme.palette.text_secondary.clone()
        ),
        background = theme.palette.background_paper.clone(),
        shadow = format!(
            "0 12px 24px color-mix(in srgb, {} 18%, transparent)",
            theme.palette.text_primary.clone()
        )
    )
}

fn themed_item_style() -> Style {
    css_with_theme!(
        r#"
        padding: ${padding_y} ${padding_x};
        border-radius: ${radius};
        font-family: ${font_family};
        font-size: ${font_size};
        color: ${text_color};
        cursor: pointer;

        &[data-highlighted='true'],
        &:hover {
            background: ${hover_background};
        }
    "#,
        padding_y = format!("{}px", theme.spacing(1)),
        padding_x = format!("{}px", theme.spacing(3)),
        radius = format!("{:.1}px", (theme.joy.radius as f32) / 2.0),
        font_family = theme.typography.font_family.clone(),
        font_size = format!("{:.3}rem", theme.typography.body2),
        text_color = theme.palette.text_primary.clone(),
        hover_background = format!(
            "color-mix(in srgb, {} 12%, {})",
            theme.palette.secondary.clone(),
            theme.palette.background_paper.clone()
        )
    )
}

pub mod yew {
    use super::*;

    pub fn render(props: &MenuProps, state: &MenuState) -> String {
        super::render_html(props, state)
    }
}

pub mod leptos {
    use super::*;

    pub fn render(props: &MenuProps, state: &MenuState) -> String {
        super::render_html(props, state)
    }
}

pub mod dioxus {
    use super::*;

    pub fn render(props: &MenuProps, state: &MenuState) -> String {
        super::render_html(props, state)
    }
}

pub mod sycamore {
    use super::*;

    pub fn render(props: &MenuProps, state: &MenuState) -> String {
        super::render_html(props, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_state(item_count: usize) -> MenuState {
        MenuState::new(
            item_count,
            false,
            // Mirror the reasoning from `select` tests – we transmute the
            // private `ControlStrategy::Uncontrolled` variant to exercise the
            // integration layer without widening the headless API surface.
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        )
    }

    fn sample_props() -> MenuProps {
        MenuProps::new(
            "Menu",
            vec![
                MenuItem::new("Profile", "profile"),
                MenuItem::new("Settings", "settings"),
            ],
        )
        .with_automation_id("sample-menu")
    }

    #[test]
    fn trigger_attributes_include_menu_contract() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let attrs = trigger_attributes(&props, &state);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-haspopup" && v == "menu"));
        assert!(attrs.iter().any(|(k, _)| k == "aria-controls"));
    }

    #[test]
    fn surface_attributes_track_open_state() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.open(|_| {});
        let attrs = surface_attributes(&props, &state);
        assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "true"));
    }

    #[test]
    fn render_html_emits_command_hooks() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = render_html(&props, &state);
        assert!(html.contains("data-command=\"profile\""));
        assert!(html.contains("data-automation-id=\"sample-menu\""));
    }
}
