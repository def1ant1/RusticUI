# Rust CI and Local Reproduction Guide

This document outlines the automation used in our Rust workspace CI and how to reproduce the steps locally. The CI pipeline is designed to minimize manual work and provide repeatable builds for enterprise-grade reliability, covering unit tests, Joy headless suites, WebAssembly smoke tests across every framework adapter, and the Joy snapshot parity checks.

## Prerequisites
- Rust stable toolchain with `rustfmt`, `clippy`, and `llvm-tools-preview` components
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for WebAssembly tests
- Latest Chrome or Chromium for headless browser execution (Firefox is optional for local debugging)
- [`wasm-bindgen-test`](https://rustwasm.github.io/docs/wasm-bindgen/reference/wasm-bindgen-test/introduction.html) (already listed as a dev-dependency in the crates, add it when authoring new suites)
- [grcov](https://github.com/mozilla/grcov) for coverage reports

Install prerequisites:
```bash
rustup component add rustfmt clippy llvm-tools-preview
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install wasm-bindgen-cli # provides wasm-bindgen-test runners if you extend the suites
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

This is the quickest way to surface failures in the shared headless state machines, Material adapter suites, and the Joy headless unit tests under `crates/mui-joy/tests/headless_state_tests.rs`. CI calls the same entrypoint via `cargo xtask test`, which additionally checks that each example still compiles for `wasm32-unknown-unknown`.

### Joy snapshot parity suites
Joy UI ships SSR renderers for every supported framework. The parity suites compare each adapter to the canonical React output so teams can guarantee hydration-safe markup whenever Joy tokens evolve. Target a single framework or run the whole matrix:

```bash
# Yew parity
cargo test -p mui-material --test joy_yew --features yew

# Leptos parity
cargo test -p mui-material --test joy_leptos --features leptos

# Dioxus parity
cargo test -p mui-material --test joy_dioxus --features dioxus

# Sycamore parity
cargo test -p mui-material --test joy_sycamore --features sycamore
```

Each suite consumes the shared fixtures in `crates/mui-material/tests/common/fixtures.rs` so updating the canonical props or Joy analytics hooks automatically propagates across frameworks.

### WebAssembly integration tests
Interactive components execute inside a headless Chrome instance using the `wasm-bindgen-test` harness. Install Chrome/Chromium locally so `wasm-pack test --headless --chrome` can launch the browser. The fastest way to exercise every crate/feature pair is:

```bash
cargo xtask wasm-test
```

CI relies on this command to build and run WebAssembly tests for both `mui-joy` and `mui-material`. To run suites individually (useful when isolating regressions) call the underlying commands directly—one per framework adapter:

```bash
# Joy UI adapters
(cd crates/mui-joy && wasm-pack test --headless --chrome -- --no-default-features --features yew)
(cd crates/mui-joy && wasm-pack test --headless --chrome -- --no-default-features --features leptos)
(cd crates/mui-joy && wasm-pack test --headless --chrome -- --no-default-features --features dioxus)
(cd crates/mui-joy && wasm-pack test --headless --chrome -- --no-default-features --features sycamore)

# Material adapters
(cd crates/mui-material && wasm-pack test --headless --chrome -- --no-default-features --features yew)
(cd crates/mui-material && wasm-pack test --headless --chrome -- --no-default-features --features leptos)
(cd crates/mui-material && wasm-pack test --headless --chrome -- --no-default-features --features dioxus)
(cd crates/mui-material && wasm-pack test --headless --chrome -- --no-default-features --features sycamore)
```

The `--no-default-features` flag mirrors CI by ensuring optional adapters declare their dependencies explicitly. When a run fails because Chrome cannot be located, set `CHROME` or `CHROMIUM` to the browser executable path. Browser console output is captured automatically, so rerun with `-- --nocapture` to view detailed logs.

### Snapshot maintenance workflow
When a Joy snapshot test fails, the panic message includes both the framework-specific markup and the React baseline. Use `-- --nocapture --exact` to focus on the failing test:

```bash
cargo test -p mui-material yew_button_matches_react_baseline --features yew -- --nocapture --exact
```

Typical remediation steps:

1. Confirm whether the React renderer (`mui_material::button::react`, `mui_material::chip::react`, etc.) changed intentionally. If so, update the corresponding framework adapter module so it emits the new markup.
2. If analytics hooks or accessibility IDs changed globally, adjust the shared fixtures in `crates/mui-material/tests/common/fixtures.rs` so every parity suite receives the same canonical data.
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
