# Docs

This directory powers the RusticUI documentation site maintained by Apotheon.ai. It covers the RusticUI component families,
headless primitives, automation tooling, and migration paths. The latest update
introduces dedicated coverage for the automation utilities (click-away
listeners, focus traps, telemetry streams) plus the Material renderers and
examples that exercise them end-to-end.

To start the docs site in development mode, from the project root, run:

```bash
pnpm docs:dev
```

If you do not have pnpm installed, select your OS and follow the instructions on the [pnpm website](https://pnpm.io/installation).

Package managers other than pnpm (like npm or Yarn) are not supported and will not work.

## How can I add a new demo to the documentation?

1. Open a discussion in the [RusticUI RFC board](https://github.com/apotheon-ai/rusticui/discussions/categories/rfcs) describing the
   problem the demo should solve.
2. Once approved, start from the closest Rust blueprint in [`examples/`](../examples) and follow its bootstrap instructions
   (for example `./examples/navigation-tabs-yew/scripts/bootstrap.sh` or `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml`).
   These scripts hydrate deterministic automation IDs, SSR snapshots, and framework manifests so you avoid manual wiring.
   The [Rust example gallery](src/pages/examples/index.md) documents the available demos, parity guarantees, and
   follow-up verification steps.
3. Commit the generated files and update the appropriate page inside `docs/src/pages`.

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
- **Material adapter deep dives** –
  [`crates/rustic-ui-material/README.md`](../crates/rustic-ui-material/README.md#architectural-rationale-for-the-new-renderers-and-adapters)
  now records how each framework adapter consumes the utilities and how to wire
  telemetry into enterprise monitoring stacks.
- **Example automation harnesses** – The refreshed
  [Rust example gallery](src/pages/examples/index.md#automation-focused-blueprints)
  calls out which blueprints exercise the utilities and which `cargo xtask`
  groups to run when validating telemetry output.
