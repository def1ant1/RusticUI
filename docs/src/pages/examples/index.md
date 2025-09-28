# Rust example gallery

RusticUI's example suites pair shared headless state machines with framework
adapters, deterministic automation hooks, and scripts that eliminate manual
scaffolding. Every blueprint below documents its bootstrap command, the
automation metadata it emits, and the parity checks you should run before
shipping changes.

## Scaffolding workflow

1. **Select a baseline.** Start from the closest crate under
   [`examples/`](../../../examples). Each README calls out the shared state
   machines, automation identifiers, and CI expectations for that surface.
2. **Run the bundled bootstrap.** Execute the example's script or binary (for
   example `./examples/navigation-tabs-yew/scripts/bootstrap.sh` or
   `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`)
   to materialise a ready-to-run workspace with SSR snapshots, hydration stubs,
   and analytics markers baked in.【F:examples/navigation-tabs-yew/README.md†L20-L33】【F:examples/feedback-tooltips/README.md†L9-L22】
3. **Wire automation into CI.** Validate formatting and compile the workspace via
   `cargo xtask fmt`, `cargo xtask clippy`, and
   `cargo xtask test --examples` so the new blueprint participates in the shared
   Wasm targets and parity checks.【F:crates/xtask/src/main.rs†L49-L70】
4. **Document the flow.** Extend this gallery and the example README with any
   bespoke analytics requirements or parity notes before running
   `cargo xtask build-docs` to refresh the docs site for review.【F:crates/xtask/src/main.rs†L118-L119】

## Marketing microsite suite (`examples/mui-*`)

The Material marketing blueprints compose `mui-shared` layout primitives with
Yew, Leptos, Dioxus, and Sycamore adapters to reproduce the archived React
experience. Each crate documents CSR, SSR, and release builds, while the shared
route descriptors guarantee deterministic automation IDs across frameworks.【F:examples/mui-yew/README.md†L1-L60】

- **Run locally:** `trunk serve --open` for CSR bundles and
  `cargo run --manifest-path examples/mui-*/Cargo.toml --features ssr` for the
  HTML snapshot used in parity audits.【F:examples/mui-yew/README.md†L11-L31】
- **Automation:** Navigation, hero, and showcase sections expose
  `data-rustic-*` selectors sourced from `mui_shared::AutomationIdBuilder` so QA
  suites can diff SSR and CSR output safely.【F:examples/mui-yew/README.md†L62-L70】
- **Parity checks:** `cargo test --package rustic_ui_yew_example` and its sister
  crates assert router wiring, hydration safety, and automation determinism
  before merges.【F:examples/mui-yew/README.md†L33-L39】
- **SSR harness:** `examples/mui-ssr-accessibility` streams the same shell for
  accessibility and release snapshots, then reuses the CSR bundles for hydration
  validation.【F:examples/mui-ssr-accessibility/README.md†L1-L39】

## Navigation drawer suite (`examples/navigation-drawer-*`)

These demos showcase responsive drawers with shared automation metadata, modal
and anchored variants, and ready-to-run bootstrap scripts for every framework.【F:examples/navigation-drawer-yew/README.md†L1-L26】【F:examples/navigation-drawer-leptos/README.md†L1-L18】

- **Automation:** Drawer surface, backdrop, and navigation markup carry
  `aria-modal`, labelled headings, and deterministic data attributes so analytics
  and accessibility audits remain stable.【F:examples/navigation-drawer-yew/README.md†L10-L18】
- **Bootstrap:** Run the framework script (for example
  `./examples/navigation-drawer-yew/scripts/bootstrap.sh`) to generate a Trunk
  workspace with axe-core wiring and exhaustive inline comments.【F:examples/navigation-drawer-yew/README.md†L20-L26】
- **Parity:** Generated projects include routing callbacks and keyboard
  shortcuts to keep CSR and SSR behaviour aligned; review the scaffolded
  README under `target/` to capture any framework-specific extensions.【F:examples/navigation-drawer-yew/README.md†L28-L59】【F:examples/navigation-drawer-leptos/README.md†L7-L18】

## Navigation tabs suite (`examples/navigation-tabs-*`)

Tab demos pair the shared headless `TabsState` with responsive layout options and
framework routers to keep orientation, selection, and analytics markers in sync.【F:examples/navigation-tabs-yew/README.md†L1-L86】

- **Bootstrap:** Execute `./examples/navigation-tabs-*/scripts/bootstrap.sh` to
  emit a documented project with Trunk manifests and hydration stubs.【F:examples/navigation-tabs-yew/README.md†L20-L33】
- **Automation:** Each tab uses the shared render helpers to stamp
  `data-rustic-*` selectors, enabling QA to reuse scenarios across Yew, Leptos,
  Dioxus, and Sycamore.【F:examples/navigation-tabs-yew/README.md†L11-L18】
- **Parity:** The scaffolded code wires router callbacks and axe-core checks so
  teams can assert hydration parity without hand-written harnesses.【F:examples/navigation-tabs-yew/README.md†L35-L86】

## Select menu suite (`examples/select-menu-*`)

Controlled listboxes demonstrate asynchronous data loading, SSR mirroring, and
shared automation attributes centralised in `select-menu-shared`.【F:examples/select-menu-yew/README.md†L1-L26】

- **CSR workflow:** Run `trunk serve --open` to hydrate the server snapshot and
  exercise async loaders with deterministic automation IDs.【F:examples/select-menu-yew/README.md†L18-L26】
- **Headless overrides:** Toggle option availability through `SelectState` to
  verify how ARIA metadata and automation selectors react without touching the
  adapter code.【F:examples/select-menu-yew/README.md†L28-L40】
- **SSR parity:** `cargo run --manifest-path examples/select-menu-*/Cargo.toml --features ssr`
  emits the HTML snapshot consumed by CSR bundles, ensuring parity across
  renderers.【F:examples/select-menu-yew/README.md†L42-L50】

## Selection controls (`examples/selection-controls-*`)

The selection control samples wire checkbox, switch, and radio state machines
into framework adapters via shared render helpers, keeping automation and ARIA
contracts identical across runtimes.【F:examples/selection-controls-yew/README.md†L1-L44】

Use the examples as reference implementations when integrating multiple control
surfaces into dashboards; the snippets highlight how to reuse the headless
machines without duplicating markup.

## Feedback surfaces (`examples/feedback-*`)

Feedback blueprints produce multi-framework SSR snapshots, hydration stubs, and
automation contracts with a single bootstrap command.

- **Chips:** `cargo run --bin bootstrap --manifest-path examples/feedback-chips/Cargo.toml`
  renders dismissible and read-only variants for every framework, including
  deterministic `data-rustic-chip-id` selectors and regression tests that guard
  against automation drift.【F:examples/feedback-chips/README.md†L1-L45】
- **Tooltips:** The tooltip bootstrap emits SSR HTML, hydration starters, and
  portal metadata so analytics and monitoring can bind before hydration. Run the
  bundled tests to confirm shared `data-rustic-tooltip-id` selectors remain
  intact.【F:examples/feedback-tooltips/README.md†L1-L45】

## Data display surfaces (`examples/data-display-*`)

Data display demos illustrate how list, table, and avatar surfaces share headless
state and automation IDs across frameworks.

- **List + table:** `cargo run --package data-display-yew` renders compact list
  and zebra-striped tables with deterministic automation selectors while reusing
  the shared headless state machine.【F:examples/data-display-yew/README.md†L1-L24】
- **Avatar widget:** `cargo run --bin bootstrap --manifest-path examples/data-display-avatar/Cargo.toml`
  builds combined chip/tooltip avatars with themed overrides and shared
  automation hooks, plus regression tests that ensure parity.【F:examples/data-display-avatar/README.md†L1-L35】

## Joy workflows (`examples/joy-*`)

The Joy workflow adapters consume `joy-workflows-core` to expose identical
stepper, snackbar, and analytics behaviour across frameworks.【F:examples/joy-yew/README.md†L1-L38】

- **CSR:** Serve the bundle via `trunk serve --open` (or the equivalent for other
  frameworks) to validate Joy design tokens and lifecycle logging.【F:examples/joy-yew/README.md†L19-L28】
- **SSR snapshot:** `cargo run --manifest-path examples/joy-*/Cargo.toml --features ssr`
  prints deterministic lifecycle summaries for parity verification.【F:examples/joy-yew/README.md†L30-L38】

## Shared dialog state (`examples/shared-dialog-state-*`)

Overlay demos rely on the `shared-dialog-state-core` crate to synchronise dialogs,
popovers, and validation flows across frameworks, with automation hooks ready for
enterprise QA pipelines.【F:examples/shared-dialog-state-yew/README.md†L1-L47】

- **Bootstrap:** Run `./examples/shared-dialog-state-*/scripts/bootstrap.sh` to
  materialise a workspace with SSR markup, hydration hooks, analytics logging,
  and inline documentation for lifecycle events.【F:examples/shared-dialog-state-yew/README.md†L24-L47】
- **Parity:** Deterministic `data-*` attributes (`data-transition`,
  `data-popover-placement`, validation markers) ensure Playwright and Cypress
  suites stay stable as the shared overlay state evolves.【F:examples/shared-dialog-state-yew/README.md†L11-L19】

By following these automation-first workflows and verifying changes through the
shared `cargo xtask` entry points, teams can add or extend examples without
reintroducing manual scaffolding or diverging automation hooks.
