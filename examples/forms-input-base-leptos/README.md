# InputBase blueprint – Leptos

Leptos signals pair nicely with RusticUI's headless `InputState` builders.
This example shares automation configuration with the other frameworks via [`forms-input-base-shared`](../forms-input-base-shared) and focuses on wiring signal updates, hydration notes, and analytics capture.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Run the interactive example | `cargo run --features csr --manifest-path examples/forms-input-base-leptos/Cargo.toml` | Mounts the component using the browser renderer. |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/forms-input-base-leptos/Cargo.toml` | Emits deterministic HTML and hydration stubs in `target/forms-input-base/leptos`. |
| Compile for WebAssembly hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/forms-input-base-leptos/Cargo.toml` | Ensures the Leptos adapter compiles for the wasm target used in CI. |

Use `./examples/forms-input-base-leptos/scripts/bootstrap.sh` to install toolchains and refresh the SSR snapshot.
