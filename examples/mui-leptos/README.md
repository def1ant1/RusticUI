# MUI Leptos Example

This example pairs `rustic_ui_system` with the [Leptos](https://leptos.dev) framework
and showcases a minimal component, theming and optional server-side rendering.

## Prerequisites
- Rust nightly or stable with the `wasm32-unknown-unknown` target installed
- [`trunk`](https://trunkrs.dev) for bundling and serving the client build

## Running the demo

### Client side rendering
```bash
trunk serve --open
```
This compiles to WebAssembly and serves the result with live reload enabled.

### Server side rendering
```bash
cargo run --manifest-path examples/rustic_ui_leptos/Cargo.toml --features ssr
# Then in another terminal build the hydrated client bundle
trunk build --release
```
The printed HTML from the server run can be served by any web framework and
hydrated by the client-side bundle to enable a full isomorphic setup.
