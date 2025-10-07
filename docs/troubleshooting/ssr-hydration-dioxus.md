# Dioxus SSR + hydration troubleshooting

Leverage the Quick Start Dioxus bootstrap so the shared scripts install `dx`, wasm targets, and telemetry scaffolding before you investigate hydration bugs.【F:docs/src/pages/getting-started/quick-start.md†L63-L102】 The generated project highlights how automation IDs and hydration helpers map back to the shared crates.

## Primary runbooks

- **Marketing shell parity** – Follow `examples/mui-dioxus`: `npx dioxus-cli@latest serve --platform web` for CSR hydration, `cargo run --manifest-path examples/mui-dioxus/Cargo.toml --features ssr > prerendered.html` for deterministic SSR HTML, and `npx dioxus-cli@latest build --platform web --release` for production bundles.【F:examples/mui-dioxus/README.md†L11-L33】
- **Integration tests** – `cargo test --package rustic_ui_dioxus_example` exercises router descriptors, automation IDs, and hydration-aware theming to catch regressions early.【F:examples/mui-dioxus/README.md†L29-L37】 Pair these runs with the Quick Start automation harness during larger refactors.【F:docs/src/pages/getting-started/quick-start.md†L14-L33】

## Diagnosing hydration mismatches

1. **Regenerate SSR snapshots** – Re-run the SSR command and hydrate it with the dev server. Because both flows reuse `mui_shared::layout::AppShell`, DOM deltas usually indicate adapter state bugs rather than template drift.【F:examples/mui-dioxus/README.md†L11-L47】
2. **Audit automation IDs** – The README documents deterministic `data-rustic-*` selectors produced by `AutomationIdBuilder`. Diff SSR/CSR output to ensure those attributes survive hydration; missing IDs imply outdated descriptors or telemetry wiring.【F:examples/mui-dioxus/README.md†L1-L60】
3. **Replay telemetry harnesses** – `selection-controls-dioxus` provides a reusable `TelemetryRecorder` and `simulate_telemetry_cycle` helper; use them to confirm render/telemetry ordering whenever hydration warnings appear.【F:examples/selection-controls-dioxus/README.md†L1-L64】

## Automation and logging tips

- **Shared smoke orchestration** – The Dioxus selection controls Justfile delegates to `examples/scripts/selection-controls-smoke.sh`, matching CI and producing newline-delimited telemetry you can archive for audits.【F:examples/selection-controls-dioxus/README.md†L17-L64】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】
- **Cross-environment parity** – Host and wasm tests reuse the same `TelemetryRecorder`, so add new assertions there whenever you extend analytics payloads. This keeps hydration regressions visible without manual browser reproduction.【F:examples/selection-controls-dioxus/README.md†L33-L64】

## Regression checklist

- Run `cargo xtask examples --group hydration` to compile Dioxus bootstrap binaries (forms + selection controls) alongside their Yew, Leptos, and Sycamore peers, guaranteeing SSR generators still succeed.【F:crates/xtask/src/main.rs†L439-L516】【F:crates/xtask/src/main.rs†L516-L708】
- Execute `cargo run --manifest-path examples/mui-dioxus/Cargo.toml --features ssr` and `cargo test --package rustic_ui_dioxus_example` after every fix to confirm deterministic HTML and telemetry output.【F:examples/mui-dioxus/README.md†L17-L37】
- Capture `just test-host`, `just test-wasm`, and `just automation-smoke` logs from `selection-controls-dioxus` and attach them to incident retrospectives so multi-framework teams can replay the exact sequence.【F:examples/selection-controls-dioxus/README.md†L17-L64】
