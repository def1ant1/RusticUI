use mui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use mui_material::drawer::{self, DrawerLayoutOptions, DrawerProps};
use mui_material::Theme;

/// Helper that constructs an uncontrolled [`DrawerState`]. Similar to other
/// tests we lean on the documented discriminants for `ControlStrategy` inside
/// `mui-headless` because the type itself lives in a private module.
fn build_state(anchor: DrawerAnchor, variant: DrawerVariant, open: bool) -> DrawerState {
    DrawerState::new(
        open,
        // SAFETY: `ControlStrategy::Uncontrolled` is represented by the `1`
        // discriminant. This mirrors the adapter tests bundled with the crate.
        unsafe { std::mem::transmute(1u8) },
        variant,
        anchor,
    )
}

/// Build a set of props that can be fanned out to any framework adapter. Keeping
/// the helper centralized makes it easy to tweak the DOM contract without
/// touching every test.
fn build_props<'a>(
    state: &'a DrawerState,
    layout: &'a DrawerLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
) -> DrawerProps<'a> {
    DrawerProps {
        state,
        surface: state
            .surface_attributes()
            .id("demo-drawer")
            .labelled_by("demo-heading"),
        backdrop: state.backdrop_attributes(),
        body: "<nav aria-label=\"Primary\"><a href=\"/\">Home</a></nav>",
        layout,
        theme,
        viewport: Some(viewport),
        on_toggle_event: Some("drawer-toggle"),
    }
}

#[test]
fn react_surface_and_backdrop_expose_semantic_attributes() {
    let layout = DrawerLayoutOptions::default();
    let theme = Theme::default();
    let state = build_state(DrawerAnchor::Start, DrawerVariant::Modal, true);

    let render = drawer::react::render(build_props(&state, &layout, &theme, 640));

    assert!(render.surface.contains("role=\"dialog\""));
    assert!(render.surface.contains("data-anchor=\"start\""));
    assert!(render.surface.contains("data-variant=\"modal\""));
    assert!(render.surface.contains("data-on-toggle=\"drawer-toggle\""));
    assert!(render.surface.contains("id=\"demo-drawer\""));
    assert!(render
        .backdrop
        .as_ref()
        .unwrap()
        .contains("data-variant=\"modal\""));
}

#[test]
fn persistent_variant_omits_backdrop_and_marks_state() {
    let mut layout = DrawerLayoutOptions::default();
    layout.anchor = mui_system::responsive::Responsive::constant(DrawerAnchor::End);
    let theme = Theme::default();
    let state = build_state(DrawerAnchor::End, DrawerVariant::Persistent, true);

    let render = drawer::react::render(build_props(&state, &layout, &theme, 960));

    assert!(render.surface.contains("data-variant=\"persistent\""));
    assert!(render.surface.contains("data-open=\"true\""));
    assert!(render.backdrop.is_none());
}

#[test]
fn responsive_anchor_switches_at_breakpoint() {
    let mut layout = DrawerLayoutOptions::default();
    layout.anchor.lg = Some(DrawerAnchor::Top);
    layout.anchor.xl = Some(DrawerAnchor::Top);

    let theme = Theme::default();
    let start_state = build_state(DrawerAnchor::Start, DrawerVariant::Persistent, true);
    let top_state = build_state(DrawerAnchor::Top, DrawerVariant::Persistent, true);

    let start_render = drawer::react::render(build_props(
        &start_state,
        &layout,
        &theme,
        theme.breakpoints.sm,
    ));
    let top_render = drawer::react::render(build_props(
        &top_state,
        &layout,
        &theme,
        theme.breakpoints.xl,
    ));

    assert!(start_render.surface.contains("data-anchor=\"start\""));
    assert!(top_render.surface.contains("data-anchor=\"top\""));
}

#[cfg(feature = "yew")]
mod yew {
    use super::*;

    #[test]
    fn yew_renderer_matches_react_output_for_modal_variant() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(DrawerAnchor::Start, DrawerVariant::Modal, true);
        let props = build_props(&state, &layout, &theme, 640);

        let react = drawer::react::render(props.clone());
        let yew = drawer::yew::render(props);

        assert_eq!(yew.surface, react.surface);
        assert_eq!(yew.backdrop, react.backdrop);
    }
}

#[cfg(feature = "leptos")]
mod leptos {
    use super::*;

    #[test]
    fn leptos_renderer_matches_react_output_for_modal_variant() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(DrawerAnchor::Start, DrawerVariant::Modal, true);
        let props = build_props(&state, &layout, &theme, 640);

        let react = drawer::react::render(props.clone());
        let leptos = drawer::leptos::render(props);

        assert_eq!(leptos.surface, react.surface);
        assert_eq!(leptos.backdrop, react.backdrop);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore {
    use super::*;

    #[test]
    fn sycamore_renderer_matches_react_output_for_modal_variant() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(DrawerAnchor::Start, DrawerVariant::Modal, true);
        let props = build_props(&state, &layout, &theme, 640);

        let react = drawer::react::render(props.clone());
        let sycamore = drawer::sycamore::render(props);

        assert_eq!(sycamore.surface, react.surface);
        assert_eq!(sycamore.backdrop, react.backdrop);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus {
    use super::*;

    #[test]
    fn dioxus_renderer_matches_react_output_for_modal_variant() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(DrawerAnchor::Start, DrawerVariant::Modal, true);
        let props = build_props(&state, &layout, &theme, 640);

        let react = drawer::react::render(props.clone());
        let dioxus = drawer::dioxus::render(props);

        assert_eq!(dioxus.surface, react.surface);
        assert_eq!(dioxus.backdrop, react.backdrop);
    }
}
