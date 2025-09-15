#![cfg(any(feature = "dioxus", feature = "sycamore"))]

/// Validate that dialog adapters attach the generated class and ARIA metadata
/// when open while emitting no markup when closed.

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use mui_material::dialog::dioxus;

    #[test]
    fn open_and_closed_states() {
        let props = dioxus::DialogProps {
            open: true,
            children: "body".into(),
            aria_label: "Settings".into(),
        };
        let out = dioxus::render(&props);
        assert!(out.starts_with("<div"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"dialog\""));
        assert!(out.contains("aria-modal=\"true\""));
        assert!(out.contains("aria-label=\"Settings\""));

        let closed = dioxus::render(&dioxus::DialogProps { open: false, ..Default::default() });
        assert!(closed.is_empty());
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use mui_material::dialog::sycamore;

    #[test]
    fn open_and_closed_states() {
        let props = sycamore::DialogProps {
            open: true,
            children: "body".into(),
            aria_label: "Settings".into(),
        };
        let out = sycamore::render(&props);
        assert!(out.starts_with("<div"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"dialog\""));
        assert!(out.contains("aria-modal=\"true\""));
        assert!(out.contains("aria-label=\"Settings\""));

        let closed = sycamore::render(&sycamore::DialogProps { open: false, ..Default::default() });
        assert!(closed.is_empty());
    }
}

