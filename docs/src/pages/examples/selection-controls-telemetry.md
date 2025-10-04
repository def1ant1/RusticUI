# Selection control telemetry walkthrough

Enterprise dashboards can now treat RusticUI selection controls as rich telemetry
producers. Every checkbox, switch, **and radio** adapter shares the same render
instrumentation: the adapter enters `instrument_render`, emits a
`TelemetryContext` describing the component, analytics, automation identifiers,
and descriptor snapshot, and only then renders the DOM node. The walkthrough
mirrors the inline comments that live in each runnable crate and central smoke
script so developers can cross-reference docs with the source of truth.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1012】【F:crates/rustic-ui-material/src/checkbox.rs†L1126-L1228】【F:crates/rustic-ui-material/src/switch.rs†L1031-L1150】【F:crates/rustic-ui-material/src/switch.rs†L1325-L1420】【F:crates/rustic-ui-material/src/radio.rs†L1885-L1930】【F:crates/rustic-ui-material/src/telemetry.rs†L22-L78】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】

The sections below demonstrate how to seed shared hooks, attach adapter-specific
telemetry delegates, and decode the resulting payloads across frameworks.

## Build shared `TelemetryHooks`

Start by constructing a reusable analytics helper that stamps the automation and
analytics identifiers, subscribes to render success/error callbacks, and pushes
those contexts into an observability sink. This mirrors the inline reminder in
`selection-controls-smoke.sh` that the automation identifiers are centrally
managed—when the helper changes, update the smoke script so the `--list-automation`
output remains authoritative for QA tooling.【F:examples/scripts/selection-controls-smoke.sh†L32-L63】

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

## Attach telemetry to radio groups

Radio controls reuse the same hooks as checkboxes and switches. Every
`RadioGroupProps` exposes a `telemetry: TelemetryHooks` field, and the adapter
props (`YewRadioGroupProps`, `ReactRadioGroupProps`, `LeptosRadioGroupProps`,
`DioxusRadioGroupProps`, `SycamoreRadioGroupProps`) merge the hooks supplied at
the adapter level with the group-level hooks. This guarantees that analytics and
automation identifiers emitted while rendering each framework stay aligned with
the descriptors shipped by the headless state machine—even when teams inject
framework overrides.【F:crates/rustic-ui-material/src/radio.rs†L120-L135】【F:crates/rustic-ui-material/src/radio.rs†L1885-L1930】【F:crates/rustic-ui-material/src/radio.rs†L2316-L2355】【F:crates/rustic-ui-material/src/radio.rs†L3310-L3344】【F:crates/rustic-ui-material/src/radio.rs†L382-408】

The descriptors themselves stay authoritative for SSR and hydration. Every
adapter renders attributes and inline styles provided by the descriptor snapshot,
and regression tests assert that server output retains ARIA metadata, themed
attributes, and custom data hooks so the client runtime can hydrate safely with
no manual DOM reconciliation.【F:crates/rustic-ui-material/src/radio.rs†L270-L315】【F:crates/rustic-ui-material/src/radio.rs†L562-L579】【F:crates/rustic-ui-material/src/radio.rs†L2799-L2856】【F:crates/rustic-ui-material/src/radio.rs†L3823-L3896】【F:crates/rustic-ui-material/src/radio.rs†L4561-L4597】

### Radio telemetry payloads

Radio events include analytics context, focus transitions, selection intents,
commit snapshots, and key metadata. Each payload mirrors the descriptor’s
automation IDs and label so monitoring pipelines can correlate telemetry with
the rendered DOM.

| Variant | Purpose | Key fields |
| --- | --- | --- |
| `Analytics` | Snapshot of the option prior to any interaction. | `index`, `selected`, `disabled`, `analytics_id`, `automation_id`, `label` |
| `Focus` / `Blur` | Visibility transitions for an option. | `index`, `focused`, `disabled`, `analytics_id`, `automation_id`, `label` |
| `Change` | Selection intent emitted before state mutates. | `previous`, `next`, `disabled`, `analytics_id`, `automation_id`, `label` |
| `Commit` | Post-mutation selection snapshot, including controlled state. | `selected`, `controlled`, `analytics_id`, `automation_id`, `label` |
| `Key` | Keyboard interaction metadata. | `key`, `previous`, `next`, `disabled`, `analytics_id`, `automation_id`, `label` |

All adapters emit events in the deterministic order `Analytics → Key →
Focus/Blur → Change → Commit → callback`, with pointer interactions simply
skipping the keyboard payload. Tests across every framework validate that
telemetry fires before user callbacks and that commits capture the final
selection snapshot—even in controlled radio groups.【F:crates/rustic-ui-material/src/radio.rs†L118-L213】【F:crates/rustic-ui-material/src/radio.rs†L2620-L2798】【F:crates/rustic-ui-material/src/radio.rs†L3190-L3286】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L272】

## Automation harness alignment

- Run `cargo xtask selection-controls` before publishing updates. The xtask
  compiles host + wasm targets, executes the Playwright harness, and streams the
  canonical automation IDs so CI and local workflows remain identical.【F:crates/xtask/src/main.rs†L163-L2381】
- Call `examples/scripts/selection-controls-smoke.sh --list-automation --format json`
  when onboarding QA teams; the helper prints the exact identifiers emitted by
  every framework and is heavily annotated to explain why the list exists.【F:examples/scripts/selection-controls-smoke.sh†L1-L63】
- Inside React and Yew packages, `just automation-smoke` shells out to the same
  helper so JavaScript and Rust pipelines reuse the documented orchestration
  graph.【F:examples/selection-controls-react/README.md†L33-L75】【F:examples/selection-controls-yew/README.md†L19-L60】

## Yew example: register a telemetry delegate

Yew adapters accept idiomatic `Callback<T>` handlers and forward
`CheckboxTelemetryEvent`, `SwitchTelemetryEvent`, and `RadioTelemetryEvent`
payloads to the telemetry delegate **before** invoking user callbacks,
guaranteeing deterministic analytics ordering.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1120】【F:crates/rustic-ui-material/src/switch.rs†L883-L1042】【F:crates/rustic-ui-material/src/radio.rs†L1538-L1708】【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L17-L74】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L260】

```rust
use rustic_ui_headless::{
    checkbox::CheckboxState,
    radio::{RadioGroupState, RadioOrientation},
    switch::SwitchState,
};
use rustic_ui_material::{
    checkbox::{
        CheckboxChangeEvent, CheckboxProps, CheckboxTelemetryEvent,
        yew::{YewCheckbox, YewCheckboxProps},
    },
    radio::{
        self, RadioChangeEvent, RadioGroupProps, RadioTelemetryEvent,
        yew::{YewRadioGroup, YewRadioGroupProps},
    },
    switch::{
        SwitchProps, SwitchTelemetryEvent,
        yew::{YewSwitch, YewSwitchProps},
    },
    TelemetryHooks,
};
use yew::prelude::*;

#[function_component(MarketingOptIn)]
fn marketing_opt_in() -> Html {
    let state = CheckboxState::uncontrolled(false, false);
    let radio_state = RadioGroupState::uncontrolled(
        vec!["Cash".into(), "Card".into(), "Invoice".into()],
        false,
        RadioOrientation::Horizontal,
        Some(0),
    );
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

    let radio_delegate = {
        let telemetry_log = telemetry_log.clone();
        Callback::from(move |event: RadioTelemetryEvent| {
            if let RadioTelemetryEvent::Commit(commit) = &event {
                metrics::gauge!(
                    "marketing.opt_in.radio.selected",
                    commit.selected.unwrap_or_default() as f64,
                    "controlled" => commit.controlled.to_string(),
                );
            }

            telemetry_log.set({
                let mut next = (*telemetry_log).clone();
                next.push(format!("radio::{event:?}"));
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
            <YewRadioGroup
                group={RadioGroupProps {
                    option_labels: radio_state.options().to_vec(),
                    telemetry: analytics_hooks("marketing.opt_in.radio"),
                    additional_group_attributes: vec![],
                    additional_option_attributes: vec![],
                }}
                state={radio_state.clone()}
                telemetry={TelemetryHooks::default()}
                telemetry_delegate={Some(radio_delegate.clone())}
                on_change={Some(Callback::from(|event: RadioChangeEvent| {
                    tracing::info!(target: "marketing.radio", ?event, "radio change");
                }))}
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
    let telemetry: Rc<dyn Fn(RadioTelemetryEvent)> = Rc::new(|event| match event {
        RadioTelemetryEvent::Commit(commit) => {
            println!("radio commit => {:?}", commit.selected);
        }
        other => println!("telemetry::{:?}", other),
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

The Dioxus renderer now mirrors the other adapters by automatically
propagating every themed attribute (`class`, `style`, `data-*`, analytics IDs)
emitted by the headless descriptor. When design tokens roll out new keys the
adapter spreads them across the `<div>` and option `<span>` nodes without any
manual wiring, keeping enterprise theming layers and analytics probes intact
during upgrades.

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
        if let RadioTelemetryEvent::Commit(commit) = &event {
            println!("commit => {:?}", commit.selected);
        }
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
`kind` identifies the event (`"analytics"`, `"change"`, `"focus"`, `"blur"`,
`"key"`, or `"commit"` for radios), and subsequent fields mirror the Rust
structs. Register a single delegate to forward those events into your analytics
provider for checkboxes, switches, and radios.【F:crates/rustic-ui-material/src/checkbox.rs†L600-L831】【F:crates/rustic-ui-material/src/switch.rs†L697-L882】【F:crates/rustic-ui-material/src/radio.rs†L480-L703】

> **Enterprise focus management tip:** Even when a radio group is fully
> controlled from React state, the adapter still calls
> `RadioGroupState::on_key` internally so the roving `focus_visible_index()`
> advances in lockstep with user intent. Telemetry emitted from controlled
> keyboard flows therefore reports the upcoming option (via the `key.next` and
> `change.next` fields) while the final `commit.selected` remains anchored to
> your externally managed selection. This keeps analytics and automation logs
> consistent without forcing additional glue code in product surfaces.

```tsx
import {
  ReactCheckbox,
  CheckboxTelemetryEvent,
} from 'rustic-ui-material/checkbox';
import {
  ReactSwitch,
  SwitchTelemetryEvent,
} from 'rustic-ui-material/switch';
import {
  ReactRadioGroup,
  RadioTelemetryEvent,
  RadioCommitEvent,
} from 'rustic-ui-material/radio';

const telemetryDelegate = (
  event: CheckboxTelemetryEvent | SwitchTelemetryEvent | RadioTelemetryEvent,
) => {
  switch (event.kind) {
    case 'analytics':
      analytics.track('selection.analytics', event);
      break;
    case 'change':
      analytics.track('selection.change', {
        previous: event.previous,
        next: event.next,
        analyticsId: event.analyticsId,
        automationId: event.automationId,
      });
      break;
    case 'commit':
      analytics.track('selection.commit', event as RadioCommitEvent);
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

const onRadioCommit = (event: RadioCommitEvent) => {
  posthog.capture('radio.commit', event);
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
    on_commit={onRadioCommit}
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
`on_change`, `on_focus`, `on_blur`, `on_key_down`, and `on_commit` callbacks that
run *after* telemetry and state updates, guaranteeing consumer side effects
observe a fully committed selection snapshot and the descriptor-authored
attributes already attached to the DOM.【F:crates/rustic-ui-material/src/radio.rs†L480-L703】【F:crates/rustic-ui-material/src/radio.rs†L270-L315】

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
4. Radio groups emit a `Commit` payload after the shared runner resolves the
   final selection, ensuring controlled groups surface the pre-hydration state
   alongside the committed snapshot.【F:crates/rustic-ui-material/src/radio.rs†L2633-L2709】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L260】

Use these guarantees to stitch telemetry into enterprise monitoring stacks
without writing adapter-specific plumbing.
