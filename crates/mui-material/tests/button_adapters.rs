use mui_headless::button::ButtonState;
use mui_material::button::{self, ButtonProps};

#[test]
fn all_adapters_render_consistently() {
    let props = ButtonProps::new("Save");
    let state = ButtonState::new(false, None);
    let expected = "<button role=\"button\" aria-pressed=\"false\">Save</button>";
    assert_eq!(button::yew::render(&props, &state), expected);
    assert_eq!(button::leptos::render(&props, &state), expected);
    assert_eq!(button::dioxus::render(&props, &state), expected);
    assert_eq!(button::sycamore::render(&props, &state), expected);
}

#[test]
fn pressed_state_reflects_in_output() {
    let props = ButtonProps::new("Toggle");
    let mut state = ButtonState::new(false, None);
    state.press(|_| {}); // toggle to pressed
    let rendered = button::yew::render(&props, &state);
    assert!(rendered.contains("aria-pressed=\"true\""));
}
