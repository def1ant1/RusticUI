# Focus Trap Utilities – Sycamore

Sycamore can hydrate the same focus trap snapshot shared across the other Rust
frameworks.  The generated harness renders the SSR markup verbatim so automation
and analytics IDs remain stable.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/utils-trap-focus-sycamore/Cargo.toml` | Creates `target/utils-trap-focus/sycamore/ssr.html` and `hydrate.rs`. |
| Run unit tests | `cargo test --manifest-path examples/utils-trap-focus-sycamore/Cargo.toml` | Asserts that the sentinel markup and automation hooks are emitted. |
| Compile for WASM hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/utils-trap-focus-sycamore/Cargo.toml` | Matches the CI pipeline used to validate hydration bundles. |

## Bootstrap script

Execute `./examples/utils-trap-focus-sycamore/scripts/bootstrap.sh` to install
the WASM toolchain and regenerate the SSR snapshot in a single command.
