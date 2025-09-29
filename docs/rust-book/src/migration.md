# Migrating from React/JS to Rust WebAssembly

Many teams start with a React application and gradually move performance
critical paths to Rust compiled to WebAssembly.  The `mui-*` crates expose the
same design vocabulary as their JavaScript counterparts which makes the
migration incremental.

1. **Identify hot components** – profiler output in React often reveals the
   components that dominate runtime.  Re‑implement these in a Rust framework
   such as [Yew](https://yew.rs) or [Leptos](https://leptos.dev) and compile to
   WebAssembly using `wasm-bindgen`.
2. **Share styles and themes** – the `Theme` structure is serializable with
   `serde`, allowing existing JSON theme definitions to be reused directly in
   Rust.
3. **Leverage cargo workspaces** – colocate Rust crates and JavaScript packages
   in a single repository.  Automated scripts can build both via CI ensuring the
   generated assets stay in lock‑step.
4. **Deploy static artifacts** – `trunk build --release` produces an optimized
   WebAssembly binary.  Serve it from a CDN alongside the `cargo doc` output for
   near‑zero runtime overhead and effortless horizontal scaling.

> Tip: WebAssembly modules are immutable and cacheable.  Enable HTTP cache
> headers so repeat visitors avoid re‑downloading the module.

### Next steps: responsive layouts

Once the initial migration compiles, audit any usage of legacy `mui_*` layout
helpers. The [responsive layout primitives](./layout-primitives.md) chapter
documents how to port those calls to the new headless states, wire the Material
renderers, and update CI so breakpoint snapshots stay fresh across adapters.

### Automating focus, click-away, and telemetry utilities

With layouts migrated, enable the automation utilities that coordinate overlays
and analytics across frameworks:

1. Read through the headless utility guide to understand how click-away, focus
   trap, and telemetry state machines expose deterministic hooks for SSR and
   hydration.【F:crates/rustic-ui-headless/README.md†L241-L305】
2. Mirror the Material adapter recommendations so each framework integrates the
   utilities without diverging telemetry or attribute wiring.【F:crates/rustic-ui-material/README.md†L233-L275】
3. Add `cargo xtask examples --group automation --release` to your migration
   checklist so every release replays the blueprint harnesses that validate the
   shared utilities end-to-end.【F:crates/rustic-ui-headless/README.md†L297-L305】
