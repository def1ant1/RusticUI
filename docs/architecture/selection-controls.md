# Selection Control Attribute Builders

> Status: Adopted for `rustic-ui-material` selection controls

## Motivation

Material adapters relied on ad-hoc helpers that eagerly produced HTML strings.
While simple for SSR, those helpers made it difficult for enterprise consumers to
plug in telemetry, centralized automation identifiers, or custom theming.  The
new attribute builders embrace strongly typed descriptors so that:

- **Server renderers** can stringify deterministic markup via `Display` or the
  `to_ssr_html` helpers.
- **Hydration adapters** can read the exact ARIA, data, and automation maps
  without reparsing HTML.
- **Platform teams** can register global hooks once to stamp analytics payloads
  onto every control instead of repeating the same wiring in each adapter.

## Builder Overview

The selection control module exposes three core builders:

| Builder | Purpose |
| --- | --- |
| `SelectionControlAttributes::builder` | Configures a single checkbox/switch label pair. |
| `RadioOptionAttributes::builder` | Describes an individual radio option within a group. |
| `RadioGroupAttributes::builder` | Orchestrates group-level metadata plus a list of option descriptors. |

Each builder yields an immutable descriptor containing:

- `classes`: A deduplicated list of CSS classes.
- `aria_attributes`: Deterministic ARIA metadata.
- `data_attributes`: `data-*` flags for analytics, QA, and focus management.
- `automation_ids`: A map that renders as `data-automation-*` attributes.
- `extra_attributes`: Remaining passthrough attributes such as `role` and
  `tabindex`.

Descriptors expose `to_ssr_html` and implement `Display` so that server renderers
can emit markup without handcrafting templates.  Hydration paths can call
`themed_attributes` to obtain framework-friendly `(String, String)` vectors.

## Extensibility Hooks

To minimise repetitive plumbing for enterprise telemetry, the module ships with
three one-time registration hooks:

- `register_selection_control_hook`
- `register_radio_option_hook`
- `register_radio_group_hook`

Each hook receives the corresponding builder before it is finalised.  Platform
integrators can use this to append global classes, analytics metadata, or
automation identifiers exactly once per process.  Because the hooks are powered
by `OnceLock`, the first registration wins; this protects against double
registration in multi-tenant hosting environments.  Consumers that need dynamic
changes should register a dispatcher that reads from their own configuration
store.

## Migration Guidelines for Adapter Authors

1. Replace calls to `render_toggle_html` and `render_radio_group_html` with the
   appropriate builder sequence.  For example:

   ```rust
   let descriptor = SelectionControlAttributes::builder(label, style)
       .aria("role", "switch")
       .data("state", state.as_str())
       .build();
   let html = descriptor.to_ssr_html();
   ```

2. For client adapters, prefer `descriptor.themed_attributes()` instead of
   deconstructing HTML strings.  This guarantees that theming and automation
   metadata stay consistent across SSR and hydration.

3. Register any analytics/theming hook once during application start-up.  Avoid
   per-render registration because the hook is stored in a `OnceLock`.

4. Remove bespoke automation attribute plumbing.  Use
   `builder.automation_id("qa", id)` to automatically surface
   `data-automation-qa` attributes in SSR and hydration outputs.

5. Lean on the inline documentation within `selection_control.rs` for the latest
   field-level behavior and extension points.  The doc comments include
   rationale for defaults so future migrations remain low-risk.

### Helper-driven render plans

Material checkboxes and switches now expose helper-driven render plans (for
example `CheckboxRenderPlan`) that pre-compute the descriptor, merge telemetry
defaults, and emit a `TelemetryContext`. Adapter implementations invoke the plan
once per render and then project its `themed_attributes` into React, Yew,
Leptos, Dioxus, or Sycamore without recomputing attribute maps. The same plan
also feeds SSR by calling `SelectionControlAttributes::to_ssr_html`, ensuring
hydration receives an identical snapshot.

Enterprise teams extending adapters should prefer tapping into these helpers
instead of cloning descriptor assembly logic. Doing so keeps analytics spans,
automation hooks, and SSR output consistent even as new telemetry sinks or
attributes are introduced centrally.

## Telemetry orchestration helpers

Selection controls now lean on two telemetry-centric helpers that eliminate
adapter-specific glue code:

- `SelectionControlTelemetry` wraps a configured `TelemetryHooks` instance and
  exposes builder-like methods (`with_analytics_id`, `with_automation_id`,
  `with_data_keys`, `enforce_defaults`) for centrally managed identifiers.
- `TelemetryHooks` acts as the enterprise integration surface, carrying
  analytics/focus/change/commit/error callbacks plus panic handling. Hooks are
  thread-safe so adapters can clone them into concurrent renderers without
  additional locking.

### Integration steps for adapters

1. Construct a `TelemetryHooks` value during application/bootstrap and register
   its delegates with your observability systems (tracing, logging, data lake,
   compliance archive). The hooks expose optional analytics and automation IDs
   that downstream descriptors inherit automatically.
2. Wrap the hooks with `SelectionControlTelemetry::new(hooks.clone())`. Override
   managed data keys or identifiers if your renderer needs bespoke attribute
   names (for example, to integrate with legacy DOM listeners).
3. Pass the helper into `SelectionControlDescriptor::from_headless`, the
   dedicated render plans, or the fluent attribute builders. The descriptor will
   merge ARIA/data/automation metadata, Material defaults, and telemetry context
   without requiring adapter-level mutation.
4. When rendering, call the descriptor’s `to_ssr_html` or feed
   `themed_attributes()` into your framework. After the render finishes, the
   telemetry helper ensures analytics, focus, state change, commit, and error
   callbacks fire in a deterministic order before consumer callbacks, keeping
   enterprise dashboards trustworthy. The runnable examples mirror this flow and
   surface identical inline notes reminding teams to update the shared smoke
   script whenever analytics identifiers change.【F:examples/selection-controls-yew/README.md†L1-L60】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】

### Enterprise telemetry considerations

- **Consistent context** – Telemetry callbacks receive a `TelemetryContext`
  containing the fully qualified component path, merged analytics/automation
  identifiers, and snapshot metadata describing the descriptor label and final
  attributes. Incident response teams can diff SSR/CSR output using this
  context without scraping HTML.
- **Deterministic ordering** – Checkbox/switch/radio adapters emit telemetry
  before invoking consumer handlers. Maintain this sequencing when authoring
  additional adapters so automation harnesses observe identical lifecycles.
- **Central overrides** – Use `SelectionControlTelemetry::enforce_defaults()`
  when platform governance mandates canonical identifiers. Builders will refuse
  to overwrite analytics/automation IDs, protecting compliance-critical hooks
  from local overrides.
- **Testing** – Extend the existing wasm-bindgen tests and integration suites to
  cover new telemetry delegates. Tests should assert ordering and attribute
  propagation to prevent regressions during framework upgrades.

## Automation + CI guardrails

- **`cargo xtask selection-controls`** – Executes the Rust integration tests,
  invokes the central smoke script across every framework, and triggers the
  Playwright runner so both Rust and JavaScript adapters stay in lockstep.【F:crates/xtask/src/main.rs†L163-L2381】
- **`examples/scripts/selection-controls-smoke.sh`** – Annotated harness that
  provisions toolchains, prints the canonical automation IDs, and forwards to
  framework-specific commands. Keep its notes aligned with adapter comments so
  auditors can trace why each step exists.【F:examples/scripts/selection-controls-smoke.sh†L1-L120】
- **Framework task runners** – Each package exposes `just automation-smoke` or
  comparable npm scripts that delegate back to the shared harness, reinforcing
  the “document once, run everywhere” workflow.【F:examples/selection-controls-react/README.md†L33-L75】【F:examples/selection-controls-yew/README.md†L19-L60】

## Testing Expectations

Unit tests now live under `crates/rustic-ui-material/tests/selection_control.rs`
and validate:

- Builder separation of ARIA/data/automation maps.
- SSR string consistency.
- Hook registration behavior.
- `Display` trait parity with SSR helpers.

Adapter authors should keep these tests running (`cargo test -p rustic-ui-material selection_control`) when introducing new hooks or metadata.
