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

## Testing Expectations

Unit tests now live under `crates/rustic-ui-material/tests/selection_control.rs`
and validate:

- Builder separation of ARIA/data/automation maps.
- SSR string consistency.
- Hook registration behavior.
- `Display` trait parity with SSR helpers.

Adapter authors should keep these tests running (`cargo test -p rustic-ui-material selection_control`) when introducing new hooks or metadata.
