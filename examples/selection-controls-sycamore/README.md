# Selection Controls with Sycamore

Typed Sycamore components let us mount interactive selection controls without
`dangerously_set_inner_html` while still registering deterministic telemetry
hooks for analytics and automation.

```rust
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};
use rustic_ui_material::checkbox::{
    CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
    sycamore::{SycamoreCheckbox, SycamoreCheckboxProps},
};
use rustic_ui_material::radio::{
    RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
    sycamore::{SycamoreRadioGroup, SycamoreRadioGroupProps},
};
use rustic_ui_material::switch::{
    SwitchChangeEvent, SwitchProps, SwitchTelemetryEvent,
    sycamore::{SycamoreSwitch, SycamoreSwitchProps},
};
use std::{rc::Rc, sync::Arc};
use sycamore::prelude::*;

fn telemetry(channel: &'static str) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection-controls.sycamore.{channel}"));
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

    let checkbox_props = CheckboxProps {
        label: "Light theme".into(),
        telemetry: telemetry("checkbox"),
    };
    let switch_props = SwitchProps {
        label: "Enable system overrides".into(),
        telemetry: telemetry("switch"),
    };
    let radio_props = RadioGroupProps::from_state(&radio_state)
        .with_telemetry(telemetry("radio"));

    let checkbox_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = Rc::new(|event| {
        println!("checkbox telemetry::{event:?}");
    });
    let switch_delegate: Rc<dyn Fn(SwitchTelemetryEvent)> = Rc::new(|event| {
        println!("switch telemetry::{event:?}");
    });
    let radio_delegate: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        println!("radio telemetry::{event:?}");
    });

    let checkbox_component = SycamoreCheckboxProps {
        checkbox: checkbox_props.clone(),
        state: checkbox_state.clone(),
        on_change: Some(Rc::new(|event: CheckboxChangeEvent| {
            println!("checkbox::change next={} disabled={}", event.next, event.disabled);
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(checkbox_delegate.clone()),
    };
    let switch_component = SycamoreSwitchProps {
        switch: switch_props.clone(),
        state: switch_state.clone(),
        on_change: Some(Rc::new(|event: SwitchChangeEvent| {
            println!("switch::change next={} disabled={}", event.next, event.disabled);
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(switch_delegate.clone()),
    };
    let radio_component = {
        let mut props = SycamoreRadioGroupProps::new(radio_props.clone(), radio_state.clone());
        props.telemetry = telemetry("radio.component");
        props.on_change = Some(Rc::new(|event: RadioChangeEvent| {
            println!(
                "radio::change previous={:?} next={} label={}",
                event.previous,
                event.next,
                event.label,
            );
        }));
        props.on_focus = None;
        props.on_blur = None;
        props.on_key = None;
        props.telemetry_delegate = Some(radio_delegate.clone());
        props
    };

    view! { cx,
        SycamoreCheckbox(checkbox_component)
        SycamoreSwitch(switch_component)
        SycamoreRadioGroup(radio_component)
    }
}
```

> **Compile-time note:** Pull in `rustic-ui-material` with the `sycamore`
> feature and the matching `forms` feature from `rustic-ui-headless` before
> running `cargo check` within a Sycamore example crate.
