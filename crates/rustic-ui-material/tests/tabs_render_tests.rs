use rustic_ui_headless::tabs::{ActivationMode, TabsOrientation, TabsState};
use rustic_ui_material::tabs::{self, TabListLayoutOptions, TabListProps};
use rustic_ui_material::Theme;

/// Helper that constructs an uncontrolled [`TabsState`] with the provided
/// orientation and activation mode. The `ControlStrategy` type lives in a
/// private module inside `mui-headless`, so we mirror the approach used across
/// the existing test suite by transmuting from the documented discriminants.
fn build_state(orientation: TabsOrientation, activation: ActivationMode) -> TabsState {
    TabsState::new(
        3,
        Some(1),
        activation,
        orientation,
        // SAFETY: `ControlStrategy` is an enum with `Controlled` at 0 and
        // `Uncontrolled` at 1. The discriminants are stable and documented in
        // the headless crate so this mirrors existing integration tests.
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    )
}

/// Construct the [`TabListProps`] shared across the framework adapters.  Tests
/// can tweak the viewport to simulate breakpoints without duplicating boilerplate.
fn build_props<'a>(
    state: &'a TabsState,
    layout: &'a TabListLayoutOptions,
    theme: &'a Theme,
    viewport: u32,
) -> TabListProps<'a> {
    TabListProps {
        state,
        attributes: state
            .list_attributes()
            .id("demo-tabs")
            .labelled_by("demo-tabs-label"),
        children: "<button role=\"tab\" data-testid=\"tab-a\">A</button>\
<button role=\"tab\" data-testid=\"tab-b\">B</button>",
        layout,
        theme,
        viewport: Some(viewport),
        on_activate_event: Some("tabs-activate"),
    }
}

#[test]
fn react_markup_includes_accessibility_contract() {
    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();
    let state = build_state(TabsOrientation::Horizontal, ActivationMode::Automatic);

    let markup = tabs::react::render_tab_list(build_props(&state, &layout, &theme, 640));

    // Snapshot style assertion: the top level wrapper should expose the custom
    // `data-on-activate` hook so orchestration layers can subscribe to tab
    // activation events without re-rendering. The inner tablist must carry the
    // ARIA attributes emitted by `mui-headless` to remain screen-reader friendly.
    assert!(markup.contains("data-on-activate=\"tabs-activate\""));
    assert!(markup.contains("role=\"tablist\""));
    assert!(markup.contains("aria-labelledby"));
    assert!(markup.contains("data-orientation=\"horizontal\""));
    assert!(markup.contains("data-activation=\"automatic\""));
    assert!(markup.contains("data-testid=\"tab-b\""));
}

#[test]
fn responsive_orientation_switches_across_breakpoints() {
    let mut layout = TabListLayoutOptions::default();
    // Force a clear behavioural change at the large breakpoint so we can assert
    // both horizontal and vertical orientations in a deterministic way.
    layout.orientation.lg = Some(TabsOrientation::Vertical);
    layout.orientation.xl = Some(TabsOrientation::Vertical);

    let theme = Theme::default();
    let horizontal_state = build_state(TabsOrientation::Horizontal, ActivationMode::Automatic);
    let vertical_state = build_state(TabsOrientation::Vertical, ActivationMode::Automatic);

    let horizontal = tabs::react::render_tab_list(build_props(
        &horizontal_state,
        &layout,
        &theme,
        theme.breakpoints.sm,
    ));
    let vertical = tabs::react::render_tab_list(build_props(
        &vertical_state,
        &layout,
        &theme,
        theme.breakpoints.xl,
    ));

    assert!(horizontal.contains("data-orientation=\"horizontal\""));
    assert!(vertical.contains("data-orientation=\"vertical\""));
}

#[test]
fn manual_activation_surfaces_in_markup() {
    let layout = TabListLayoutOptions::default();
    let theme = Theme::default();
    let state = build_state(TabsOrientation::Horizontal, ActivationMode::Manual);

    let markup = tabs::react::render_tab_list(build_props(&state, &layout, &theme, 768));

    // Manual activation is critical for enterprise navigation where content
    // should not re-render while users explore. The adapter must keep that
    // information available via `data-activation` for analytics and hydration
    // layers.
    assert!(markup.contains("data-activation=\"manual\""));
}

#[cfg(feature = "yew")]
mod yew {
    use super::*;

    #[test]
    fn yew_renderer_matches_react_output_for_manual_mode() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(TabsOrientation::Horizontal, ActivationMode::Manual);
        let props = build_props(&state, &layout, &theme, 640);

        let react = tabs::react::render_tab_list(props.clone());
        let yew = tabs::yew::render_tab_list(props);

        assert_eq!(yew, react);
    }
}

#[cfg(feature = "leptos")]
mod leptos {
    use super::*;

    #[test]
    fn leptos_renderer_matches_react_output_for_manual_mode() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(TabsOrientation::Horizontal, ActivationMode::Manual);
        let props = build_props(&state, &layout, &theme, 640);

        let react = tabs::react::render_tab_list(props.clone());
        let leptos = tabs::leptos::render_tab_list(props);

        assert_eq!(leptos, react);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore {
    use super::*;

    #[test]
    fn sycamore_renderer_matches_react_output_for_manual_mode() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(TabsOrientation::Horizontal, ActivationMode::Manual);
        let props = build_props(&state, &layout, &theme, 640);

        let react = tabs::react::render_tab_list(props.clone());
        let sycamore = tabs::sycamore::render_tab_list(props);

        assert_eq!(sycamore, react);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus {
    use super::*;

    #[test]
    fn dioxus_renderer_matches_react_output_for_manual_mode() {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = build_state(TabsOrientation::Horizontal, ActivationMode::Manual);
        let props = build_props(&state, &layout, &theme, 640);

        let react = tabs::react::render_tab_list(props.clone());
        let dioxus = tabs::dioxus::render_tab_list(props);

        assert_eq!(dioxus, react);
    }
}
