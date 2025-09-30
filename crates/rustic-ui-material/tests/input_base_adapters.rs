#![cfg(any(
    feature = "react",
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]

use rustic_ui_headless::input_base::{InputSelection, InputState};
use rustic_ui_material::input_base::{
    render_input_base_html, InputBaseColor, InputBaseRenderConfig, InputBaseSize, InputBaseVariant,
};

fn hydrated_state() -> InputState {
    let mut state = InputState::uncontrolled("seed", Some(InputSelection::new(0, 4)));
    state.change("updated", Some(InputSelection::collapsed(7)));
    state.set_errors(vec![String::from("Required"), String::from("Unique")]);
    let _ = state.commit();
    state
}

fn assert_contract(html: &str) {
    assert!(html.starts_with("<input"));
    assert!(
        html.contains("class=\""),
        "missing class attribute: {}",
        html
    );
    assert!(html.contains("aria-label=\"Project\""));
    assert!(html.contains("placeholder=\"Enter value\""));
    assert!(html.contains("value=\"updated\""));
    assert!(html.contains("data-dirty=\"true\""));
    assert!(html.contains("data-visited=\"true\""));
    assert!(html.contains("data-status-message=\"Required\\nUnique\""));
    assert!(html.contains("aria-invalid=\"true\""));
    assert!(html.contains("aria-describedby=\"status-node\""));
    assert!(html.contains("data-analytics-id=\"input-analytics\""));
    assert!(html.contains("data-selection-start=\"7\""));
}

fn render_config<'a>(state: &'a InputState) -> InputBaseRenderConfig<'a> {
    InputBaseRenderConfig {
        state,
        placeholder: "Enter value",
        aria_label: "Project",
        input_type: "text",
        status_id: Some("status-node"),
        analytics_id: Some("input-analytics"),
        color: InputBaseColor::Primary,
        variant: InputBaseVariant::Outlined,
        size: InputBaseSize::Medium,
        style_overrides: None,
    }
}

#[cfg(feature = "react")]
mod react_tests {
    use super::*;
    use rustic_ui_material::input_base::react;

    #[test]
    fn react_adapter_renders_deterministic_markup() {
        let state = hydrated_state();
        let props = react::InputBaseProps {
            state: &state,
            placeholder: "Enter value",
            aria_label: "Project",
            input_type: "text",
            status_id: Some("status-node"),
            analytics_id: Some("input-analytics"),
            color: InputBaseColor::Primary,
            variant: InputBaseVariant::Outlined,
            size: InputBaseSize::Medium,
            style_overrides: None,
        };
        let html = react::render(&props);
        assert_contract(&html);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use rustic_ui_material::input_base::dioxus;

    #[test]
    fn dioxus_adapter_matches_ssr_contract() {
        let state = hydrated_state();
        let props = dioxus::InputBaseProps {
            placeholder: "Enter value".into(),
            aria_label: "Project".into(),
            input_type: "text".into(),
            color: InputBaseColor::Primary,
            variant: InputBaseVariant::Outlined,
            size: InputBaseSize::Medium,
            style_overrides: None,
            status_id: Some("status-node".into()),
            analytics_id: Some("input-analytics".into()),
        };
        let html = dioxus::render(&props, &state);
        assert_contract(&html);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use rustic_ui_material::input_base::sycamore;

    #[test]
    fn sycamore_adapter_mirrors_dioxus_output() {
        let state = hydrated_state();
        let props = sycamore::InputBaseProps {
            placeholder: "Enter value".into(),
            aria_label: "Project".into(),
            input_type: "text".into(),
            color: InputBaseColor::Primary,
            variant: InputBaseVariant::Outlined,
            size: InputBaseSize::Medium,
            style_overrides: None,
            status_id: Some("status-node".into()),
            analytics_id: Some("input-analytics".into()),
        };
        let html = sycamore::render(&props, &state);
        assert_contract(&html);
    }
}

#[cfg(feature = "yew")]
mod yew_ssr_tests {
    use super::*;

    #[test]
    fn yew_hydration_snapshot_matches_helpers() {
        let state = hydrated_state();
        let html = render_input_base_html(&render_config(&state));
        assert_contract(&html);
    }
}

#[cfg(feature = "leptos")]
mod leptos_ssr_tests {
    use super::*;

    #[test]
    fn leptos_hydration_snapshot_matches_helpers() {
        let state = hydrated_state();
        let html = render_input_base_html(&render_config(&state));
        assert_contract(&html);
    }
}
