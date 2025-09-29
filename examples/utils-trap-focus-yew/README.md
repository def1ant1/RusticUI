# Focus Trap Utilities – Yew

This workspace demonstrates how the rustic-ui focus trap sentinels integrate
with a Yew component tree.  The implementation mirrors the shared snapshot from
[`utils-trap-focus-core`](../utils-trap-focus-core) so the SSR HTML and the
hydrated DOM stay byte-for-byte compatible for automation diffing.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/utils-trap-focus-yew/Cargo.toml` | Writes `ssr.html` and `hydrate.rs` under `target/utils-trap-focus/yew` using the shared story. |
| Run unit tests | `cargo test --manifest-path examples/utils-trap-focus-yew/Cargo.toml` | Exercises the server renderer to confirm sentinel attributes and automation hooks remain intact. |
| Compile for WASM hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/utils-trap-focus-yew/Cargo.toml` | Ensures the component compiles with the WebAssembly target that CI uses for hydration bundles. |

## Bootstrap script

Use `./examples/utils-trap-focus-yew/scripts/bootstrap.sh` to provision the
toolchain (host + wasm) and regenerate the SSR snapshot.  The script delegates to
`cargo run --bin bootstrap` so CI jobs can reuse the same entry point without
shell duplication.
