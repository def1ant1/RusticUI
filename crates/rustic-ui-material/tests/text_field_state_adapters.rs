#![cfg(any(feature = "dioxus", feature = "sycamore"))]

use rustic_ui_headless::text_field::TextFieldState;
use rustic_ui_material::text_field::{self, TextFieldColor, TextFieldSize, TextFieldVariant};
use std::time::Duration;

fn hydrated_state() -> TextFieldState {
    let mut state = TextFieldState::uncontrolled("seed", Some(Duration::from_millis(250)));
    state.change("updated", |_| {});
    state.set_errors(vec!["Required".into(), "Must be unique".into()]);
    state.commit(|_| {});
    state
}

#[cfg(feature = "dioxus")]
fn dioxus_props() -> text_field::dioxus::TextFieldProps {
    text_field::dioxus::TextFieldProps {
        placeholder: "Enter value".into(),
        aria_label: "Project name".into(),
        color: TextFieldColor::Primary,
        size: TextFieldSize::Medium,
        variant: TextFieldVariant::Outlined,
        style_overrides: None,
        status_id: Some("status-node".into()),
        analytics_id: Some("tf-analytics".into()),
    }
}

#[cfg(feature = "sycamore")]
fn sycamore_props() -> text_field::sycamore::TextFieldProps {
    text_field::sycamore::TextFieldProps {
        placeholder: "Enter value".into(),
        aria_label: "Project name".into(),
        color: TextFieldColor::Primary,
        size: TextFieldSize::Medium,
        variant: TextFieldVariant::Outlined,
        style_overrides: None,
        status_id: Some("status-node".into()),
        analytics_id: Some("tf-analytics".into()),
    }
}

fn assert_text_field_contract(html: &str) {
    assert!(html.starts_with("<input"));
    assert!(html.contains("class=\""));
    assert!(html.contains("aria-label=\"Project name\""));
    assert!(html.contains("placeholder=\"Enter value\""));
    assert!(html.contains("value=\"updated\""));
    assert!(html.contains("data-dirty=\"true\""));
    assert!(html.contains("data-visited=\"true\""));
    assert!(html.contains("aria-invalid=\"true\""));
    assert!(html.contains("aria-describedby=\"status-node\""));
    assert!(html.contains("data-analytics-id=\"tf-analytics\""));
    assert!(html.contains("data-status-message"));
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_text_field_reflects_state_metadata() {
    let state = hydrated_state();
    let html = text_field::dioxus::render(&dioxus_props(), &state);
    assert_text_field_contract(&html);
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_text_field_mirrors_headless_state() {
    let state = hydrated_state();
    let html = text_field::sycamore::render(&sycamore_props(), &state);
    assert_text_field_contract(&html);
}

#[cfg(all(feature = "dioxus", feature = "sycamore"))]
#[test]
fn dioxus_and_sycamore_outputs_match() {
    let state = hydrated_state();
    let dioxus_html = text_field::dioxus::render(&dioxus_props(), &state);
    let sycamore_html = text_field::sycamore::render(&sycamore_props(), &state);
    assert_eq!(dioxus_html, sycamore_html);
}
