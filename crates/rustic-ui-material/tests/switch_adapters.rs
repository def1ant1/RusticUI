#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use rustic_ui_headless::switch::SwitchState;
use rustic_ui_material::switch::{self, SwitchProps};

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_on_state() {
        let props = SwitchProps::new("Notifications");
        let mut state = SwitchState::uncontrolled(false, false);
        state.toggle(|_| {});
        let out = switch::yew::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.contains("data-on=\"true\""));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn renders_off_state() {
        let props = SwitchProps::new("Notifications");
        let state = SwitchState::uncontrolled(false, false);
        let out = switch::leptos::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.contains("aria-checked=\"false\""));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn includes_focus_attribute() {
        let mut state = SwitchState::uncontrolled(false, false);
        state.focus();
        let props = SwitchProps::new("Notifications");
        let out = switch::dioxus::render(&props, &state);
        assert!(out.contains("data-focus-visible"));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn renders_basic_markup() {
        let props = SwitchProps::new("Notifications");
        let state = SwitchState::uncontrolled(false, false);
        let out = switch::sycamore::render(&props, &state);
        assert!(out.contains("role=\"switch\""));
        assert!(out.ends_with(">Notifications</span>"));
    }
}
