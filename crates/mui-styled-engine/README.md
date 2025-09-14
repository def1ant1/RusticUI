# mui-styled-engine

`mui-styled-engine` binds the [stylist] CSS-in-Rust library with the
`mui-system` theme primitives. It generates scoped CSS at compile time and
provides Yew components for global style injection and style management.

## Macros

To minimize repetitive boilerplate when working with themes, the crate ships
with two procedural macros:

* `#[derive(Theme)]` - converts a user defined struct into a full
  [`Theme`](https://docs.rs/mui-styled-engine/latest/mui_styled_engine/struct.Theme.html)
  by merging the provided fields with `Theme::default()`. This is useful for
  creating lightweight theme overrides.
* `styled_component!` - wraps a regular function and turns it into a Yew
  component that automatically wires up `use_theme()`. The body of the function
  can reference a `theme` binding without additional setup.

Both macros are re-exported from this crate so downstream code only needs a
single dependency.

## Usage

```rust
use mui_styled_engine::{css_with_theme, GlobalStyles, StyledEngineProvider, Theme};

let theme = Theme::default();
let style = css_with_theme!(theme, r#"color: ${c};"#, c = theme.palette.primary.clone());
assert!(style.get_class_name().starts_with("css-"));
```

## Server Side Rendering

The [`ssr` module](https://docs.rs/mui-styled-engine/latest/mui_styled_engine/ssr/index.html)
provides helpers that run a render closure inside an isolated style manager and
return both the generated HTML and the associated style tags. The
`render_to_string` convenience function produces a complete HTML document ready
to be returned from an Axum or Actix handler.

## Benchmarks

Run `cargo bench -p mui-styled-engine` to compare compile-time CSS generation
against dynamic string formatting. The benchmark reports both runtime and the
size of generated CSS strings.

[stylist]: https://crates.io/crates/stylist

