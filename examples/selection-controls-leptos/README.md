# Selection Controls with Leptos

Typed Leptos components expose the same telemetry lifecycle as the Yew/Dioxus
adapters while remaining hydration safe – no more unchecked HTML fragments.

```rust
use leptos::*;
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{TelemetryContext, TelemetryHooks};
use rustic_ui_material::checkbox::{
    CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
    leptos::{LeptosCheckbox, LeptosCheckboxProps},
};
use rustic_ui_material::radio::{
    RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
    leptos::{LeptosRadioGroup, LeptosRadioGroupProps},
};
use rustic_ui_material::switch::{
    SwitchChangeEvent, SwitchProps, SwitchTelemetryEvent,
    leptos::{LeptosSwitch, LeptosSwitchProps},
};
use std::{rc::Rc, sync::Arc};

fn telemetry(channel: &'static str) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection-controls.leptos.{channel}"));
    hooks.automation_id = Some(format!("automation.selection-controls.{channel}"));
    let channel_label = format!("selection_controls::{channel}");
    hooks.on_render = Some(Arc::new(move |context: TelemetryContext| {
        leptos::logging::log!(
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
pub fn SelectionControls() -> impl IntoView {
    let checkbox_state = CheckboxState::uncontrolled(false, true);
    let switch_state = SwitchState::uncontrolled(false, false);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Visa".into(), "Mastercard".into(), "Amex".into()],
        false,
        RadioOrientation::Vertical,
        Some(1),
    );

    let checkbox_props = CheckboxProps {
        label: "Save card".into(),
        telemetry: telemetry("checkbox"),
    };
    let switch_props = SwitchProps {
        label: "Enable auto-pay".into(),
        telemetry: telemetry("switch"),
    };
    let radio_props = RadioGroupProps::from_state(&radio_state)
        .with_telemetry(telemetry("radio"));

    let checkbox_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = Rc::new(|event| {
        leptos::logging::log!("checkbox telemetry: {:?}", event);
    });
    let switch_delegate: Rc<dyn Fn(SwitchTelemetryEvent)> = Rc::new(|event| {
        leptos::logging::log!("switch telemetry: {:?}", event);
    });
    let radio_delegate: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        leptos::logging::log!("radio telemetry: {:?}", event);
    });

    let checkbox_component = LeptosCheckboxProps {
        checkbox: checkbox_props.clone(),
        state: checkbox_state.clone(),
        on_change: Some(Rc::new(|event: CheckboxChangeEvent| {
            leptos::logging::log!(
                "checkbox::change next={} disabled={}",
                event.next,
                event.disabled,
            );
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(checkbox_delegate.clone()),
    };
    let switch_component = LeptosSwitchProps {
        switch: switch_props.clone(),
        state: switch_state.clone(),
        on_change: Some(Rc::new(|event: SwitchChangeEvent| {
            leptos::logging::log!(
                "switch::change next={} disabled={}",
                event.next,
                event.disabled,
            );
        })),
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(switch_delegate.clone()),
    };
    let radio_component = {
        let mut props = LeptosRadioGroupProps::new(radio_props.clone(), radio_state.clone());
        props.telemetry = telemetry("radio.component");
        props.on_change = Some(Rc::new(|event: RadioChangeEvent| {
            leptos::logging::log!(
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

    view! {
        <div>
            {LeptosCheckbox(checkbox_component)}
            {LeptosSwitch(switch_component)}
            {LeptosRadioGroup(radio_component)}
        </div>
    }
}
```

> **Compile-time note:** Add `rustic-ui-material` with the `leptos` feature and
> enable the matching `forms` feature on `rustic-ui-headless` before running
> `cargo leptos build` or `cargo check` inside an example crate.
