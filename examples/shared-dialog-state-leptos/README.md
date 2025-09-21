# Shared Dialog State – Leptos Blueprint

The Leptos workspace mirrors the Yew automation demo while proving that the
shared overlay state can be dropped into a reactive `view!` environment without
rewriting validation or focus management.  By depending on
[`shared-dialog-state-core`](../shared-dialog-state-core) every interaction
remains identical to the Yew/Dioxus/Sycamore variants.

## Highlights

- **Signal driven updates** – the bootstrap project wraps `SharedOverlayState`
  inside Leptos signals so each intent helper (`request_dialog_open`,
  `toggle_popover`, `commit_text`) updates the UI and the lifecycle journal with
  one call.
- **SSR parity** – controlled state ensures that pre-rendered HTML (for example
  from `cargo leptos serve`) matches the hydrated DOM attributes, keeping
  automation selectors (`data-state`, `data-focus-trap`, `aria-*`) stable.
- **Automation journal** – lifecycle messages are streamed into a reactive list
  so QA engineers can diff snapshots or persist them to telemetry sinks.
- **Anchor diagrams** – the app prints the ASCII anchor diagram exported by the
  core crate on startup so designers, QA, and developers share the same mental
  model for collision handling.

## Bootstrapping

```bash
./examples/shared-dialog-state-leptos/scripts/bootstrap.sh
cd target/shared-dialog-state-leptos-demo
trunk serve --open
```

The generated project contains:

- A Cargo workspace with a Leptos binary crate wired for the `csr` feature.
- A `main.rs` that demonstrates how to map attribute builders from the shared
  state onto Leptos `view!` templates while preserving automation hooks.
- Inline notes explaining how to extend the pattern with server side rendering
  or telemetry streaming.

Because the shared crate centralizes validation and geometry bookkeeping, the
Leptos, Yew, Dioxus, and Sycamore demos all emit the same `data-*` hooks making
cross-framework smoke tests trivial to maintain.
