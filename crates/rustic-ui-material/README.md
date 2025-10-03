# rustic_ui_material

Rust translation of Material UI components. Built on top of
[`rustic_ui_styled_engine`](../rustic-ui-styled-engine) and [`rustic_ui_system`](../rustic-ui-system).
The crate exposes high level widgets like `Button`, `AppBar`, `TextField` and
`Snackbar` which all pull colors, sizes and variants from a shared [`Theme`].
Common property boilerplate is generated through `material_component_props!`
macro so adding new widgets requires minimal manual code.

Components such as `Dialog` leverage the `css_with_theme!` macro so padding and
border colors are resolved from the active theme. The resulting class is
attached to the root element together with accessibility metadata (for example
`role="dialog"` and `aria-modal="true"`) ensuring assistive technologies can
accurately describe the UI without additional boilerplate.

Utilities from [`rustic_ui_utils`](../rustic-ui-utils) are integrated to provide
enterprise-friendly ergonomics: button callbacks can be throttled,
text inputs debounced and style overrides appended directly within
`css_with_theme!` blocks.

## Component Coverage

Parity with the upstream React package is tracked automatically in the
[Material component parity report](../../docs/material-component-parity.md).
The snapshot lists every export from `packag../rustic-ui-material/src` and highlights
which widgets are implemented in this crate or delegated to `rustic_ui_headless`.

Current gaps most relevant to enterprise adopters include:

- Advanced form helpers such as `Autocomplete`.
- The data-heavy `Table` family (`Table`, `TableBody`, `TablePagination`).
- Navigation primitives including `Tabs` and related panels.

Contributions that land these components should reference the report to keep
the automation in sync; `cargo xtask material-parity` refreshes the metrics.

## Selection control descriptor factories

Enterprise teams integrating checkboxes, switches, and radio buttons across SSR
and multiple front-end runtimes can now rely on the shared
[`SelectionControlDescriptor`](src/selection_control.rs) factories. The helpers
wrap headless state-machine snapshots and automatically merge themed defaults,
managed telemetry identifiers, and ARIA/data attributes so adapters avoid
duplicating boilerplate.

- [`SelectionControlThemeTokens`](src/selection_control.rs) encapsulates the
  automation selectors and classes emitted by centralized configuration
  services, keeping SSR and hydration output aligned.
- [`SelectionControlTelemetry`](src/selection_control.rs) carries the configured
  [`TelemetryHooks`](src/telemetry.rs) plus analytics/automation identifiers and
  enforces override policies when required by compliance platforms.
- `SelectionControlDescriptor::from_headless` consumes the headless
  [`CheckboxState`](../rustic-ui-headless/src/checkbox.rs) or
  [`SwitchState`](../rustic-ui-headless/src/switch.rs) and returns both the
  themed attributes and resolved telemetry metadata for adapters and SSR
  renderers.

The accompanying unit tests (`cargo test -p rustic-ui-material selection_control`)
demonstrate telemetry defaults, attribute overrides, and strict error handling
when managed analytics identifiers conflict with headless overrides.

## Feature Flags

Select a single front-end framework to keep builds lean. All features are
disabled by default so applications opt in explicitly:

| Feature | Enables | Notes |
|---------|---------|-------|
| `yew` | Yew adapter | pulls in `yew`, `wasm-bindgen`, `web-sys` and `stylist` |
| `leptos` | Leptos adapter | activates `wasm-bindgen` and `rustic_ui_system/leptos` |
| `dioxus` | Dioxus adapter | compiles `rustic_ui_system/dioxus` and `rustic_ui_styled_engine/dioxus` |
| `sycamore` | Sycamore adapter | hooks into `rustic_ui_system/sycamore` |

See the [Cargo feature guide](../../docs/cargo-features.md) for examples of
disabling defaults and enabling only the framework your application requires.

## WebAssembly test harness

Enterprise teams frequently validate React bindings inside the WebAssembly
target to guarantee the `wasm_bindgen` bridges and telemetry delegates behave as
expected. The crate ships a convenience alias so there is no bespoke scripting
required to exercise the suite:

```bash
# one-time toolchain setup
rustup target add wasm32-unknown-unknown

# execute every React adapter test (including the new telemetry assertions)
cargo wasm-react-test
```

The alias expands to `cargo test --target wasm32-unknown-unknown --features
react`, keeping CI and local workflows aligned without copy/pasting the verbose
command. The tests depend on [`wasm-bindgen-test`], so running them in a browser
enabled environment (headless or via `wasm_bindgen_test_configure!(run_in_browser)`) is
fully supported.

## Feedback primitives (Tooltip & Chip)

Enterprise telemetry, accessibility, and automation pipelines lean heavily on
the tooltip and chip primitives. `rustic_ui_material` layers themed markup on top of
the deterministic state machines provided by [`rustic_ui_headless`](../rustic-ui-headless).
The headless
crate documents every transition in [`TooltipState`](../rustic-ui-headless/src/tooltip.rs)
and [`ChipState`](../rustic-ui-headless/src/chip.rs) so QA suites, SSR renderers, and
framework adapters can all share the same assumptions.

### Tooltip API overview

- [`TooltipProps`](src/tooltip.rs) centralizes the automation hooks, ARIA
  metadata, and portal wiring. The shared renderer returns SSR-safe markup that
  matches hydration output for Yew, Leptos, Dioxus, and Sycamore adapters.
- [`TooltipTriggerAttributes` and `TooltipSurfaceAttributes`](../rustic-ui-headless/src/tooltip.rs)
  expose fine-grained attribute builders when teams need to augment the
  baseline HTML emitted by `rustic_ui_material`.
- Portal containers derive their identifiers from `automation_id`, ensuring QA
  selectors stay stable across frameworks and rendering modes.

The [`feedback-tooltips`](../../examples/feedback-tooltips) blueprint packages a
ready-to-run SSR snapshot plus hydration stubs for each supported framework.
Run `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`
to materialize the scaffolding under `target/feedback-tooltips` with themed
overrides, portal markup, and automation IDs pre-wired.

### Chip API overview

- [`ChipProps`](src/chip.rs) mirrors the headless [`ChipConfig`](../rustic-ui-headless/src/chip.rs)
  so automation identifiers, delete affordances, and ARIA relationships are
  consistent between SSR and hydration.
- [`ChipAttributes` and `ChipDeleteAttributes`](../rustic-ui-headless/src/chip.rs)
  expose the underlying attribute builders when custom renderers or analytics
  hooks need direct access to the state machine.
- The renderer emits deterministic `data-*` hooks for visibility, deletion, and
  control affordances which downstream telemetry can stream without per
  framework adapters.

The [`feedback-chips`](../../examples/feedback-chips) demo bootstraps the same
multi-framework scaffolding with dismissible and non-dismissible variants so QA
teams can validate automation flows with a single command.

## Stepper workflow renderer

`render_stepper` translates the shared [`StepperState`](../rustic-ui-headless/src/stepper.rs)
into themed classes, ARIA attributes, and automation IDs so Material steppers
maintain parity across React, Yew, Leptos, Dioxus, and Sycamore adapters.  The
renderer surfaces deterministic `id`/`data-*` selectors derived from the
`rustic-stepper` automation prefix, allowing Playwright or Cypress suites to pin
against the same DOM contract regardless of rendering mode.  Controlled hooks
(`use_state`, signals, etc.) and uncontrolled flows (per-render constructors)
share the same [`StepperAdapterProps`](src/stepper.rs) ensuring SSR snapshots and
hydrated DOM trees remain byte-identical while analytics streams reuse the
automation metadata without additional boilerplate.

### Theming and automation hooks

Both components pull palette, typography, and spacing tokens from
[`rustic_ui_styled_engine`](../rustic-ui-styled-engine) through the `css_with_theme!`
macro. During SSR the [`StyleRegistry`](../rustic-ui-styled-engine/src/context.rs)
collects the generated CSS so automation can snapshot the rendered document
without manual wiring. The blueprints above return the themed `Theme` instance
alongside the markup to keep hydration shells and analytics dashboards in sync.

### Additional examples

- [`data-display-avatar`](../../examples/data-display-avatar) renders team
  presence chips with optional tooltips to demonstrate cross-framework data
  display patterns.
- [`rustic_ui_ssr_accessibility`](../../exampl../rustic-ui-ssr-accessibility) continues to
  document broader SSR pipelines including global style flushing and automated
  accessibility checks.

## Select component guide

Material select adapters consume the headless [`SelectState`](../rustic-ui-headless/src/select.rs)
directly.  Disabled bookkeeping is centralized in the state machine so renderers
emit consistent ARIA/data attributes without duplicating logic:

- Call `state.set_option_disabled(index, bool)` whenever async data or business
  rules change option availability. Uncontrolled selects automatically advance
  the highlight/selection to the next enabled entry.
- Use `state.option_accessibility_attributes(index)` to pull the `role` and
  optional disabled metadata straight from the state machine. The shared
  renderer extends the returned vector with automation hooks so SSR and
  hydration markup stay aligned without manual `data-disabled="false"`
  bookkeeping.
- Navigation helpers (`on_key`, `on_typeahead`) skip disabled islands; adapters
  only need to forward the callbacks and respond to the returned indices (for
  example to scroll newly highlighted rows into view).

The framework-specific tests under `tests/select_adapters.rs` assert that Yew,
Leptos, Dioxus, and Sycamore renders all include the disabled metadata. When
augmenting the component ensure any additional markup preserves these hooks so
end-to-end automation continues to function.

## Radio group telemetry orchestration

Material radio groups now expose a telemetry contract that mirrors the
checkbox and switch integrations. `RadioTelemetryEvent` includes analytics,
key, focus, blur, change, and commit variants so enterprise dashboards can trace
every interaction across SSR and hydration renders. Adapters emit telemetry in
the deterministic order `analytics → key → focus/blur → change → commit` before
invoking consumer callbacks, ensuring QA suites observe the exact state the user
experienced.【F:crates/rustic-ui-material/src/radio.rs†L95-L214】【F:crates/rustic-ui-material/src/radio.rs†L930-L1110】【F:crates/rustic-ui-material/src/radio.rs†L2620-L2798】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L272】

Keyboard flows now forward a dedicated `RadioKeyEvent` to telemetry delegates in
addition to the existing change payloads. Regression tests cover pointer
selection, focus transitions, and keyboard commits across Yew, Leptos, Dioxus,
and Sycamore adapters to guarantee analytics ordering remains stable as new
frameworks or descriptors are added.【F:crates/rustic-ui-material/src/radio.rs†L2700-L2798】【F:crates/rustic-ui-material/src/radio.rs†L3460-L3568】【F:crates/rustic-ui-material/src/radio.rs†L4580-L4662】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L272】

## Dialog, popover, and text field adapters

The Material adapters for `Dialog`, `Popover`, and `TextField` lean directly on
the new headless state machines documented in
[`shared-dialog-state-core`](../../examples/shared-dialog-state-core). Each
adapter mirrors the controlled workflow to keep SSR snapshots, hydration output,
and client updates in lockstep.

- **Dialog** – framework modules (`dialog::yew`, `dialog::leptos`,
  `dialog::dioxus`, `dialog::sycamore`) accept a `DialogState` and call
  `surface_attributes()` to emit `role`, `aria-modal`, `data-state`, and
  `data-transition` markers. Portal/backdrop helpers rely on the same state
  object so automation IDs stay consistent across renders.
- **Popover** – the Material popover helpers (used by `Menu`, `Select`, and the
  shared dialog state examples) forward anchor geometry, preferred placement,
  and collision outcomes from `PopoverState`. The adapters emit
  `data-preferred-placement`, `data-resolved-placement`, and
  `data-open` attributes so SSR snapshots and hydrated DOM trees are identical.
- **TextField** – the high-level `TextField` component wraps
  `TextFieldStateHandle` which internally stores a `TextFieldState` inside an
  `Rc<RefCell<_>>`. Change, commit, and reset handlers invoke the headless state
  methods and surface the corresponding `TextFieldChangeEvent`,
  `TextFieldCommitEvent`, and `TextFieldResetEvent` structs. Attribute builders
  from `rustic_ui_headless` ensure analytics IDs and validation metadata stay
  deterministic.

The automation-focused examples under `examples/shared-dialog-state-*` reuse
the Material adapters to prove that SSR and hydration output match the
framework-agnostic state orchestration. When integrating the components into a
product, defer to the state machine APIs for all intent handling rather than
duplicating open/close or validation logic in UI code.

## Framework adapters & portal orchestration

Every Material component exposes framework-specific adapter modules (`yew`,
`leptos`, `dioxus`, `sycamore`) that simply forward props/state into shared
renderers.  The adapters return HTML strings suitable for SSR pipelines and are
careful to reuse the central markup helpers so hydration is deterministic across
frameworks.

Floating surfaces such as `Select` and `Menu` now leverage
[`rustic_ui_system::PortalMount`](../rustic-ui-system/src/portal.rs) to emit deterministic
`data-portal-*` anchors during SSR. Each adapter renders the trigger, appends a
hidden anchor placeholder, and then emits a detached container that client
frameworks attach to `document.body` once lifecycle hooks fire (`Component::view`
for Yew, `create_effect`/`spawn_local` for Leptos, `use_future` for Dioxus and
`create_effect` for Sycamore).  Because the portal IDs derive from the
`automation_id` prop, QA suites can target the surfaces without caring about the
host framework.

When integrating the adapters in an application ensure the portal metadata is
consumed during hydration—each framework has a lightweight bootstrap helper that
looks up the `data-portal-anchor` element and positions the floating surface
relative to it once the runtime is ready.  This keeps server and client output in
lock-step and eliminates duplicate popover markup.

## Example

```rust
use rustic_ui_material::{Button, AppBar, TextField};
use rustic_ui_styled_engine::{ThemeProvider, Theme};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ThemeProvider theme={Theme::default()}>
            <AppBar
                title="My App"
                aria_label="main navigation"
                automation_id={Some("global.app-bar".into())}
                analytics_view_id={Some("nav.global.view".into())}
            />
            // Throttle rapid clicks to once every 200ms
            <Button label="Press" throttle_ms={200} />
            // Debounced text input with custom background color
            <TextField
                value="".into()
                placeholder="Search"
                aria_label="search"
                debounce_ms={300}
                style_overrides={"background: #eee;"}
            />
        </ThemeProvider>
    }
}
```

Additional enterprise patterns such as server side rendering can be found under
[`exampl../rustic-ui-ssr-accessibility`](../../exampl../rustic-ui-ssr-accessibility).

## Architectural rationale for the new renderers and adapters

- **Shared render helpers** – The new automation utilities rely on
  `render_helpers::AutomationAttributes` to ensure click-away, focus trap, and
  telemetry metadata emit the same `data-rustic-*` selectors regardless of the
  adapter.  Keeping these helpers in a single module means downstream frameworks
  inherit identical SSR output and hydration fingerprints.
- **Portal lifecycle alignment** – Dialog, popover, and snackbar renderers now
  share a `PortalLifecycleController`.  It defers DOM mutations until hydration,
  allowing headless state machines to run during SSR without referencing window
  APIs.  Material adapters register callbacks through this controller so CSR
  updates feed the same analytics hooks.
- **Telemetry streaming** – Adapter-specific modules (`dialog::yew`,
  `dialog::leptos`, etc.) ingest the headless `EventStream` and forward records
  into the framework’s preferred logging primitive (tracing spans for Yew,
  console batching for Leptos, and signal-aware loggers for Dioxus and Sycamore).
  This keeps observability uniform even though each framework exposes different
  runtime ergonomics.

## Troubleshooting Material adapters for the utilities

1. **Mismatched SSR attributes** – Regenerate the automation fixtures via
   `cargo test -p rustic-ui-material -- --ignored --test automation_examples`. The
   ignored suite snapshots SSR and CSR output from every adapter to confirm that
   the new utilities emit identical `data-*` markers.
2. **Focus trap regressions** – Enable the adapter-level `LogFocusDiagnostics`
   feature (gated behind `cfg(feature = "diagnostics")`) to print the headless
   focus timeline.  Compare the timestamps with the telemetry ndjson emitted by
   `cargo xtask examples --group automation --release` to confirm the renderer is
   consuming the stream correctly.
3. **Click-away listeners firing twice** – Ensure the adapter defers
   `ClickAwayState::arm()` until the framework confirms hydration.  Yew exposes
   this via `use_effect_with`, Leptos via `on_mount`, Dioxus via `use_future`, and
   Sycamore via `on_mount`.  The shared documentation at the top of each module
   calls out the pattern so the behaviour stays consistent.

## Observability guidance

- Opt into the `telemetry` Cargo feature when you need high-cardinality logging
  for overlays or transient surfaces.  The feature wires the headless
  `EventStream` directly into the Material adapters and surfaces a
  `TelemetrySubscriber` prop so applications can batch or forward events into
  enterprise monitoring stacks.
- Every adapter exposes a `with_analytics_layer` helper that wraps the rendered
  HTML in data attributes mirroring the headless metadata (for example
  `data-rustic-focus-trap="active"`).  Capture these markers in Playwright or
  Cypress to guarantee the utilities stay observable without DOM spelunking.
- The automation blueprints under `examples/` stream telemetry into
  `target/rustic-ui-automation/automation-events.ndjson`.  Tail the file while
  running your harness to confirm click-away dismissals, focus hand-offs, and
  snackbar queue transitions are accounted for end-to-end.
- Integration tests under `tests/checkbox_adapters.rs` now fabricate framework-
  specific telemetry delegates and consumer callbacks, asserting that
  instrumentation hooks capture analytics/automation identifiers before
  forwarding `CheckboxTelemetryEvent` payloads to user code.  Downstream teams
  can mirror this pattern to validate their own adapters by providing
  deterministic fakes for the delegate and callbacks, invoking the telemetry
  payloads in the documented order (change → focus → blur → key) and asserting
  both the context metadata and payloads include the configured identifiers.
