use rustic_ui_headless::checkbox::{CheckboxState, CheckboxValue};
use rustic_ui_headless::interaction::ControlKey;

fn attr<'a>(attrs: &'a [(&'static str, String)], key: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find_map(|(k, v)| if *k == key { Some(v.as_str()) } else { None })
}

#[test]
fn keyboard_space_cycles_binary_states() {
    let mut state = CheckboxState::uncontrolled(false, CheckboxValue::Off);
    state.on_key(ControlKey::Space, |_| {});
    assert_eq!(state.checked(), CheckboxValue::On);
    state.on_key(ControlKey::Space, |_| {});
    assert_eq!(state.checked(), CheckboxValue::Off);
}

#[test]
fn keyboard_space_promotes_indeterminate_to_on() {
    let mut state = CheckboxState::uncontrolled(false, CheckboxValue::Indeterminate);
    state.on_key(ControlKey::Space, |_| {});
    assert_eq!(state.checked(), CheckboxValue::On);
}

#[test]
fn controlled_checkbox_syncs_after_callback() {
    let mut state = CheckboxState::controlled(false, CheckboxValue::Off);
    let mut requested = None;
    state.toggle(|value| requested = Some(value));
    assert_eq!(state.checked(), CheckboxValue::Off);
    let next = requested.expect("callback should fire");
    assert_eq!(next, CheckboxValue::On);
    state.sync_checked(next);
    assert_eq!(state.checked(), CheckboxValue::On);
    state.sync_checked(CheckboxValue::Indeterminate);
    assert_eq!(state.checked(), CheckboxValue::Indeterminate);
}

#[test]
fn aria_attributes_cover_three_states() {
    let states = [
        CheckboxValue::Off,
        CheckboxValue::On,
        CheckboxValue::Indeterminate,
    ];
    for value in states {
        let state = CheckboxState::controlled(false, value);
        let attrs = state.aria_attributes();
        let aria_checked = attr(&attrs, "aria-checked").unwrap();
        let data_checked = attr(&attrs, "data-checked").unwrap();
        let indeterminate = attr(&attrs, "data-indeterminate").unwrap();
        match value {
            CheckboxValue::Off => {
                assert_eq!(aria_checked, "false");
                assert_eq!(data_checked, "false");
                assert_eq!(indeterminate, "false");
            }
            CheckboxValue::On => {
                assert_eq!(aria_checked, "true");
                assert_eq!(data_checked, "true");
                assert_eq!(indeterminate, "false");
            }
            CheckboxValue::Indeterminate => {
                assert_eq!(aria_checked, "mixed");
                assert_eq!(data_checked, "false");
                assert_eq!(indeterminate, "true");
            }
        }
    }
}
