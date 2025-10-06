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
   `cargo xtask fmt`, `cargo xtask clippy`, `cargo xtask test --examples`, and
   `cargo xtask examples --group layout --release` so the new blueprint
   participates in the shared Wasm targets, native layout builds, and parity
   checks.【F:crates/xtask/src/main.rs†L59-L70】【F:crates/xtask/src/main.rs†L439-L576】
4. **Document the flow.** Extend this gallery and the example README with any
   bespoke analytics requirements or parity notes before running
   `cargo xtask docs-build`, `cargo xtask docs-test`, and
   `cargo xtask docs-package --dry-run` to refresh the docs site for review
   without mutating the canonical export directory.【F:crates/xtask/src/main.rs†L150-L200】

## Quick-start CTA playground

Explore the [quick-start button gallery](./quick-start-gallery.md) to preview the
Material CTA that anchors every scaffold. The page pipes the shared generator
directly into Sandpack, StackBlitz, and a JSON snapshot so docs, automation, and
multi-framework adapters all share the same source of truth.【F:docs/src/components/examples/QuickStartButtonGenerator.ts†L1-L187】【F:docs/scripts/quickStartButtonSandbox.ts†L1-L43】【F:docs/data/examples/quick-start-button-sandbox.json†L1-L179】


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

## Navigation controls (`examples/navigation-*-yew/leptos/dioxus`)

Bottom navigation, pagination, and speed dial demos round out the navigation
family with SSR parity, telemetry logging, and automation markers baked in for
Yew, Leptos, and Dioxus adapters. Each README documents the command surface so
teams can hydrate CSR builds, capture SSR fixtures, and stream analytics without
manual wiring.【F:examples/navigation-bottom-navigation-yew/README.md†L9-L47】【F:examples/navigation-pagination-leptos/README.md†L9-L41】【F:examples/navigation-speed-dial-dioxus/README.md†L9-L43】

- **Run locally:** `trunk serve --open` (Yew/Leptos) or `dx serve --open`
  (Dioxus) hydrate the CSR bundles. `cargo run --features ssr` prints the same
  markup used by hydration alongside newline-delimited telemetry logs.【F:examples/navigation-bottom-navigation-yew/README.md†L9-L24】【F:examples/navigation-pagination-leptos/README.md†L9-L24】【F:examples/navigation-speed-dial-dioxus/README.md†L9-L24】
- **Automation:** Every example stamps deterministic `data-rustic-*` selectors
  and analytics IDs, letting QA suites assert behaviour without bespoke DOM
  queries.【F:examples/navigation-bottom-navigation-yew/README.md†L26-L47】【F:examples/navigation-pagination-leptos/README.md†L26-L41】【F:examples/navigation-speed-dial-dioxus/README.md†L26-L43】
- **CI parity:** Invoke `just bootstrap`, `just test`, and `just run-ssr` before
  committing changes, then exercise the full set via
  `cargo xtask examples --group navigation --release` to validate native and
  `wasm32` builds together.【F:examples/navigation-bottom-navigation-yew/README.md†L9-L24】【F:examples/navigation-pagination-leptos/README.md†L9-L24】【F:examples/navigation-speed-dial-dioxus/README.md†L9-L24】【F:crates/xtask/src/main.rs†L439-L516】

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

The selection control suites now ship as runnable crates and JavaScript
workspaces (React) with centralised automation scripts, so you can bootstrap
checkbox, switch, and radio demos without copying inline snippets. Each README
documents the hosted + wasm checks, Trunk/Dioxus servers, telemetry delegates,
and CI entry points in the same order surfaced by the inline comments inside the
examples and scripts.【F:examples/selection-controls-yew/README.md†L1-L60】【F:examples/selection-controls-react/README.md†L21-L75】

> **Note:** The shared `selection-controls-smoke.sh` helper intentionally echoes
> the inline script comments—toolchain provisioning, analytics logging, and the
> canonical automation ID list all live in one place so CI, `just` recipes, and
> Playwright harnesses stay aligned.【F:examples/scripts/selection-controls-smoke.sh†L1-L63】

- **Bootstrap + smoke:** Run `cargo xtask selection-controls` to compile every
  Rust crate (host + `wasm32`), execute the wasm and Playwright smoke tests, and
  print the automation IDs before forwarding the suite to React. The command
  shells out to `examples/scripts/selection-controls-smoke.sh`, so local runs and
  CI reuse the same automation and telemetry orchestration.【F:crates/xtask/src/main.rs†L163-L2378】【F:examples/scripts/selection-controls-smoke.sh†L1-L120】
- **Automation IDs:** Query `examples/scripts/selection-controls-smoke.sh --list-automation --format json`
  (or invoke `just automation-smoke` inside any framework package) to print the
  `automation.selection-controls.*` identifiers stamped into every demo. The
  helper mirrors the inline script comment that keeps QA selectors in sync with
  the emitted DOM contract.【F:examples/scripts/selection-controls-smoke.sh†L32-L63】【F:examples/selection-controls-yew/README.md†L19-L60】
- **Framework servers:** Use the documented `just serve`, `dx serve`, or
  `npm run dev` flows to launch hydration demos with telemetry consoles enabled.
  Each command emits the same newline-delimited analytics payloads captured by
  the smoke harness so you can diff SSR vs CSR logs without bespoke wiring.【F:examples/selection-controls-yew/README.md†L9-L60】【F:examples/selection-controls-react/README.md†L33-L85】

Recent telemetry updates mean every checkbox, switch, **and radio** adapter
(React, Yew, Leptos, Dioxus, and Sycamore) emits a deterministic event stream
**before** user callbacks fire. Each render enters the shared
`instrument_render` span with a `TelemetryContext` that captures the component
path, analytics/automation IDs, and a descriptor snapshot of the rendered
attributes so observability pipelines see exactly which DOM attributes were
generated.【F:crates/rustic-ui-material/src/checkbox.rs†L205-L227】【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1012】【F:crates/rustic-ui-material/src/checkbox.rs†L1126-L1228】【F:crates/rustic-ui-material/src/radio.rs†L1885-L1930】【F:crates/rustic-ui-material/src/telemetry.rs†L22-L78】

All adapters expose matching props: optional `on_change`, `on_focus`, `on_blur`,
`on_key`, and a `telemetry_delegate` hook that receives normalized payloads.
Pointer toggles deliver a single `Change` payload, focus transitions emit
`Focus`/`Blur`, keyboard interactions publish a `Key` event immediately followed
by a `Change`, and radio groups emit an additional `Commit` payload to summarise
the post-mutation selection (including controlled-state context) before consumer
callbacks run.【F:crates/rustic-ui-material/src/checkbox.rs†L928-L1120】【F:crates/rustic-ui-material/src/checkbox.rs†L1333-L1463】【F:crates/rustic-ui-material/src/radio.rs†L2633-L2709】【F:crates/rustic-ui-material/tests/checkbox_adapters.rs†L210-L299】【F:crates/rustic-ui-material/tests/radio_adapters.rs†L188-L260】

Use the examples as reference implementations when integrating multiple control
surfaces into dashboards. Start by seeding `TelemetryHooks` with analytics and
automation identifiers (plus optional `on_render`/`on_error` callbacks) and pass
a framework-specific telemetry delegate that forwards the normalized payloads to
your analytics sink before executing local business logic.【F:crates/rustic-ui-material/src/checkbox.rs†L1-L78】【F:crates/rustic-ui-material/src/telemetry.rs†L132-L189】

The standalone [selection control telemetry walkthrough](./selection-controls-telemetry.md)
now includes copy-and-paste snippets for registering handlers in each supported
adapter, plus references back to the smoke harness comments so the inline docs
and walkthrough stay synchronized.

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
- **Rating:** `cargo run --package feedback-rating-shared` prints the Material
  rating markup generated by the new headless state machine so React, Yew,
  Leptos, Dioxus, and Sycamore adapters can hydrate the same SSR snapshot while
  preserving analytics channels and automation identifiers.【F:examples/feedback-rating-shared/README.md†L1-L15】【F:examples/feedback-rating-shared/src/main.rs†L1-L22】

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

## Material workflows (`render_stepper`)

`render_stepper` in `rustic_ui_material` now exposes the same automation-rich
metadata used by the Joy examples, allowing Material experiences to wire Playwright
and Cypress suites directly against shared selectors.  Run
`cargo test -p rustic-ui-material stepper` to exercise the renderer and adapter
parity checks before shipping workflow updates.【F:crates/rustic-ui-material/src/stepper.rs†L1-L318】

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

## Automation-focused blueprints

The `automation` example group exercises the headless click-away, focus trap, and
telemetry utilities alongside the Material renderers and adapters:

- Review [Automation blueprints](./automation.md) for an end-to-end checklist of
  commands, telemetry expectations, and diagnostic hooks before running CI.
- Run `cargo xtask examples --group automation --release` to regenerate the
  shared SSR snapshots, telemetry ndjson, and hydration manifests that back the
  observability workflows.【F:crates/rustic-ui-headless/README.md†L297-L305】
- Inspect `target/rustic-ui-automation/automation-events.ndjson` after the run to
  validate that click-away dismissals, focus trap transitions, and snackbar queue
  events are logged for every framework adapter.【F:crates/rustic-ui-material/README.md†L268-L275】
- Feed the drained telemetry into your enterprise monitoring stack using the
  adapter helpers described in the Material README so browser, server, and test
  harnesses share the same automation contract.【F:crates/rustic-ui-material/README.md†L255-L267】
