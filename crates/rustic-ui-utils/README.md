# rustic_ui_utils

Utility helpers shared across the Material UI Rust ecosystem.

## Feature flags

| Feature | Enables | Notes |
|---------|---------|-------|
| `web` | WebAssembly helpers | pulls in `wasm-bindgen`, `js-sys` and `web-sys` for timer APIs |

No features are enabled by default. Opt into `web` when compiling for the
browser:

```toml
[dependencies]
rustic_ui_utils = { version = "0.1", default-features = false, features = ["web"] }
```

See the [Cargo feature guide](../../docs/cargo-features.md) for additional
examples.
