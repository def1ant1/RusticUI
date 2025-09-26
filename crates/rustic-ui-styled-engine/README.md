# rustic_ui_styled_engine

`rustic_ui_styled_engine` binds the [stylist] CSS-in-Rust library with the
`rustic_ui_system` theme primitives. It generates scoped CSS at compile time and
provides components for global style injection, style collection and server
side rendering (SSR) across Yew, Leptos, Dioxus and Sycamore front-ends.

## Macros

To minimize repetitive boilerplate when working with themes, the crate ships
with several procedural macros:

* `#[derive(Theme)]` - converts a user defined struct into a full
  [`Theme`](https://docs.../rustic-ui-styled-engine/latest/rustic_ui_styled_engine/struct.Theme.html)
  by merging the provided fields with `Theme::default()`. Nested structs and
  `Option<T>` fields are recursively merged which keeps custom themes compact.
* `css_with_theme!` - wraps the [`stylist::css!`] macro and automatically
  injects a `use_theme()` call. The macro exposes a `theme` binding inside the
  style block and works across Yew, Leptos, Dioxus and Sycamore backends.
* `styled_component!` - wraps a regular function and turns it into a Yew
  component that automatically wires up `use_theme()`. The body of the function
  can reference a `theme` binding without additional setup.

All macros are re-exported from this crate so downstream code only needs a
single dependency.

## Feature Flags

Activate front-end integrations with Cargo features: `yew`, `leptos`, `dioxus`
or `sycamore`.

## Usage

```rust
use rustic_ui_styled_engine::{css_with_theme, GlobalStyles, StyledEngineProvider};

// `css_with_theme!` exposes a `theme` variable in the CSS block
let style = css_with_theme!(r#"color: ${p};"#, p = theme.palette.primary.clone());
assert!(style.get_class_name().starts_with("css-"));
```

## Migration

Prior versions required passing an explicit theme to `css_with_theme!`:

```rust
// Old
// let style = css_with_theme!(theme, r#"color: ${c};"#, c = theme.palette.primary.clone());

// New
let style = css_with_theme!(r#"color: ${p};"#, p = theme.palette.primary.clone());
```

The derive macro now understands nested structs and optional fields so theme
overrides can be expressed succinctly:

```rust
use rustic_ui_styled_engine::{Theme, Palette};

#[derive(Theme)]
struct MyTheme {
    palette: Option<Palette>,
}
```

## Server Side Rendering

The [`ssr` module](https://docs.../rustic-ui-styled-engine/latest/rustic_ui_styled_engine/ssr/index.html)
provides helpers that run a render closure inside an isolated style manager and
return both the generated HTML and the associated style tags. The
`render_to_string` convenience function produces a complete HTML document ready
to be returned from an Axum or Actix handler.

### Yew Provider and Style Registry

For component based applications the crate exposes a `StyledEngineProvider`
which wraps its children with both a `ThemeProvider` and a `ContextProvider`
carrying a `StyleRegistry`. Components can obtain the registry through
`use_context::<StyleRegistry>()` and create styles via
`Style::new_with_manager(..., registry.style_manager())`.  During SSR each HTTP
request should instantiate its own registry to avoid cross-request leakage.  The
registry implements `flush_styles()` which drains and returns `<style>` blocks:

```rust
use rustic_ui_styled_engine::{StyleRegistry, StyledEngineProvider, Theme};
use yew::ServerRenderer;

let registry = StyleRegistry::new(Theme::default());
let html = yew::platform::block_on(
    ServerRenderer::<StyledEngineProvider>::with_props(StyledEngineProviderProps {
        theme: Theme::default(),
        registry: Some(registry.clone()),
        children: yew::html::ChildrenRenderer::new(vec![/* ... */]),
    }).render(),
);
let styles = registry.flush_styles();
```

Because style data is drained on flush, the registry can be reused for multiple
renders within the same request without manual cleanup. This design scales to
highly concurrent environments since each request owns its registry and the
internal reader is protected by a mutex.

## Benchmarks

Run `cargo bench -p rustic_ui_styled_engine` to compare compile-time CSS generation
against dynamic string formatting. The benchmark reports both runtime and the
size of generated CSS strings.

[stylist]: https://crates.io/crates/stylist

