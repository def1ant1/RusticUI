# Data display with Yew

This cookbook sample demonstrates how RusticUI's shared renderers can compose a
Material themed list and table without writing any framework specific markup.
Both widgets rely on `rustic_ui_headless` for keyboard navigation and selection
state, while the Yew adapter simply injects the returned HTML string into the
component tree.

## Running locally

```bash
wasm-pack build --target web # ensure the WASM toolchain is configured
cargo run --package data-display-yew
```

The example renders two panels:

- **List** – Compact density with primary and secondary typography showcasing
  deterministic `data-automation-*` hooks.
- **Table** – Zebra striped grid with numeric alignment that reuses the same
  headless `ListState` for selection.

Because the renderers live in `rustic_ui_material`, the exact same HTML is emitted for
SSR and CSR environments which keeps QA automation and accessibility audits
stable.
