use rustic_ui_headless::tabs::{ActivationMode, TabsOrientation, TabsState};
use rustic_ui_material::tabs::{self, TabListLayoutOptions, TabListProps};
use rustic_ui_material::Theme;

fn sample_state(orientation: TabsOrientation) -> TabsState {
    TabsState::new(
        3,
        Some(1),
        ActivationMode::Automatic,
        orientation,
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    )
}

fn build_props<'a>(
    state: &'a TabsState,
    layout: &'a TabListLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
) -> TabListProps<'a> {
    TabListProps {
        state,
        attributes: state.list_attributes().id("tabs"),
        children: "<button role=\"tab\">One</button>",
        layout,
        theme,
        viewport: Some(viewport),
        on_activate_event: Some("activate-tab"),
    }
}

#[test]
fn react_adapter_renders_wrapped_tab_list() {
    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();
    let state = sample_state(TabsOrientation::Horizontal);
    let html = tabs::react::render_tab_list(build_props(&state, &layout, &theme, 640));

    assert!(html.starts_with("<div class=\""));
    assert!(html.contains("data-on-activate=\"activate-tab\""));
    assert!(html.contains("role=\"tablist\""));
    assert!(html.contains("data-orientation=\"horizontal\""));
}

#[test]
fn react_adapter_honours_vertical_breakpoint() {
    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();
    let state = sample_state(TabsOrientation::Vertical);
    let html = tabs::react::render_tab_list(build_props(&state, &layout, &theme, 1200));

    assert!(html.contains("data-orientation=\"vertical\""));
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(TabsOrientation::Horizontal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = tabs::react::render_tab_list(props.clone());
        let yew = tabs::yew::render_tab_list(props);
        assert_eq!(yew, react);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(TabsOrientation::Horizontal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = tabs::react::render_tab_list(props.clone());
        let leptos = tabs::leptos::render_tab_list(props);
        assert_eq!(leptos, react);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(TabsOrientation::Horizontal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = tabs::react::render_tab_list(props.clone());
        let sycamore = tabs::sycamore::render_tab_list(props);
        assert_eq!(sycamore, react);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn matches_react_output() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = sample_state(TabsOrientation::Horizontal);
        let props = build_props(&state, &layout, &theme, 640);
        let react = tabs::react::render_tab_list(props.clone());
        let dioxus = tabs::dioxus::render_tab_list(props);
        assert_eq!(dioxus, react);
    }
}
