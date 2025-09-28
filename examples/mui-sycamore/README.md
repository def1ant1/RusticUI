# MUI Sycamore Example

This example consumes the shared `mui-shared` crate to render the Material UI
hero layout with Sycamore. Routes, layout copy, automation identifiers, and the
dual light/dark theme blueprint are all sourced from the shared primitives so
the Sycamore surface stays in lock-step with the Leptos/Yew/Dioxus adapters.

## Highlights

- **Typed routing:** `sycamore-router` drives both CSR navigation and the SSR
  fallback via `StaticRouter`, ensuring `/` and `/about` emit identical markup
  regardless of render mode.
- **Deterministic automation:** Every `data-rustic-*` attribute is generated via
  `mui_shared::automation::AutomationIdBuilder`, matching the selectors used by
  other frameworks and the archival Next.js reference.
- **Hydration-aware theme switch:** The `ModeSwitch` component implements a
  two-phase state machine (`HydrationPhase::Server` → `HydrationPhase::Client`)
  so the markup remains inert during SSR yet reacts immediately after client
  hydration.
- **Showcase parity:** Alert, slider, and popover samples mirror the shared
  demo content with explicit comments documenting SSR/CSR behaviour for QA
  automation.

## Usage

### Client-side hydration

```bash
trunk serve --open
```

The CSR bundle will hydrate any SSR markup emitted by the server build; when no
markup is present it renders from scratch.

### Server-side rendering

```bash
cargo run --manifest-path examples/mui-sycamore/Cargo.toml --features ssr
```

The process prints a full HTML document using the shared `AppShell` helper. Pipe
the output into your preferred response writer and allow the CSR build above to
hydrate it in the browser.

### Tests

```bash
cargo test --manifest-path examples/mui-sycamore/Cargo.toml
```

Unit tests cover the typed route descriptors, automation builder determinism,
and the mode switch state transitions to guarantee SSR/CSR parity across
refactors.
