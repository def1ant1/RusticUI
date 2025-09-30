# InputBase blueprint – Sycamore

This example shows how to hydrate the shared `InputBase` blueprint inside Sycamore applications.
It keeps automation metadata centralised via [`forms-input-base-shared`](../forms-input-base-shared) and focuses on reconciling server rendered markup with Sycamore signals.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Run the interactive example | `cargo run --manifest-path examples/forms-input-base-sycamore/Cargo.toml` | Uses the default native renderer for quick iteration. |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/forms-input-base-sycamore/Cargo.toml` | Creates `target/forms-input-base/sycamore` with SSR, hydration, and automation docs. |
| Compile for WebAssembly hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/forms-input-base-sycamore/Cargo.toml` | Ensures the Sycamore adapter builds for wasm in CI. |

Use `./examples/forms-input-base-sycamore/scripts/bootstrap.sh` to sync toolchains and regenerate the SSR fixture.
