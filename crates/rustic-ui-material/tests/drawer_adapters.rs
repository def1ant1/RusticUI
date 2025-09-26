use rustic_ui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use rustic_ui_material::drawer::{self, DrawerLayoutOptions, DrawerProps};
use rustic_ui_material::Theme;

fn sample_state(anchor: DrawerAnchor, variant: DrawerVariant) -> DrawerState {
    DrawerState::new(true, unsafe { std::mem::transmute(1u8) }, variant, anchor)
}

fn build_props<'a>(
    state: &'a DrawerState,
    layout: &'a DrawerLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
) -> DrawerProps<'a> {
    DrawerProps {
        state,
        surface: state.surface_attributes().id("drawer"),
        backdrop: state.backdrop_attributes(),
        body: "<nav>Links</nav>",
        layout,
        theme,
        viewport: Some(viewport),
        on_toggle_event: Some("drawer-toggle"),
    }
}

#[test]
fn react_adapter_wraps_surface_and_backdrop() {
    let layout = DrawerLayoutOptions::default();
    let theme = Theme::default();
    let state = sample_state(DrawerAnchor::Start, DrawerVariant::Modal);
    let render = drawer::react::render(build_props(&state, &layout, &theme, 640));

    assert!(render.surface.starts_with("<div class=\""));
    assert!(render.surface.contains("data-on-toggle=\"drawer-toggle\""));
    assert!(render.surface.contains("data-anchor=\"start\""));
    assert!(render.backdrop.is_some());
}

#[test]
fn react_adapter_resolves_top_anchor_on_large_viewports() {
    let mut layout = DrawerLayoutOptions::default();
    layout.anchor.md = Some(DrawerAnchor::Top);
    layout.anchor.lg = Some(DrawerAnchor::Top);
    let theme = Theme::default();
    let state = sample_state(DrawerAnchor::Top, DrawerVariant::Persistent);
    let render = drawer::react::render(build_props(&state, &layout, &theme, 1280));

    assert!(render.surface.contains("data-anchor=\"top\""));
    assert!(render.backdrop.is_none());
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(DrawerAnchor::Start, DrawerVariant::Modal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = drawer::react::render(props.clone());
        let yew = drawer::yew::render(props);
        assert_eq!(yew.surface, react.surface);
        assert_eq!(yew.backdrop, react.backdrop);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(DrawerAnchor::Start, DrawerVariant::Modal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = drawer::react::render(props.clone());
        let leptos = drawer::leptos::render(props);
        assert_eq!(leptos.surface, react.surface);
        assert_eq!(leptos.backdrop, react.backdrop);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(DrawerAnchor::Start, DrawerVariant::Modal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = drawer::react::render(props.clone());
        let sycamore = drawer::sycamore::render(props);
        assert_eq!(sycamore.surface, react.surface);
        assert_eq!(sycamore.backdrop, react.backdrop);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = DrawerLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(DrawerAnchor::Start, DrawerVariant::Modal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = drawer::react::render(props.clone());
        let dioxus = drawer::dioxus::render(props);
        assert_eq!(dioxus.surface, react.surface);
        assert_eq!(dioxus.backdrop, react.backdrop);
    }
}
