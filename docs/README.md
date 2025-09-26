# Docs

This directory powers the RusticUI documentation site maintained by Apotheon.ai. It covers the RusticUI component families,
headless primitives, automation tooling, and migration paths.

To start the docs site in development mode, from the project root, run:

```bash
pnpm docs:dev
```

If you do not have pnpm installed, select your OS and follow the instructions on the [pnpm website](https://pnpm.io/installation).

Package managers other than pnpm (like npm or Yarn) are not supported and will not work.

## How can I add a new demo to the documentation?

1. Open a discussion in the [RusticUI RFC board](https://github.com/apotheon-ai/rusticui/discussions/categories/rfcs) describing the
   problem the demo should solve.
2. Once approved, use `cargo xtask scaffold-component` or `cargo xtask scaffold-demo` to generate the baseline files. The script
   injects accessibility tests, analytics tags, and translation scaffolding automatically to minimize manual wiring.
3. Commit the generated files and update the appropriate page inside `docs/src/pages`.

## How do I help to improve the translations?

RusticUI translations are managed through the Apotheon.ai Crowdin workspace: <https://crowdin.com/project/rusticui-docs>.
Please avoid submitting pull requests with manual translation edits; instead comment on the Crowdin strings so the localization
team can propagate the updates across the automation pipeline.

## Rustic theming resources

Looking for the Rust-specific theming workflow (Rustic palettes, overrides via `#[derive(Theme)]`, and global baseline styles)?
Start with [`crates/rustic-ui-system/README.md`](../crates/rustic-ui-system/README.md#theming-and-global-styles) while the crates transition to
their RusticUI names. That guide documents the automation steps such as `cargo xtask generate-theme` and explains how the design
tokens integrate with the continuous delivery workflows.
