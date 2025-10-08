# Rust CI and Local Reproduction Guide

This document outlines the automation used in our Rust workspace CI and how to reproduce the steps locally. The CI pipeline is designed to minimize manual work and provide repeatable builds for enterprise-grade reliability, covering unit tests, Joy headless suites, WebAssembly smoke tests across every framework adapter, and the Joy snapshot parity checks.

## Prerequisites
- Rust stable toolchain with `rustfmt`, `clippy`, and `llvm-tools-preview` components
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for WebAssembly tests
- Latest Chrome or Chromium for headless browser execution (Firefox is optional for local debugging)
- [`wasm-bindgen-test`](https://rustwasm.github.io/docs/wasm-bindgen/reference/wasm-bindgen-test/introduction.html) (already listed as a dev-dependency in the crates, add it when authoring new suites)
- [`mdBook`](https://rust-lang.github.io/mdBook/) for the Rust-first guide rendered during docs packaging
- [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) for bundling the docs wasm artifacts
- [Playwright](https://playwright.dev/docs/intro) with Chromium support so docs wasm tests can execute without pnpm
- [grcov](https://github.com/mozilla/grcov) for coverage reports

Install prerequisites:
```bash
rustup component add rustfmt clippy llvm-tools-preview
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install wasm-bindgen-cli # provides wasm-bindgen-test runners if you extend the suites
cargo install mdbook
npm install -g pnpm@9 # optional: required only when touching archived tooling manifests
npx playwright install --with-deps chromium
cargo install grcov
```

## Commands
Run the following from the repository root.

### Formatting and Lints
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Running `cargo xtask fmt --check` mirrors the CI lint job by wrapping the two commands above with consistent logging.

### Core workspace tests
```bash
cargo test --workspace --all-features
```

Immediately follow the workspace suite with a targeted feature matrix build to
ensure the `rustic-ui-system` primitives continue compiling when multiple
adapters are toggled:

```bash
cargo check -p rustic-ui-system --features "yew leptos"
```

The guard now runs automatically as part of `cargo xtask test`, catching
duplicate re-export regressions the moment a new primitive is wired up for both
frameworks.

This is the quickest way to surface failures in the shared headless state machines, Material adapter suites, and the Joy headless unit tests under `crates/rustic-ui-joy/tests/headless_state_tests.rs`. CI calls the same entrypoint via `cargo xtask test --examples`, which additionally compiles every Rust example crate for `wasm32-unknown-unknown`.

To reproduce the example verification locally (mirroring CI), install the WebAssembly target and run:

```bash
cargo xtask test --examples
```

The command enumerates every `examples/*/Cargo.toml`, runs `cargo check --target wasm32-unknown-unknown`, and executes `cargo test --target wasm32-unknown-unknown --no-run` for deterministic build coverage. Each example logs its own status block and the task exits non-zero if any example fails to compile.

Compile the layout blueprints for both the native host and WebAssembly targets with:

```bash
cargo xtask examples --group layout --release
```

The curated command wraps `cargo build` for `examples/layout-box-leptos` and `examples/layout-grid-yew`, applies a consistent profile to native and wasm invocations, and fails fast if a manifest ever drifts or the cross-compile breaks.【F:crates/xtask/src/main.rs†L439-L576】

### Joy snapshot parity suites
Joy UI ships SSR renderers for every supported framework. The parity suites compare each adapter to the canonical React output so teams can guarantee hydration-safe markup whenever Joy tokens evolve. Target a single framework or run the whole matrix:

```bash
# Yew parity
cargo test -p rustic-ui-material --test joy_yew --features yew

# Leptos parity
cargo test -p rustic-ui-material --test joy_leptos --features leptos

# Dioxus parity
cargo test -p rustic-ui-material --test joy_dioxus --features dioxus

# Sycamore parity
cargo test -p rustic-ui-material --test joy_sycamore --features sycamore
cargo test -p rustic-ui-material sycamore:: --features sycamore
# Sycamore telemetry harnesses (wasm32)
rustup target add wasm32-unknown-unknown
cargo test --features sycamore -p rustic-ui-material sycamore::telemetry --target wasm32-unknown-unknown -- --nocapture
```

Each suite consumes the shared fixtures in `crates/rustic-ui-material/tests/common/fixtures.rs` so updating the canonical props or Joy analytics hooks automatically propagates across frameworks.

Install the `wasm32-unknown-unknown` target before running the suites; the Sycamore feature relies on the web backend exposed under `sycamore::web`, and the telemetry harnesses expect that backend even though they stub out runtime scheduling. Enable the `sycamore` feature flag so the zero-allocation handlers that power the Sycamore `view!` macros and their analytics-first tests are compiled.

### Yew adapter telemetry harnesses
The Material telemetry harnesses for Yew live alongside the adapters so governance teams can assert analytics ordering without spinning up React parity checks. Because the tests emit synthetic `web_sys` events, they must execute against the `wasm32-unknown-unknown` target with the `wasm-bindgen-test` runner enabled:

```bash
rustup target add wasm32-unknown-unknown
cargo test -p rustic-ui-material --features yew --target wasm32-unknown-unknown -- --nocapture
```

The command spins up the headless wasm harness, executes the inline unit suites (including the switch and radio telemetry choreography checks), and leaves console logging enabled so analytics sequencing can be audited locally. When browser automation is required, the same assertions run under Chrome via `wasm-pack test --headless --chrome -- --no-default-features --features yew`.

### Leptos adapter telemetry harnesses
The Leptos radio telemetry harness mirrors the Yew coverage so security and analytics teams can validate focus/change/commit ordering across frameworks. Because the closures now expect real `leptos::ev` instances, run the suites against the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
cargo test -p rustic-ui-material --features leptos --target wasm32-unknown-unknown -- --nocapture
```

For full browser automation (required when validating Chrome-specific behaviour), execute the same suites via `wasm-pack test --headless --chrome -- --no-default-features --features leptos`. Both commands ensure the telemetry delegates exercise analytics → focus/blur → change → commit ordering for controlled and uncontrolled scenarios before changes reach staging.

### WebAssembly integration tests
Interactive components execute inside a headless Chrome instance using the `wasm-bindgen-test` harness. Install Chrome/Chromium locally so `wasm-pack test --headless --chrome` can launch the browser. The fastest way to exercise every crate/feature pair is:

```bash
cargo xtask wasm-test
```

CI relies on this command to build and run WebAssembly tests for both `rustic-ui-joy` and `rustic-ui-material`. To run suites individually (useful when isolating regressions) call the underlying commands directly—one per framework adapter:

```bash
# Joy UI adapters
(cd crates/rustic-ui-joy && wasm-pack test --headless --chrome -- --no-default-features --features yew)
(cd crates/rustic-ui-joy && wasm-pack test --headless --chrome -- --no-default-features --features leptos)
(cd crates/rustic-ui-joy && wasm-pack test --headless --chrome -- --no-default-features --features dioxus)
(cd crates/rustic-ui-joy && wasm-pack test --headless --chrome -- --no-default-features --features sycamore)

# Material adapters
(cd crates/rustic-ui-material && wasm-pack test --headless --chrome -- --no-default-features --features react)
(cd crates/rustic-ui-material && wasm-pack test --headless --chrome -- --no-default-features --features yew)
(cd crates/rustic-ui-material && wasm-pack test --headless --chrome -- --no-default-features --features leptos)
(cd crates/rustic-ui-material && wasm-pack test --headless --chrome -- --no-default-features --features dioxus)
(cd crates/rustic-ui-material && wasm-pack test --headless --chrome -- --no-default-features --features sycamore)
```

The `--no-default-features` flag mirrors CI by ensuring optional adapters declare their dependencies explicitly. When a run fails because Chrome cannot be located, set `CHROME` or `CHROMIUM` to the browser executable path. Browser console output is captured automatically, so rerun with `-- --nocapture` to view detailed logs.

### Documentation pipeline

CI builds, tests, and stages the documentation site using dedicated `cargo xtask` subcommands. Run the same flow locally to verify mdBook content, the Leptos SSR binary, and Playwright-driven wasm smoke tests:

```bash
cargo xtask docs-build
cargo xtask docs-test
cargo xtask docs-package --dry-run
```

- `docs-build` compiles the SSR binary, static renderer, and wasm bundle in parallel, reusing `CARGO_TARGET_DIR` caches.
- `docs-test` launches Playwright's Chromium bundle against the freshly built wasm assets. Install the browsers once via `npx playwright install --with-deps chromium` or point `PLAYWRIGHT_BROWSERS_PATH` at a cached directory.
- `docs-package --dry-run` validates the static export manifest without mutating the canonical staging directory. Drop the flag when you need to refresh production artifacts in `RUSTIC_DOCS_EXPORT_DIR`.

### Overlay automation suites
The click-away detector and focus-trap state machines now back every modal, drawer and menu overlay. To keep telemetry hooks and accessibility metadata aligned across frameworks, CI exercises a dedicated set of suites:

```bash
# Headless state machine concurrency + property checks
cargo test -p rustic-ui-headless --test click_away_state --test focus_trap_state

# Material adapter parity assertions for Dioxus/Sycamore renderers
cargo test -p rustic-ui-material --test click_away_parity --features dioxus
cargo test -p rustic-ui-material --test click_away_parity --features sycamore
cargo test -p rustic-ui-material --test focus_trap_parity --features dioxus
cargo test -p rustic-ui-material --test focus_trap_parity --features sycamore

# WebAssembly verification for Yew adapters (mirrors CI via wasm-pack)
wasm-pack test --headless --chrome crates/rustic-ui-material -- --features yew --test wasm
```

The parity suites emit Insta snapshots so changes to automation IDs or scoped classes require an intentional snapshot review. Accessibility sweeps now execute through the dedicated Playwright + axe harness under `test/accessibility/`, which boots the docs + example gallery, runs WCAG 2.0/2.1 AA rules, and writes JSON/HTML reports into `test-results/accessibility`. The wasm harness still validates interactive adapters, but the browser-level audits moved into the shared Playwright suite to keep CI orchestration consistent across languages.

### Snapshot maintenance workflow
When a Joy snapshot test fails, the panic message includes both the framework-specific markup and the React baseline. Use `-- --nocapture --exact` to focus on the failing test:

```bash
cargo test -p rustic-ui-material yew_button_matches_react_baseline --features yew -- --nocapture --exact
```

Typical remediation steps:

1. Confirm whether the React renderer (`rustic_ui_material::button::react`, `rustic_ui_material::chip::react`, etc.) changed intentionally. If so, update the corresponding framework adapter module so it emits the new markup.
2. If analytics hooks or accessibility IDs changed globally, adjust the shared fixtures in `crates/rustic-ui-material/tests/common/fixtures.rs` so every parity suite receives the same canonical data.
3. Re-run the targeted test, then `cargo test --workspace --all-features` to ensure no other suites regressed.

This approach keeps the parity harness self-healing—updating either the fixtures or adapter renderers refreshes the "snapshot" without maintaining external files.

### Coverage and documentation
```bash
cargo xtask coverage            # runs cargo test --workspace --all-features and emits lcov.info
cargo doc --no-deps --workspace --all-features
cargo bench --workspace || true
```

Artifacts can be found under `target/doc` and `target/criterion` respectively. Upload `lcov.info` to Codecov (CI does this automatically).

## Caching
CI uses [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache) to reuse build output across jobs. Locally, Cargo's own cache handles this automatically.

## Notes
- Coverage results are uploaded to Codecov in CI.
- Benchmark and documentation outputs are uploaded as artifacts for easy inspection.
- When adding new crates, ensure they are listed in `Cargo.toml` and include any necessary test or bench targets.
- For new wasm suites, add `wasm-bindgen-test` and mark functions with `#[wasm_bindgen_test]` so they run consistently via `wasm-pack` locally and in CI.
