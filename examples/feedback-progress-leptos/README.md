# feedback-progress-leptos

Automation harness demonstrating Material progress renderers with Leptos.
The crate compiles for both native and WASM targets so CI can assert that the
renderers remain deterministic and cross-framework adapters stay in sync.

## Running locally

```bash
cargo run --package feedback-progress-leptos
cargo run --package feedback-progress-leptos --target wasm32-unknown-unknown
```
