# RusticUI changelog

RusticUI documents every step of the transition from Material UI for Rust to the Apotheon.ai–stewarded RusticUI platform. The
archived Material UI change history now lives in [`docs/archives/material-ui-changelog.md`](docs/archives/material-ui-changelog.md).

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
