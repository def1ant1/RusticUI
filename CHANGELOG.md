# RusticUI changelog

RusticUI documents every step of the transition from Material UI for Rust to the Apotheon.ai–stewarded RusticUI platform. The
archived Material UI change history now lives in [`docs/archives/material-ui-changelog.md`](docs/archives/material-ui-changelog.md).

## Unreleased – Selection control automation harness

### Highlights

- Published full documentation for the runnable selection control crates,
  pointing contributors at the central smoke script, framework task runners, and
  xtask entry point so automation stays consistent across languages.【F:docs/src/pages/examples/index.md†L111-L148】【F:docs/src/pages/examples/selection-controls-telemetry.md†L1-L40】【F:docs/architecture/selection-controls.md†L118-L164】
- Refreshed the example catalog to link to the new bootstrap commands, telemetry
  logging expectations, and automation ID governance instead of embedding ad-hoc
  snippets.【F:examples/README.md†L118-L123】
- Added changelog guidance for adopters summarising the selection control
  adapters, smoke harness, and xtask integration so migration steps surface in a
  single location.【F:CHANGELOG.md†L8-L24】

### Verification

- Documentation only change (no runtime impact).

## Unreleased – Leptos radio attribute forwarding

### Highlights

- Documented the new `SelectionControlTelemetry` and `TelemetryHooks` helpers so
  adapter authors can integrate the builder workflow, centralise enterprise
  telemetry, and enforce managed analytics/automation identifiers across
  renderers. 【F:README.md†L170-L209】【F:docs/architecture/selection-controls.md†L73-L127】
- Expanded inline rustdoc on the telemetry helpers to guide future maintainers
  when evolving builder invariants or adding lifecycle hooks for adapters.
  【F:crates/rustic-ui-material/src/selection_control.rs†L334-L368】【F:crates/rustic-ui-material/src/telemetry.rs†L198-L205】
- Expanded the selection control builder documentation to cover keyboard
  navigation, focus-visible semantics, telemetry hooks, SSR/hydration
  guarantees, and enterprise bootstrap patterns shared across checkbox, radio,
  and switch adapters.
- Extended the Leptos radio adapter to automatically spread any extra themed
  attributes (such as inline `style` overrides) captured from the descriptor so
  automation suites no longer need manual updates when design tokens evolve.
  【F:crates/rustic-ui-material/src/radio.rs†L383-L433】【F:crates/rustic-ui-material/src/radio.rs†L2268-L2374】
- Documented the forwarding contract in the telemetry guide so developers know
  Leptos keeps new automation hooks in sync with other frameworks by default.
  【F:docs/src/pages/examples/selection-controls-telemetry.md†L70-L92】
- Mirrored the automation-friendly attribute spreading inside the Sycamore
  radio adapter, interning descriptor keys once and reusing Sycamore's
  `..attrs` spread so future style/class/data additions land without touching
  the adapter source. 【F:crates/rustic-ui-material/src/radio.rs†L3879-L3940】【F:crates/rustic-ui-material/src/radio.rs†L4205-L4277】

### Verification

- `cargo xtask accessibility-audit`
- `cargo fmt`
- `cargo test -p rustic-ui-material --lib --features leptos`
- `cargo test -p rustic-ui-material --lib --features sycamore`

## 2025-07-19 – React radio telemetry orchestration

### Highlights

- Wrapped the React radio adapter with centralized option handler builders so telemetry emissions (`analytics`, `focus`, `blur`, `change`, `commit`) occur before shared state mutations and consumer callbacks, mirroring the checkbox/switch governance guarantees.【F:crates/rustic-ui-material/src/radio.rs†L480-L835】
- Added wasm-bindgen tests that exercise uncontrolled/controlled transitions, telemetry ordering, and attribute preservation for React radio groups to keep regressions from landing unnoticed.【F:crates/rustic-ui-material/src/radio.rs†L700-L803】
- Documented the new React radio callbacks and CI command so teams wire telemetry delegates consistently across frameworks.【F:docs/src/pages/examples/selection-controls-telemetry.md†L239-L309】【F:docs/rust-ci.md†L104-L111】

### Verification

- `cargo fmt`
- `cargo test -p rustic-ui-material --lib --features react` *(fails on host because the React adapters depend on wasm-only `wasm-bindgen` APIs; run `wasm-pack test --headless --chrome -- --no-default-features --features react` once the toolchain is available).*
- `wasm-pack test --node crates/rustic-ui-material -- --no-default-features --features react` *(tool unavailable in container; see docs for CI instructions).*

## 2025-07-12 – InputBase automation blueprints

### Highlights

- Added an InputBase developer guide detailing analytics hooks, SSR guidance,
  and migration tasks so adopters can move from bespoke inputs to the shared
  state machine without guesswork.【F:docs/data/material/guides/input-base/input-base.md†L1-L75】
- Shipped `forms-input-base-*` examples for Dioxus, Leptos, Sycamore, and Yew,
  each reusing the centralised shared crate to emit deterministic
  `data-rustic-input-base-*` selectors, hydration notes, and SSR bootstrap
  assets.【F:examples/forms-input-base-yew/src/lib.rs†L1-L170】【F:examples/forms-input-base-leptos/src/lib.rs†L1-L170】【F:examples/forms-input-base-dioxus/src/lib.rs†L1-L86】【F:examples/forms-input-base-sycamore/src/lib.rs†L1-L94】【F:examples/forms-input-base-shared/src/lib.rs†L1-L199】
- Extended `cargo xtask examples` with a `forms` group so CI and local runs
  compile the new blueprints for native and `wasm32-unknown-unknown` targets via
  a single managed entry point.【F:crates/xtask/src/main.rs†L68-L110】【F:crates/xtask/src/main.rs†L523-L572】

### Verification

- `cargo fmt`
- `cargo clippy --workspace --all-targets --all-features`
- `cargo test --workspace --all-features`
- `cargo xtask examples --group forms --release`

## 2025-07-05 – Multi-adapter primitive guards

### Highlights

- Hardened the `rustic-ui-system` primitives (`Box`, `Container`, `Grid`,
  `Stack`, and `Typography`) so Yew and Leptos adapters now expose explicit
  framework-qualified aliases while avoiding duplicate `pub use` collisions when
  both features are enabled.【F:crates/rustic-ui-system/src/box.rs†L348-L360】【F:crates/rustic-ui-system/src/lib.rs†L28-L63】
- Added a `cargo check -p rustic-ui-system --features "yew leptos"` guard to the
  `cargo xtask test` automation, ensuring CI instantly detects multi-feature
  regressions introduced by new primitives or re-export changes.【F:crates/xtask/src/main.rs†L254-L285】
- Documented the new guard in the Rust CI guide so contributors run the
  multi-adapter build matrix locally before submitting patches.【F:docs/rust-ci.md†L45-L56】
- Introduced the headless-driven app bar workflow: `AppBarState` centralises
  automation and analytics identifiers while the Material adapters, framework
  shims, and new `surfaces-app-bar-yew` example consume the shared builders for
  deterministic SSR output.【F:crates/rustic-ui-headless/src/app_bar.rs†L1-L220】【F:crates/rustic-ui-material/src/app_bar.rs†L1-L248】【F:examples/surfaces-app-bar-yew/src/lib.rs†L1-L20】

### Verification

- `cargo fmt`
- `cargo clippy --workspace --all-targets -D warnings`
- `cargo check -p rustic-ui-system --features "yew leptos"`
- `cargo test --workspace --all-features`

## 2025-06-28 – Experimental focus loop instrumentation

### Highlights

- Added `unstable_trap_focus` to `rustic-ui-headless`, layering loop counters,
  direction metadata, and observer hooks on top of the stable focus trap so
  enterprise teams can trial advanced analytics without forking renderers.【F:crates/rustic-ui-headless/src/unstable_trap_focus.rs†L1-L228】
- Implemented Material renderers and multi-framework adapters that reuse the
  existing sentinel helpers while emitting `data-rustic-focus-loop-*` telemetry
  attributes and detailed ARIA documentation for assistive tech.【F:crates/rustic-ui-material/src/unstable_trap_focus.rs†L1-L246】
- Documented the migration plan in the headless README and modal guidelines so
  adopters know how to enable the `unstable` feature today and pivot back to the
  stable focus trap once the instrumentation graduates.【F:crates/rustic-ui-headless/README.md†L63-L104】【F:docs/data/material/components/modal/modal.md†L94-L102】

### Verification

- `cargo fmt`
- `cargo test --workspace --all-features`

## 2025-06-20 – Navigation example automation

### Highlights

- Shipped enterprise-ready bottom navigation (Yew), pagination (Leptos), and
  speed dial (Dioxus) examples with exhaustive inline documentation, telemetry
  hooks, and SSR harnesses to exercise the new navigation primitives end-to-end.【F:examples/navigation-bottom-navigation-yew/README.md†L1-L47】【F:examples/navigation-pagination-leptos/README.md†L1-L41】【F:examples/navigation-speed-dial-dioxus/README.md†L1-L43】
- Wired the trio into `cargo xtask examples --group navigation --release` so CI
  and local runs compile native + `wasm32` targets together without bespoke
  scripts.【F:crates/xtask/src/main.rs†L439-L516】
- Added snapshot-based SSR tests for each demo to guard telemetry and markup
  regressions in future releases.【F:examples/navigation-bottom-navigation-yew/tests/ssr.rs†L1-L7】【F:examples/navigation-pagination-leptos/tests/ssr.rs†L1-L7】【F:examples/navigation-speed-dial-dioxus/tests/ssr.rs†L1-L7】

### Verification

- `cargo fmt`
- `INSTA_UPDATE=always cargo test --manifest-path examples/navigation-bottom-navigation-yew/Cargo.toml --all-features`
- `INSTA_UPDATE=always cargo test --manifest-path examples/navigation-pagination-leptos/Cargo.toml --all-features`
- `INSTA_UPDATE=always cargo test --manifest-path examples/navigation-speed-dial-dioxus/Cargo.toml --all-features`
- `cargo xtask examples --group navigation --release`

## 2025-06-10 – Navigation primitives and analytics alignment

### Highlights

- Added headless state machines for bottom navigation, breadcrumbs, link,
  pagination, and speed dial widgets with controlled/uncontrolled flows,
  deterministic keyboard handling, and structured analytics payloads.
- Implemented Material renderers plus React, Yew, Leptos, Sycamore, and Dioxus
  adapters for each primitive, emitting automation-focused data attributes and
  SSR-safe inline styles that reuse the shared helper modules.
- Expanded the Rust integration test suite with navigation-focused coverage so
  telemetry hooks and accessibility contracts stay locked across frameworks and
  server-side rendering pipelines.

### Verification

- `cargo fmt`
- `cargo test -p rustic-ui-headless --test navigation_primitives`
- `cargo test -p rustic-ui-material --lib`

## 2025-06-02 – Headless utility expansion and observability rails

### Highlights

- Published automation-friendly headless utilities for click-away listeners,
  focus traps, ARIA transitions, and global event telemetry so downstream
  renderers can reuse the deterministic orchestration without copying
  lifetimes or analytics wiring.
- Landed Material renderers and multi-framework adapters (Yew, Leptos,
  Dioxus, Sycamore) for the new utilities, consolidating shared markup helpers
  and documenting the portal/backdrop lifecycle so SSR and hydration stay in
  lockstep.
- Refreshed the example gallery with blueprint updates that showcase the
  utilities in automation harnesses, including deterministic logging hooks and
  environment variable toggles for enterprise monitoring stacks.

### Verification

- `cargo fmt`
- `cargo clippy --workspace --all-targets -D warnings --all-features`
- `cargo test --workspace --all-features`
- `cargo test --workspace --all-features --target wasm32-unknown-unknown`
- `cargo xtask examples --group automation --release`

## 2025-05-25 – Feedback primitives automation harnesses

### Highlights

- Delivered headless state machines for enterprise feedback and skeleton
  primitives (`form_control`, `input_adornment`, `alert`, `backdrop`,
  `circular_progress`, `linear_progress`, `skeleton`) with controlled and
  uncontrolled modes plus automation identifiers for integration harnesses.
- Implemented Material renderers for the new primitives alongside the long
  awaited slider renderer, consolidating shared HTML routines in
  `render_helpers.rs` and documenting ARIA expectations inline.
- Added Yew and Leptos automation examples that compile on both host and wasm
  targets so CI can smoke test SSR output without manual QA.

### Verification

- `cargo fmt`
- `cargo clippy --workspace --all-targets -D warnings --all-features`
- `cargo test --workspace --all-features`
- `cargo test --workspace --all-features --target wasm32-unknown-unknown`

## 2025-05-20 – Responsive layout regression harness

### Highlights

- Added snapshot-driven coverage for the new `Box`, `Container`, `Grid`, `Stack`,
  `Hidden`, and `ImageList` headless state machines under
  `crates/rustic-ui-headless/tests/layout_primitives.rs`, guaranteeing
  deterministic breakpoint behaviour and automation hooks across viewports.
- Mirrored the coverage in `crates/rustic-ui-material/tests/layout_renderers.rs`
  so React, Yew, Leptos, Sycamore, and Dioxus adapters render identical CSS
  variables and inline styles during SSR and hydration.
- Documented the migration workflow in the README and Rust book, including
  guidance on the new `EMPTY_SEGMENTS` helper, accessibility data attributes, and
  CI guardrails (`cargo fmt`, `cargo clippy --workspace --all-features`,
  `INSTA_UPDATE=always cargo test …`, and `cargo xtask build-docs`).

### Backlog

- [ ] Extend the responsive docs with framework-specific code samples once the
  adapters expose high-level JSX/Yew components.

### Verification

- `cargo fmt`
- `cargo clippy --workspace --all-targets -D warnings --all-features`
- `cargo test --workspace --all-features`
- `INSTA_UPDATE=always cargo test -p rustic-ui-headless --test layout_primitives`
- `INSTA_UPDATE=always cargo test -p rustic-ui-material --test layout_renderers`
- `cargo xtask examples --group layout --release`
- `cargo xtask wasm-test`
- `cargo xtask build-docs`

## 2025-05-06 – Supply-chain automation and archive governance

### Highlights

- Finalized the JavaScript package archival plan by wiring the new `deny` make target and xtask guardrail into CI, ensuring the Rust crates and frozen npm snapshots stay coordinated for regulated adopters.
- Added a `cargo xtask deny` subcommand that wraps `cargo deny check` with workspace-aware logging so dependency advisories, license drift, and yanked crates surface alongside the existing `fmt` and `clippy` checks.
- Updated the npm-to-Rust migration guide and contributor playbook to call out the new audit requirement, making it clear that every migration run must finish with a Rust-native supply-chain review.

### Breaking changes

- CI and local workflows now require the `cargo-deny` binary. Downstream pipelines must install the tool (for example via `cargo install cargo-deny --locked`) before invoking `cargo xtask deny`, otherwise the lint stage will fail fast.

### Backlog

- [ ] Automate cargo-deny database caching in CI so nightly runs avoid re-downloading the advisory index on large monorepos.

## 2025-04-22 – RusticUI styling macros only

### Highlights

- Removed the final `@mui/styles` shims (`makeStyles`, `withStyles`, `withTheme`, and `createStyles`) from `@mui/material/styles` so RusticUI depends exclusively on the macro-based styling engine.
- Updated the v4→v5 migration docs, troubleshooting guide, and error-code catalog to direct enterprises to the automated `scripts/migrate-crate-prefix.sh` workflow rather than manual package installs.
- Documented the breaking change across the changelog and upgrade playbooks so downstream teams can schedule codemod runs and CI verification before upgrading.

### Backlog

- [ ] Extend `scripts/migrate-crate-prefix.sh` with a dry-run reporter that lists every remaining JSS artifact before the rewrite executes, making change management sign-off easier for regulated environments.

## 2025-04-01 – Regression harness styling migration guardrails

### Highlights

- Updated the regression Vite harness to stop aliasing the legacy `@mui/styles`
  path, guaranteeing that contributors exercise the RusticUI styling
  toolchain end-to-end while developing fixes.
- Documented the change so downstream consumers can remove any remaining
  compatibility shims and rely solely on the maintained RusticUI styling
  adapters.

### Backlog

- [ ] Wire an automated alert that flags any reintroduction attempts of the
  deprecated alias during review so the guardrail stays enforced.

## 2025-03-25 – GridLegacy removal and Grid v2 consolidation

### Highlights

- Removed the deprecated `@mui/material/GridLegacy` entry point, deleting its implementation, documentation, and tests while
  expanding inline Grid v2 documentation to clarify the streamlined API.
- Updated premium theme showcases, migration guides, and codemod fixtures to demonstrate the modern `Grid` layout patterns and
  direct readers to the automated migration tooling.
- Documented the breaking change across the migration guides and release notes so downstream teams can schedule codemod runs
  and CI validation before upgrading.

### Backlog

- [ ] Evaluate additional codemod coverage for wrapped or styled Grid usages that fall outside the current `grid-props`
  transform.

## 2025-03-18 – Rustic crate rename docs complete

### Highlights

- Updated the top-level README, migration guide, and changelog to reference the
  published `rustic-ui-*` crates directly, replacing the temporary aliasing
  instructions.
- Documented the `compat-mui` feature flag alongside the new
  `scripts/migrate-crate-prefix.sh` helper so downstream workspaces can automate
  import rewrites and lint verification.
- Verified that documentation examples compile against the renamed crates via
  `cargo doc --no-deps`.

### Backlog

- [ ] Expand the migration automation script to toggle crate features per
  framework (Leptos, Sycamore, Dioxus) automatically.

## 2025-03-11 – Navigation orchestration blueprint

### Highlights

- Added a [Navigation orchestration guide](docs/data/material/guides/navigation/navigation.md) consolidating router integration, theming hooks, accessibility, and CI guardrails for Tabs and Drawer deployments.

### Backlog

- [ ] Automate navigation-registry validation across micro-frontend bundles using the shared docs examples.

## 2025-03-04 – Reintroducing RusticUI

### Highlights

- Rebranded the public documentation to RusticUI and Apotheon.ai stewardship.
- Archived legacy Material UI guidance under `docs/archives/` for historical access.
- Established translation, demo scaffolding, and theming guidance aligned with the new automation-first workflow.

### Backlog

- [ ] Publish crates under the `rustic-ui-*` namespace and update all imports accordingly.
- [ ] Replace Material icon assets with the Rustic iconography pipeline.
- [ ] Produce end-to-end migration guides for Leptos, Yew, Dioxus, and Sycamore consumers.
- [ ] Stand up nightly accessibility and visual regression suites tailored to RusticUI branding.
