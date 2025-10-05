# rustic-docs

`rustic-docs` is the canonical documentation portal for the RusticUI Rust
component ecosystem. The crate demonstrates how Leptos based experiences can
share a single routing tree across CSR, SSR, and static export workflows while
remaining automation-first.

## Deployment contracts

- **Static exports**: `cargo leptos build --release --features ssr` will write
  deterministic HTML snapshots via `rustic_docs::render_static_snapshot`. The
  output directory is controlled by the `RUSTIC_DOCS_EXPORT_DIR` environment
  variable (see `build.rs` for the orchestration contract).
- **SSR service**: the `rustic-docs-server` binary starts an Axum server bound
  to the socket defined in `rustic_docs::default_leptos_options`. Observability
  sinks and runtime feature flags are read from `DocsSiteConfig`, giving
  operators a single struct to customise during deployments.
- **Browser bundle**: the `rustic-docs` binary is compiled to WebAssembly and
  mounts the shared `App` component in the browser. Hydration markers align with
  the SSR/static markup so QA pipelines can diff the DOM across execution
  strategies.

## Automation-first philosophy

The project assumes infrastructure-as-code from day one:

- **Managed observability**: tracing is initialised via environment variables so
  platform teams can point to hosted collectors without patching the binaries.
- **Repeatable CI**: integration tests render the application using the same
  `ThemeProvider` and router stack as production, ensuring pipeline drift is
  detected immediately.
- **Extension points**: the `App` component documents how to plug in additional
  routes, theme overrides, or telemetry sinks. Teams can fork the crate and keep
  the automation scaffolding intact.

## Rust-first content pipeline

- **Theme module**: `rustic_docs::theme` exposes `DocsThemeShell`, `DocsSurface`
  and `ThemeToggleControl` which wrap `rustic-ui-system` tokens so downstream
  applications can compose Material primitives without re-implementing the
  provider wiring.
- **Content inventory**: build-time code generation populates
  `rustic_docs::content::docs_inventory()` and
  `rustic_docs::content::markdown_documents()`. The Leptos/Yew components under
  `rustic_docs::content::leptos_components` and
  `rustic_docs::content::yew_components` project that metadata using RusticUI
  cards, app bars, and Markdown rendering via `pulldown_cmark`.
- **Automation tasks**: `cargo xtask docs-assets` replaces the Node scripts used
  by the legacy site. The command can regenerate the service worker banner and
  emit a screenshot manifest that CI tooling can consume without reaching for
  `pnpm`.

## Running locally

```bash
# serve with SSR
env RUST_LOG=debug cargo run -p rustic-docs --bin rustic-docs-server

# build WebAssembly bundle
cargo build -p rustic-docs --bin rustic-docs --target wasm32-unknown-unknown
```

These commands are intentionally optimised for reuse in CI/CD workflows. They
avoid manual steps and can be dropped into container images or orchestrated by
`cargo xtask` helpers.
