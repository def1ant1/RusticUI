# Cross-suite coverage dashboard

The RusticUI workspace now exports a single coverage dashboard that merges every
Rust and TypeScript testing surface: unit tests, integration checks, axe-core
accessibility sweeps, and Playwright visual snapshot reviews. Instead of
triaging multiple artifacts, `cargo xtask coverage-report` consolidates the
underlying telemetry into deterministic JSON and Markdown so release managers
can answer **which suites ran**, **whether they met the release thresholds**, and
**where to triage regressions** without shuffling between CI jobs.

## Running the aggregator

```bash
# Run your normal language-specific suites first.
cargo xtask coverage                          # grcov line/branch metrics
pnpm --dir docs test --reporter=junit         # Vitest + Playwright unit/integration snapshots
pnpm test:regressions:run                    # Visual snapshot diffs (Playwright)

# Then aggregate everything into JSON + Markdown dashboards.
cargo xtask coverage-report
```

The command writes two files under `test-results/coverage/`:

- `coverage-report.json` – machine-readable payload for CI and data tooling.
- `coverage-report.md` – human-focused summary with tables, thresholds, and
  remediation notes.

Both files are timestamped and include a note section that spells out which
pipelines fed the report. The Markdown variant is intended to be attached to CI
artifacts or the release readiness wiki so the broader team can review the same
information without pulling the repo.

## Bundle-size telemetry

Pair the coverage dashboard with the automated bundle-size report to understand
the cost of toggling optional features. `cargo xtask bundle-report` compiles the
Material and headless crates across their feature matrices, records the release
`*.rlib` sizes, and emits both JSON and Markdown summaries. The Markdown export
is committed to `docs/performance/bundle-costs.md` so the performance playbook
always reflects the latest measurements.

## Pipeline expectations

| Suite | Track | Discipline | How to generate source data | Threshold | Failure mode |
| :---- | :---- | :--------- | :--------------------------- | :-------- | :----------- |
| **Rust workspace** | Rust | Integration | `cargo xtask coverage` (writes `lcov.info` via grcov across unit + integration suites) | Line ≥ 75%, Branch ≥ 60% | Missing `lcov.info` marks the suite as skipped; low coverage fails the command. |
| **TypeScript automation** | TypeScript | Unit | Any runner that exports `test-results/junit.xml` (Vitest, Karma, Playwright component/unit suites). Snapshot expectations are counted alongside classic specs so the pass-rate mirrors combined unit + snapshot health. | Pass rate ≥ 97.5% (computed from `tests`, `failures`, `errors`, `skipped`) | A missing or all-skipped JUnit summary fails the report. |
| **Accessibility audits** | Cross-stack | Accessibility | `cargo xtask accessibility-audit` (Markdown hygiene) plus `cargo xtask accessibility-nightly` (delegates to the Playwright + axe harness under `test/accessibility/`). The nightly run exports JSON + HTML reports to `test-results/accessibility` by invoking `pnpm --dir test run accessibility`. | Zero blocking axe violations or Markdown lint errors | Any markdown issue or axe violation with impact above the configured gate fails the command and the CI job. |
| **Visual regressions** | TypeScript | Visual snapshot | Ensure Playwright saves `test-results/visual-regressions.json` with snapshot counts. The summary includes updated/diff/skipped counts so screenshot drift is immediately visible. | Zero diff images | Missing JSON or non-zero diffs fail the aggregator. |
| **Adapter Storybooks** | Cross-stack | Visual snapshot | `pnpm --dir test/regressions/adapters playwright` captures Storybook screenshots and writes `test-results/visual-regressions-adapters.json`. Chromatic uploads mirror the same builds so design reviews use identical assets. | Zero diff images | Missing JSON marks the suite as skipped; non-zero diffs fail the aggregator. |

> **Tip:** Enterprise teams often run TypeScript tests in multiple packages. As
long as the combined CI pipeline writes a consolidated `test-results/junit.xml`
you can still aggregate pass rates across all suites. If you maintain multiple
reports, concatenate them into a single file before invoking `cargo xtask
coverage-report`.

## Interpreting the dashboard

- ✅ Status indicates the threshold was met or exceeded for that discipline.
- ⚠️ Signals that the aggregator could not find the source data (treated as a
  failure in CI to prevent silent skips). Review the `Artifacts` section to spot
  missing files or mismatched paths.
- ❌ Denotes a real regression. The Markdown file expands each failing suite
  with bullet points pointing to the underlying artifact (for example the
  Playwright diff summary, axe-core violation list, or the Markdown file missing
  alt text).

Every suite lists the artifacts it relied on and records the discipline it
represents. Use that combination to jump directly to the raw `lcov.info`,
`junit.xml`, axe-core summary, or visual regression report when investigating.

## Automating in CI

The default GitHub Actions workflow now runs `cargo xtask coverage-report` and
uploads `test-results/coverage/` as an artifact. Integrate the step after your
language-specific jobs to ensure the aggregator sees the latest coverage data.
For custom CI systems replicate the same pattern:

1. Run Rust, TypeScript, accessibility, and visual regression suites.
2. Invoke `cargo xtask coverage-report`.
3. Publish `test-results/coverage/` for engineers and release tooling.

Because the command exits with a non-zero status when a suite is skipped or a
threshold is missed, you can gate deployment stages on the dashboard without
writing additional scripting.
