# Selection Controls with Dioxus

```rust
use dioxus::prelude::*;
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use std::rc::Rc;
use rustic_ui_material::radio::{self, RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent};

pub fn selection_controls(cx: Scope) -> Element {
    let checkbox_state = CheckboxState::uncontrolled(false, false);
    let switch_state = SwitchState::uncontrolled(false, true);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(2),
    );

    let on_change: Rc<dyn Fn(RadioChangeEvent)> = Rc::new(|event| {
        println!("radio-change next={}", event.next);
    });
    let telemetry: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        println!("telemetry::{:?}", event);
    });

    cx.render(rsx! {
        div { dangerous_inner_html: checkbox::dioxus::render(&CheckboxProps::new("Accept terms"), &checkbox_state) }
        div { dangerous_inner_html: switch::dioxus::render(&SwitchProps::new("Enable quick checkout"), &switch_state) }
        radio::dioxus::DioxusRadioGroup {
            group: RadioGroupProps::from_state(&radio_state),
            state: radio_state.clone(),
            on_change: Some(on_change),
            telemetry_delegate: Some(telemetry),
        }
    })
}
```
