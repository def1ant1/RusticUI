# Joy workflow – Yew

This demo renders the shared [`joy-workflows-core`](../joy-workflows-core)
blueprint with the Yew adapter from `rustic_ui_joy`. The entire pipeline – stepper
state, snackbar logic, analytics identifiers, and Joy design tokens – lives in
the shared crate so every framework emits identical automation hooks.

## Highlights

- **Single source of truth** – Buttons, slider thresholds, and the release
  checklist are all described in `joy-workflows-core`. The Yew component simply
  maps callbacks to `JoyWorkflowMachine::apply` and re-renders the snapshot.
- **Joy design tokens** – The example resolves surface tokens via
  `rustic_ui_joy::helpers::resolve_surface_tokens` to keep styling consistent with the
  Joy design language.
- **Analytics ready** – Every button, chip, and snackbar exposes data/analytics
  identifiers from the blueprint so CI can assert SSR + hydration parity.

## Running the demo (CSR)

```bash
rustup target add wasm32-unknown-unknown
cd examples/joy-yew
trunk serve --open
```

The Trunk dev server builds the WebAssembly bundle, applies live reload, and
hosts the Joy workflow at `http://127.0.0.1:8080`.

## Server-side rendering snapshot

```bash
cargo run --manifest-path examples/joy-yew/Cargo.toml --features ssr
```

The SSR binary prints a concise snapshot summarising the active step and
capacity allocation. Because the underlying state machine is deterministic, the
Yew, Leptos, Dioxus, and Sycamore variants all emit the same lifecycle log and
analytics identifiers.
