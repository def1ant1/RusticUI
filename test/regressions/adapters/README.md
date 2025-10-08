# Adapter visual regression harness

This directory contains the dedicated Playwright + Chromatic harness that
exercises Storybook builds for the Rust-driven adapter packages. The scripts are
heavily documented because adapter snapshots double as infrastructure notes for
teams that integrate RusticUI into enterprise portals.

The entry points are:

- `adapter-visual-regressions.spec.ts` – Playwright test that discovers local
  Storybook builds, spins up ephemeral static servers, and captures screenshots
  for every published story. The suite writes
  `test-results/visual-regressions-adapters.json` so the coverage dashboard can
  reason about adapter health separately from the docs-driven fixtures.
- `chromatic.ts` – Thin wrapper around the Chromatic CLI. It normalises the
  environment variables used in CI, enforces caching, and keeps the command line
  invocation in source control so downstream teams can follow the same pattern.
- `manifest.ts` / `staticServer.ts` / `summary.ts` – Utility modules with
  extensive commentary explaining how discovery works and why we prefer
  automation over hand-maintained story lists.

Install the local dependencies once and launch the suite with

```bash
pnpm --dir test/regressions/adapters install
pnpm --dir test/regressions/adapters playwright
```

When Chromatic tokens are available:

```bash
pnpm --dir test/regressions/adapters chromatic
```

Storybook builds can be orchestrated via the helper script:

```bash
pnpm --dir test/regressions/adapters build-storybooks
```

The workflow defined in `.github/workflows/visual-regressions.yml` wires these
scripts into CI. See `docs/testing/visual-regressions.md` for the full
explanation, including how we cache Storybook builds and aggregate the results
into the coverage dashboard.
