# Selection Controls with Yew

This example demonstrates how to wire the headless selection control state
machines from `rustic_ui_headless` into a Yew application using the render helpers
from `rustic_ui_material`.

```rust
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use rustic_ui_material::radio::{self, RadioGroupProps};
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use yew::prelude::*;

#[function_component(SelectionControls)]
fn selection_controls() -> Html {
    let checkbox_state = CheckboxState::uncontrolled(false, false);
    let switch_state = rustic_ui_headless::switch::SwitchState::uncontrolled(false, true);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Email".into(), "SMS".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    );

    let checkbox = Html::from_html_unchecked(AttrValue::from(
        checkbox::yew::render(&CheckboxProps::new("Receive updates"), &checkbox_state),
    ));
    let switch = Html::from_html_unchecked(AttrValue::from(
        switch::yew::render(&SwitchProps::new("Enable notifications"), &switch_state),
    ));
    let radio = Html::from_html_unchecked(AttrValue::from(
        radio::yew::render(&RadioGroupProps::from_state(&radio_state), &radio_state),
    ));

    html! {
        <>
            {checkbox}
            {switch}
            {radio}
        </>
    }
}
```
