# Selection control telemetry walkthrough

Enterprise dashboards can now treat RusticUI selection controls as rich telemetry
producers. Every checkbox **and switch** adapter shares the same render
instrumentation: the adapter enters `instrument_render`, emits a
`TelemetryContext` describing the component, analytics, automation identifiers,
and descriptor snapshot, and only then renders the DOM
node.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1012】【F:crates/rustic-ui-material/src/checkbox.rs†L1126-L1228】【F:crates/rustic-ui-material/src/switch.rs†L1031-L1150】【F:crates/rustic-ui-material/src/switch.rs†L1325-L1420】【F:crates/rustic-ui-material/src/telemetry.rs†L22-L78】【F:crates/rustic-ui-material/src/telemetry.rs†L132-L189】

The sections below demonstrate how to seed shared hooks, attach adapter-specific
telemetry delegates, and decode the resulting payloads across frameworks.

## Build shared `TelemetryHooks`

Start by constructing a reusable analytics helper that stamps the automation and
analytics identifiers, subscribes to render success/error callbacks, and pushes
those contexts into an observability sink.

```rust
use rustic_ui_material::{TelemetryContext, TelemetryHooks};
use std::sync::Arc;

fn analytics_hooks(channel: &'static str) -> TelemetryHooks {
    let mut hooks = TelemetryHooks::default();
    hooks.analytics_id = Some(format!("selection.{channel}"));
    hooks.automation_id = Some(format!("{channel}-checkbox"));
    hooks.on_render = Some(Arc::new(|ctx: TelemetryContext| {
        tracing::info!(
            target: "selection.controls.telemetry",
            component = ctx.component,
            analytics = ctx.analytics_id.as_deref().unwrap_or("n/a"),
            automation = ctx.automation_id.as_deref().unwrap_or("n/a"),
            descriptor_label = ctx
                .descriptor
                .as_ref()
                .map(|meta| meta.label.as_str())
                .unwrap_or("n/a"),
            descriptor_attributes = ?ctx
                .descriptor
                .as_ref()
                .map(|meta| meta.attributes.clone())
        );
    }));
    hooks.on_error = Some(Arc::new(|ctx: TelemetryContext, err| {
        tracing::error!(
            target: "selection.controls.telemetry",
            component = ctx.component,
            message = %err.message
        );
    }));
    hooks
}
```

The helper mirrors the shape of `TelemetryHooks`, which includes optional
analytics/automation IDs, span overrides, and render callbacks that fire for all
adapters.【F:crates/rustic-ui-material/src/telemetry.rs†L132-L189】

## Yew example: register a telemetry delegate

Yew adapters accept idiomatic `Callback<T>` handlers and forward
`CheckboxTelemetryEvent` and `SwitchTelemetryEvent` payloads to the telemetry
delegate **before** invoking user callbacks, guaranteeing deterministic
analytics ordering.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1120】【F:crates/rustic-ui-material/src/switch.rs†L883-L1042】【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L17-L74】【F:crates/rustic-ui-material/tests/switch_adapters.rs†L1-L38】

```rust
use rustic_ui_headless::{checkbox::CheckboxState, switch::SwitchState};
use rustic_ui_material::{
    checkbox::{
        CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
        yew::{YewCheckbox, YewCheckboxProps},
    },
    switch::{
        SwitchProps, SwitchTelemetryEvent,
        yew::{YewSwitch, YewSwitchProps},
    },
};
use yew::prelude::*;

#[function_component(MarketingOptIn)]
fn marketing_opt_in() -> Html {
    let state = CheckboxState::uncontrolled(false, false);
    let telemetry_log = use_state(|| Vec::<String>::new());

    let checkbox_delegate = {
        let telemetry_log = telemetry_log.clone();
        Callback::from(move |event: CheckboxTelemetryEvent| {
            telemetry_log.set({
                let mut next = (*telemetry_log).clone();
                next.push(format!("checkbox::{event:?}"));
                next
            });
        })
    };

    let switch_delegate = {
        let telemetry_log = telemetry_log.clone();
        Callback::from(move |event: SwitchTelemetryEvent| {
            telemetry_log.set({
                let mut next = (*telemetry_log).clone();
                next.push(format!("switch::{event:?}"));
                next
            });
        })
    };

    html! {
        <>
            <YewCheckbox
                checkbox={CheckboxProps {
                    label: "Email updates".into(),
                    telemetry: analytics_hooks("marketing.opt_in"),
                }}
                state={state.clone()}
                telemetry_delegate={Some(checkbox_delegate.clone())}
                on_change={Some(Callback::from(|event: CheckboxChangeEvent| {
                    metrics::increment!("marketing.opt_in.change", "state" => format!("{:?}", event.next));
                }))}
                on_focus={None}
                on_blur={None}
                on_key={None}
            />
            <YewSwitch
                switch={SwitchProps {
                    label: "Push alerts".into(),
                    telemetry: analytics_hooks("marketing.opt_in.switch"),
                }}
                state={SwitchState::uncontrolled(false, false)}
                telemetry_delegate={Some(switch_delegate)}
                on_change={None}
                on_focus={None}
                on_blur={None}
                on_key={None}
            />
        </>
    }
}
```

The component records every telemetry payload, and the change handler can still
perform local side effects after analytics capture, matching the adapter’s
internal ordering.

## Signal-based adapters (Leptos, Dioxus, Sycamore)

Leptos, Dioxus, and Sycamore share the same lifecycle: each closure emits the
telemetry payload, then the consumer callback, and finally mutates the headless
state machine. Pass an `Rc<dyn Fn(CheckboxTelemetryEvent)>` or
`Rc<dyn Fn(SwitchTelemetryEvent)>` delegate to receive that
stream.【F:crates/rustic-ui-material/src/checkbox.rs†L1522-L1703】【F:crates/rustic-ui-material/src/checkbox.rs†L1200-L1399】【F:crates/rustic-ui-material/src/switch.rs†L1494-L1666】

The radio adapters for Dioxus **and** Sycamore now mirror this choreography. A
shared closure factory constructs per-option runners that emit analytics before
invoking `RadioGroupState::select`, `RadioGroupState::focus`, `RadioGroupState::blur`,
or `RadioGroupState::on_key`. Optional `Rc` callbacks (`on_change`, `on_focus`,
`on_blur`, `on_key`) and telemetry delegates are captured once per option index so
enterprise automation observes deterministic analytics → callback → state
ordering across renderers.【F:crates/rustic-ui-material/src/radio.rs†L2633-L3436】【F:crates/rustic-ui-material/src/radio.rs†L3827-L4239】

```rust
use dioxus::prelude::*;
use std::rc::Rc;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::radio::{self, RadioGroupProps, RadioTelemetryEvent, RadioChangeEvent};

pub fn payment_methods(cx: Scope) -> Element {
    let state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(2),
    );
    let telemetry: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        println!("telemetry::{:?}", event);
    });
    let on_change: Rc<dyn Fn(RadioChangeEvent)> = Rc::new(|event| {
        println!("radio-change next={}", event.next);
    });

    cx.render(rsx! {
        radio::dioxus::DioxusRadioGroup {
            group: RadioGroupProps::from_state(&state),
            state: state.clone(),
            on_change: Some(on_change),
            telemetry_delegate: Some(telemetry),
        }
    })
}
```

```rust
use std::rc::Rc;
use sycamore::prelude::*;
use rustic_ui_headless::radio::{RadioGroupState, RadioOrientation};
use rustic_ui_material::radio::{self, RadioGroupProps, RadioTelemetryEvent, RadioChangeEvent};

#[component]
fn PaymentMethods<G: Html>(cx: Scope) -> View<G> {
    let state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(2),
    );
    let telemetry: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        println!("telemetry::{:?}", event);
    });
    let on_change: Rc<dyn Fn(RadioChangeEvent)> = Rc::new(|event| {
        println!("change::next={}", event.next);
    });

    let mut props = radio::sycamore::SycamoreRadioGroupProps::new(
        RadioGroupProps::from_state(&state),
        state.clone(),
    );
    props.on_change = Some(on_change);
    props.telemetry_delegate = Some(telemetry);

    view! { cx, radio::sycamore::SycamoreRadioGroup(props) }
}
```

```rust
use std::rc::Rc;
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{
    checkbox::{
        CheckboxProps, CheckboxTelemetryEvent,
        leptos::{LeptosCheckbox, LeptosCheckboxProps},
    },
    switch::{
        SwitchProps, SwitchTelemetryEvent,
        leptos::{LeptosSwitch, LeptosSwitchProps},
    },
    radio::{
        RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
        leptos::{LeptosRadioGroup, LeptosRadioGroupProps},
    },
};

fn leptos_checkbox_view() -> impl leptos::IntoView {
    let checkbox_delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = Rc::new(|event| {
        match event {
            CheckboxTelemetryEvent::Change(change) => {
                analytics::record("selection.change", change.analytics_id.clone());
            }
            CheckboxTelemetryEvent::Focus(focus) => analytics::record("selection.focus", focus.analytics_id.clone()),
            CheckboxTelemetryEvent::Blur(blur) => analytics::record("selection.blur", blur.analytics_id.clone()),
            CheckboxTelemetryEvent::Key(key) => analytics::record("selection.key", Some(format!("{:?}", key.key))),
        }
    });

    let switch_delegate: Rc<dyn Fn(SwitchTelemetryEvent)> = Rc::new(|event| {
        match event {
            SwitchTelemetryEvent::Change(change) => {
                analytics::record("selection.switch.change", change.analytics_id.clone());
            }
            SwitchTelemetryEvent::Focus(focus) => {
                analytics::record("selection.switch.focus", focus.analytics_id.clone())
            }
            SwitchTelemetryEvent::Blur(blur) => {
                analytics::record("selection.switch.blur", blur.analytics_id.clone())
            }
            SwitchTelemetryEvent::Key(key) => {
                analytics::record("selection.switch.key", Some(format!("{:?}", key.key)))
            }
        }
    });

    let checkbox = LeptosCheckbox(LeptosCheckboxProps {
        checkbox: CheckboxProps {
            label: "SMS updates".into(),
            telemetry: analytics_hooks("marketing.opt_in"),
        },
        state: CheckboxState::uncontrolled(false, false),
        on_change: None,
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(checkbox_delegate),
    });

    let switch = LeptosSwitch(LeptosSwitchProps {
        switch: SwitchProps {
            label: "Geo-fencing".into(),
            telemetry: analytics_hooks("marketing.opt_in"),
        },
        state: SwitchState::uncontrolled(false, false),
        on_change: None,
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(switch_delegate),
    });

    let radio_delegate: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| {
        analytics::record("selection.radio.telemetry", Some(format!("{:?}", event)));
    });

    let radio_change: Rc<dyn Fn(RadioChangeEvent)> = Rc::new(|event| {
        analytics::record("selection.radio.change", Some(event.next.to_string()));
    });

    let radio_state = RadioGroupState::uncontrolled(
        vec!["Visa".into(), "Mastercard".into(), "Amex".into()],
        false,
        RadioOrientation::Vertical,
        Some(1),
    );

    let mut radio_props = LeptosRadioGroupProps::new(
        RadioGroupProps::from_state(&radio_state),
        radio_state.clone(),
    );
    radio_props.on_change = Some(radio_change);
    radio_props.telemetry_delegate = Some(radio_delegate);

    let radio_group = LeptosRadioGroup(radio_props);

    (checkbox, switch, radio_group)
}
```

> **Automation note:** Leptos adapters now forward every themed attribute that
> reaches the descriptors (including inline `style` hooks produced by
> automation rollouts). Teams can rely on design tokens evolving centrally
> without revisiting component glue code each time new data attributes appear.

This pattern applies unchanged to `DioxusCheckboxProps`, `DioxusSwitchProps`,
`SycamoreCheckboxProps`, and `SycamoreSwitchProps`, which all consume
`Rc<dyn Fn(_TelemetryEvent)>` delegates with identical ordering
guarantees.【F:crates/rustic-ui-material/src/checkbox.rs†L1333-L1463】【F:crates/rustic-ui-material/src/checkbox.rs†L1548-L1662】【F:crates/rustic-ui-material/src/switch.rs†L1152-L1323】

## React adapter integration

React consumers receive a JavaScript object describing each telemetry payload.
`kind` identifies the event (`"change"`, `"focus"`, `"blur"`, or `"key"`), and
subsequent fields mirror the Rust structs. Register a single delegate to forward
those events into your analytics provider for both checkboxes and
switches.【F:crates/rustic-ui-material/src/checkbox.rs†L600-L831】【F:crates/rustic-ui-material/src/switch.rs†L697-L882】

```tsx
import { ReactCheckbox } from 'rustic-ui-material/checkbox';
import { ReactSwitch } from 'rustic-ui-material/switch';
import { ReactRadioGroup } from 'rustic-ui-material/radio';

const telemetryDelegate = (event: any) => {
  switch (event.kind) {
    case 'change':
      analytics.track('selection.change', {
        previous: event.previous,
        next: event.next,
        analyticsId: event.analyticsId,
        automationId: event.automationId,
      });
      break;
    case 'focus':
      analytics.track('selection.focus', event);
      break;
    case 'blur':
      analytics.track('selection.blur', event);
      break;
    case 'key':
      analytics.track('selection.key', { key: event.key, next: event.next });
      break;
  }
};

<>
  <ReactCheckbox
    checkbox={checkboxPropsFromWasm}
    state={checkboxStateFromWasm}
    telemetry_delegate={telemetryDelegate}
  />
  <ReactSwitch
    switch={switchPropsFromWasm}
    state={switchStateFromWasm}
    telemetry_delegate={telemetryDelegate}
  />
  <ReactRadioGroup
    group={radioGroupPropsFromWasm}
    state={radioGroupStateFromWasm}
    on_change={handleRadioChange}
    on_focus={handleRadioFocus}
    on_blur={handleRadioBlur}
    on_key_down={handleRadioKeyDown}
    telemetry_delegate={telemetryDelegate}
  />
</>;
```

The delegate signature matches the `Function` stored in
`ReactCheckboxProps::telemetry_delegate`, `ReactSwitchProps::telemetry_delegate`,
and `ReactRadioGroupProps::telemetry_delegate`, and events are emitted before any
user handlers run.【F:crates/rustic-ui-material/src/checkbox.rs†L600-L831】【F:crates/rustic-ui-material/src/switch.rs†L697-L882】【F:crates/rustic-ui-material/src/radio.rs†L480-L703】

Radio groups additionally sequence telemetry in the order `analytics → focus/
blur → change → commit` so analytics pipelines see the interaction intent before
the shared headless state mutates. The React adapter exposes explicit
`on_change`, `on_focus`, `on_blur`, and `on_key_down` callbacks that run *after*
telemetry and state updates, guaranteeing consumer side effects observe a fully
committed selection snapshot.【F:crates/rustic-ui-material/src/radio.rs†L480-L703】

`checkboxPropsFromWasm` and `checkboxStateFromWasm` represent the `CheckboxProps`
and `CheckboxState` values exported by the wasm bundle; the React adapter expects
those Rust structs because it shares the same headless state machine as the
other frameworks.【F:crates/rustic-ui-material/src/checkbox.rs†L600-L720】

## Event timeline

All adapters follow the same deterministic order, verified by the shared test
suite:

1. Pointer toggles emit a `Change` payload and then update the headless state.
2. Focus transitions emit a `Focus` payload on gain and a `Blur` payload on loss
   before mutating state.
3. Keyboard interactions emit a `Key` payload **and** an immediately-following
   `Change` payload so analytics pipelines can correlate the physical key with
   the resulting checked state.【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L210-L299】

Use these guarantees to stitch telemetry into enterprise monitoring stacks
without writing adapter-specific plumbing.
