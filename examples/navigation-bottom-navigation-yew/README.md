# RusticUI Bottom Navigation — Yew

Enterprise-grade reference for wiring RusticUI's bottom navigation primitives
inside a Yew shell. The crate doubles as living documentation: inline comments
explain the headless state integration, analytics fan-out, SSR harness, and
automation hooks so regulated teams can adopt the component without reverse
engineering the repository.

## Quick start

```bash
# Ensure the WASM + SSR toolchains are provisioned (Trunk, wasm32 target, tokio).
just bootstrap

# Run focused checks before opening a pull request.
just check
just test

# Hydrate the demo in the browser.
trunk serve --open

# Capture the SSR document with telemetry notes.
just run-ssr
```

## Architecture notes

- **Controlled state mirroring production shells.** The Yew component keeps the
  selected/focused indices in `UseStateHandle`s and feeds them back into the
  headless `BottomNavigationState`. This mirrors how large dashboards own global
  navigation state while reusing shared renderers.
- **Analytics-ready telemetry stream.** `telemetry.rs` serialises
  `BottomNavigationAnalyticsEvent` payloads into newline-delimited JSON with
  RFC3339 timestamps so the same log can be shipped to browser consoles,
  OpenTelemetry collectors, or saved as SSR fixtures.
- **Deterministic SSR output.** The `ssr` module renders the navigation and a
  sample telemetry log into a static HTML document. Integration tests snapshot
  the document to guard against markup regressions.
- **Automation hooks baked in.** Every surface exposes `data-rustic-*`
  attributes plus `data-rustic-analytics-*` telemetry markers. QA suites and
  observability collectors can assert on these selectors without bespoke glue.

## Telemetry + observability

The demo streams analytics to the synthetic channel `navigation.bottom`. Each
selection produces a JSON line similar to the following:

```json
{"channel":"navigation.bottom","item_tag":"destination.analytics","index":1,"occurred_at":"2024-01-01T12:00:00Z"}
```

Reuse the `TelemetryLog` helper if you need to persist additional metadata or
forward events to OpenTelemetry—its API mirrors production fan-out pipelines.

## CI hooks

The crate participates in the `navigation` example group via `cargo xtask` once
registered. CI can validate the demo across native and `wasm32` targets with:

```bash
cargo xtask examples --group navigation --release
```

Run this command locally before promoting changes to keep SSR/CSR parity and
automation hooks consistent.
