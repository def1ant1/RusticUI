# Selection Controls with Yew

This example demonstrates how to wire the headless selection control state
machines from `rustic_ui_headless` into a Yew application using the render helpers
from `rustic_ui_material`.

```rust
use rustic_ui_headless::checkbox::CheckboxState;
use gloo_console::log;
use rustic_ui_material::checkbox::{self, CheckboxProps};
use rustic_ui_material::switch::{self, SwitchProps};
use rustic_ui_material::radio::{self, RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent};
use rustic_ui_material::radio::yew::YewRadioGroup;
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

    let on_radio_change = Callback::from(|event: RadioChangeEvent| {
        log!(format!("selected index: {}", event.next));
    });
    let on_radio_telemetry = Callback::from(|event: RadioTelemetryEvent| {
        log!(format!("telemetry: {:?}", event));
    });

    html! {
        <>
            { Html::from_html_unchecked(AttrValue::from(
                checkbox::yew::render(&CheckboxProps::new("Receive updates"), &checkbox_state),
            )) }
            { Html::from_html_unchecked(AttrValue::from(
                switch::yew::render(&SwitchProps::new("Enable notifications"), &switch_state),
            )) }
            <YewRadioGroup
                group={RadioGroupProps::from_state(&radio_state)}
                state={radio_state}
                on_change={Some(on_radio_change)}
                telemetry_delegate={Some(on_radio_telemetry)}
            />
        </>
    }
}
```

The radio group now emits structured telemetry before any consumer callbacks
execute, ensuring analytics capture remains deterministic even as the UI mutates
through `RadioGroupState`.
