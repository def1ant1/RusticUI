#![cfg(any(feature = "dioxus", feature = "sycamore"))]

//! Snapshot-style assertions that validate the HTML emitted by each
//! framework specific Snackbar adapter. Keeping these checks close to the
//! render functions guards against regressions in the scoped class generation
//! and ARIA wiring shared across frameworks.

use mui_material::snackbar::{SnackbarColor, SnackbarSize, SnackbarVariant};

/// Dioxus adapter coverage ensures the SSR-focused `render` helper attaches
/// the theme-derived class and politely announces updates with
/// `role="status"`. These targeted assertions minimise future maintenance by
/// avoiding brittle full string comparisons while still proving that critical
/// attributes are present.
#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;
    use mui_material::snackbar::dioxus;

    #[test]
    fn renders_status_snackbar_with_class() {
        let props = dioxus::SnackbarProps {
            message: "Profile saved".into(),
            color: SnackbarColor::Primary,
            size: SnackbarSize::Medium,
            variant: SnackbarVariant::Contained,
        };
        let out = dioxus::render(&props);

        assert!(out.starts_with("<div"), "unexpected markup: {}", out);
        assert!(out.contains("class=\""), "missing scoped class: {}", out);
        assert!(
            out.contains("role=\"status\""),
            "missing ARIA role: {}",
            out
        );
        assert!(out.contains(">Profile saved</div>"));
    }
}

/// Sycamore adapter coverage mirrors the Dioxus test so both SSR renderers
/// are exercised in isolation. This guarantees each framework surfaces the
/// generated class and ARIA role without duplicating snapshot payloads.
#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;
    use mui_material::snackbar::sycamore;

    #[test]
    fn renders_status_snackbar_with_class() {
        let props = sycamore::SnackbarProps {
            message: "Profile saved".into(),
            color: SnackbarColor::Primary,
            size: SnackbarSize::Medium,
            variant: SnackbarVariant::Contained,
        };
        let out = sycamore::render(&props);

        assert!(out.starts_with("<div"), "unexpected markup: {}", out);
        assert!(out.contains("class=\""), "missing scoped class: {}", out);
        assert!(
            out.contains("role=\"status\""),
            "missing ARIA role: {}",
            out
        );
        assert!(out.contains(">Profile saved</div>"));
    }
}
