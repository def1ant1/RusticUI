# Shared Dialog State Core

This crate hosts the shared `DialogState`, `PopoverState`, and `TextFieldState`
composition consumed by the framework specific blueprints under
`examples/shared-dialog-state-*`.  Keeping the orchestration in one place ensures
Yew, Leptos, Dioxus, and Sycamore demos all:

- **Mirror SSR and hydration** – the helpers synchronise controlled state so the
  same snapshot can be rendered on the server and re-used by the client runtime.
- **Expose automation hooks** – analytics identifiers, validation metadata, and
  anchor diagrams all derive from the state container, guaranteeing deterministic
  output for CI pipelines.
- **Scale effortlessly** – product teams can copy this crate into mono-repos or
  publish it internally, allowing dozens of applications to share identical
  overlays without duplicating validation or focus trap plumbing.

Run the unit tests with:

```bash
cargo test --manifest-path examples/shared-dialog-state-core/Cargo.toml
```

The ASCII diagram exported via `ANCHOR_DIAGRAM` appears in every example README
so stakeholders share the same mental model for anchor geometry and collision
resolution.
