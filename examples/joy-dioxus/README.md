# Joy workflow – Dioxus

The Dioxus flavour of the Joy workflow consumes the same
[`joy-workflows-core`](../joy-workflows-core) machine used by Yew and Leptos. The
rendering logic relies on `rsx!` templates and Dioxus signals while all business
rules (step transitions, snackbar logic, analytics IDs) remain centralised.

## Highlights

- **Framework agnostic state** – `use_ref` stores `JoyWorkflowMachine` so event
  handlers mutate the shared logic directly before replacing the snapshot state.
- **Joy token styling** – Buttons, chips, and snackbars use `resolve_surface_tokens`
  ensuring visual parity with the Yew and Leptos builds.
- **Analytics ready** – data attributes surface the blueprint analytics IDs so
  cross-framework parity checks can assert identical SSR output.

## Running the demo (CSR)

```bash
cd examples/joy-dioxus
dx serve --open
```

Any Dioxus-compatible dev server (`dx serve`) will compile the workflow for
WebAssembly and apply live reload.

## Server-side rendering snapshot

```bash
cargo run --manifest-path examples/joy-dioxus/Cargo.toml --features ssr
```

The SSR binary prints a short summary including the active step and capacity
allocation, matching the other framework adapters exactly.
