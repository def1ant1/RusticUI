# Selection Controls with Dioxus

```rust
use dioxus::prelude::*;
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_headless::switch::SwitchState;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use rustic_ui_material::radio::{self, RadioGroupProps};

pub fn selection_controls(cx: Scope) -> Element {
    let checkbox_state = CheckboxState::uncontrolled(false, false);
    let switch_state = SwitchState::uncontrolled(false, true);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(2),
    );

    cx.render(rsx! {
        div { dangerous_inner_html: checkbox::dioxus::render(&CheckboxProps::new("Accept terms"), &checkbox_state) }
        div { dangerous_inner_html: switch::dioxus::render(&SwitchProps::new("Enable quick checkout"), &switch_state) }
        div { dangerous_inner_html: radio::dioxus::render(&RadioGroupProps::from_state(&radio_state), &radio_state) }
    })
}
```
