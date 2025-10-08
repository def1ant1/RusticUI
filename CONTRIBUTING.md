# Contributing to RusticUI

Thank you for helping shape RusticUI—the Rust-native evolution of Material UI under the Apotheon.ai open-source program. This
guide outlines the contribution workflow, automation entry points, and expectations for code and documentation changes.

## Code of conduct

RusticUI follows the [Contributor Covenant](https://www.contributor-covenant.org/) (v2.1). Please review the
[`CODE_OF_CONDUCT.md`](https://github.com/apotheon-ai/.github/blob/main/CODE_OF_CONDUCT.md) file in the Apotheon.ai organization
before interacting with the community. Violations can be reported confidentially to [conduct@apotheon.ai](mailto:conduct@apotheon.ai).

## Ways to contribute

RusticUI thrives on a wide range of contributions:

- **Crates & components** – Implement new components, fix bugs, or improve performance in any crate under `crates/`.
- **Documentation & demos** – Expand the docs hosted in `docs/` or add examples in `examples/`. Consult [`examples/README.md`](examples/README.md) for the automation matrix, renderer coverage, and bootstrap commands that each demo must maintain.
- **Automation** – Enhance the `cargo xtask` CLI, CI workflows, and observability pipelines.
- **Community** – Review pull requests, triage issues, or mentor newcomers in the [discussion board](https://github.com/apotheon-ai/rusticui/discussions).

### Join the 2025 contributor experience program

- Fill out the Typeform survey at [`https://form.typeform.com/to/rusticui-cx-2025`](https://form.typeform.com/to/rusticui-cx-2025). Responses sync into the shared insights warehouse and the [`projects/apotheon-ai/rusticui/6`](https://github.com/orgs/apotheon-ai/projects/6) board each night via the `contributor-experience-intake` workflow.
- Prefer GitHub-native tooling? Submit feedback through the [Contributor Experience 2025 Discussion form](https://github.com/apotheon-ai/rusticui/discussions/new?category=contributor-experience-2025); automation mirrors the Typeform schema so no manual triage is required.
- Governance cadence:
  - **Weekly triage (Mondays 16:00 UTC)** – Maintainers review new responses and promote high-signal insights into `cx-survey` issues.
  - **Monthly deep dive (First Thursday)** – Roadmap leads analyze trend dashboards and adjust swimlanes on the project board.
  - **Quarterly retrospective** – Publish a public summary alongside the release retrospective, ensuring enterprise adopters see actionable outcomes.

Before starting large efforts, open a GitHub discussion or issue so the maintainers can align on goals and avoid duplicated work.

## Development setup

### Managed devcontainer and Codespaces workflow

The `.devcontainer` configuration bootstraps a ready-to-ship environment with
Rust, Node 20, pnpm, Playwright dependencies, and the `cargo xtask` CLI suite
preinstalled. Opening the repository in GitHub Codespaces or a local VS Code
Dev Container automatically:

- Mounts persistent caches for Cargo, Playwright, and pnpm so rebuilds reuse the
  same directories across sessions (`/workspaces/.cargo-target`,
  `/workspaces/.cache/ms-playwright`, `/workspaces/.pnpm-store`).【F:.devcontainer/devcontainer.json†L24-L44】【F:.devcontainer/Dockerfile†L15-L60】
- Runs `.devcontainer/scripts/post-create.sh` to install docs dependencies,
  provision Chromium via Playwright, and execute both `cargo xtask
  verify-toolchain` and `cargo xtask dev --dry-run` for a smoke check.【F:.devcontainer/devcontainer.json†L46-L49】【F:.devcontainer/scripts/post-create.sh†L1-L38】
- Launches the unified docs + gallery harness (`cargo xtask dev`) after attach
  so you land in a live hot-reload loop with ports 3000 (Leptos gallery) and
  3100 (Next.js docs) forwarded automatically.【F:.devcontainer/codespaces.json†L5-L23】

You can rerun the validation routines at any time from the integrated tasks
palette—`RusticUI · verify toolchain` invokes `cargo xtask verify-toolchain`
while `RusticUI · dry-run dev harness` replays the combined docs/gallery
bootstrap without starting long-running processes.【F:.devcontainer/codespaces.json†L24-L34】 Capture the output in pull request
summaries to demonstrate parity with the managed environment when iterating on
tooling or docs.

### Manual setup (fallback)

1. Install the latest stable Rust toolchain and ensure `wasm32-unknown-unknown` is available via `rustup target add`.
2. Install supporting CLI tools with Cargo: `cargo install mdbook grcov wasm-pack cargo-deny` (the automation will leverage
   them when present).
3. Run `make bootstrap` to install workspace prerequisites and run quick smoke tests.

> **Important:** The root pnpm workspace has been retired. Do not run `pnpm install` from the repository root and ignore any
> legacy references to `pnpm-lock.yaml` in older documentation. All active automation now flows through `cargo xtask`, and the
> docs site manages its own dependencies locally within `docs/` when needed.

> **Guardrail:** Run `cargo xtask verify-toolchain` whenever you touch repository plumbing or CI configuration. The command
> verifies that the archived Node manifests stay quarantined under `archives/tooling/node-workspace/` and fails fast if a
> `package.json`, `pnpm-workspace.yaml`, `lerna.json`, `nx.json`, or `webpackBaseConfig.js` reappears at the workspace root.
> Keeping this guard green ensures CI only depends on the Rust toolchain and mdBook.【F:archives/tooling/node-workspace/README.md†L1-L11】【F:crates/xtask/src/main.rs†L1-L120】【F:crates/xtask/src/main.rs†L240-L330】

All repetitive chores are encapsulated inside the `Makefile` or `cargo xtask`. Prefer these entry points over ad-hoc scripts.

### Unified error handling

RusticUI crates share a common error vocabulary via the [`rustic-ui-error`](crates/rustic-ui-error) crate. Public APIs should
return `RusticUiResult<T>` (an alias for `Result<T, rustic_ui_error::RusticUiError>`) rather than `anyhow::Result`. Leverage the
provided [`ResultContextExt`](crates/rustic-ui-error/src/lib.rs) trait to attach context instead of `anyhow::Context` so callers
retain structured error variants. When you introduce a new failure domain add a `#[cfg(feature = "...")]` variant to the shared
enum, document the intended usage inline, and extend the unit tests to prove `source()` chains expose the underlying error. CI
verifies these conversions remain lossless by exercising the helpers in `crates/rustic-ui-error` during `cargo test`.

### Documentation hosting pipeline

The documentation site, API reference, and wasm demos ship through the docs subcommands exposed by `cargo xtask`. The
pipeline splits into explicit `docs-build`, `docs-test`, and `docs-package` phases so contributors and CI can validate each
stage independently before staging artifacts in `target/deploy/docs`. Hosting integrations (Netlify, Vercel, GitHub Pages,
internal CDNs) point at that directory without invoking pnpm or bespoke shell scripts. All orchestration code lives in
[`crates/xtask-docs`](crates/xtask-docs/README.md), with the Leptos/SSR implementation residing in [`crates/rustic-docs`](crates/rustic-docs/README.md) for deeper context.

Key commands and required tooling:

- `cargo xtask docs-build` – Compiles the docs server binary and wasm bundle in parallel, reusing the shared `CARGO_TARGET_DIR` cache. Install [`mdBook`](https://rust-lang.github.io/mdBook/) and [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) via Cargo so the helper can render the Rust book and run `wasm-bindgen` without fallback scripts.
- `cargo xtask docs-test` – Runs the wasm smoke tests in headless Chromium via Playwright. Ensure `npx playwright install --with-deps chromium` and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) are available locally. Cache the browsers by exporting `PLAYWRIGHT_BROWSERS_PATH=0` (the repository default) or pointing the variable at a shared directory in CI to avoid repeated downloads.
- `cargo xtask docs-package --dry-run` – Executes the release packaging flow without mutating the canonical export directory. Useful in CI pull requests and local spot checks. Drop `--dry-run` once the staged payload looks correct to mirror production deploys.

Environment overrides:

- `RUSTIC_DOCS_EXPORT_DIR` – Override the default staging directory (`target/deploy/docs`) when CI needs a dedicated artifact volume.
- `RUSTIC_DOCS_TRACING_ENDPOINT` – Surface SSR/CSR telemetry destinations to the `rustic-docs` binaries without recompiling.
- `RUSTIC_UI_DEPLOY_PROFILE` / `RUSTIC_UI_DEPLOY_GROUPS` – Remain available when invoking the legacy `cargo xtask deploy-docs` wrapper for teams that have not yet migrated bespoke automation.

The root `Makefile` exposes matching entry points via `make docs-build`, `make docs-test`, and `make docs-package` (alongside the
backward-compatible `make deploy-docs`). Invoke them locally before modifying Netlify/Vercel configuration so reviewers can
inspect the staged output.

### Quick-start scaffolds and verification

RusticUI maintains automation-first quick-start blueprints across Yew, Leptos, Dioxus, Sycamore, and React so every framework
shares the same telemetry, SSR snapshots, and automation selectors. The gallery in
[`docs/src/pages/getting-started/quick-start.md`](docs/src/pages/getting-started/quick-start.md) maps each scaffold to its
bootstrap script and follow-up smoke tests, keeping enterprise rollouts reproducible without bespoke glue code.【F:docs/src/pages/getting-started/quick-start.md†L1-L136】

- Run `cargo xtask quick-start` before sending pull requests that touch the gallery, example bootstraps, or docs references. The
  orchestration shells through every documented scaffold, executes the framework-specific checks, and writes transcripts to
  `target/logs/quick-start.log` so reviewers can audit results without rerunning the suite.【F:crates/xtask/src/main.rs†L206-L233】【F:crates/xtask/src/main.rs†L3078-L3296】
- Pass `--skip-checks` only when the target environment lacks optional toolchains (for example, when Playwright browsers or
  npm-based headless harnesses are unavailable). Even then, note the limitation in the pull request summary so CI reruns the full
  verification path.【F:crates/xtask/src/main.rs†L206-L233】
- Cache heavy dependencies to minimise bootstrap time: export `PLAYWRIGHT_BROWSERS_PATH=0` (or a shared CI directory) so docs
  specs reuse installed Chromium builds, share `CARGO_TARGET_DIR` across jobs to avoid recompiling scaffolds, and persist
  `TRUNK_HOME`, `DX_CACHE_DIR`, or `DIOXUS_CONFIG_DIR` when working with Trunk/dx-powered examples.【F:CONTRIBUTING.md†L54-L87】【F:docs/src/pages/getting-started/quick-start.md†L9-L88】

Document the command output in your changelog or pull request template when it influences review (for example, when refreshing a
StackBlitz snapshot or updating automation IDs). Contributors are expected to keep the quick-start verification log alongside
other CI evidence (fmt, clippy, deny, tests) so consumers inherit a proven automation baseline.

### Archived JavaScript workspace

Legacy Material UI sources now live under `archives/mui-packages/`. Each folder is a symlink back to
the historical JavaScript snapshot so Rust-first contributors can trace provenance without
reintroducing Node-centric build chains. The pnpm workspace no longer indexes these directories,
which keeps `pnpm -r` and Nx pipelines exclusively focused on the Rust-first crates and
TypeScript bridges that still evolve.

When docs or tooling refer to a `mui-*` package use the shared pnpm catalog entries instead of
adding the directories back to the workspace. For example, `catalog:@mui/material` resolves to
`archives/mui-packages/mui-material`, and `pnpm config list --json` exposes the full mapping for
automation. Scripts that previously globbed through `archives/mui-packages/**` should prefer the
catalog map so they follow future archive relocations automatically.

> **Tip for enterprise teams:** Keep custom scripts pointed at `archives/mui-packages/<package>`
> via the catalog map and rely on the manifest contract described in `archives/README.md`. Doing so
> ensures internal CI/CD runners pick up future archive reorganizations without manual edits.

### Component parity tracker

To monitor progress toward full Material UI coverage run the automated scanner:

```bash
cargo xtask material-parity
```

The command invokes the Rust CLI under `tools/material-parity` which parses the
React source (`archives/mui-packages/mui-material/src`) and generates the consolidated
report at `docs/material-component-parity.md`. Keep this artifact up to date in
pull requests that add or remove components so downstream teams have a reliable
signal when planning migrations.

### Joy UI inventory guardrail

Joy UI follows the same automation-first strategy. Rebuild the Joy coverage
report whenever a pull request touches Joy components or headless primitives:

```bash
cargo xtask joy-inventory
```

The xtask delegates to `tools/joy-parity`, a standalone Rust binary that walks
`archives/mui-packages/mui-joy/src/**/index.ts` via SWC, normalizes aliases, and compares the
exports with the Rust crates (`crates/rustic-ui-joy` and `crates/rustic-ui-headless`). The
command rewrites `docs/joy-component-parity.md` with a markdown dashboard plus a
machine-readable JSON blob embedded in the same file. Commit the refreshed
artifact so CI stays clean and enterprise adopters can spot parity gaps without
replicating the analysis locally.

### Theme artifact regeneration

Regenerating the serialized Material theme is a fully automated flow powered by
`cargo xtask generate-theme`. Always prefer this command over hand-editing the
files under `crates/rustic-ui-system/templates`:

```bash
cargo xtask generate-theme --overrides crates/xtask/tests/fixtures/material_overrides.json --format json
```

- **Local + CI parity** – Invoke the same command in local workflows and CI to
  eliminate drift. The task wipes legacy single-file artifacts, merges optional
  JSON/TOML overrides (shared plus per-scheme), then writes
  `material_theme.<scheme>.<ext>` alongside `material_css_baseline.<scheme>.css`.
- **Fixture-driven overrides** – Check fixtures such as
  `crates/xtask/tests/fixtures/material_overrides.json` into the repo and point
  the command at them. This keeps bespoke palettes and typography centralized
  and makes the job trivially reproducible in automation.
- **Repeatable validation** – The integration test in
  [`crates/xtask/tests/generate_theme.rs`](crates/xtask/tests/generate_theme.rs)
  exercises the full pipeline (override parsing, multi-scheme output, CSS
  generation). Add new fixtures or schemes via the test so enterprise teams can
  depend on a green build before promoting artefacts.

Contributors must rerun the generator and commit the refreshed artifacts any
time `material_theme()` defaults, override fixtures, or CSS baselines change.
This expectation keeps documentation samples, binary integrations, and SDKs in
lockstep without manual editing.

### Icon library maintenance

Multi-set icon support is fully automated. Always run the consolidated pipeline
after adding or removing SVGs in `crates/rustic-ui-icons/icons/**` or when pulling a
fresh drop from upstream Material sources:

```bash
cargo xtask icon-update
```

The task performs two coordinated steps:

1. `mui-icons-material` downloads and unpacks the official Material Design
   archive, pruning obsolete files so the crate mirrors the upstream source of
   truth.
2. `mui-icons` executes its `update_features` helper, scanning every icon set on
   disk and regenerating the `[features]` manifest with `set-<set>` and
   `icon-<set>-<name>` entries sorted alphabetically. Guard comments in
   `Cargo.toml` mark the generated section so reviews immediately recognize the
   automated edits.

Because both crates are updated by the same command, enterprise adopters can
depend on deterministic feature wiring regardless of how many icon families are
checked into the repository. Commit the refreshed manifest alongside any SVG
changes so CI stays green and local developers avoid manual cleanup.

## Branching and pull requests

- Fork the repository and branch from `main`.
- Ensure commits are logically grouped and reference any related issues.
- Run `make fmt clippy deny test` to execute formatting, linting, supply-chain auditing, and the full test suite. The `deny`
  target wraps `cargo xtask deny` so dependency advisories or license drift fail fast before review. Execute the command prior
  to pushing branches so CI mirrors a known-good local run.
- Fill in the PR template, summarizing the change, testing evidence, and migration considerations.

Pull requests must pass CI and include relevant documentation updates. Enterprise consumers rely on our docs to automate upgrades,
so keeping them accurate is a release gate.

## Documentation and demo workflow

Use the automation shipped with each example to avoid manual setup:

1. Pick the closest blueprint in [`examples/`](examples/). The
   [Rust example gallery](docs/src/pages/examples/index.md) lists every demo,
   their automation hooks, and parity guarantees.
2. Run the documented bootstrap command (for example
   `./examples/navigation-tabs-yew/scripts/bootstrap.sh` or `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`).
   These scripts materialise a ready-to-run workspace with SSR snapshots,
   hydration stubs, analytics markers, and framework manifests.
3. Update the generated markdown or README copy with narrative context, keeping
   the automation IDs and translation scaffolding intact so localisation and QA
   pipelines remain deterministic.

## Component development checklist

- Write unit tests in the relevant crate (Rust) and adapter packages when adding new functionality.
- Update the component's story/demo to reflect new props or behaviors.
- Document breaking changes in `CHANGELOG.md` under a new dated section.
- Verify accessibility via `cargo xtask accessibility-audit`.
- Exercise the full selection-control matrix with `cargo xtask selection-controls --skip-web` whenever you touch telemetry, SSR rendering, or Joy Select adapters. Drop the flag to launch the Rust-native headless Chrome harness; [`docs/testing/selection-controls.md`](docs/testing/selection-controls.md) outlines the CI profile and the optional dry-run environment variables used by integration tests.【F:crates/xtask/src/main.rs†L1-L120】【F:crates/xtask/src/selection_controls_web.rs†L1-L420】【F:docs/testing/selection-controls.md†L1-L120】

## Release cadence and backlog

The active backlog for the RusticUI transition lives at the end of [`CHANGELOG.md`](CHANGELOG.md). If your contribution advances a
backlog item, mention it in your pull request description so the maintainers can mark the progress.

## Getting help

If you have questions about the contribution process:

- Ask in the [RusticUI Discord](https://discord.gg/apotheon-ai) community channel.
- Open a discussion under “Q&A” on GitHub.
- Contact the core team via [rusticui@apotheon.ai](mailto:rusticui@apotheon.ai) for sensitive topics.

We appreciate your effort in building a scalable, automation-friendly UI stack for the Rust ecosystem.
