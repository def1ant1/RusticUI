#![cfg(any(feature = "dioxus", feature = "sycamore"))]

/// Verify that each framework adapter for [`AppBar`] emits a `<header>` element
/// with the generated class and required ARIA attributes. Tests are compiled
/// per feature so frameworks can be exercised independently.

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use rustic_ui_material::app_bar::dioxus;

    #[test]
    fn renders_header_with_aria() {
        let props = dioxus::AppBarProps {
            title: "Dashboard".into(),
            aria_label: "Application header".into(),
            color: rustic_ui_material::app_bar::AppBarColor::Primary,
            size: rustic_ui_material::app_bar::AppBarSize::Medium,
        };
        let out = dioxus::render(&props);
        assert!(out.starts_with("<header"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"banner\""));
        assert!(out.contains("aria-label=\"Application header\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use rustic_ui_material::app_bar::sycamore;

    #[test]
    fn renders_header_with_aria() {
        let props = sycamore::AppBarProps {
            title: "Dashboard".into(),
            aria_label: "Application header".into(),
            color: rustic_ui_material::app_bar::AppBarColor::Primary,
            size: rustic_ui_material::app_bar::AppBarSize::Medium,
        };
        let out = sycamore::render(&props);
        assert!(out.starts_with("<header"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"banner\""));
        assert!(out.contains("aria-label=\"Application header\""));
    }
}
