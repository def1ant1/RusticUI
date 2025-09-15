# Rust CI and Local Reproduction Guide

This document outlines the automation used in our Rust workspace CI and how to reproduce the steps locally. The CI pipeline is designed to minimize manual work and provide repeatable builds for enterprise-grade reliability.

## Prerequisites
- Rust stable toolchain with `rustfmt`, `clippy`, and `llvm-tools-preview` components
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for WebAssembly tests
- [grcov](https://github.com/mozilla/grcov) for coverage reports
- Chrome or Firefox installed for headless browser tests

Install prerequisites:
```bash
rustup component add rustfmt clippy llvm-tools-preview
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install grcov
```

## Commands
Run the following from the repository root.

### Formatting and Lints
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Tests and Coverage
```bash
cargo test --workspace
# Coverage
grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing -o lcov.info
```

### WebAssembly Tests
Only crates with `wasm-bindgen-test` are executed. Interactive components rely
on the `yew` feature and are automatically audited for accessibility using
[`axe-core`](https://github.com/dequelabs/axe-core).
```bash
for crate in crates/mui-joy crates/mui-material; do
  (cd "$crate" && wasm-pack test --headless --chrome --features yew)
done
```

### Accessibility Audits
The `mui-material` test suite integrates `axe-core` via `wasm-bindgen` to
validate ARIA roles, keyboard navigation and overall accessibility. Tests fail
if any violation is detected, making a11y compliance part of the standard CI
pipeline.

### Component Rendering Assertions

Cross-framework unit tests in `mui-material` render each component and verify
that the generated HTML includes the hashed CSS class and expected ARIA
metadata. These structured assertions guard against regressions in both styling
and accessibility across adapters.

### Documentation and Benchmarks
```bash
cargo doc --no-deps --workspace
cargo bench --workspace || true
```
Artifacts can be found under `target/doc` and `target/criterion` respectively.

## Caching
CI uses [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache) to reuse build output across jobs. Locally, Cargo's own cache handles this automatically.

## Notes
- Coverage results are uploaded to Codecov in CI.
- Benchmark and documentation outputs are uploaded as artifacts for easy inspection.
- When adding new crates, ensure they are listed in `Cargo.toml` and include any necessary test or bench targets.
