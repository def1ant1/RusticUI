# Focus Trap Utilities – Leptos

Leptos consumers can reuse the same automation-friendly focus trap state shared
by the Yew/Dioxus/Sycamore demos.  This workspace renders the sentinel pair and
modal surface using pure Leptos components while keeping the SSR snapshot in
lock-step with [`utils-trap-focus-core`](../utils-trap-focus-core).

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/utils-trap-focus-leptos/Cargo.toml` | Produces `target/utils-trap-focus/leptos/ssr.html` and `hydrate.rs`. |
| Run unit tests | `cargo test --manifest-path examples/utils-trap-focus-leptos/Cargo.toml` | Confirms the SSR renderer emits both focus trap sentinels and surface automation IDs. |
| Compile for WASM hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/utils-trap-focus-leptos/Cargo.toml` | Matches the configuration we expect CI to use for hydration smoke tests. |

## Bootstrap script

Execute `./examples/utils-trap-focus-leptos/scripts/bootstrap.sh` to ensure the
WASM target is installed and regenerate the SSR snapshot in one step.  CI jobs
can invoke the same script to avoid duplicating toolchain checks.
