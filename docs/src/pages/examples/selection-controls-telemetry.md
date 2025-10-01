# Selection control telemetry walkthrough

Enterprise dashboards can now treat RusticUI selection controls as rich telemetry
producers. Every checkbox adapter shares the same render instrumentation: the
adapter enters `instrument_render`, emits a `TelemetryContext` describing the
component, analytics, automation identifiers, and descriptor snapshot, and only
then renders the DOM node.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1012】【F:crates/rustic-ui-material/src/checkbox.rs†L1126-L1228】【F:crates/rustic-ui-material/src/telemetry.rs†L22-L78】【F:crates/rustic-ui-material/src/telemetry.rs†L132-L189】

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
`CheckboxTelemetryEvent` payloads to the telemetry delegate **before** invoking
user callbacks, guaranteeing deterministic analytics ordering.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1120】【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L17-L74】【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L210-L299】

```rust
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_material::checkbox::{
    CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
    yew::{YewCheckbox, YewCheckboxProps},
};
use yew::prelude::*;

#[function_component(MarketingOptIn)]
fn marketing_opt_in() -> Html {
    let state = CheckboxState::uncontrolled(false, false);
    let telemetry_log = use_state(|| Vec::new());

    let delegate = {
        let telemetry_log = telemetry_log.clone();
        Callback::from(move |event: CheckboxTelemetryEvent| {
            telemetry_log.set({
                let mut next = (*telemetry_log).clone();
                next.push(event);
                next
            });
        })
    };

    html! {
        <YewCheckbox
            checkbox={CheckboxProps {
                label: "Email updates".into(),
                telemetry: analytics_hooks("marketing.opt_in"),
            }}
            state={state}
            telemetry_delegate={Some(delegate)}
            on_change={Some(Callback::from(|event: CheckboxChangeEvent| {
                metrics::increment!("marketing.opt_in.change", "state" => format!("{:?}", event.next));
            }))}
            on_focus={None}
            on_blur={None}
            on_key={None}
        />
    }
}
```

The component records every telemetry payload, and the change handler can still
perform local side effects after analytics capture, matching the adapter’s
internal ordering.

## Signal-based adapters (Leptos, Dioxus, Sycamore)

Leptos, Dioxus, and Sycamore share the same lifecycle: each closure emits the
telemetry payload, then the consumer callback, and finally mutates the headless
state machine. Pass an `Rc<dyn Fn(CheckboxTelemetryEvent)>` delegate to receive
that stream.【F:crates/rustic-ui-material/src/checkbox.rs†L1126-L1228】【F:crates/rustic-ui-material/src/checkbox.rs†L1333-L1463】【F:crates/rustic-ui-material/src/checkbox.rs†L1548-L1662】

```rust
use std::rc::Rc;
use rustic_ui_headless::checkbox::CheckboxState;
use rustic_ui_material::checkbox::{
    CheckboxProps, CheckboxTelemetryEvent,
    leptos::{LeptosCheckbox, LeptosCheckboxProps},
};

fn leptos_checkbox_view() -> impl leptos::IntoView {
    let delegate: Rc<dyn Fn(CheckboxTelemetryEvent)> = Rc::new(|event| {
        match event {
            CheckboxTelemetryEvent::Change(change) => {
                analytics::record("selection.change", change.analytics_id.clone());
            }
            CheckboxTelemetryEvent::Focus(focus) => analytics::record("selection.focus", focus.analytics_id.clone()),
            CheckboxTelemetryEvent::Blur(blur) => analytics::record("selection.blur", blur.analytics_id.clone()),
            CheckboxTelemetryEvent::Key(key) => analytics::record("selection.key", Some(format!("{:?}", key.key))),
        }
    });

    LeptosCheckbox(LeptosCheckboxProps {
        checkbox: CheckboxProps {
            label: "SMS updates".into(),
            telemetry: analytics_hooks("marketing.opt_in"),
        },
        state: CheckboxState::uncontrolled(false, false),
        on_change: None,
        on_focus: None,
        on_blur: None,
        on_key: None,
        telemetry_delegate: Some(delegate),
    })
}
```

This pattern applies unchanged to `DioxusCheckboxProps` and
`SycamoreCheckboxProps`, which both consume `Rc<dyn Fn(CheckboxTelemetryEvent)>`
telemetry delegates.【F:crates/rustic-ui-material/src/checkbox.rs†L1333-L1463】【F:crates/rustic-ui-material/src/checkbox.rs†L1548-L1662】

## React adapter integration

React consumers receive a JavaScript object describing each telemetry payload.
`kind` identifies the event (`"change"`, `"focus"`, `"blur"`, or `"key"`), and
subsequent fields mirror the Rust structs. Register a single delegate to forward
those events into your analytics provider.

```tsx
import { ReactCheckbox } from 'rustic-ui-material/checkbox';

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

<ReactCheckbox
  checkbox={checkboxPropsFromWasm}
  state={checkboxStateFromWasm}
  telemetry_delegate={telemetryDelegate}
/>;
```

The delegate signature matches the `Function` stored in
`ReactCheckboxProps::telemetry_delegate`, and events are emitted before any user
handlers run.【F:crates/rustic-ui-material/src/checkbox.rs†L600-L831】

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
