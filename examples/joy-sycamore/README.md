# Joy workflow – Sycamore

Sycamore rounds out the Joy workflow examples by rendering the shared
[`joy-workflows-core`](../joy-workflows-core) machine with fine-grained
reactivity. Each state transition flows through the core crate so analytics IDs,
snackbar behaviour, and the lifecycle journal remain identical to the other
frameworks.

## Highlights

- **Shared machine** – `Rc<RefCell<JoyWorkflowMachine>>` keeps all logic in the
  central crate while Sycamore signals mirror the resulting snapshot into the UI.
- **Joy token styling** – surfaces reuse `resolve_surface_tokens` ensuring the
  same colors/variants as the Yew, Leptos, and Dioxus variants.
- **Automation parity** – data attributes expose the same analytics IDs, enabling
  parity checks across SSR + hydration.

## Running the demo (CSR)

```bash
rustup target add wasm32-unknown-unknown
cd examples/joy-sycamore
trunk serve --open
```

The workflow is available at `http://127.0.0.1:8082` with live reload.

## Server-side rendering snapshot

```bash
cargo run --manifest-path examples/joy-sycamore/Cargo.toml --features ssr
```

As with the other adapters, the SSR binary prints a concise summary containing
step progress and capacity allocation so teams can diff results across
frameworks.
