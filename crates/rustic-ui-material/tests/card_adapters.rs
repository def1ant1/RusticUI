#![cfg(any(feature = "dioxus", feature = "sycamore"))]

/// Ensure the lightweight card adapters emit a themed class on the root `<div>`
/// container. Cards do not expose additional ARIA metadata today so verifying
/// the presence of the class guards against style regressions.

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use rustic_ui_material::card::dioxus;

    #[test]
    fn renders_div_with_class() {
        let props = dioxus::CardProps {
            children: "content".into(),
        };
        let out = dioxus::render(&props);
        assert!(out.starts_with("<div"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains(">content</div>"));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use rustic_ui_material::card::sycamore;

    #[test]
    fn renders_div_with_class() {
        let props = sycamore::CardProps {
            children: "content".into(),
        };
        let out = sycamore::render(&props);
        assert!(out.starts_with("<div"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains(">content</div>"));
    }
}
