# Leptos SSR + hydration troubleshooting

Follow the Quick Start bootstrap for Leptos so Trunk, wasm32 targets, and shared automation IDs are provisioned before you debug mismatches.【F:docs/src/pages/getting-started/quick-start.md†L39-L74】 The generated demo documents how route descriptors, hydration helpers, and automation hooks flow from the shared crates.

## Primary runbooks

- **Rebuild the marketing shell** – `examples/mui-leptos` documents the canonical flow: `trunk serve --open` to hydrate CSR, `cargo run --manifest-path examples/mui-leptos/Cargo.toml --features ssr > prerendered.html` for deterministic SSR HTML, and `trunk build --release` for deployable wasm bundles.【F:examples/mui-leptos/README.md†L11-L33】
- **Run integration tests** – `cargo test --package rustic_ui_leptos_example` verifies router parity, automation IDs, and the hydration-aware mode switch state machine before changes land.【F:examples/mui-leptos/README.md†L29-L37】 Combine this with the Quick Start harness when adding new descriptors or showcases.【F:docs/src/pages/getting-started/quick-start.md†L14-L33】

## Diagnosing hydration mismatches

1. **Diff SSR vs CSR** – Rebuild the SSR snapshot then reload the running Trunk server; both paths consume `mui_shared::layout::AppShell`, so any attribute delta indicates an adapter bug.【F:examples/mui-leptos/README.md†L11-L47】
2. **Validate automation IDs** – The README calls out `data-rustic-*` selectors and their shared origin. When hydration fails, inspect the rendered IDs and update `mui_shared::automation` if new selectors are required across frameworks.【F:examples/mui-leptos/README.md†L1-L60】
3. **Replay telemetry instrumentation** – Port snippets from `selection-controls-leptos` into a scratch view to ensure telemetry hooks still log as expected; the example shows how to wire `TelemetryHooks` and `leptos::logging` so hydration order issues become obvious.【F:examples/selection-controls-leptos/README.md†L1-L64】

## Automation and logging tips

- **Structured logging** – `selection-controls-leptos` logs render and change events with channel labels (`selection_controls::{channel}`), making hydration regressions easy to correlate with component state.【F:examples/selection-controls-leptos/README.md†L1-L64】 Ensure new adapters continue to call `TelemetryHooks::on_render` before consumer callbacks so analytics spans wrap the full lifecycle.
- **Deterministic automation IDs** – When adding telemetry fields, update `AutomationIdBuilder` once and let the Leptos adapter ingest the new selectors; this keeps SSR/CSR diffs and logging aligned across frameworks.【F:examples/mui-leptos/README.md†L1-L60】 Archive diffs in incident docs so Dioxus, Sycamore, and Yew maintainers can reuse them.

## Regression checklist

- Run `cargo xtask examples --group hydration` to rebuild Leptos and peer bootstrap binaries, ensuring SSR generators and wasm targets still compile together.【F:crates/xtask/src/main.rs†L439-L516】【F:crates/xtask/src/main.rs†L516-L708】
- Execute `cargo run --manifest-path examples/mui-leptos/Cargo.toml --features ssr` and `cargo test --package rustic_ui_leptos_example` before merging to prove the marketing shell renders deterministic HTML and telemetry spans.【F:examples/mui-leptos/README.md†L17-L37】
- When telemetry payloads change, update the snippets in `selection-controls-leptos` and capture sample logs (`leptos::logging::log!` output) so QA automation can compare pre/post hydration behaviour without writing new harnesses.【F:examples/selection-controls-leptos/README.md†L1-L64】
