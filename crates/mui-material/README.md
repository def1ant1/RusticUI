# mui-material

Rust translation of Material UI components. Built on top of
[`mui-styled-engine`](../mui-styled-engine) and [`mui-system`](../mui-system).
The crate exposes high level widgets like `Button`, `AppBar`, `TextField` and
`Snackbar` which all pull colors, sizes and variants from a shared [`Theme`].
Common property boilerplate is generated through `material_component_props!`
macro so adding new widgets requires minimal manual code.

## Example

```rust
use mui_material::{Button, AppBar};
use mui_styled_engine::{ThemeProvider, Theme};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ThemeProvider theme={Theme::default()}>
            <AppBar title="My App" aria_label="main navigation" />
            <Button label="Press" />
        </ThemeProvider>
    }
}
```

Additional enterprise patterns such as server side rendering can be found under
[`examples/mui-ssr-accessibility`](../../examples/mui-ssr-accessibility).
