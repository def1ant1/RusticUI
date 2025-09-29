# RusticUI Pagination — Leptos

A telemetry-focused pagination showcase built with RusticUI's headless
navigation primitives and Material renderer. The crate documents how to hydrate
server output, stream analytics to shared collectors, and exercise the headless
state in integration tests without bespoke plumbing.

## Quick start

```bash
# Provision wasm + SSR toolchains (Trunk, wasm32, tokio) once.
just bootstrap

# Run repository-aligned checks before opening a PR.
just check
just test

# Serve the CSR bundle for interactive validation.
trunk serve --open

# Print the SSR document with inline telemetry fixtures.
just run-ssr
```

## Architecture callouts

- **Headless state mirrored across renderers.** `configure_state` builds the
  `PaginationState` with controlled selection and analytics tags so SSR and CSR
  runs share deterministic attributes and telemetry output.
- **Telemetry-ready logging.** The `TelemetryLog` helper serialises
  `PaginationAnalyticsEvent` payloads into newline-delimited JSON with RFC3339
  timestamps, mirroring production observability flows.
- **Hydration parity.** The SSR harness renders the pagination markup and a
  canonical telemetry log, then the browser hydrates the same structure via
  `mount_to_body`, ensuring deterministic markup for diff-based QA.
- **Automation metadata.** The rendered nodes expose `data-rustic-pagination-*`
  selectors so automated tests and monitoring agents can assert behaviour
  without DOM spelunking.

## CI integration

Once the example is registered with `cargo xtask`, the navigation example group
builds it for native and `wasm32` targets via:

```bash
cargo xtask examples --group navigation --release
```

Run this command locally when modifying the demo to keep SSR snapshots, telemetry
fixtures, and automation hooks aligned with the rest of the workspace.
