# MUI Dioxus Example

A tiny showcase integrating `mui-system` theming with the
[Dioxus](https://dioxuslabs.com) renderer.

## Running the demo

### Client side
```bash
# Using the official CLI for best DX
npx dx serve examples/mui-dioxus
```

### Server side rendering
```bash
cargo run --manifest-path examples/mui-dioxus/Cargo.toml --features ssr
```
The printed HTML can be served by your backend and hydrated on the client with
`dx build --release`.
