# RusticUI Select Menu — Leptos

The Leptos example mirrors the Yew implementation to show how the shared
`select-menu-shared` crate enables consistent rendering, automation hooks, and
SSR behaviour across frameworks. State is stored in `RwSignal`s so Leptos can
reactively control the headless `SelectState` and keep hydration in sync with
the server snapshot.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Client-side development

```bash
cd examples/select-menu-leptos
trunk serve --open
```

The component spins up immediately with a fallback message, then updates once
the async loader resolves. Toggle buttons demonstrate how controlled props can
be wired into existing signals without bypassing the shared render helpers.

Whenever business rules deactivate a region you can flip the headless state in
place and let the renderer mirror the disabled affordance:

```rust
state.update(|state| state.set_option_disabled(1, true));
```

This keeps the SSR snapshot and CSR hydration path aligned because the
underlying `SelectState` skips disabled options for keyboard/typeahead
navigation while emitting the appropriate ARIA/data hooks for analytics.

## Server-side rendering smoke test

```bash
cargo run --manifest-path examples/select-menu-leptos/Cargo.toml --no-default-features --features ssr > ssr.html
```

The generated HTML is the exact markup consumed by the CSR build. Automation
ids and ARIA attributes are identical across server and client passes to keep
end-to-end tests stable.
