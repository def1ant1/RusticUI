# MUI Dioxus Example

A tiny showcase integrating `rustic_ui_system` theming with the
[Dioxus](https://dioxuslabs.com) renderer.

## Running the demo

### Client side
```bash
# Using the official CLI for best DX
npx dx serve examples/rustic_ui_dioxus
```

### Server side rendering
```bash
cargo run --manifest-path examples/rustic_ui_dioxus/Cargo.toml --features ssr
```
The printed HTML can be served by your backend and hydrated on the client with
`dx build --release`.
