#![cfg(any(feature = "dioxus", feature = "sycamore"))]

use mui_material::text_field::{TextFieldColor, TextFieldSize, TextFieldVariant};

/// The text field adapters render an `<input>` element with a themed class and
/// accessible `aria-label`. Assertions are performed per framework so that each
/// adapter can be compiled independently.

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use mui_material::text_field::dioxus;

    #[test]
    fn renders_input_with_class_and_label() {
        let props = dioxus::TextFieldProps {
            value: "v".into(),
            placeholder: "p".into(),
            aria_label: "Email".into(),
            color: TextFieldColor::Primary,
            size: TextFieldSize::Medium,
            variant: TextFieldVariant::Outlined,
            style_overrides: None,
        };
        let out = dioxus::render(&props);
        assert!(out.starts_with("<input"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("aria-label=\"Email\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use mui_material::text_field::sycamore;

    #[test]
    fn renders_input_with_class_and_label() {
        let props = sycamore::TextFieldProps {
            value: "v".into(),
            placeholder: "p".into(),
            aria_label: "Email".into(),
            color: TextFieldColor::Primary,
            size: TextFieldSize::Medium,
            variant: TextFieldVariant::Outlined,
            style_overrides: None,
        };
        let out = sycamore::render(&props);
        assert!(out.starts_with("<input"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("aria-label=\"Email\""));
    }
}

