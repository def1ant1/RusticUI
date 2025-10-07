# React SSR + hydration troubleshooting

RusticUI's Quick Start flow is the fastest way to bootstrap the React + WebAssembly selection controls stack, install shared tooling, and align your local environment with CI before debugging hydration issues.【F:docs/src/pages/getting-started/quick-start.md†L1-L126】 Keep that guide open while you work through the targeted remediation playbook below.

## Primary runbooks

- **Bootstrap + dev loop** – Run `just bootstrap` followed by `just dev` inside `examples/selection-controls-react` to install npm/WebAssembly dependencies and launch the hydrated Vite preview that mirrors CI.【F:examples/selection-controls-react/README.md†L25-L72】 The Just recipes call the same scripts documented for manual usage, so falling back to `npm run build:wasm`, `npm run build:web`, and `npm run dev` preserves parity with automation.【F:examples/selection-controls-react/README.md†L44-L72】
- **Canonical smoke coverage** – `npm run test:e2e` delegates to `cargo xtask selection-controls --framework react`, replaying the Rust-native telemetry harness in headless Chrome before Jest executes.【F:examples/selection-controls-react/README.md†L73-L109】【F:crates/xtask/src/main.rs†L163-L378】 Use this after every dependency upgrade or adapter change to confirm telemetry and hydration both succeed.

## Diagnosing hydration mismatches

1. **Reproduce with deterministic SSR HTML** – Run `npm run build:wasm && npm run build:web` and load the resulting bundle against the SSR snapshot produced by `npm run test:e2e`. Chrome's hydration warning plus the structured telemetry output highlight the earliest divergence.【F:examples/selection-controls-react/README.md†L44-L109】
2. **Inspect automation selectors** – The Rust crate emits automation IDs (`automation.selection-controls.*`) and analytics payloads before any React handlers fire, so mismatches usually correlate with missing delegate wiring or stale IDs.【F:examples/selection-controls-react/README.md†L91-L135】 Compare the pre-hydration telemetry lines with the CSR render to isolate dropped attributes.
3. **Cross-check Quick Start assumptions** – If the React shell diverges from the shared bootstrap (wrong automation IDs, inconsistent state order), re-run `cargo xtask quick-start` from the repository root to rebuild the canonical stack and diff local changes against the transcript in `target/logs/quick-start.log`.【F:docs/src/pages/getting-started/quick-start.md†L14-L33】

## Automation and logging tips

- **Shared smoke harness** – `just automation-smoke` invokes `examples/scripts/selection-controls-smoke.sh`, producing newline-delimited telemetry that your logging pipeline can ingest without custom glue.【F:examples/selection-controls-react/README.md†L25-L109】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】 Tail the log while recreating mismatches to correlate render order with React lifecycle callbacks.
- **Telemetry ordering assertions** – Jest and wasm-bindgen tests both assert that telemetry hooks emit before React callbacks, ensuring hydration mismatches surface as failed assertions instead of silent DOM drift.【F:examples/selection-controls-react/README.md†L58-L92】 When failures appear only in browsers, instrument `npm run dev` with Chrome DevTools performance recordings to spot which hook stops firing.

## Regression checklist

- Run `cargo xtask examples --group hydration` after fixing a mismatch to rebuild every SSR bootstrapper and guarantee the shared generators still emit HTML/telemetry stubs for Rust adapters.【F:crates/xtask/src/main.rs†L439-L516】【F:crates/xtask/src/main.rs†L516-L708】
- Execute `npm run test:e2e` and `npm run test` to validate the React + wasm unit, integration, and headless telemetry suites stay green.【F:examples/selection-controls-react/README.md†L44-L109】
- Document any new automation IDs or telemetry fields inside `examples/selection-controls-react/README.md` so downstream teams inherit the contract without manual sync work.【F:examples/selection-controls-react/README.md†L1-L135】
