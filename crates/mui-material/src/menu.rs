//! Material themed menu button renderer powered by the headless [`MenuState`].
//!
//! The design mirrors [`select`](crate::select) and [`button`](crate::button)
//! by concentrating HTML string generation and theme-aware styling within a
//! single module. Enterprise teams can therefore adopt the component across Yew,
//! Leptos, Dioxus and Sycamore without duplicating CSS or ARIA wiring. The
//! shared helpers also inject deterministic automation hooks so QA pipelines have
//! stable selectors regardless of the adapter being used.

use mui_headless::{
    menu::MenuState,
    popover::{CollisionOutcome, PopoverState},
};
use mui_styled_engine::{css_with_theme, Style};
use mui_system::portal::PortalMount;
use mui_utils::attributes_to_html;

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
///
/// The menu state is still responsible for focus and disclosure semantics while
/// the popover state injects anchor/placement analytics. Passing both ensures
/// server rendered snapshots mirror the hydrated tree without adapters
/// reimplementing attribute composition.
fn render_html(props: &MenuProps, menu_state: &MenuState, popover_state: &PopoverState) -> String {
    let portal = popover_mount(props);
    let outcome = popover_state.last_outcome();
    let anchor_meta = popover_state.anchor_attributes();
    let surface_meta = popover_surface_metadata(props, popover_state);
    let root_attrs = crate::style_helpers::themed_attributes_html(
        themed_root_style(),
        root_attributes(props, menu_state, &surface_meta, outcome, &portal),
    );
    let trigger_attrs = crate::style_helpers::themed_attributes_html(
        themed_trigger_style(),
        trigger_attributes(
            props,
            menu_state,
            &surface_meta,
            &anchor_meta,
            outcome,
            &portal,
        ),
    );
    let surface_attrs = crate::style_helpers::themed_attributes_html(
        themed_surface_style(),
        surface_attributes(
            props,
            menu_state,
            &surface_meta,
            &anchor_meta,
            outcome,
            &portal,
        ),
    );

    let mut items_html = String::new();
    for (index, item) in props.items.iter().enumerate() {
        let item_attrs = crate::style_helpers::themed_attributes_html(
            themed_item_style(),
            item_attributes(props, menu_state, index),
        );
        items_html.push_str(&format!("<li {item_attrs}>{}</li>", item.label));
    }

    let anchor_attrs = anchor_attributes(&anchor_meta, &portal);
    let anchor_html = format!("<span {}></span>", attributes_to_html(&anchor_attrs));
    let portal_markup = portal.wrap(format!("<ul {surface_attrs}>{items_html}</ul>"));

    format!(
        "<div {root_attrs}><button {trigger_attrs}>{}</button>{}</div>{}",
        props.label,
        anchor_html,
        portal_markup.into_html()
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

fn root_attributes(
    props: &MenuProps,
    menu_state: &MenuState,
    surface_meta: &mui_headless::popover::PopoverSurfaceAttributes<'_>,
    outcome: CollisionOutcome,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("data-component".into(), "mui-menu".into()));
    let (open_key, open_value) = surface_meta.data_open();
    attrs.push((open_key.into(), open_value.into()));
    let (preferred_key, preferred_value) = surface_meta.data_preferred();
    attrs.push((preferred_key.into(), preferred_value.into()));
    let (resolved_key, resolved_value) = surface_meta.data_resolved();
    attrs.push((resolved_key.into(), resolved_value.into()));
    attrs.push((
        "data-placement-outcome".into(),
        collision_outcome(outcome).into(),
    ));
    attrs.push(("data-open-menu".into(), menu_state.is_open().to_string()));
    attrs.push((
        "data-portal-layer".into(),
        portal.layer().as_str().to_string(),
    ));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-id".into(), id.clone()));
    }
    attrs
}

fn trigger_attributes(
    props: &MenuProps,
    menu_state: &MenuState,
    surface_meta: &mui_headless::popover::PopoverSurfaceAttributes<'_>,
    anchor_meta: &mui_headless::popover::PopoverAnchorAttributes<'_>,
    outcome: CollisionOutcome,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("role".into(), menu_state.trigger_role().into()));
    let (key, value) = menu_state.trigger_haspopup();
    attrs.push((key.into(), value.into()));
    let (expanded_key, expanded_value) = menu_state.trigger_expanded();
    attrs.push((expanded_key.into(), expanded_value.into()));
    attrs.push(("aria-controls".into(), surface_id(props)));
    let (open_key, open_value) = surface_meta.data_open();
    attrs.push((open_key.into(), open_value.into()));
    if let Some((_, anchor_id)) = anchor_meta.id() {
        attrs.push(("data-portal-anchor".into(), anchor_id.to_string()));
    } else {
        attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    }
    attrs.push(("data-portal-root".into(), portal.container_id()));
    let (placement_key, placement_value) = anchor_meta.data_placement();
    attrs.push((placement_key.into(), placement_value.into()));
    attrs.push((
        "data-placement-outcome".into(),
        collision_outcome(outcome).into(),
    ));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-trigger".into(), id.clone()));
    }
    attrs
}

fn surface_attributes(
    props: &MenuProps,
    menu_state: &MenuState,
    surface_meta: &mui_headless::popover::PopoverSurfaceAttributes<'_>,
    anchor_meta: &mui_headless::popover::PopoverAnchorAttributes<'_>,
    outcome: CollisionOutcome,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), surface_id(props)));
    attrs.push(("role".into(), menu_state.menu_role().into()));
    attrs.push((
        "aria-hidden".into(),
        (surface_meta.data_open().1 != "true").to_string(),
    ));
    if let Some(highlighted) = menu_state.highlighted() {
        attrs.push(("data-highlighted".into(), highlighted.to_string()));
    }
    let (open_key, open_value) = surface_meta.data_open();
    attrs.push((open_key.into(), open_value.into()));
    let (preferred_key, preferred_value) = surface_meta.data_preferred();
    attrs.push((preferred_key.into(), preferred_value.into()));
    let (resolved_key, resolved_value) = surface_meta.data_resolved();
    attrs.push((resolved_key.into(), resolved_value.into()));
    if let Some((analytics_key, analytics_value)) = surface_meta.data_analytics_id() {
        attrs.push((analytics_key.into(), analytics_value.into()));
    }
    if let Some((_, anchor_id)) = anchor_meta.id() {
        attrs.push(("data-portal-anchor".into(), anchor_id.to_string()));
    } else {
        attrs.push(("data-portal-anchor".into(), portal.anchor_id()));
    }
    attrs.push(("data-portal-root".into(), portal.container_id()));
    attrs.push((
        "data-placement-outcome".into(),
        collision_outcome(outcome).into(),
    ));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-surface".into(), id.clone()));
    }
    attrs
}

fn item_attributes(props: &MenuProps, state: &MenuState, index: usize) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    attrs.push(("id".into(), item_id(props, index)));
    for (key, value) in state.item_accessibility_attributes(index) {
        attrs.push((key.into(), value));
    }
    let is_highlighted = state.highlighted() == Some(index);
    attrs.push(("data-highlighted".into(), is_highlighted.to_string()));
    attrs.push(("data-index".into(), index.to_string()));
    attrs.push(("data-command".into(), props.items[index].command.clone()));
    if let Some(id) = &props.automation_id {
        attrs.push(("data-automation-item".into(), format!("{id}-{index}")));
    }
    attrs
}

fn popover_mount(props: &MenuProps) -> PortalMount {
    let base = format!("{}-popover", automation_base(props));
    PortalMount::popover(base)
}

fn anchor_attributes(
    anchor_meta: &mui_headless::popover::PopoverAnchorAttributes<'_>,
    portal: &PortalMount,
) -> Vec<(String, String)> {
    // Merge the static portal metadata with the runtime placement analytics so
    // automation tooling can target the anchor while design systems inspect the
    // resolved placement without querying state objects directly.
    let mut attrs = portal.anchor_attributes();
    if let Some((key, value)) = anchor_meta.id() {
        if let Some(existing) = attrs.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.to_string();
        } else {
            attrs.push((key.into(), value.into()));
        }
    }
    let (placement_key, placement_value) = anchor_meta.data_placement();
    attrs.push((placement_key.into(), placement_value.into()));
    attrs
}

fn popover_surface_metadata<'a>(
    props: &'a MenuProps,
    popover_state: &'a PopoverState,
) -> mui_headless::popover::PopoverSurfaceAttributes<'a> {
    // The headless popover exposes a fluent attribute builder. Centralising the
    // analytics wiring here keeps the menu rendering paths in sync regardless of
    // which adapter invokes them.
    let surface_meta = popover_state.surface_attributes();
    if let Some(id) = props.automation_id.as_deref() {
        surface_meta.analytics_id(id)
    } else {
        surface_meta
    }
}

fn collision_outcome(outcome: CollisionOutcome) -> &'static str {
    // Normalise the enum into deterministic strings that analytics dashboards
    // and integration tests can assert against without depending on Debug
    // representations.
    match outcome {
        CollisionOutcome::Preferred => "preferred",
        CollisionOutcome::Repositioned => "repositioned",
    }
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

    pub fn render(
        props: &MenuProps,
        menu_state: &MenuState,
        popover_state: &PopoverState,
    ) -> String {
        super::render_html(props, menu_state, popover_state)
    }
}

pub mod leptos {
    use super::*;

    pub fn render(
        props: &MenuProps,
        menu_state: &MenuState,
        popover_state: &PopoverState,
    ) -> String {
        super::render_html(props, menu_state, popover_state)
    }
}

pub mod dioxus {
    use super::*;

    pub fn render(
        props: &MenuProps,
        menu_state: &MenuState,
        popover_state: &PopoverState,
    ) -> String {
        super::render_html(props, menu_state, popover_state)
    }
}

pub mod sycamore {
    use super::*;

    pub fn render(
        props: &MenuProps,
        menu_state: &MenuState,
        popover_state: &PopoverState,
    ) -> String {
        super::render_html(props, menu_state, popover_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::popover::{AnchorGeometry, PopoverPlacement};

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

    fn build_popover(props: &MenuProps) -> PopoverState {
        let mut popover = PopoverState::uncontrolled(false, PopoverPlacement::Bottom);
        // Precompute the anchor identifier with the same helper used during SSR
        // rendering so the snapshot mirrors the automation friendly naming
        // scheme derived from the automation id.
        let anchor_id = popover_mount(props).anchor_id();
        popover.set_anchor_metadata(Some(anchor_id), None);
        popover
    }

    #[test]
    fn trigger_attributes_include_menu_contract() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let popover = build_popover(&props);
        let portal = popover_mount(&props);
        let surface_meta = popover_surface_metadata(&props, &popover);
        let anchor_meta = popover.anchor_attributes();
        let attrs = trigger_attributes(
            &props,
            &state,
            &surface_meta,
            &anchor_meta,
            popover.last_outcome(),
            &portal,
        );
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-haspopup" && v == "menu"));
        assert!(attrs.iter().any(|(k, _)| k == "aria-controls"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-root" && v.ends_with("-portal")));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-popover-placement" && v == "bottom"));
        assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "false"));
    }

    #[test]
    fn root_attributes_merge_menu_and_popover_metadata() {
        let props = sample_props();
        let menu_state = build_state(props.items.len());
        let popover_state = build_popover(&props);
        let portal = popover_mount(&props);
        let surface_meta = popover_surface_metadata(&props, &popover_state);
        let attrs = root_attributes(
            &props,
            &menu_state,
            &surface_meta,
            popover_state.last_outcome(),
            &portal,
        );
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-preferred-placement" && v == "bottom"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-resolved-placement" && v == "bottom"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-placement-outcome" && v == "preferred"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-open-menu" && v == "false"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-layer" && v == "popover"));
    }

    #[test]
    fn surface_attributes_track_open_state() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.open(|_| {});
        let mut popover = build_popover(&props);
        popover.open(|_| {});
        let portal = popover_mount(&props);
        let surface_meta = popover_surface_metadata(&props, &popover);
        let anchor_meta = popover.anchor_attributes();
        let attrs = surface_attributes(
            &props,
            &state,
            &surface_meta,
            &anchor_meta,
            popover.last_outcome(),
            &portal,
        );
        assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-anchor" && v.ends_with("-anchor")));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-resolved-placement" && v == "bottom"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-placement-outcome" && v == "preferred"));
    }

    #[test]
    fn surface_attributes_reflect_repositioned_outcome() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let portal = popover_mount(&props);
        let mut popover = build_popover(&props);
        let anchor_id = portal.anchor_id();
        popover.set_anchor_metadata(
            Some(anchor_id),
            Some(AnchorGeometry {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 40.0,
            }),
        );
        popover.resolve_with(|_, _| PopoverPlacement::Top);
        let surface_meta = popover_surface_metadata(&props, &popover);
        let anchor_meta = popover.anchor_attributes();
        let attrs = surface_attributes(
            &props,
            &state,
            &surface_meta,
            &anchor_meta,
            popover.last_outcome(),
            &portal,
        );
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-resolved-placement" && v == "top"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-placement-outcome" && v == "repositioned"));
    }

    #[test]
    fn render_html_emits_command_hooks() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let popover = build_popover(&props);
        let html = render_html(&props, &state, &popover);
        assert!(html.contains("data-command=\"profile\""));
        assert!(html.contains("data-automation-id=\"sample-menu\""));
        assert!(html.contains("data-portal-root"));
        assert!(html.contains("data-portal-anchor"));
        assert!(html.contains("data-resolved-placement"));
    }

    #[test]
    fn render_html_renders_single_surface_instance() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let popover = build_popover(&props);
        let html = render_html(&props, &state, &popover);
        assert_eq!(
            html.matches("<ul").count(),
            1,
            "menu surface should only render once"
        );
        assert!(html.contains("data-portal-layer=\"popover\""));
        assert!(html.contains("data-preferred-placement=\"bottom\""));
    }

    #[test]
    fn render_html_reflects_repositioned_popover_metadata() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let portal = popover_mount(&props);
        let mut popover = build_popover(&props);
        let anchor_id = portal.anchor_id();
        popover.set_anchor_metadata(
            Some(anchor_id),
            Some(AnchorGeometry {
                x: 4.0,
                y: 4.0,
                width: 48.0,
                height: 16.0,
            }),
        );
        popover.resolve_with(|_, _| PopoverPlacement::Top);
        let html = render_html(&props, &state, &popover);
        assert!(html.contains("data-resolved-placement=\"top\""));
        assert!(html.contains("data-placement-outcome=\"repositioned\""));
    }

    #[test]
    fn item_attributes_reflect_disabled_flags() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.set_item_disabled(1, true);
        let attrs = item_attributes(&props, &state, 1);
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-disabled" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-disabled" && v == "true"));
    }
}
