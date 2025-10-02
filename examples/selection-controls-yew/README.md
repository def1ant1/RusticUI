# Selection Controls with Yew

Replace legacy render helpers with typed Yew components so hydration stays
predictable and every control registers deterministic telemetry hooks upfront.

```rust
use gloo_console::log;
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};
use rustic_ui_material::checkbox::{
    CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
    yew::{YewCheckbox, YewCheckboxProps},
};
use rustic_ui_material::radio::{
    RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
    yew::{YewRadioGroup, YewRadioGroupProps},
};
use rustic_ui_material::switch::{
    SwitchChangeEvent, SwitchProps, SwitchTelemetryEvent,
    yew::{YewSwitch, YewSwitchProps},
};
use std::sync::Arc;
use yew::prelude::*;

fn telemetry(channel: &'static str) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection-controls.yew.{channel}"));
    hooks.automation_id = Some(format!("automation.selection-controls.{channel}"));
    let channel_label = format!("selection_controls::{channel}");
    hooks.on_render = Some(Arc::new(move |context: TelemetryContext| {
        log!(format!(
            "telemetry::render channel={} component={} analytics={:?} automation={:?}",
            channel_label,
            context.component,
            context.analytics_id,
            context.automation_id,
        ));
    }));
    hooks
}

#[function_component(SelectionControls)]
pub fn selection_controls() -> Html {
    let checkbox_state = CheckboxState::uncontrolled(false, false);
    let switch_state = SwitchState::uncontrolled(false, true);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Email".into(), "SMS".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    );

    let checkbox_props = CheckboxProps {
        label: "Receive updates".into(),
        telemetry: telemetry("checkbox"),
    };
    let switch_props = SwitchProps {
        label: "Enable notifications".into(),
        telemetry: telemetry("switch"),
    };
    let radio_props = RadioGroupProps::from_state(&radio_state)
        .with_telemetry(telemetry("radio"));

    let checkbox_delegate = Callback::from(|event: CheckboxTelemetryEvent| {
        log!(format!("checkbox telemetry: {:?}", event));
    });
    let switch_delegate = Callback::from(|event: SwitchTelemetryEvent| {
        log!(format!("switch telemetry: {:?}", event));
    });
    let radio_delegate = Callback::from(|event: RadioTelemetryEvent| {
        log!(format!("radio telemetry: {:?}", event));
    });

    let checkbox_component = YewCheckboxProps {
        checkbox: checkbox_props.clone(),
        state: checkbox_state.clone(),
        on_change: Some(Callback::from(|event: CheckboxChangeEvent| {
            log!(format!(
                "checkbox::change next={} disabled={}",
                event.next,
                event.disabled,
            ));
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(checkbox_delegate.clone()),
    };
    let switch_component = YewSwitchProps {
        switch: switch_props.clone(),
        state: switch_state.clone(),
        on_change: Some(Callback::from(|event: SwitchChangeEvent| {
            log!(format!(
                "switch::change next={} disabled={}",
                event.next,
                event.disabled,
            ));
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(switch_delegate.clone()),
    };
    let radio_component = YewRadioGroupProps {
        group: radio_props.clone(),
        state: radio_state.clone(),
        telemetry: telemetry("radio.component"),
        on_change: Some(Callback::from(|event: RadioChangeEvent| {
            log!(format!(
                "radio::change previous={:?} next={} label={}",
                event.previous,
                event.next,
                event.label,
            ));
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(radio_delegate.clone()),
    };

    html! {
        <>
            <YewCheckbox ..checkbox_component />
            <YewSwitch ..switch_component />
            <YewRadioGroup ..radio_component />
        </>
    }
}
```

> **Compile-time note:** Add `rustic-ui-material` with the `yew` feature and the
> `forms` feature on `rustic-ui-headless`, then run `cargo check --target wasm32-unknown-unknown`
> inside the example crate to validate the snippet.
