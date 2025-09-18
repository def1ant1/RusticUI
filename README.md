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

## Theming automation with `css_with_theme!`

Enterprise teams demand consistent design tokens without repetitive wiring.
`css_with_theme!` automatically injects the active workspace
[`Theme`](crates/mui-styled-engine/src/theme.rs) inside a scoped CSS block so
styling remains centralized. The example below renders a Leptos button that
consumes the generated class and publishes ARIA metadata for assistive tech:

```rust
use leptos::*;
use mui_material::Button;
use mui_styled_engine::css_with_theme;

#[component]
fn SaveButton() -> impl IntoView {
    // One macro call taps into palette, spacing and shape tokens. Centralizing
    // this logic avoids copy/paste configuration across dozens of crates.
    let style = css_with_theme!(
        r#"
        background-color: ${theme.palette.primary.main};
        color: ${theme.palette.primary.contrast_text};
        padding: ${theme.spacing(1.5)} ${theme.spacing(3)};
        border-radius: ${theme.shape.border_radius}px;
        "#,
    );

    view! {
        // Attach the scoped class plus ARIA and automation hooks. Screen
        // readers and QA pipelines now share the same semantic contract.
        <Button
            class=style.get_class_name()
            aria-label="save file"
            data-automation="primary-save-action"
        >
            "Save"
        </Button>
    }
}
```

Centralized theme tokens keep palettes, spacing and elevations synchronized
across microfrontends while the ARIA attributes guarantee accessible markup.
The additional `data-automation` hook encourages robust end-to-end tests,
meeting enterprise expectations for compliance, observability and maintainable
automation.

## Cargo features

The workspace crates disable most features by default so applications pull in
only the components or icons they use. Consult
[docs/cargo-features.md](docs/cargo-features.md) for a breakdown of available
flags and example `Cargo.toml` snippets.

## Workspace layout

The workspace is organized under the `crates/` directory:

- `mui-system` – styling primitives.
- `mui-headless` – framework-agnostic component state machines used by adapters.
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

For fine-grained routines the repository exposes a small companion CLI via
`cargo xtask`. It mirrors the approach used in many large Rust workspaces by
codifying repeatable maintenance in a single binary. Common subcommands
include:

```bash
cargo xtask update-components   # regenerate component metadata from source
cargo xtask refresh-icons       # pull the latest Material icons
cargo xtask accessibility-audit # run Playwright accessibility tests
cargo xtask build-docs          # build the documentation site
```

Each task emits verbose logs and returns a non-zero exit code on failure so it
can be safely wired into CI pipelines.

For end-to-end style orchestration that plays well with build pipelines and CI, see [docs/styled-engine/automation.md](docs/styled-engine/automation.md).

## Migrating from React/TypeScript

Teams moving from React or TypeScript can leverage familiar patterns:

- Frameworks like [Yew](https://yew.rs) and [Leptos](https://leptos.dev) offer JSX-like syntax.
- `wasm-bindgen` bridges existing JS libraries when necessary.
- Progressive migration is possible: render Rust widgets inside a React app via Web Components.

See the example in `examples/mui-yew` for an end-to-end WASM setup.

## Legacy JavaScript guidance

The original React/TypeScript instructions are preserved for historical context in [docs/legacy-js.md](docs/legacy-js.md).
