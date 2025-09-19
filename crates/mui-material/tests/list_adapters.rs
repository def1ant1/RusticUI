#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use mui_headless::list::{ListState, SelectionMode};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use mui_material::list::{self, ListDensity, ListItem, ListProps, ListTypography};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn sample_props() -> ListProps {
    ListProps::new(vec![
        ListItem::new("Primary").with_secondary("secondary"),
        ListItem::new("Analytics").with_meta("42"),
    ])
    .with_density(ListDensity::Comfortable)
    .with_primary_typography(ListTypography::Body1)
    .with_selection_mode(SelectionMode::Single)
    .with_automation_id("adapter-list")
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn build_state(len: usize) -> ListState {
    ListState::uncontrolled(len, &[], SelectionMode::Single)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn assert_markup(html: &str) {
    assert!(html.contains("data-component=\"mui-list\""));
    assert!(html.contains("data-automation-item"));
    assert!(html.contains("role=\"listbox\""));
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = list::yew::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = list::leptos::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = list::dioxus::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = list::sycamore::render(&props, &state);
        assert_markup(&html);
    }
}
