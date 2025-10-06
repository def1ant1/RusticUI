# Docs

This directory powers the RusticUI documentation site maintained by Apotheon.ai. It covers the RusticUI component families,
headless primitives, automation tooling, and migration paths. The latest update
introduces dedicated coverage for the automation utilities (click-away
listeners, focus traps, telemetry streams) plus the Material renderers and
examples that exercise them end-to-end.

## Strategic improvement program

We are incrementally delivering the enterprise hardening efforts outlined in
[`improvement-plan.md`](./improvement-plan.md). Start there for a phased roadmap
spanning discovery work, developer experience enhancements, reliability
initiatives, ecosystem investments, and long-term evolution tracks. Each phase
is designed to minimize manual toil through automation-first tooling, thorough
documentation, and scalable architectural guardrails.

To start the docs site in development mode, from the project root, run:

```bash
pnpm docs:dev
```

If you do not have pnpm installed, select your OS and follow the instructions on the [pnpm website](https://pnpm.io/installation).

Package managers other than pnpm (like npm or Yarn) are not supported and will not work.

Prefer `cargo xtask dev` when you need both the Next.js docs and the Leptos example gallery running together. The harness
launches both servers, wires shared logging to `target/logs/dev.log`, and reuses `target/dev` as the Cargo cache so hot-reload
iterations stay fast across restarts.

## Coverage and reliability dashboards

- [Cross-suite coverage dashboard](testing/coverage-overview.md) – documents how to
  generate the aggregated coverage report, thresholds for each suite, and how the
  `cargo xtask coverage-report` command stitches Rust grcov data together with the
  Vitest/Playwright pipelines. Pair it with the quick-start verification guide to
  keep docs demos and automation blueprints healthy across releases.【F:docs/testing/coverage-overview.md†L1-L72】

## How can I add a new demo to the documentation?

1. Open a discussion in the [RusticUI RFC board](https://github.com/apotheon-ai/rusticui/discussions/categories/rfcs) describing the
   problem the demo should solve.
2. Once approved, start from the closest Rust blueprint in [`examples/`](../examples) and follow its bootstrap instructions
   (for example `./examples/navigation-tabs-yew/scripts/bootstrap.sh` or `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`).
   These scripts hydrate deterministic automation IDs, SSR snapshots, and framework manifests so you avoid manual wiring.
   The [Rust example gallery](src/pages/examples/index.md) documents the available demos, parity guarantees, and
   follow-up verification steps.
3. Commit the generated files and update the appropriate page inside `docs/src/pages`.

When editing those demos or the quick-start gallery, consult the
[quick-start automation verification guide](testing/quick-start.md) and run `cargo xtask quick-start` so the docs site, StackBlitz
snapshots, and Rust bootstraps stay aligned without manual interventions. The playbook also captures caching strategies for
Playwright, Trunk, and dx so contributors avoid re-downloading browsers or recompiling scaffolds on every iteration.【F:docs/testing/quick-start.md†L1-L96】【F:crates/xtask/src/main.rs†L206-L233】

## How do I help to improve the translations?

RusticUI translations are managed through the Apotheon.ai Crowdin workspace: <https://crowdin.com/project/rusticui-docs>.
Please avoid submitting pull requests with manual translation edits; instead comment on the Crowdin strings so the localization
team can propagate the updates across the automation pipeline.

## Rustic theming resources

Looking for the Rust-specific theming workflow (Rustic palettes, overrides via `#[derive(Theme)]`, and global baseline styles)?
Start with [`crates/rustic-ui-system/README.md`](../crates/rustic-ui-system/README.md#theming-and-global-styles) now that the
`rustic-ui-*` crates are published. That guide documents the automation steps such as `cargo xtask generate-theme`, the
`compat-mui` feature flag used during migrations, and the `scripts/migrate-crate-prefix.sh` helper that rewrites imports at
scale before the compatibility layer is removed.

## Automation utility playbooks

- **Headless rationale and observability** – See
  [`crates/rustic-ui-headless/README.md`](../crates/rustic-ui-headless/README.md#architectural-rationale-for-the-new-utility-suite)
  for architectural notes, troubleshooting guidance, and observability hooks.
- **State machine lifecycles** –
  [`architecture/headless-state-machines.md`](./architecture/headless-state-machines.md)
  visualises the controlled/uncontrolled transitions, token orchestration, and
  focus-loop analytics markers behind `collapsible_region` and `focus_trap`.
- **Material adapter deep dives** –
  [`crates/rustic-ui-material/README.md`](../crates/rustic-ui-material/README.md#architectural-rationale-for-the-new-renderers-and-adapters)
  now records how each framework adapter consumes the utilities and how to wire
  telemetry into enterprise monitoring stacks.
- **Example automation harnesses** – The refreshed
  [Rust example gallery](src/pages/examples/index.md#automation-focused-blueprints)
  calls out which blueprints exercise the utilities and which `cargo xtask`
  groups to run when validating telemetry output.
