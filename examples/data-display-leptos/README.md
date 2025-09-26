# Data display with Leptos

This cookbook sample mirrors the Yew variant but targets Leptos. Both the list
and table reuse the shared HTML renderers from `rustic_ui_material`, giving SSR and
CSR the exact same markup while still benefiting from `rustic_ui_headless` keyboard
semantics.

## Running locally

```bash
cargo run --package data-display-leptos --features csr
```

Switch to SSR mode with:

```bash
cargo run --package data-display-leptos --no-default-features --features ssr
```

The demo renders two `div.demo-panel` containers that mount the generated HTML
via the `inner_html` attribute. Because the renderers stamp deterministic
`data-automation-*` attributes, QA teams can reuse the same selectors across
frameworks and runtime modes.
