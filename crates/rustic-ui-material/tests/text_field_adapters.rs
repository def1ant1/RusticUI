#![cfg(any(feature = "dioxus", feature = "sycamore"))]

use mui_headless::text_field::TextFieldState;
use mui_material::text_field::{TextFieldColor, TextFieldSize, TextFieldVariant};

/// The text field adapters render an `<input>` element with a themed class and
/// accessible `aria-label`. Assertions are performed per framework so that each
/// adapter can be compiled independently.

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use mui_material::text_field::dioxus;

    #[test]
    fn renders_state_driven_metadata() {
        let mut state = TextFieldState::uncontrolled("hello", None);
        state.change("updated", |_| {});
        state.set_errors(vec!["Required".into()]);
        state.commit(|_| {});

        let props = dioxus::TextFieldProps {
            placeholder: "p".into(),
            aria_label: "Email".into(),
            color: TextFieldColor::Primary,
            size: TextFieldSize::Medium,
            variant: TextFieldVariant::Outlined,
            style_overrides: None,
            status_id: Some("status-node".into()),
            analytics_id: Some("analytics-42".into()),
        };
        let out = dioxus::render(&props, &state);
        assert!(out.starts_with("<input"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("aria-label=\"Email\""));
        assert!(out.contains("value=\"updated\""));
        assert!(out.contains("data-dirty=\"true\""));
        assert!(out.contains("data-visited=\"true\""));
        assert!(out.contains("aria-invalid=\"true\""));
        assert!(out.contains("aria-describedby=\"status-node\""));
        assert!(out.contains("data-analytics-id=\"analytics-42\""));
        assert!(out.contains("data-status-message"));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use mui_material::text_field::sycamore;

    #[test]
    fn renders_state_driven_metadata() {
        let mut state = TextFieldState::uncontrolled("hello", None);
        state.change("updated", |_| {});
        state.set_errors(vec!["Required".into()]);
        state.commit(|_| {});

        let props = sycamore::TextFieldProps {
            placeholder: "p".into(),
            aria_label: "Email".into(),
            color: TextFieldColor::Primary,
            size: TextFieldSize::Medium,
            variant: TextFieldVariant::Outlined,
            style_overrides: None,
            status_id: Some("status-node".into()),
            analytics_id: Some("analytics-42".into()),
        };
        let out = sycamore::render(&props, &state);
        assert!(out.starts_with("<input"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("aria-label=\"Email\""));
        assert!(out.contains("value=\"updated\""));
        assert!(out.contains("data-dirty=\"true\""));
        assert!(out.contains("data-visited=\"true\""));
        assert!(out.contains("aria-invalid=\"true\""));
        assert!(out.contains("aria-describedby=\"status-node\""));
        assert!(out.contains("data-analytics-id=\"analytics-42\""));
        assert!(out.contains("data-status-message"));
    }
}
