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
            <AppBar title="My App" aria_label="main navigation" />
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
