# MUI System (Rust)

This crate provides the low level layout and theming primitives that power the
Material UI ecosystem in Rust.  Components target modern frameworks like
[`yew`](https://yew.rs), `leptos`, `dioxus` and `sycamore` and favor compile‑time
safety over runtime configuration.

## Usage

```rust
use mui_system::{Box, Stack, style_props, ThemeProvider, Theme};
# #[cfg(feature = "yew")]
# fn render() -> yew::Html {
let theme = Theme::default();
html! {
    <ThemeProvider theme={theme}>
        <Stack spacing={Some("8px".into())} justify_content={Some("center".into())}>
            <Box sx={style_props!{ padding: "4px" }}>{"Item"}</Box>
        </Stack>
    </ThemeProvider>
}
# }
```

Enable the desired front‑end framework via Cargo features:

```toml
mui-system = { version = "0.1", features = ["yew"] }
```

Available features include `yew`, `leptos`, `dioxus` and `sycamore`.

## Legacy JavaScript Package

The original `packages/mui-system` directory from the upstream project has been
**archived**.  All new development happens in this Rust crate which offers the
same API surface with stronger typing and zero runtime dependencies.  Consumers
are encouraged to migrate and report any missing features.

## Testing

Unit tests cover layout math, theming and WebAssembly compatibility.  Run the
suite with:

```bash
cargo test -p mui-system
wasm-pack test --node crates/mui-system
```

## Contributing

The crate aims to be heavily documented so that enterprise teams can build on
it with confidence.  Contributions that further automate repetitive styling
tasks via macros or code generation are especially welcome.
