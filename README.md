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
cargo xtask icon-update           # pull the latest Rustic icon sets
cargo xtask accessibility-audit   # run Playwright accessibility tests
cargo xtask build-docs            # build the documentation site
```

Each task emits verbose logs and returns a non-zero exit code on failure so it can be safely wired into CI pipelines.

The Material icon updater persists ETag/Last-Modified metadata in
`target/.icon-cache` so repeated runs skip unnecessary downloads. To bypass the
cache or test alternate archives, call the binary directly:

```bash
cargo run -p mui-icons-material --features update-icons --bin update_icons -- \
  --force-refresh --source-url https://internal.example/icons.zip
```

### Rust CI quick reference

The Rust workspace CI expects every pull request to validate the full adapter matrix locally. Install Chrome/Chromium plus `wasm-pack` (see [`docs/rust-ci.md`](docs/rust-ci.md) for setup) and run:

```bash
cargo test --workspace --all-features
cargo xtask wasm-test
cargo test -p mui-material --test joy_yew --features yew
cargo test -p mui-material --test joy_leptos --features leptos
cargo test -p mui-material --test joy_dioxus --features dioxus
cargo test -p mui-material --test joy_sycamore --features sycamore
```

Use the parity suites above to chase snapshot mismatches: rerun the failing test with `-- --nocapture --exact` to inspect the React versus adapter markup before refreshing fixtures or renderers. The [Rust CI guide](docs/rust-ci.md) documents deeper troubleshooting steps, snapshot refresh flows, and coverage tooling so teams can keep automation green without guesswork.

## Select and menu reference implementations

The `examples/select-menu-*` packages provide production-ready blueprints for
accessible listboxes with deterministic automation hooks. Each example shares
mock data loaders, theme overrides, and controlled state helpers via the
[`select-menu-shared`](examples/select-menu-shared) crate so teams can lift the
same primitives into any framework without drift.【F:examples/select-menu-shared/src/lib.rs†L1-L114】

Key capabilities showcased in the examples:

- **Asynchronous loading** – the shared `fetch_regions` helper yields to the
  runtime before returning datacenter records so both SSR and CSR flows exercise
  loading states.【F:examples/select-menu-shared/src/lib.rs†L34-L68】
- **Controlled props** – every renderer uses `controlled_state` to keep the
  open flag and selected option owned by the host application, mirroring the
  expectations of enterprise analytics and RBAC pipelines.【F:examples/select-menu-shared/src/lib.rs†L70-L89】
- **Automation ready markup** – deterministic `data-automation` attributes flow
  from `AUTOMATION_ID`, giving QA suites stable selectors across SSR and
  hydration.【F:examples/select-menu-shared/src/lib.rs†L16-L32】【F:examples/select-menu-yew/src/main.rs†L69-L106】
- **Theme overrides** – both demos wrap the select in the high contrast
  `enterprise_theme` so accessibility palettes stay consistent across surface
  areas.【F:examples/select-menu-shared/src/lib.rs†L91-L109】【F:examples/select-menu-leptos/src/main.rs†L13-L84】【F:examples/select-menu-yew/src/main.rs†L96-L132】

The `select-menu-shared` crate documents every helper and provides the entry
point for new frameworks to consume the renderer, summary generator, and theme
overrides without reimplementing any plumbing.【F:examples/select-menu-shared/README.md†L1-L27】 Both framework demos
hydrate client-side while reusing the same SSR shell emitted by
`render_select_markup`, guaranteeing that automation and accessibility hooks
stay in sync across environments.【F:examples/select-menu-yew/src/main.rs†L137-L157】【F:examples/select-menu-leptos/src/main.rs†L103-L123】

## Data display blueprints

Material themed data display components ship alongside the interactive widgets.
The [`list`](crates/mui-material/src/list.rs) renderer exposes density and
typography variants while emitting deterministic automation hooks for every
item.【F:crates/mui-material/src/list.rs†L1-L355】 The [`table`](crates/mui-material/src/table.rs) module layers column
metadata, zebra striping, and numeric alignment on top of the same headless
state machine so selectable rows behave consistently.【F:crates/mui-material/src/table.rs†L1-L356】 Cookbook examples for
Yew and Leptos live under `examples/data-display-*` and can be run with
`cargo run --package data-display-yew` or
`cargo run --package data-display-leptos --features csr` respectively.【F:examples/data-display-yew/README.md†L1-L21】【F:examples/data-display-leptos/README.md†L1-L21】 The
new [`data-display-avatar`](examples/data-display-avatar) blueprint combines the
chip and tooltip renderers into a reusable presence widget complete with SSR
snapshots and hydration stubs for every supported framework.【F:examples/data-display-avatar/README.md†L1-L26】

## Feedback primitives blueprints

Contextual help and escalation cues now have dedicated blueprints under
`examples/feedback-*`. `feedback-tooltips` renders automation-aware tooltips
with portal metadata and emits hydration stubs for Yew, Leptos, Dioxus, and
Sycamore in a single command.【F:examples/feedback-tooltips/README.md†L1-L30】 `feedback-chips` pairs dismissible and
read-only chips across the same frameworks so QA suites can verify hover and
deletion affordances without re-authoring markup.【F:examples/feedback-chips/README.md†L1-L30】 Both bootstraps are backed by
Rust libraries with unit tests ensuring the generated HTML stays in sync as
`mui-headless` evolves.【F:examples/feedback-tooltips/src/lib.rs†L1-L86】【F:examples/feedback-chips/src/lib.rs†L1-L95】

### Running the demos

Each package ships with a README describing CSR development flows and the SSR
smoke tests that CI executes. Locally you can validate the examples with:

```bash
cargo check --target wasm32-unknown-unknown --manifest-path examples/select-menu-yew/Cargo.toml
cargo run --manifest-path examples/select-menu-yew/Cargo.toml --no-default-features --features ssr
cargo check --target wasm32-unknown-unknown --manifest-path examples/select-menu-leptos/Cargo.toml
cargo run --manifest-path examples/select-menu-leptos/Cargo.toml --no-default-features --features ssr
```

The dedicated CI job mirrors these commands so regressions in the shared
renderers or framework integrations are caught immediately.【F:.github/workflows/rust-ci.yml†L120-L150】

The Yew variant wires the shared renderer into a component while exposing
explicit controls for toggling the popover and cycling the selected region:

```rust
let props = props_from_options("Primary replication region", AUTOMATION_ID, &*options);
let state = controlled_state(props.options.len(), *selected, *open);
let html = mui_material::select::yew::render(&props, &state);
Html::from_html_unchecked(AttrValue::from(html));
```

The Leptos example mirrors the pattern with `RwSignal`s, ensuring the same state
machine drives both SSR and CSR paths without duplicating logic.【F:examples/select-menu-leptos/src/main.rs†L13-L84】

## Workspace layout

The workspace is organized under the `crates/` directory:

- `mui-system` – styling primitives (will be published as `rustic-ui-system`).
- `mui-headless` – framework-agnostic component state machines.
  - Checkbox primitives expose a tri-state (`Off`/`On`/`Indeterminate`) toggle
    API with ARIA-compliant metadata (`aria-checked="mixed"` plus a
    `data-indeterminate` hook) so adapters can animate complex selection flows
    without reimplementing keyboard orchestration.
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
