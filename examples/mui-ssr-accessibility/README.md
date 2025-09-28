# MUI SSR Accessibility Example

This example demonstrates how the shared Material UI primitives from
[`mui_shared`](../mui-shared) orchestrate a full server-side render (SSR) while
preserving the automation markers consumed by the client-side hydration (CSR)
examples.

## Architecture overview

- [`mui_shared::layout::AppShell`](../mui-shared/src/lib.rs) renders the
  deterministic container, headline copy, and automation markers used by every
  framework variant.
- [`mui_shared::theme::material_example_theme`](../mui-shared/src/lib.rs) keeps
  the colour palette in sync with the CSR demos so `StyledEngineProvider` emits
  the same CSS.
- The SSR entry point renders the header and navigation with Yew so components
  like [`AppBar`](../../crates/rustic-ui-material/src/app_bar.rs) inherit the
  correct ARIA metadata.

Because the HTML document is composed from the shared shell, the output embeds
identical `data-rustic-*` attributes as the CSR counterparts. Automation suites
can diff the SSR snapshot against hydrated DOM trees and assert parity without
framework-specific locators.

## Running the example

```bash
cd examples/mui-ssr-accessibility
cargo run
```

The command prints a complete HTML document that can be embedded directly into a
response body.

## Regression tests

Execute the parity checks with:

```bash
cd examples/mui-ssr-accessibility
cargo test
```

The tests render the document through the same `render_document` helper and
assert that critical `data-rustic-*` markers (shell container, hydration root,
and action block) match the CSR expectations from `examples/mui-yew`.
