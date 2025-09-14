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
Only crates with `wasm-bindgen-test` are executed.
```bash
for crate in crates/mui-joy crates/mui-material; do
  (cd "$crate" && wasm-pack test --headless --chrome)
done
```

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
