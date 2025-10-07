# Yew SSR + hydration troubleshooting

Start with the five-minute Quick Start so Trunk, the wasm toolchain, and the shared automation scripts are installed before you chase Yew-specific mismatches.【F:docs/src/pages/getting-started/quick-start.md†L35-L74】 The guide links directly to the navigation tabs bootstrapper that seeds deterministic SSR snapshots and hydration stubs under `target/`.

## Primary runbooks

- **Rebuild the marketing shell** – Follow the `examples/mui-yew` README sequence: `trunk serve --open` for CSR hydration, `cargo run --manifest-path examples/mui-yew/Cargo.toml --features ssr > prerendered.html` for deterministic SSR HTML, and `trunk build --release` for production bundles.【F:examples/mui-yew/README.md†L11-L33】
- **Exercise framework tests** – `cargo test --package rustic_ui_yew_example` validates router parity, automation determinism, and hydration-aware theming so regressions surface before they reach production.【F:examples/mui-yew/README.md†L29-L37】 Pair this with the Quick Start `cargo xtask quick-start` harness when new routes or showcases land.【F:docs/src/pages/getting-started/quick-start.md†L14-L33】

## Diagnosing hydration mismatches

1. **Capture SSR + CSR diffs** – Re-run the SSR command above and point your running `trunk serve` session at the regenerated HTML. Because both flows share `mui_shared::layout::AppShell`, any attribute drift highlights adapter bugs instead of template divergence.【F:examples/mui-yew/README.md†L11-L47】
2. **Inspect automation IDs** – The README documents `data-rustic-*` selectors and their shared source in `AutomationIdBuilder`. When hydration strips or duplicates nodes, diff the SSR snapshot against CSR markup to confirm those attributes still exist.【F:examples/mui-yew/README.md†L1-L60】 Feed gaps back into `mui-shared` to keep every adapter aligned.
3. **Replay telemetry-focused demos** – `cargo run -p selection-controls-yew` prints each SSR fragment and telemetry payload before handlers fire, making it a quick sanity check when hydration order changes.【F:examples/selection-controls-yew/README.md†L1-L44】

## Automation and logging tips

- **Tail deterministic telemetry** – The selection controls demo logs render, focus, change, and commit spans to stdout in both host and wasm builds so you can diff hydration behaviour without browser tooling.【F:examples/selection-controls-yew/README.md†L1-L64】 Use `cargo host-test` / `cargo wasm-test` (or the `just` recipes) to confirm ordering before shipping fixes.【F:examples/selection-controls-yew/README.md†L9-L36】
- **Share smoke harnesses** – `just automation-smoke` shells into `examples/scripts/selection-controls-smoke.sh`, matching the CI flow and producing newline-delimited telemetry for log aggregation.【F:examples/selection-controls-yew/README.md†L33-L64】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】 Attach these logs to incident reports so other framework owners can replay the same sequence.

## Regression checklist

- Run `cargo xtask examples --group hydration` to rebuild every SSR bootstrapper (forms + selection controls) across Yew, Leptos, Dioxus, and Sycamore before merging fixes.【F:crates/xtask/src/main.rs†L439-L516】【F:crates/xtask/src/main.rs†L516-L708】
- Execute `cargo run --manifest-path examples/mui-yew/Cargo.toml --features ssr` and `cargo test --package rustic_ui_yew_example` to confirm the marketing shell renders stable SSR and telemetry output.【F:examples/mui-yew/README.md†L17-L37】
- Capture logs from `selection-controls-yew` (`cargo host-test`, `cargo wasm-test`, `just smoke`) and archive them alongside the SSR HTML whenever you close a hydration incident so QA can rerun the exact scenario later.【F:examples/selection-controls-yew/README.md†L9-L64】
