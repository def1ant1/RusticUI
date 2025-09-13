# mui-styled-engine

`mui-styled-engine` binds the [stylist] CSS-in-Rust library with the
`mui-system` theme primitives. It generates scoped CSS at compile time and
provides Yew components for global style injection and style management.

## Usage

```rust
use mui_styled_engine::{css_with_theme, GlobalStyles, StyledEngineProvider, Theme};

let theme = Theme::default();
let style = css_with_theme!(theme, r#"color: ${c};"#, c = theme.palette.primary.clone());
assert!(style.get_class_name().starts_with("css-"));
```

## Server Side Rendering

`StyledEngineProvider` accepts an optional `StyleManager` which collects CSS
rules during server side rendering. After rendering, call
`manager.render().to_string()` to obtain the CSS payload for the `<head>` of the
generated HTML.

## Benchmarks

Run `cargo bench -p mui-styled-engine` to compare compile-time CSS generation
against dynamic string formatting. The benchmark reports both runtime and the
size of generated CSS strings.

[stylist]: https://crates.io/crates/stylist

