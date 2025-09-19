#![cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]

use mui_headless::radio::{RadioGroupState, RadioOrientation};
use mui_material::radio::{self, RadioGroupProps};

fn sample_state() -> RadioGroupState {
    RadioGroupState::uncontrolled(
        vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    )
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_group_and_options() {
        let state = sample_state();
        let props = RadioGroupProps::from_state(&state);
        let out = radio::yew::render(&props, &state);
        assert!(out.contains("role=\"radiogroup\""));
        assert!(out.contains("Alpha"));
        assert!(out.contains("Beta"));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn renders_orientation_attribute() {
        let state = sample_state();
        let props = RadioGroupProps::from_state(&state);
        let out = radio::leptos::render(&props, &state);
        assert!(out.contains("aria-orientation=\"horizontal\""));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn renders_data_indices() {
        let state = sample_state();
        let props = RadioGroupProps::from_state(&state);
        let out = radio::dioxus::render(&props, &state);
        assert!(out.contains("data-index=\"0\""));
        assert!(out.contains("data-index=\"2\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn renders_all_labels() {
        let state = sample_state();
        let props = RadioGroupProps::from_state(&state);
        let out = radio::sycamore::render(&props, &state);
        assert!(out.contains("Alpha"));
        assert!(out.contains("Gamma"));
    }
}
