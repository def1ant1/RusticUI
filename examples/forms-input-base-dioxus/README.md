# InputBase blueprint – Dioxus

Dioxus consumes the shared `InputBase` story to surface automation selectors in both SSR and client renderers.
This example mirrors the other frameworks but uses Dioxus components to display the markup, analytics hints, and hydration guidance.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Run the interactive example | `cargo run --manifest-path examples/forms-input-base-dioxus/Cargo.toml` | Renders the blueprint in the desktop shell. |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/forms-input-base-dioxus/Cargo.toml` | Produces deterministic assets under `target/forms-input-base/dioxus`. |
| Compile for WebAssembly hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/forms-input-base-dioxus/Cargo.toml` | Confirms the Dioxus adapter is wasm-ready for CI. |

Use `./examples/forms-input-base-dioxus/scripts/bootstrap.sh` to prep the toolchain and refresh the SSR snapshot in one command.
