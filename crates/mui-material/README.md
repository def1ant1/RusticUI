# mui-material

Rust translation of Material UI components. Built on top of [`mui-styled-engine`](../mui-styled-engine) and [`mui-system`](../mui-system).

## Example

```rust
use mui_material::{Button};
use mui_styled_engine::{ThemeProvider, Theme};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ThemeProvider theme={Theme::default()}>
            <Button label="Press" />
        </ThemeProvider>
    }
}
```
