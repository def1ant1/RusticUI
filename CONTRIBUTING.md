# Contributing to RusticUI

Thank you for helping shape RusticUI—the Rust-native evolution of Material UI under the Apotheon.ai open-source program. This
guide outlines the contribution workflow, automation entry points, and expectations for code and documentation changes.

## Code of conduct

RusticUI follows the [Contributor Covenant](https://www.contributor-covenant.org/) (v2.1). Please review the
[`CODE_OF_CONDUCT.md`](https://github.com/apotheon-ai/.github/blob/main/CODE_OF_CONDUCT.md) file in the Apotheon.ai organization
before interacting with the community. Violations can be reported confidentially to [conduct@apotheon.ai](mailto:conduct@apotheon.ai).

## Ways to contribute

RusticUI thrives on a wide range of contributions:

- **Crates & components** – Implement new components, fix bugs, or improve performance in any crate under `crates/` or
  `packages/`.
- **Documentation & demos** – Expand the docs hosted in `docs/` or add examples in `examples/`.
- **Automation** – Enhance the `cargo xtask` CLI, CI workflows, and observability pipelines.
- **Community** – Review pull requests, triage issues, or mentor newcomers in the [discussion board](https://github.com/apotheon-ai/rusticui/discussions).

Before starting large efforts, open a GitHub discussion or issue so the maintainers can align on goals and avoid duplicated work.

## Development setup

1. Install the latest stable Rust toolchain and ensure `wasm32-unknown-unknown` is available via `rustup target add`.
2. Install [pnpm](https://pnpm.io/installation) for documentation tooling.
3. Run `make bootstrap` to install workspace prerequisites and run quick smoke tests.

All repetitive chores are encapsulated inside the `Makefile` or `cargo xtask`. Prefer these entry points over ad-hoc scripts.

### Component parity tracker

To monitor progress toward full Material UI coverage run the automated scanner:

```bash
cargo xtask material-parity
```

The command invokes the Rust CLI under `tools/material-parity` which parses the
React source (`packages/mui-material/src`) and generates the consolidated
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
`packages/mui-joy/src/**/index.ts` via SWC, normalizes aliases, and compares the
exports with the Rust crates (`crates/mui-joy` and `crates/mui-headless`). The
command rewrites `docs/joy-component-parity.md` with a markdown dashboard plus a
machine-readable JSON blob embedded in the same file. Commit the refreshed
artifact so CI stays clean and enterprise adopters can spot parity gaps without
replicating the analysis locally.

### Theme artifact regeneration

Regenerating the serialized Material theme is a fully automated flow powered by
`cargo xtask generate-theme`. Always prefer this command over hand-editing the
files under `crates/mui-system/templates`:

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
after adding or removing SVGs in `crates/mui-icons/icons/**` or when pulling a
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
- Run `make fmt lint test` to execute the standard formatting, linting, and test suite. The command fan-outs to Rust, TypeScript,
  and Markdown checks as appropriate.
- Fill in the PR template, summarizing the change, testing evidence, and migration considerations.

Pull requests must pass CI and include relevant documentation updates. Enterprise consumers rely on our docs to automate upgrades,
so keeping them accurate is a release gate.

## Documentation and demo workflow

Use the scripted scaffolding to avoid manual setup:

```bash
cargo xtask scaffold-demo --component button --framework leptos
```

The generator produces the doc page, localized strings, Playwright tests, and analytics markers. Update the generated markdown
with narrative context, but keep the structural conventions intact so the translation pipeline succeeds.

## Component development checklist

- Write unit tests in the relevant crate (Rust) and adapter packages when adding new functionality.
- Update the component's story/demo to reflect new props or behaviors.
- Document breaking changes in `CHANGELOG.md` under a new dated section.
- Verify accessibility via `cargo xtask accessibility-audit`.

## Release cadence and backlog

The active backlog for the RusticUI transition lives at the end of [`CHANGELOG.md`](CHANGELOG.md). If your contribution advances a
backlog item, mention it in your pull request description so the maintainers can mark the progress.

## Getting help

If you have questions about the contribution process:

- Ask in the [RusticUI Discord](https://discord.gg/apotheon-ai) community channel.
- Open a discussion under “Q&A” on GitHub.
- Contact the core team via [rusticui@apotheon.ai](mailto:rusticui@apotheon.ai) for sensitive topics.

We appreciate your effort in building a scalable, automation-friendly UI stack for the Rust ecosystem.
