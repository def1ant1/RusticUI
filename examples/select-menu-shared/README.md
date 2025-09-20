# RusticUI Select Menu — Shared Primitives

This crate centralizes the mock data loaders, theme overrides, automation ids,
and SSR helpers used by the framework-specific select menu examples. Keeping the
logic in a single package ensures the Yew and Leptos demos render identical
markup, expose the same `data-*` hooks, and hydrate against the same HTML shell.

## What lives here

- `fetch_regions` &mdash; asynchronous loader used by client and server builds to
  exercise loading states.
- `enterprise_theme` &mdash; opinionated palette and typography settings reused by
  every demo so screenshots and SSR output stay consistent.
- `render_select_markup` &mdash; deterministic HTML renderer that bypasses private
  state constructors in `mui-headless` while mirroring Material attributes.
- `selection_summary` &mdash; utility that produces a human-readable status line for
  automation and screen readers.

The helpers bubble the headless disabled bookkeeping through to the shared
renderer so any framework can call `state.set_option_disabled(index, true)` and
receive matching `aria-disabled`/`data-disabled` hooks in both SSR and CSR
renders without patching adapter code.

## Running the unit tests

The crate currently exposes helpers only, but you can still compile the
package and run future unit tests by invoking:

```bash
cargo check --manifest-path examples/select-menu-shared/Cargo.toml
```

`cargo test` will become useful once we add golden HTML assertions around the
renderer.
