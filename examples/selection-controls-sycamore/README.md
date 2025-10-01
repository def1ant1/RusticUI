# Selection Controls with Sycamore

```rust
use std::rc::Rc;
use sycamore::prelude::*;
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use rustic_ui_material::radio::{self, RadioGroupProps, RadioChangeEvent, RadioTelemetryEvent};

#[component]
pub fn SelectionControls<G: Html>(cx: Scope) -> View<G> {
    let checkbox_state = CheckboxState::uncontrolled(false, true);
    let switch_state = SwitchState::uncontrolled(false, false);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Light".into(), "Dark".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    );

    let radio_props = RadioGroupProps::from_state(&radio_state);
    let mut interactive = radio::sycamore::SycamoreRadioGroupProps::new(radio_props.clone(), radio_state.clone());
    interactive.telemetry_delegate = Some(Rc::new(|event: RadioTelemetryEvent| {
        println!("telemetry::{:?}", event);
    }));
    interactive.on_change = Some(Rc::new(|event: RadioChangeEvent| {
        println!("change::next={}", event.next);
    }));

    view! { cx,
        div(dangerously_set_inner_html=checkbox::sycamore::render(&CheckboxProps::new("Light theme"), &checkbox_state)) {}
        div(dangerously_set_inner_html=switch::sycamore::render(&SwitchProps::new("Enable system overrides"), &switch_state)) {}
        radio::sycamore::SycamoreRadioGroup(interactive)
    }
}
```
