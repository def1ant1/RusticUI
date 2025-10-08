# Accessibility harness

This folder hosts the Playwright + axe-core runner responsible for auditing the
Next.js documentation site and the shared example gallery. The goal is to keep
enterprise-ready automation self-contained so CI and local development reuse the
same configuration instead of copying shell scripts into bespoke pipelines.

## How the suite works

- `playwright.config.ts` wires a deterministic base URL, trace/asset retention,
  JSON + HTML reporting, and a web server command that bootstraps the docs
  application on demand. The defaults spin up `pnpm --dir docs run dev` against
  `http://127.0.0.1:4321`, but every option can be overridden via environment
  variables documented inline.
- `accessibility.spec.ts` enumerates the routes under test (see
  `targets.ts`). Each scenario loads the page, waits for the main content to
  render, runs axe-core with WCAG 2.0/2.1 AA rules, and persists both the full
  violation payload and a Markdown summary. Failures are gated on severity so
  low-impact warnings can be reviewed without breaking the build.
- `targets.ts` centralises the example catalogue. Adding a new doc or gallery
  demo only requires appending to this manifest. Common escape hatches—such as
  deferring flaky widgets or allowing "minor" violations temporarily—live
  alongside the target entry with extensive comments so future clean-up is
  obvious.

## Running locally

```bash
# 1. Install dependencies (first run only)
pnpm --dir docs install
pnpm --dir test install
pnpm --dir test exec playwright install --with-deps chromium

# 2. Execute the audits (Playwright starts the docs dev server automatically)
pnpm --dir test run accessibility
```

Set `RUSTIC_UI_ACCESSIBILITY_SKIP_WEB_SERVER=1` if you already have the docs dev
server running, or override the defaults with:

- `RUSTIC_UI_ACCESSIBILITY_BASE_URL` – absolute origin Playwright should use.
- `RUSTIC_UI_ACCESSIBILITY_WEB_COMMAND` – spawn command when no server is
  running (defaults to the Next.js dev server).
- `RUSTIC_UI_ACCESSIBILITY_RESULTS_DIR` – directory where JSON/HTML reports and
  raw axe payloads are written (defaults to `test-results/accessibility`).

## CI expectations

The GitHub Actions workflow caches the pnpm store, installs docs + test
dependencies, provisions Playwright's Chromium bundle, and finally runs the
script above. Reports are uploaded to the `test-results/accessibility` artifact
for quick triage.

Because the suite emits machine-readable JSON (mirroring axe-core's schema), the
`cargo xtask accessibility-nightly` command simply executes this harness and
leaves aggregation to downstream tooling like `cargo xtask coverage-report`.
