use mui_headless::button::ButtonState;
use mui_material::button::{self, ButtonProps};

/// Each framework adapter should emit a `<button>` element with the generated
/// class, `role="button"` and the current `aria-pressed` state.

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn renders_with_aria() {
        let props = ButtonProps::new("Save");
        let state = ButtonState::new(false, None);
        let out = button::yew::render(&props, &state);
        assert!(out.starts_with("<button"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"button\""));
        assert!(out.contains("aria-pressed=\"false\""));
        assert!(out.ends_with(">Save</button>"));
    }

    #[test]
    fn pressed_state_reflects_in_output() {
        let props = ButtonProps::new("Toggle");
        let mut state = ButtonState::new(false, None);
        state.press(|_| {}); // toggle to pressed
        let out = button::yew::render(&props, &state);
        assert!(out.contains("class=\""));
        assert!(out.contains("aria-pressed=\"true\""));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn renders_with_aria() {
        let props = ButtonProps::new("Save");
        let state = ButtonState::new(false, None);
        let out = button::leptos::render(&props, &state);
        assert!(out.starts_with("<button"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"button\""));
        assert!(out.contains("aria-pressed=\"false\""));
        assert!(out.ends_with(">Save</button>"));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn renders_with_aria() {
        let props = ButtonProps::new("Save");
        let state = ButtonState::new(false, None);
        let out = button::dioxus::render(&props, &state);
        assert!(out.starts_with("<button"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"button\""));
        assert!(out.contains("aria-pressed=\"false\""));
        assert!(out.ends_with(">Save</button>"));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn renders_with_aria() {
        let props = ButtonProps::new("Save");
        let state = ButtonState::new(false, None);
        let out = button::sycamore::render(&props, &state);
        assert!(out.starts_with("<button"));
        assert!(out.contains("class=\""), "missing class attribute: {}", out);
        assert!(out.contains("role=\"button\""));
        assert!(out.contains("aria-pressed=\"false\""));
        assert!(out.ends_with(">Save</button>"));
    }
}
