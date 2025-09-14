# mui-material

Rust translation of Material UI components. Built on top of
[`mui-styled-engine`](../mui-styled-engine) and [`mui-system`](../mui-system).
The crate exposes high level widgets like `Button`, `AppBar`, `TextField` and
`Snackbar` which all pull colors, sizes and variants from a shared [`Theme`].
Common property boilerplate is generated through `material_component_props!`
macro so adding new widgets requires minimal manual code.

Utilities from [`mui-utils`](../mui-utils) are integrated to provide
enterprise-friendly ergonomics: button callbacks can be throttled,
text inputs debounced and style overrides merged via JSON using `deep_merge`.

## Example

```rust
use mui_material::{Button, AppBar, TextField};
use mui_styled_engine::{ThemeProvider, Theme};
use yew::prelude::*;
use serde_json::json;

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
                style_overrides={json!({"background": "#eee"})}
            />
        </ThemeProvider>
    }
}
```

Additional enterprise patterns such as server side rendering can be found under
[`examples/mui-ssr-accessibility`](../../examples/mui-ssr-accessibility).
