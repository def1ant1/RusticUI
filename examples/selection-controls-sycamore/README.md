# Selection Controls with Sycamore

```rust
use sycamore::prelude::*;
use mui_headless::checkbox::CheckboxState;
use mui_headless::switch::SwitchState;
use mui_headless::radio::{RadioGroupState, RadioOrientation};
use mui_material::checkbox::{self, CheckboxProps};
use mui_material::switch::{self, SwitchProps};
use mui_material::radio::{self, RadioGroupProps};

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

    view! { cx,
        div(dangerously_set_inner_html=checkbox::sycamore::render(&CheckboxProps::new("Light theme"), &checkbox_state)) {}
        div(dangerously_set_inner_html=switch::sycamore::render(&SwitchProps::new("Enable system overrides"), &switch_state)) {}
        div(dangerously_set_inner_html=radio::sycamore::render(&RadioGroupProps::from_state(&radio_state), &radio_state)) {}
    }
}
```
