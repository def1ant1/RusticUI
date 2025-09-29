use rustic_ui_headless::click_away::ClickAwayState;
use rustic_ui_material::click_away::{
    click_away_root_attributes, render_click_away_boundary_html, ClickAwayBoundaryOptions,
};

fn seeded_state() -> ClickAwayState {
    let mut state = ClickAwayState::new();
    state.set_root_id(Some("checkout-surface"));
    state.engage();
    state
}

fn boundary_options() -> ClickAwayBoundaryOptions {
    ClickAwayBoundaryOptions {
        id: Some("checkout-surface".into()),
        analytics_id: Some("checkout-flow".into()),
        automation_id: None,
    }
}

#[test]
fn material_click_away_boundary_contains_expected_attributes() {
    let state = seeded_state();
    let options = boundary_options();
    let html = render_click_away_boundary_html(&state, &options, "dialog::checkout", "<p>Body</p>");
    assert!(html.contains("data-rustic-click-away=\"root\""));
    assert!(html.contains("data-automation-id=\"dialog::checkout\""));
    assert!(html.contains("id=\"checkout-surface\""));
}

#[test]
fn root_attributes_mirror_automation_contract() {
    let state = seeded_state();
    let options = boundary_options();
    let attrs = click_away_root_attributes(state.root_attributes(), "dialog::checkout", &options);

    assert_eq!(attrs.len(), 4);
    assert!(attrs
        .iter()
        .any(|(key, value)| key == "data-rustic-click-away" && value == "root"));
    assert!(attrs
        .iter()
        .any(|(key, value)| key == "id" && value == "checkout-surface"));
    assert!(attrs
        .iter()
        .any(|(key, value)| key == "data-rustic-analytics-id" && value == "checkout-flow"));
    assert!(attrs
        .iter()
        .any(|(key, value)| key == "data-automation-id" && value == "dialog::checkout"));
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_click_away_matches_material_baseline() {
    use rustic_ui_material::click_away::dioxus;

    let state = seeded_state();
    let options = boundary_options();
    let fallback = "dialog::checkout".to_string();
    let children = "<p>Body</p>".to_string();

    let baseline = render_click_away_boundary_html(&state, &options, &fallback, &children);
    let html = dioxus::render(&dioxus::ClickAwayBoundaryProps {
        state,
        options,
        automation_fallback: fallback.clone(),
        children,
        telemetry: Default::default(),
    });

    assert_eq!(html, baseline);
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_click_away_matches_material_baseline() {
    use rustic_ui_material::click_away::sycamore;

    let state = seeded_state();
    let options = boundary_options();
    let fallback = "dialog::checkout".to_string();
    let children = "<p>Body</p>".to_string();

    let baseline = render_click_away_boundary_html(&state, &options, &fallback, &children);
    let html = sycamore::render(&sycamore::ClickAwayBoundaryProps {
        state,
        options,
        automation_fallback: fallback.clone(),
        children,
        telemetry: Default::default(),
    });

    assert_eq!(html, baseline);
}
