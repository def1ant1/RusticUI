# Selection Controls with Dioxus

Use the typed Dioxus components so hydration stays deterministic, telemetry is
attached up-front, and automation suites can reuse the same patterns that power
the full design system.

```rust
use dioxus::prelude::*;
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};
use rustic_ui_material::checkbox::{
    CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
    dioxus::{DioxusCheckbox, DioxusCheckboxProps},
};
use rustic_ui_material::radio::{
    RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
    dioxus::{DioxusRadioGroup, DioxusRadioGroupProps},
};
use rustic_ui_material::switch::{
    SwitchChangeEvent, SwitchProps, SwitchTelemetryEvent,
    dioxus::{DioxusSwitch, DioxusSwitchProps},
};
use std::{rc::Rc, sync::Arc};

fn telemetry(channel: &'static str) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection-controls.dioxus.{channel}"));
    hooks.automation_id = Some(format!("automation.selection-controls.{channel}"));
    let channel_label = format!("selection_controls::{channel}");
    hooks.on_render = Some(Arc::new(move |context: TelemetryContext| {
        println!(
            "telemetry::render channel={} component={} analytics={:?} automation={:?}",
            channel_label,
            context.component,
            context.analytics_id,
            context.automation_id,
        );
    }));
    hooks
}

pub fn selection_controls(cx: Scope) -> Element {
    let checkbox_state = CheckboxState::uncontrolled(false, false);
    let switch_state = SwitchState::uncontrolled(false, true);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(2),
    );

    let checkbox_props = CheckboxProps {
        label: "Accept terms".into(),
        telemetry: telemetry("checkbox"),
    };
    let switch_props = SwitchProps {
        label: "Enable quick checkout".into(),
        telemetry: telemetry("switch"),
    };
    let radio_props = RadioGroupProps::from_state(&radio_state)
        .with_telemetry(telemetry("radio"));

    let checkbox_telemetry: Rc<dyn Fn(CheckboxTelemetryEvent)> = Rc::new(|event| {
        println!("checkbox telemetry::{event:?}");
    });
    let switch_telemetry: Rc<dyn Fn(SwitchTelemetryEvent)> = Rc::new(|event| {
        println!("switch telemetry::{event:?}");
    });
    let radio_telemetry: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        println!("radio telemetry::{event:?}");
    });

    let checkbox_component = DioxusCheckboxProps {
        checkbox: checkbox_props.clone(),
        state: checkbox_state.clone(),
        on_change: Some(Rc::new(|event: CheckboxChangeEvent| {
            println!("checkbox::change next={} disabled={}", event.next, event.disabled);
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(checkbox_telemetry.clone()),
    };
    let switch_component = DioxusSwitchProps {
        switch: switch_props.clone(),
        state: switch_state.clone(),
        on_change: Some(Rc::new(|event: SwitchChangeEvent| {
            println!("switch::change next={} disabled={}", event.next, event.disabled);
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(switch_telemetry.clone()),
    };
    let radio_component = DioxusRadioGroupProps {
        group: radio_props.clone(),
        state: radio_state.clone(),
        on_change: Some(Rc::new(|event: RadioChangeEvent| {
            println!(
                "radio::change previous={:?} next={} label={}",
                event.previous,
                event.next,
                event.label,
            );
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(radio_telemetry.clone()),
        telemetry: Some(telemetry("radio.component")),
    };

    cx.render(rsx! {
        DioxusCheckbox { ..checkbox_component }
        DioxusSwitch { ..switch_component }
        DioxusRadioGroup { ..radio_component }
    })
}
```

> **Compile-time note:** Enable the `dioxus` feature when pulling `rustic-ui-material`
> into an example crate: `cargo add rustic-ui-material --features dioxus` and
> `cargo add rustic-ui-headless --features forms` before running `cargo check`.
