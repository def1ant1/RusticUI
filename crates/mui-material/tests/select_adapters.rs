use mui_headless::select::SelectState;
use mui_material::select::{self, SelectOption, SelectProps};

fn sample_props() -> SelectProps {
    SelectProps::new(
        "Select",
        vec![
            SelectOption::new("Alpha", "a"),
            SelectOption::new("Beta", "b"),
        ],
    )
    .with_automation_id("adapter-select")
}

fn build_state(count: usize) -> SelectState {
    SelectState::new(
        count,
        None,
        true,
        unsafe { std::mem::transmute(1u8) },
        unsafe { std::mem::transmute(1u8) },
    )
}

fn assert_portal_markup(html: &str) {
    assert!(html.contains("data-portal-root=\"adapter-select-popover\""));
    assert!(html.contains("adapter-select-popover-anchor"));
    assert_eq!(
        html.matches("<ul").count(),
        1,
        "options should only be rendered once"
    );
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = select::yew::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-select\""));
        assert_portal_markup(&html);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = select::leptos::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-select\""));
        assert_portal_markup(&html);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = select::dioxus::render(&props, &state);
        assert!(html.contains("data-component=\"mui-select\""));
        assert_portal_markup(&html);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.options.len());
        let html = select::sycamore::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-select\""));
        assert_portal_markup(&html);
    }
}
