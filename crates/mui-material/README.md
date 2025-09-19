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

- Advanced form helpers such as `Autocomplete` and `Select`.
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
