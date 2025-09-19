#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use mui_headless::checkbox::CheckboxState;
use mui_material::checkbox::{self, CheckboxProps};

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::yew::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked=\"false\""));
        assert!(out.ends_with(">Subscribe</span>"));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::leptos::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked=\"false\""));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let mut state = CheckboxState::uncontrolled(false, false);
        state.toggle(|_| {});
        let out = checkbox::dioxus::render(&props, &state);
        assert!(out.contains("aria-checked=\"true\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn renders_with_accessibility_attributes() {
        let props = CheckboxProps::new("Subscribe");
        let state = CheckboxState::uncontrolled(false, false);
        let out = checkbox::sycamore::render(&props, &state);
        assert!(out.contains("role=\"checkbox\""));
        assert!(out.contains("aria-checked"));
    }
}
