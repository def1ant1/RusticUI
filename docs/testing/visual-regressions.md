# Adapter visual regression pipeline

The adapter Storybooks live alongside the Rust-first examples (for example the
Yew, Leptos, and Sycamore playgrounds). Each adapter publishes a standalone
`storybook-static/` folder so we can take deterministic screenshots without
booting the entire docs site. This document explains how the automated pipeline
works, how to run it locally, and how the coverage dashboard consumes the
results.

## Why a dedicated pipeline?

The existing Playwright suite under `test/regressions/` focuses on the
documentation demos. Adapter Storybooks evolve separately and often target
framework-specific rendering quirks. Keeping the pipelines separate allows us to
run the adapter suite on a narrower matrix (Linux + Chromium) while still
producing rich telemetry for release managers.

Key design goals:

- **Automation-first.** Discovery walks the workspace to locate Storybook builds
  so engineers do not need to update manifests manually.
- **Shared artifacts.** Playwright writes
  `test-results/visual-regressions-adapters.json` so the coverage dashboard can
  render adapter status next to the docs suite. The GitHub Action uploads the raw
  screenshots and Chromatic shares the same builds with designers.
- **Reproducible caches.** Both Playwright and Chromatic cache the Storybook
  static output. Re-runs avoid rebuilding packages unless the source changes.

## Running locally

1. Build the Storybook bundles. Each adapter example exposes an npm script that
   compiles to `storybook-static/`. For example:

   ```bash
   pnpm --dir examples/selection-controls-react storybook:build
   ```

2. Execute the Playwright harness:

   ```bash
   pnpm --dir test/regressions/adapters install
   pnpm --dir test/regressions/adapters playwright
   ```

   The command discovers every `examples/**/storybook-static` directory,
   launches a static server per Storybook, and captures screenshots for every
   story. Screenshots live in
   `test-results/visual-regressions/adapters/<storybook>/<story>.png` and the
   suite writes the JSON summary consumed by `cargo xtask coverage-report`.

3. (Optional) Publish to Chromatic:

   ```bash
   export CHROMATIC_PROJECT_TOKEN=xxxxxxxx
   pnpm --dir test/regressions/adapters chromatic
   ```

   The script reads the same manifest as Playwright and shells out to the
   Chromatic CLI with deterministic flags (`--no-interactive`,
   `--exit-once-uploaded`, cache directory under `.chromatic-cache/`). Set
   `CHROMATIC_PROJECT_TOKEN_<STORYBOOK_ID>` to override the token per adapter.

## GitHub Actions workflow

`.github/workflows/visual-regressions.yml` wires everything together:

- Triggers on pull requests, manual dispatches, and a nightly cron job.
- Restores cached Storybook builds to avoid recompiling on incremental runs.
- Runs `pnpm --dir test/regressions/adapters playwright` to capture Playwright screenshots
  and uploads `test-results/visual-regressions/adapters/` as an artifact for
  manual inspection.
- Publishes Storybooks to Chromatic when the `CHROMATIC_PROJECT_TOKEN` secret is
  available. Nightly runs act as a sanity check even when no pull request is
  open.
- Uploads `test-results/visual-regressions-adapters.json` so
  `cargo xtask coverage-report` can merge the adapter status into the coverage
  dashboard.

## Coverage dashboard integration

The coverage aggregator (`cargo xtask coverage-report`) now reads
`test-results/visual-regressions-adapters.json` in addition to the existing docs
suite summary. Missing files mark the adapter suite as skipped, while non-zero
`differences` or `updated` counts fail the report. The default notes in the
coverage dashboard call out the new requirement so pipelines emitting the JSON
summary remain consistent.

When the GitHub Action finishes it attaches the JSON summary and screenshot
artifact to the workflow run. Release managers can inspect the adapter suite
without re-running the jobs locally.
