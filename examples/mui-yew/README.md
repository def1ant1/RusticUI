# MUI Yew Example

This example demonstrates how to combine the `rustic_ui_system` crate with the
[Yew](https://yew.rs) framework. It showcases basic components, theming and
both client-side rendering and an optional server-side rendering setup.

## Development (CSR)
```bash
trunk serve --open
```
This builds the WebAssembly bundle and hosts it with live reload.

## Server-side rendering
```bash
cargo run --manifest-path examples/rustic_ui_yew/Cargo.toml --features ssr > prerendered.html
# serve `prerendered.html` and hydrate it with the client bundle
trunk build --release
```
`trunk build` produces the `dist/` directory containing assets to hydrate the
pre-rendered HTML. Any static host or CDN can serve this directory.
