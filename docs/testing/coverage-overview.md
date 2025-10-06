# Cross-suite coverage dashboard

The RusticUI workspace now exports a single coverage dashboard that merges the
Rust `cargo xtask coverage` output, the TypeScript Vitest/Playwright runs, the
Markdown accessibility sweeps, and Playwright-driven visual regression metrics.
The goal is to give maintainers a single artifact that answers **which suites
ran**, **what thresholds they satisfied**, and **where to dig when regressions
appear**.

## Running the aggregator

```bash
# Run your normal language-specific suites first.
cargo xtask coverage
pnpm --dir docs test --reporter=junit   # or your project-wide TS runner
pnpm test:regressions:run               # Playwright visual snapshots

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

## Pipeline expectations

| Suite | How to generate source data | Threshold | Failure mode |
| :---- | :--------------------------- | :-------- | :----------- |
| **Rust workspace** | `cargo xtask coverage` (writes `lcov.info` via grcov) | Line ≥ 75%, Branch ≥ 60% | Missing `lcov.info` marks the suite as skipped; low coverage fails the command. |
| **TypeScript automation** | Any runner that exports `test-results/junit.xml` (Vitest, Karma, Playwright, etc.) | Pass rate ≥ 97.5% (computed from `tests`, `failures`, `errors`, `skipped`) | A missing or all-skipped JUnit summary fails the report. |
| **Accessibility audits** | `cargo xtask accessibility-audit` (invoked automatically when aggregating) | Zero Markdown issues | Findings are surfaced inline with the file path and break the build. |
| **Visual regressions** | Ensure Playwright saves `test-results/visual-regressions.json` with snapshot counts | Zero diff images | Missing JSON or non-zero diffs fail the aggregator. |

> **Tip:** Enterprise teams often run TypeScript tests in multiple packages. As
long as the combined CI pipeline writes a consolidated `test-results/junit.xml`
you can still aggregate pass rates across all suites. If you maintain multiple
reports, concatenate them into a single file before invoking `cargo xtask
coverage-report`.

## Interpreting the dashboard

- ✅ Status indicates the threshold was met or exceeded.
- ⚠️ Signals that the aggregator could not find the source data (treated as a
  failure in CI to prevent silent skips).
- ❌ Denotes a real regression. The Markdown file expands each failing suite
  with bullet points pointing to the underlying artifact (for example the
  Playwright diff summary or the Markdown file missing alt text).

Every suite lists the artifacts it relied on, so you can jump directly to the
raw `lcov.info`, `junit.xml`, or visual regression report when investigating.

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
