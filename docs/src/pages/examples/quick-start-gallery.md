# Quick-start button gallery

{{"component": "components/examples/QuickStartButtonGallery.tsx"}}

The embedded playground streams every file from the shared quick-start generator, so the Material theme,
call-to-action copy, automation attributes, and StackBlitz launcher all update in lockstep with the
Rust examples and docs site.【F:docs/src/components/examples/QuickStartButtonGenerator.ts†L1-L187】 The
snapshot exported for StackBlitz reuses the docs workspace dependency versions, guaranteeing the
playground and published bundles depend on the same packages.【F:docs/src/components/examples/QuickStartButtonGenerator.ts†L66-L136】

## Automation workflow

- Run `pnpm --dir docs sandbox:quick-start -- --check` before merging to confirm the JSON snapshot
  matches the generator output; this script is designed for CI and developer workstations alike.【F:docs/package.json†L11-L20】【F:docs/scripts/quickStartButtonSandbox.ts†L1-L43】
- Refresh the snapshot with `pnpm --dir docs sandbox:quick-start -- --write` whenever the generator
  changes so the checked-in data stays deterministic for reviews and follow-on automation.【F:docs/scripts/quickStartButtonSandbox.ts†L1-L43】【F:docs/data/examples/quick-start-button-sandbox.json†L1-L179】
- The snapshot lives at `docs/data/examples/quick-start-button-sandbox.json`, making it trivial to
  audit edits and feed other tooling that wants to mirror the same sandbox files.【F:docs/data/examples/quick-start-button-sandbox.json†L1-L179】
- Pair the docs snapshot check with the repository-wide [`cargo xtask quick-start`](../../../docs/testing/quick-start.md) harness
  so every framework scaffold in the getting-started guide stays in sync with the gallery CTA and automation IDs.【F:docs/testing/quick-start.md†L1-L74】

## Next steps

- Review the broader [example gallery](./index.md) to see how the quick-start CTA links into the
  multi-framework scaffolds and automation suites.【F:docs/src/pages/examples/index.md†L1-L211】
- Follow the [quick-start automation verification playbook](../../../docs/testing/quick-start.md) for prerequisites, caching
  strategies, and log interpretation tips before updating scaffolds or CTA copy.【F:docs/testing/quick-start.md†L1-L96】
