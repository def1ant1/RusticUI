<!-- #host-reference -->
<!-- markdownlint-disable-next-line -->
<p align="center">
  <a href="https://apotheon.ai/rusticui" rel="noopener" target="_blank"><img width="160" height="160" src="https://apotheon.ai/assets/rusticui-mark.svg" alt="RusticUI logo"></a>
</p>

<h1 align="center">RusticUI</h1>

<div align="center">

Rust-first, enterprise-grade UI components stewarded by the Apotheon.ai open-source collective.

</div>

RusticUI continues the Material UI for Rust initiative with a renewed focus on automation, observability, and platform-neutral
component APIs. Every crate in this workspace is designed for WebAssembly targets today and native rendering tomorrow, while the
project governance lives fully in the open under the Apache-2.0/MIT dual license.

## Quick start

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Alias the current crates to the new RusticUI namespace:

```bash
cargo add mui-material --rename rustic_ui
```

Render a button with Leptos using the RusticUI alias:

```rust
use leptos::*;
use rustic_ui::Button;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <Button>"Welcome to RusticUI"</Button> })
}
```

Run an example with the Yew adapter:

```bash
cargo run --package mui-yew --example hello_world
```

## Design system automation with `css_with_theme!`

Enterprise teams demand consistent design tokens without repetitive wiring. The RusticUI theming macros automatically inject the
active [`Theme`](crates/mui-styled-engine/src/theme.rs) into scoped CSS blocks, keeping palettes, spacing, and elevation centralized.

```rust
use leptos::*;
use rustic_ui::Button;
use mui_styled_engine::css_with_theme; // Rename to `rustic_ui_styled_engine` once the crates are published

#[component]
fn SaveButton() -> impl IntoView {
    // One macro call taps into palette, spacing and shape tokens. Centralizing this logic avoids
    // copy/paste configuration across dozens of crates.
    let style = css_with_theme!(
        r#"
        background-color: ${theme.palette.primary.main};
        color: ${theme.palette.primary.contrast_text};
        padding: ${theme.spacing(1.5)} ${theme.spacing(3)};
        border-radius: ${theme.shape.border_radius}px;
        "#,
    );

    view! {
        // Attach the scoped class plus ARIA and automation hooks. Screen readers and QA pipelines now share
        // the same semantic contract.
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

## Workspace automation

Automation is consolidated in the root `Makefile` and `cargo xtask` binary so teams can wire CI once and scale confidently.

```bash
make build    # compile every crate
make test     # run workspace tests
make doc      # generate API docs
```

For fine-grained routines the repository exposes a companion CLI via `cargo xtask`, codifying repeatable maintenance in a single
binary:

```bash
cargo xtask scaffold-component    # scaffold a fully-instrumented component package
cargo xtask refresh-icons         # pull the latest Rustic icon sets
cargo xtask accessibility-audit   # run Playwright accessibility tests
cargo xtask build-docs            # build the documentation site
```

Each task emits verbose logs and returns a non-zero exit code on failure so it can be safely wired into CI pipelines.

## Workspace layout

The workspace is organized under the `crates/` directory:

- `mui-system` – styling primitives (will be published as `rustic-ui-system`).
- `mui-headless` – framework-agnostic component state machines.
- `mui-material` – Material-inspired components during the transition period.
- `mui-icons-material` – SVG icon bindings being retooled for Rustic iconography.
- `mui-lab` – experimental widgets under active development.

## Migration and ecosystem alignment

RusticUI builds on the lessons from the React/TypeScript ecosystem while embracing idiomatic Rust patterns. Teams migrating from
React can progressively introduce RusticUI via Web Components or microfrontends backed by WebAssembly.

During the rebranding phase the previous JavaScript instructions are archived in `docs/archives/material-ui.md`. They are preserved
for reference only and no longer maintained.

## Community and support

RusticUI operates in the open. Join the discussions, roadmap reviews, and RFCs at [Apotheon.ai Discussions](https://github.com/apotheon-ai/rusticui/discussions)
and keep track of the migration backlog in [`CHANGELOG.md`](CHANGELOG.md).

Commercial support, design partnerships, and managed hosting options are available through [Apotheon.ai](https://apotheon.ai/contact).
