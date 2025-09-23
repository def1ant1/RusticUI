# Joy workflow – Leptos

The Leptos variant reuses the [`joy-workflows-core`](../joy-workflows-core)
state machine to deliver the exact same Joy workflow without duplicating any
business logic. The view is composed with `view!` templates and leverages Joy
surface tokens to keep styling consistent with the Yew build.

## Highlights

- **Signal driven orchestration** – a single `JoyWorkflowMachine` instance feeds
  a `RwSignal<JoyWorkflowSnapshot>`, so every interaction runs through the shared
  `apply` helper and re-renders declaratively.
- **Token aware styling** – `resolve_surface_tokens` drives the environment chip,
  action buttons, and snackbar just like the Yew implementation.
- **Automation hooks** – the analytics identifiers from the blueprint are wired
  into `data-analytics-id` attributes, enabling parity checks between SSR output
  and hydrated DOM trees.

## Running the demo (CSR)

```bash
rustup target add wasm32-unknown-unknown
cd examples/joy-leptos
trunk serve --open
```

The workflow is served from `http://127.0.0.1:8081` with live reload.

## Server-side rendering snapshot

```bash
cargo run --manifest-path examples/joy-leptos/Cargo.toml --features ssr
```

The SSR binary prints the same lifecycle summary as the other adapters so teams
can diff output across frameworks during parity audits.
