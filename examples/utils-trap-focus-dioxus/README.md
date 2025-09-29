# Focus Trap Utilities – Dioxus

The Dioxus harness stitches together the shared focus trap story and exposes the
same analytics hooks used by Yew, Leptos, and Sycamore.  The bootstrap command
writes an SSR snapshot plus a hydration stub that launches
`TrapFocusHarness` with `dioxus_web::launch`.

## Commands

| Task | Command | Notes |
| --- | --- | --- |
| Generate SSR + hydration assets | `cargo run --bin bootstrap --manifest-path examples/utils-trap-focus-dioxus/Cargo.toml` | Emits `target/utils-trap-focus/dioxus/ssr.html` and `hydrate.rs`. |
| Run unit tests | `cargo test --manifest-path examples/utils-trap-focus-dioxus/Cargo.toml` | Verifies the composed markup still contains both sentinel analytics hooks. |
| Compile for WASM hydration | `cargo build --target wasm32-unknown-unknown --manifest-path examples/utils-trap-focus-dioxus/Cargo.toml` | Ensures the harness builds for browser-based hydration pipelines. |

## Bootstrap script

Run `./examples/utils-trap-focus-dioxus/scripts/bootstrap.sh` to install the
required targets (wasm + native) and regenerate the SSR snapshot.  CI can reuse
this entry point to avoid bespoke shell glue.
