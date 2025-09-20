# mui-material

Rust translation of Material UI components. Built on top of
[`mui-styled-engine`](../mui-styled-engine) and [`mui-system`](../mui-system).
The crate exposes high level widgets like `Button`, `AppBar`, `TextField` and
`Snackbar` which all pull colors, sizes and variants from a shared [`Theme`].
Common property boilerplate is generated through `material_component_props!`
macro so adding new widgets requires minimal manual code.

Components such as `Dialog` leverage the `css_with_theme!` macro so padding and
border colors are resolved from the active theme. The resulting class is
attached to the root element together with accessibility metadata (for example
`role="dialog"` and `aria-modal="true"`) ensuring assistive technologies can
accurately describe the UI without additional boilerplate.

Utilities from [`mui-utils`](../mui-utils) are integrated to provide
enterprise-friendly ergonomics: button callbacks can be throttled,
text inputs debounced and style overrides appended directly within
`css_with_theme!` blocks.

## Component Coverage

Parity with the upstream React package is tracked automatically in the
[Material component parity report](../../docs/material-component-parity.md).
The snapshot lists every export from `packages/mui-material/src` and highlights
which widgets are implemented in this crate or delegated to `mui-headless`.

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
| `leptos` | Leptos adapter | activates `wasm-bindgen` and `mui-system/leptos` |
| `dioxus` | Dioxus adapter | compiles `mui-system/dioxus` and `mui-styled-engine/dioxus` |
| `sycamore` | Sycamore adapter | hooks into `mui-system/sycamore` |

See the [Cargo feature guide](../../docs/cargo-features.md) for examples of
disabling defaults and enabling only the framework your application requires.

## Feedback primitives (Tooltip & Chip)

Enterprise telemetry, accessibility, and automation pipelines lean heavily on
the tooltip and chip primitives. `mui-material` layers themed markup on top of
the deterministic state machines provided by [`mui-headless`](../mui-headless).
The headless
crate documents every transition in [`TooltipState`](../mui-headless/src/tooltip.rs)
and [`ChipState`](../mui-headless/src/chip.rs) so QA suites, SSR renderers, and
framework adapters can all share the same assumptions.

### Tooltip API overview

- [`TooltipProps`](src/tooltip.rs) centralizes the automation hooks, ARIA
  metadata, and portal wiring. The shared renderer returns SSR-safe markup that
  matches hydration output for Yew, Leptos, Dioxus, and Sycamore adapters.
- [`TooltipTriggerAttributes` and `TooltipSurfaceAttributes`](../mui-headless/src/tooltip.rs)
  expose fine-grained attribute builders when teams need to augment the
  baseline HTML emitted by `mui-material`.
- Portal containers derive their identifiers from `automation_id`, ensuring QA
  selectors stay stable across frameworks and rendering modes.

The [`feedback-tooltips`](../../examples/feedback-tooltips) blueprint packages a
ready-to-run SSR snapshot plus hydration stubs for each supported framework.
Run `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`
to materialize the scaffolding under `target/feedback-tooltips` with themed
overrides, portal markup, and automation IDs pre-wired.

### Chip API overview

- [`ChipProps`](src/chip.rs) mirrors the headless [`ChipConfig`](../mui-headless/src/chip.rs)
  so automation identifiers, delete affordances, and ARIA relationships are
  consistent between SSR and hydration.
- [`ChipAttributes` and `ChipDeleteAttributes`](../mui-headless/src/chip.rs)
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
[`mui-styled-engine`](../mui-styled-engine) through the `css_with_theme!`
macro. During SSR the [`StyleRegistry`](../mui-styled-engine/src/context.rs)
collects the generated CSS so automation can snapshot the rendered document
without manual wiring. The blueprints above return the themed `Theme` instance
alongside the markup to keep hydration shells and analytics dashboards in sync.

### Additional examples

- [`data-display-avatar`](../../examples/data-display-avatar) renders team
  presence chips with optional tooltips to demonstrate cross-framework data
  display patterns.
- [`mui-ssr-accessibility`](../../examples/mui-ssr-accessibility) continues to
  document broader SSR pipelines including global style flushing and automated
  accessibility checks.

## Framework adapters & portal orchestration

Every Material component exposes framework-specific adapter modules (`yew`,
`leptos`, `dioxus`, `sycamore`) that simply forward props/state into shared
renderers.  The adapters return HTML strings suitable for SSR pipelines and are
careful to reuse the central markup helpers so hydration is deterministic across
frameworks.

Floating surfaces such as `Select` and `Menu` now leverage
[`mui_system::PortalMount`](../mui-system/src/portal.rs) to emit deterministic
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
use mui_material::{Button, AppBar, TextField};
use mui_styled_engine::{ThemeProvider, Theme};
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
[`examples/mui-ssr-accessibility`](../../examples/mui-ssr-accessibility).
