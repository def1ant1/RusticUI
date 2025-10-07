# Sycamore SSR + hydration troubleshooting

Run through the Quick Start Sycamore bootstrap so Trunk, wasm targets, and shared automation IDs are in place before debugging hydration mismatches.【F:docs/src/pages/getting-started/quick-start.md†L74-L110】 The script emits inline commentary explaining how SSR stubs, hydration hooks, and automation metadata stay aligned with other frameworks.

## Primary runbooks

- **Marketing shell checks** – `examples/mui-sycamore` outlines the core loop: `trunk serve --open` for CSR hydration, `cargo run --manifest-path examples/mui-sycamore/Cargo.toml --features ssr > prerendered.html` for deterministic SSR HTML, and `trunk build --release` for production bundles.【F:examples/mui-sycamore/README.md†L11-L33】
- **Unit + integration tests** – `cargo test --package rustic_ui_sycamore_example` validates routing, automation IDs, and hydration-aware theming so regressions surface prior to release.【F:examples/mui-sycamore/README.md†L29-L37】 Combine with the Quick Start harness after modifying shared descriptors or telemetry hooks.【F:docs/src/pages/getting-started/quick-start.md†L14-L33】

## Diagnosing hydration mismatches

1. **Regenerate SSR output** – Re-run the SSR command and reload it via the Trunk dev server. Shared descriptors ensure markup drift points to adapter bugs instead of template skew.【F:examples/mui-sycamore/README.md†L11-L60】
2. **Inspect automation metadata** – The README details deterministic `data-rustic-*` attributes emitted from `AutomationIdBuilder`. Diff SSR/CSR HTML to confirm those selectors survive hydration, updating `mui_shared::automation` if new IDs are required.【F:examples/mui-sycamore/README.md†L1-L60】
3. **Leverage telemetry demos** – `selection-controls-sycamore` exposes `render_ssr`, `hydrate_web_app`, and `simulate_nominal_cycle` helpers so you can replay telemetry ordering without crafting new harnesses.【F:examples/selection-controls-sycamore/README.md†L1-L88】 Use them to confirm hydration order after any fix.

## Automation and logging tips

- **Deterministic telemetry recorder** – The Sycamore selection controls crate records channel, phase, and detail strings for every event; assert on these in tests to prove render → telemetry → handler ordering across host and wasm builds.【F:examples/selection-controls-sycamore/README.md†L33-L88】
- **Shared smoke orchestration** – `just automation-smoke` calls `examples/scripts/selection-controls-smoke.sh`, mirroring CI and emitting newline-delimited telemetry you can stash alongside SSR snapshots during incident response.【F:examples/selection-controls-sycamore/README.md†L21-L88】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】

## Regression checklist

- Run `cargo xtask examples --group hydration` to rebuild Sycamore bootstrap binaries and their cross-framework peers, catching SSR generator regressions before they hit CI.【F:crates/xtask/src/main.rs†L439-L516】【F:crates/xtask/src/main.rs†L516-L708】
- Execute `cargo run --manifest-path examples/mui-sycamore/Cargo.toml --features ssr` and `cargo test --package rustic_ui_sycamore_example` to verify deterministic SSR HTML and telemetry output.【F:examples/mui-sycamore/README.md†L17-L37】
- Archive logs from `selection-controls-sycamore` (`just smoke`, `just automation-smoke`, host/wasm tests) with each fix so QA and other framework teams can replay the same hydration cycle without bespoke tooling.【F:examples/selection-controls-sycamore/README.md†L17-L103】
