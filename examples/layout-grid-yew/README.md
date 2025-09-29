# Yew Responsive Grid Layout Example

## Overview

This crate demonstrates how RusticUI's Yew primitives orchestrate a responsive
marketing layout that adapts to Material breakpoints while remaining hydration
safe. The implementation mirrors enterprise grade SSR pipelines: components
share a typed layout blueprint, hydration phases are documented, and server
rendering emits deterministic automation hooks for observability.

## Quick start

```bash
cd examples/layout-grid-yew
just bootstrap
```

The shared bootstrap script validates that `cargo`, `rustup`, the
`wasm32-unknown-unknown` target, and `trunk` are ready. CI jobs invoke the same
script to fail fast when toolchains drift.

## Common workflows

| Goal | Command | Notes |
| --- | --- | --- |
| Verify the crate | `just test` | Runs `cargo test --all-features` so SSR snapshots are covered alongside CSR behaviour. |
| Lint & type-check | `just check` | Executes `cargo check` with full features to catch breakage in either render mode. |
| Produce a WASM bundle | `just build-csr` | Uses `trunk build --release`; expects `trunk` to be installed via `cargo install trunk`. |
| Stream SSR markup | `just run-ssr` | Prints a complete HTML document to stdout using the shared SSR entrypoint. |

All commands run from the example directory so they can be copied into CI
pipelines verbatim. The repository-wide automation layers (`make test`, `cargo
xtask`, etc.) pick up this crate automatically once it is registered in the
workspace manifest.

## Required toolchains

- Rust stable via `rustup`
- `wasm32-unknown-unknown` target (installed automatically by the bootstrap
  script when missing)
- [`trunk`](https://trunkrs.dev/) for WebAssembly bundling
- [`just`](https://github.com/casey/just) command runner

These align with the rest of the RusticUI examples so developers can reuse the
same container images or CI jobs across adapters.

## SSR and hydration expectations

- `cargo run --features ssr --bin layout-grid-yew` emits a full HTML document
  containing the grid showcase. Hydration markers such as
  `data-rustic-layout-grid-root` are stable for automation.
- `wasm-bindgen` based bundles hydrate into the `#layout-grid-root` container
  and promote the hydration phase indicator from `Server` to `Client`. The
  component logs which breakpoint is currently active so operations teams can
  verify viewport detection in synthetic monitors.

## Tests

Unit tests live alongside the source files and assert:

- Breakpoint resolution for each grid section
- SSR markup contains the expected automation hooks when the `ssr` feature is
  enabled

Execute `just test` (or `cargo test --manifest-path Cargo.toml --all-features`)
before opening a pull request.

## CI integration

CI runners should execute:

```bash
just bootstrap
just check
just test
```

The shared bootstrap and Just recipes ensure failures surface early with clear
messages. Because the crate participates in the Cargo workspace, repository-wide
`cargo test --workspace --all-features` automatically exercises these tests as
well.

