# RusticUI Speed Dial — Dioxus

Reference implementation for RusticUI's speed dial navigation rendered with
Dioxus. The example highlights telemetry wiring, SSR parity, and automation
metadata so enterprise teams can embed the floating command launcher without
hand-written scaffolding.

## Quick start

```bash
# Provision wasm + SSR toolchains (dioxus-cli, wasm32 target).
just bootstrap

# Run repository-aligned checks before opening a PR.
just check
just test

# Serve the CSR bundle.
dx serve --open

# Print the SSR document for parity audits.
just run-ssr
```

## Highlights

- **Controlled open/highlight state.** `build_state` keeps the headless
  `SpeedDialState` in sync with controlled open/highlight handles, mirroring
  production dashboards where parent shells own telemetry and RBAC checks.
- **Telemetry-first logging.** `TelemetryLog` serialises
  `SpeedDialAnalyticsEvent` payloads into newline-delimited JSON with RFC3339
  timestamps so browser consoles, CI harnesses, and observability pipelines reuse
  the same stream.
- **SSR determinism.** `render_document` emits the same markup and telemetry log
  consumed by CSR hydration. Integration tests snapshot the output to guard
  against regressions.
- **Automation hooks.** The rendered markup exposes `data-rustic-speed-dial-*`
  selectors and analytics tags. QA suites can assert behaviour with zero custom
  DOM traversal.

## CI integration

The example participates in the navigation example group via `cargo xtask`.
Run the following command locally and in CI to ensure native and wasm builds stay
in lockstep:

```bash
cargo xtask examples --group navigation --release
```
