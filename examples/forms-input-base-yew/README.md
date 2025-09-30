# InputBase blueprint – Yew

This example demonstrates how RusticUI's `InputBase` primitive integrates with Yew across both controlled and uncontrolled flows.
It highlights:

- Mirroring the shared `InputState` analytics buffer into deterministic `data-rustic-input-base-*` selectors.
- Hydrating server rendered markup without losing automation attributes.
- Using the shared [`forms-input-base-shared`](../forms-input-base-shared) crate to avoid copy/pasting bootstrap logic across frameworks.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Run the interactive example | `cargo run --manifest-path examples/forms-input-base-yew/Cargo.toml` | Mounts the demo in a headless renderer so analytics logs print to stdout. |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/forms-input-base-yew/Cargo.toml` | Writes deterministic artifacts under `target/forms-input-base/yew`. |
| Compile for WebAssembly hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/forms-input-base-yew/Cargo.toml` | Ensures the component compiles for the hydration target used in CI. |

Use `./examples/forms-input-base-yew/scripts/bootstrap.sh` to provision toolchains and refresh the SSR snapshot in one command.
