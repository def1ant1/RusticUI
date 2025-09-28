# MUI Leptos Example

This example composes `rustic_ui_system` with the [Leptos](https://leptos.dev)
framework and mirrors the archival Material UI Next.js demo using the new
`mui-shared` integration crate. Routes, layout copy, automation identifiers, and
theme metadata now flow from a single source of truth so the Leptos, Yew,
Dioxus, and Sycamore adapters remain perfectly aligned.

## Prerequisites
- Rust nightly or stable with the `wasm32-unknown-unknown` target installed
- [`trunk`](https://trunkrs.dev) for bundling and serving the client build

## Running the demo

### Client side rendering
```bash
trunk serve --open
```
The client bundle hydrates the SSR markup by default. Every navigation element
and layout node exposes deterministic automation identifiers (for example
`data-rustic-app-navigation="app-home-navigation"`). The Mode switch renders
inert markup during SSR and upgrades to a fully interactive select once the
browser hydrates the page.

### Server side rendering
```bash
cargo run --manifest-path examples/mui-leptos/Cargo.toml --features ssr > prerendered.html
# Then in another terminal build the hydrated client bundle
trunk build --release
```
`mui_shared::layout::AppShell::render_ssr_document` composes the deterministic
HTML skeleton (including hero copy, ProTip content, and automation attributes)
with the Leptos markup. The resulting document embeds a
`data-rustic-app-hydration-root` marker scoped to the Leptos framework so the
client runtime can hydrate without DOM drift.

## Routing orchestration

`leptos_router` is wired to the shared `HOME`/`ABOUT` descriptors. The router
feeds those descriptors into `mui_shared::layout::AppShell` so the SSR and CSR
code paths render identical markup. Navigation links, CTAs, and showcases reuse
the same automation IDs across frameworks which keeps localisation and
observability reviews centralised in the shared crate.

## Mode switch state machine

The theme switcher is implemented as a documented state machine. During SSR the
component renders inert markup in the `HydrationPhase::Server` state so the
output is deterministic. Once hydrated the effect transitions to
`HydrationPhase::Client`, captures the system colour preference via
`matchMedia`, and dispatches events so automation can observe
`ModeAction::Select` transitions. The machine preserves the recorded system
preference when toggling back to the "System" option, mirroring the original
React behaviour.

## Showcase parity

Alert, slider, and popover demos consume the shared automation helpers so QA
suites can assert behaviour parity across frameworks. The slider updates a
signal that hydrates cleanly, and the popover toggles deterministic content so
enterprise monitoring can diff SSR vs CSR output without flakiness.

## Testing

```bash
cargo test --package rustic_ui_leptos_example
```

The unit tests validate the router-to-descriptor mapping, automation identifier
determinism, and the mode switch state transitions. These guardrails ensure
future refactors continue to hydrate cleanly while keeping automation hooks
stable for synthetic monitoring.
