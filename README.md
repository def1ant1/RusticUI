<!-- #host-reference -->
<!-- markdownlint-disable-next-line -->
<p align="center">
  <a href="https://mui.com/core/" rel="noopener" target="_blank"><img width="150" height="133" src="https://mui.com/static/logo.svg" alt="Material UI logo"></a>
</p>

<h1 align="center">Material UI (Rust)</h1>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/mui-material.svg)](https://crates.io/crates/mui-material)
[![docs.rs](https://docs.rs/mui-material/badge.svg)](https://docs.rs/mui-material)
[![Rust CI](https://github.com/mui/mui-rust/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/mui/mui-rust/actions/workflows/rust-ci.yml)
[![Coverage Status](https://img.shields.io/codecov/c/github/mui/mui-rust.svg)](https://app.codecov.io/gh/mui/mui-rust)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

</div>

Material UI for Rust brings the popular component library to the Rust and WebAssembly ecosystem. It mirrors the React version while embracing idiomatic Rust patterns and a fully automated toolchain.

## Quick start

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Add the Material UI crate:

```bash
cargo add mui-material
```

Render a button with Leptos:

```rust
use leptos::*;
use mui_material::Button;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <Button>"Hello from Rust"</Button> })
}
```

Run an example:

```bash
cargo run --package mui-yew --example hello_world
```

## Workspace layout

The workspace is organized under the `crates/` directory:

- `mui-system` – styling primitives.
- `mui-material` – Material Design components.
- `mui-icons-material` – SVG icon bindings.
- `mui-lab` – unstable widgets under active development.

Automation is consolidated in the root `Makefile`:

```bash
make build    # compile all crates
make test     # run workspace tests
make doc      # generate API docs
```

These targets minimize manual steps and offer a single entry point for local development and CI.

For end-to-end style orchestration that plays well with build pipelines and CI, see [docs/styled-engine/automation.md](docs/styled-engine/automation.md).

## Migrating from React/TypeScript

Teams moving from React or TypeScript can leverage familiar patterns:

- Frameworks like [Yew](https://yew.rs) and [Leptos](https://leptos.dev) offer JSX-like syntax.
- `wasm-bindgen` bridges existing JS libraries when necessary.
- Progressive migration is possible: render Rust widgets inside a React app via Web Components.

See the example in `examples/mui-yew` for an end-to-end WASM setup.

## Legacy JavaScript guidance

The original React/TypeScript instructions are preserved for historical context in [docs/legacy-js.md](docs/legacy-js.md).
