# Feedback tooltips — multi-framework bootstrap

This example packages the shared tooltip renderer from `rustic_ui_material` into a
single command that emits SSR markup, automation identifiers, and hydration
stubs for every supported framework (Yew, Leptos, Dioxus, Sycamore). The goal is
to eliminate one-off wiring when bootstrapping enterprise dashboards that rely
on contextual help and deterministic automation hooks.

## Quick start

```bash
# From the repository root
cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml
```

The command creates `target/feedback-tooltips/<framework>/` folders containing:

- `ssr.html` – ready-to-serve HTML with portal metadata and the automation root.
- `hydrate.rs` – framework-specific bootstrap showing how to mount the markup and
  reuse the themed overrides.
- `README.md` – reminders about automation contracts and hydration notes.

Each hydration stub keeps the markup intact so automated QA suites can hydrate
the same document while toggling tooltip visibility through the shared
[`TooltipState`](../../crates/rustic-ui-headless/src/tooltip.rs).

## Theme overrides & automation ids

`enterprise_story()` ships a high contrast palette tuned for support teams. The
`automation_id` drives both DOM ids and `data-*` hooks which flow directly into
`ssr.html`. Hydration clients can reuse the returned `Theme` to ensure the
`css_with_theme!` styles match the server output. Analytics pipelines can bind to
`data-automation-root` in the generated HTML to register listeners before
hydration begins.

## Verifying the snapshot

Run the library tests to confirm SSR parity across frameworks:

```bash
cargo test --manifest-path examples/feedback-tooltips/Cargo.toml
```

`cargo test` validates that every framework variant exposes the shared
`data-automation-id` and portal metadata ensuring deterministic selectors for QA
pipelines.
