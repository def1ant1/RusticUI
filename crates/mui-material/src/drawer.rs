//! Material flavored drawer utilities that decorate the headless
//! [`DrawerState`](mui_headless::drawer::DrawerState).
//!
//! The helpers expose automation-ready attribute vectors and HTML renderers so
//! framework adapters can stay lean. Styling is centralized through
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) which pulls palette,
//! spacing, breakpoints and Joy token values from the active theme. The
//! resulting API keeps modal and persistent drawers visually aligned across
//! Yew/Leptos/Dioxus/Sycamore without forcing each team to juggle bespoke CSS.

use mui_headless::drawer::{
    DrawerAnchor, DrawerBackdropAttributes, DrawerState, DrawerSurfaceAttributes, DrawerVariant,
};
use mui_styled_engine::{css_with_theme, Style};
use mui_system::{
    r#box::{build_box_style, BoxStyleInputs},
    responsive::{viewport_width, Responsive},
    theme::Theme,
};

/// Shared configuration describing how the drawer should react to viewport
/// changes. The options are kept framework agnostic so Yew, Leptos, Sycamore,
/// Dioxus and React adapters can all lean on the same responsive wiring.
#[derive(Clone, Debug)]
pub struct DrawerLayoutOptions {
    /// Responsive anchor declaration. Persistent navigation often slides from
    /// the leading edge on mobile but transitions to a top anchored sheet on
    /// larger screens. Expressing that behaviour via [`Responsive`] keeps the
    /// logic declarative.
    pub anchor: Responsive<DrawerAnchor>,
    /// Responsive size expressed as a spacing multiplier. For start/end anchors
    /// the value controls the width while top/bottom anchors reuse it for
    /// height. Leveraging spacing units ensures theme updates propagate without
    /// touching component code.
    pub size: Responsive<u16>,
    /// Responsive padding around the drawer contents, also expressed as spacing
    /// multipliers to align with the system scale.
    pub padding: Responsive<u16>,
}

impl Default for DrawerLayoutOptions {
    fn default() -> Self {
        Self {
            anchor: Responsive::constant(DrawerAnchor::Start),
            size: Responsive {
                xs: 40,
                sm: Some(48),
                md: None,
                lg: Some(56),
                xl: Some(56),
            },
            padding: Responsive {
                xs: 2,
                sm: Some(3),
                md: None,
                lg: Some(4),
                xl: Some(4),
            },
        }
    }
}

impl DrawerLayoutOptions {
    /// Resolve the configured anchor for the provided viewport width.
    #[must_use]
    pub fn resolve_anchor(&self, theme: &Theme, viewport: u32) -> DrawerAnchor {
        self.anchor.resolve(viewport, &theme.breakpoints)
    }

    fn spacing_to_css(&self, theme: &Theme) -> Responsive<String> {
        Responsive {
            xs: format!("{}px", theme.spacing(self.size.xs)),
            sm: self
                .size
                .sm
                .map(|value| format!("{}px", theme.spacing(value))),
            md: self
                .size
                .md
                .map(|value| format!("{}px", theme.spacing(value))),
            lg: self
                .size
                .lg
                .map(|value| format!("{}px", theme.spacing(value))),
            xl: self
                .size
                .xl
                .map(|value| format!("{}px", theme.spacing(value))),
        }
    }

    fn padding_to_css(&self, theme: &Theme) -> Responsive<String> {
        Responsive {
            xs: format!("{}px", theme.spacing(self.padding.xs)),
            sm: self
                .padding
                .sm
                .map(|value| format!("{}px", theme.spacing(value))),
            md: self
                .padding
                .md
                .map(|value| format!("{}px", theme.spacing(value))),
            lg: self
                .padding
                .lg
                .map(|value| format!("{}px", theme.spacing(value))),
            xl: self
                .padding
                .xl
                .map(|value| format!("{}px", theme.spacing(value))),
        }
    }
}

/// Result returned by the drawer adapters. We expose both the surface markup
/// and optional backdrop so integrators can slot them into portals or render the
/// elements inline depending on their UX requirements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawerRenderResult {
    /// Serialized drawer surface.
    pub surface: String,
    /// Optional serialized backdrop depending on the configured variant.
    pub backdrop: Option<String>,
}

/// Shared props consumed by every framework adapter.
#[derive(Clone, Debug)]
pub struct DrawerProps<'a> {
    /// Drawer state machine provided by `mui-headless`.
    pub state: &'a DrawerState,
    /// Attribute builder for the drawer surface.
    pub surface: DrawerSurfaceAttributes<'a>,
    /// Attribute builder for the drawer backdrop.
    pub backdrop: DrawerBackdropAttributes<'a>,
    /// Serialized drawer body.
    pub body: &'a str,
    /// Layout behaviour shared across adapters.
    pub layout: &'a DrawerLayoutOptions,
    /// Active theme driving spacing and breakpoint resolution.
    pub theme: &'a Theme,
    /// Optional viewport width override for deterministic snapshot tests.
    pub viewport: Option<u32>,
    /// Optional event channel identifier surfaced via `data-on-toggle` so
    /// enterprise orchestration layers can listen to drawer visibility changes
    /// without diverging adapter implementations.
    pub on_toggle_event: Option<&'a str>,
}

/// Convert surface attributes into key/value pairs enriched with automation
/// hints.
#[must_use]
pub fn drawer_surface_attributes(
    state: &DrawerState,
    attrs: DrawerSurfaceAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(9);
    pairs.push(("role".into(), attrs.role().into()));
    let (aria_modal_key, aria_modal_value) = attrs.aria_modal();
    pairs.push((aria_modal_key.into(), aria_modal_value.into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_describedby() {
        pairs.push((key.into(), value.into()));
    }
    let (anchor_key, anchor_value) = attrs.data_anchor();
    pairs.push((anchor_key.into(), anchor_value.into()));
    pairs.push(("data-open".into(), state.is_open().to_string()));
    pairs.push((
        "data-variant".into(),
        match state.variant() {
            DrawerVariant::Modal => "modal",
            DrawerVariant::Persistent => "persistent",
        }
        .to_string(),
    ));
    pairs
}

/// Render the drawer surface to HTML using the shared renderer.
#[must_use]
pub fn render_drawer_surface_html(
    state: &DrawerState,
    attrs: DrawerSurfaceAttributes<'_>,
    body: &str,
) -> String {
    crate::render_helpers::render_element_html(
        "div",
        drawer_surface_style(),
        drawer_surface_attributes(state, attrs),
        body,
    )
}

/// Gather backdrop attributes and automation markers.
#[must_use]
pub fn drawer_backdrop_attributes(
    state: &DrawerState,
    attrs: DrawerBackdropAttributes<'_>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(4);
    let (aria_hidden_key, aria_hidden_value) = attrs.aria_hidden();
    pairs.push((aria_hidden_key.into(), aria_hidden_value.into()));
    pairs.push(("data-open".into(), state.is_open().to_string()));
    pairs.push((
        "data-variant".into(),
        match state.variant() {
            DrawerVariant::Modal => "modal",
            DrawerVariant::Persistent => "persistent",
        }
        .to_string(),
    ));
    pairs
}

/// Render the backdrop if the variant requires one.
#[must_use]
pub fn render_drawer_backdrop_html(
    state: &DrawerState,
    attrs: DrawerBackdropAttributes<'_>,
) -> Option<String> {
    if !attrs.is_visible() {
        return None;
    }
    Some(crate::render_helpers::render_backdrop_html(
        drawer_backdrop_style(),
        drawer_backdrop_attributes(state, attrs),
    ))
}

/// Drawer surface styling shared across frameworks.
fn drawer_surface_style() -> Style {
    css_with_theme!(
        r#"
        position: fixed;
        top: 0;
        bottom: 0;
        background: ${background};
        color: ${text};
        display: flex;
        flex-direction: column;
        gap: ${gap};
        padding: ${padding_mobile};
        width: min(100%, ${width_mobile});
        max-width: 100%;
        box-shadow: 0 24px 54px -36px color-mix(in srgb, ${shadow_base} 65%, transparent);
        transition: transform 200ms ease, opacity 200ms ease, visibility 200ms ease;
        &[data-anchor="start"] {
            left: 0;
            transform: translateX(-100%);
        }
        &[data-anchor="end"] {
            right: 0;
            transform: translateX(100%);
        }
        &[data-anchor="top"] {
            top: 0;
            height: min(100%, ${width_mobile});
            transform: translateY(-100%);
        }
        &[data-anchor="bottom"] {
            bottom: 0;
            height: min(100%, ${width_mobile});
            transform: translateY(100%);
        }
        &[data-open="true"][data-anchor="start"],
        &[data-open="true"][data-anchor="end"] {
            transform: translateX(0);
        }
        &[data-open="true"][data-anchor="top"],
        &[data-open="true"][data-anchor="bottom"] {
            transform: translateY(0);
        }
        &[data-open="false"] {
            opacity: 0;
            visibility: hidden;
        }
        &[data-open="true"] {
            opacity: 1;
            visibility: visible;
        }
        @media (min-width: ${sm}px) {
            padding: ${padding_tablet};
            width: min(100%, ${width_tablet});
        }
        @media (min-width: ${lg}px) {
            padding: ${padding_desktop};
            width: ${width_desktop};
        }
    "#,
        background = theme.palette.background_paper.clone(),
        text = theme.palette.text_primary.clone(),
        gap = format!("{}px", theme.spacing(2)),
        padding_mobile = format!("{}px", theme.spacing(2)),
        padding_tablet = format!("{}px", theme.spacing(3)),
        padding_desktop = format!("{}px", theme.spacing(4)),
        width_mobile = format!("{}px", theme.spacing(40)),
        width_tablet = format!("{}px", theme.spacing(48)),
        width_desktop = format!("{}px", theme.spacing(56)),
        shadow_base = theme.palette.neutral.clone(),
        sm = theme.breakpoints.sm,
        lg = theme.breakpoints.lg,
    )
}

/// Backdrop styling controlling scrim opacity and transitions.
fn drawer_backdrop_style() -> Style {
    css_with_theme!(
        r#"
        position: fixed;
        inset: 0;
        background: color-mix(in srgb, ${scrim} 65%, transparent);
        transition: opacity 200ms ease;
        opacity: 0;
        &[data-open="true"] {
            opacity: 1;
        }
        &[data-open="false"] {
            pointer-events: none;
        }
    "#,
        scrim = theme.palette.text_primary.clone(),
    )
}

struct DrawerRenderParams<'a> {
    state: &'a DrawerState,
    surface: DrawerSurfaceAttributes<'a>,
    backdrop: DrawerBackdropAttributes<'a>,
    body: &'a str,
    layout: &'a DrawerLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
    on_toggle_event: Option<&'a str>,
}

impl<'a> From<DrawerProps<'a>> for DrawerRenderParams<'a> {
    fn from(props: DrawerProps<'a>) -> Self {
        Self {
            state: props.state,
            surface: props.surface,
            backdrop: props.backdrop,
            body: props.body,
            layout: props.layout,
            theme: props.theme,
            viewport: props.viewport.unwrap_or_else(viewport_width),
            on_toggle_event: props.on_toggle_event,
        }
    }
}

fn render_drawer(params: DrawerRenderParams<'_>) -> DrawerRenderResult {
    let anchor = params.layout.resolve_anchor(params.theme, params.viewport);
    debug_assert_eq!(
        params.state.anchor(),
        anchor,
        "DrawerState anchor should align with responsive layout",
    );

    let size_css = params.layout.spacing_to_css(params.theme);
    let padding_css = params.layout.padding_to_css(params.theme);
    let position = Responsive::constant("fixed".to_string());
    let zero_top = Responsive::constant("0".to_string());
    let zero_bottom = Responsive::constant("0".to_string());
    let zero_lr = Responsive::constant("0".to_string());
    let full_height = Responsive::constant("100%".to_string());
    let full_width = Responsive::constant("100%".to_string());

    let mut inputs = BoxStyleInputs {
        margin: None,
        padding: Some(&padding_css),
        font_size: None,
        font_weight: None,
        line_height: None,
        letter_spacing: None,
        color: None,
        background_color: None,
        width: None,
        height: Some(&full_height),
        min_width: None,
        max_width: None,
        min_height: None,
        max_height: None,
        position: Some(&position),
        top: Some(&zero_top),
        right: None,
        bottom: Some(&zero_bottom),
        left: None,
        display: Some("flex"),
        align_items: Some("stretch"),
        justify_content: Some("flex-start"),
        sx: None,
    };

    match anchor {
        DrawerAnchor::Start => {
            inputs.width = Some(&size_css);
            inputs.left = Some(&zero_lr);
        }
        DrawerAnchor::End => {
            inputs.width = Some(&size_css);
            inputs.right = Some(&zero_lr);
        }
        DrawerAnchor::Top => {
            inputs.height = Some(&size_css);
            inputs.width = Some(&full_width);
            inputs.left = Some(&zero_lr);
            inputs.right = Some(&zero_lr);
            inputs.bottom = None;
        }
        DrawerAnchor::Bottom => {
            inputs.height = Some(&size_css);
            inputs.width = Some(&full_width);
            inputs.left = Some(&zero_lr);
            inputs.right = Some(&zero_lr);
            inputs.top = None;
        }
    }

    let container_css = build_box_style(params.viewport, &params.theme.breakpoints, inputs);
    let container_style =
        Style::new(container_css).expect("mui-system box builder should emit valid CSS");
    let surface_html = render_drawer_surface_html(params.state, params.surface, params.body);

    let mut outer_attrs = Vec::with_capacity(1);
    if let Some(event) = params.on_toggle_event {
        outer_attrs.push(("data-on-toggle".to_string(), event.to_string()));
    }

    let wrapped_surface = crate::render_helpers::render_element_html(
        "div",
        container_style,
        outer_attrs,
        &surface_html,
    );
    let backdrop = render_drawer_backdrop_html(params.state, params.backdrop);

    DrawerRenderResult {
        surface: wrapped_surface,
        backdrop,
    }
}

/// Adapter targeting server rendered React experiences.
pub mod react {
    use super::*;

    /// Render the drawer surface/backdrop pair into serialized HTML.
    pub fn render(props: DrawerProps<'_>) -> DrawerRenderResult {
        super::render_drawer(props.into())
    }
}

/// Adapter targeting the [`yew`] framework.  Returns serialized markup so tests
/// can validate SSR output matches client rendering.
pub mod yew {
    use super::*;

    /// Render the drawer surface/backdrop pair into serialized HTML.
    pub fn render(props: DrawerProps<'_>) -> DrawerRenderResult {
        super::render_drawer(props.into())
    }
}

/// Adapter targeting the [`leptos`] framework.
pub mod leptos {
    use super::*;

    /// Render the drawer surface/backdrop pair into serialized HTML.
    pub fn render(props: DrawerProps<'_>) -> DrawerRenderResult {
        super::render_drawer(props.into())
    }
}

/// Adapter targeting the [`sycamore`] framework.
pub mod sycamore {
    use super::*;

    /// Render the drawer surface/backdrop pair into serialized HTML.
    pub fn render(props: DrawerProps<'_>) -> DrawerRenderResult {
        super::render_drawer(props.into())
    }
}

/// Adapter targeting the [`dioxus`] framework.
pub mod dioxus {
    use super::*;

    /// Render the drawer surface/backdrop pair into serialized HTML.
    pub fn render(props: DrawerProps<'_>) -> DrawerRenderResult {
        super::render_drawer(props.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mui_headless::drawer::{DrawerAnchor, DrawerVariant};

    fn sample_state(open: bool, variant: DrawerVariant) -> DrawerState {
        DrawerState::new(
            open,
            unsafe { std::mem::transmute(1u8) },
            variant,
            DrawerAnchor::Start,
        )
    }

    #[test]
    fn surface_attributes_include_variant_and_open_flags() {
        let state = sample_state(true, DrawerVariant::Modal);
        let attrs = drawer_surface_attributes(&state, state.surface_attributes());
        assert!(attrs.iter().any(|(k, v)| k == "role" && v == "dialog"));
        assert!(attrs.iter().any(|(k, v)| k == "data-open" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-variant" && v == "modal"));
    }

    #[test]
    fn render_drawer_surface_html_includes_children() {
        let state = sample_state(false, DrawerVariant::Persistent);
        let html =
            render_drawer_surface_html(&state, state.surface_attributes(), "<nav>Links</nav>");
        assert!(html.contains("class=\""));
        assert!(html.contains("data-open=\"false\""));
        assert!(html.contains("<nav>Links</nav>"));
    }

    #[test]
    fn backdrop_renders_only_for_modal_variants() {
        let modal = sample_state(true, DrawerVariant::Modal);
        let backdrop = render_drawer_backdrop_html(&modal, modal.backdrop_attributes());
        assert!(backdrop.is_some());

        let persistent = sample_state(true, DrawerVariant::Persistent);
        let none = render_drawer_backdrop_html(&persistent, persistent.backdrop_attributes());
        assert!(none.is_none());
    }
}
