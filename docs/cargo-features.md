# Cargo feature guide

Material UI Rust crates expose fine‑grained Cargo features so applications compile only what they need. Disabling defaults and enabling a small feature set keeps builds fast and binaries lean.

## Components

`mui-material` gates each front‑end framework behind a feature flag. Start with no default features and opt into the one you use:

```toml
[dependencies]
mui-material = { version = "0.1", default-features = false, features = ["leptos"] }
```

The table below lists available framework adapters:

| Feature   | Enables | Notes |
|-----------|---------|-------|
| `yew`     | Yew components | pulls in `yew`, `wasm-bindgen`, `web-sys`, `stylist` |
| `leptos`  | Leptos components | activates `wasm-bindgen` and `mui-system/leptos` |
| `dioxus`  | Dioxus components | compiles `mui-system/dioxus` and `mui-styled-engine/dioxus` |
| `sycamore`| Sycamore components | hooks into `mui-system/sycamore` |

## Icons

`mui-icons-material` ships thousands of SVGs, each behind its own feature. The crate enables `all-icons` by default for convenience. In production disable the default and cherry‑pick only the icons your UI references:

```toml
[dependencies]
mui-icons-material = { version = "0.1", default-features = false, features = ["icon-10k_24px"] }
```

`update-icons` is a maintenance feature that exposes a CLI used by maintainers to refresh the icon list; end users rarely need it.

## Utilities

`mui-utils` contains helpers that are generic enough for server or client environments. Enable the optional `web` feature when targeting WebAssembly to pull in timer bindings:

```toml
[dependencies]
mui-utils = { version = "0.1", default-features = false, features = ["web"] }
```

## Further reading

Each crate's README documents its feature flags in more depth. For workspace‑wide automation and testing commands see [CONTRIBUTING.md](../CONTRIBUTING.md).
