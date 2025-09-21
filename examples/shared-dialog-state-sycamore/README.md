# Shared Dialog State – Sycamore Blueprint

This workspace demonstrates how the shared dialog/popover/text-field state can
be consumed from a [Sycamore](https://sycamore-rs.netlify.app/) signal graph.
The generated project mirrors the Yew, Leptos, and Dioxus demos while proving
that the automation hooks remain deterministic across frameworks.

## Key traits

- **Reactive reuse** – `SharedOverlayState` is wrapped inside Sycamore signals so
  every intent helper runs once and re-renders the view tree.
- **Automation-friendly markup** – the view sets the same `aria-*` and
  `data-*` attributes emitted by the other demos, ensuring Playwright suites can
  reuse selectors verbatim.
- **Lifecycle journaling** – interactions push entries into a reactive vector
  rendered as an ordered list so analysts can diff behaviour over time.
- **Anchor diagram parity** – the ASCII anchor diagram printed on startup keeps
  teams aligned on popover collision assumptions.

## Bootstrapping

```bash
./examples/shared-dialog-state-sycamore/scripts/bootstrap.sh
cd target/shared-dialog-state-sycamore-demo
trunk serve --open
```

The bootstrap script provisions a Cargo workspace with a Sycamore app crate that
imports `shared-dialog-state-core`. Inline commentary explains how to map the
headless state machines onto Sycamore's `view!` macro while keeping manual work
minimal for large engineering teams.
