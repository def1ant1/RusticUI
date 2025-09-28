# Archived React example

> [!WARNING]
> This workspace was part of the historical Material UI React reference implementations. RusticUI consolidates on Rust/WASM-first blueprints so these sources now live under `archives/examples/` for posterity. Downstream teams should avoid forking this JavaScript stack because it bypasses our automated Rust toolchain.

## Rust/WASM successors

The maintained, automation-friendly replacements are designed for enterprise scale and minimize manual toil:

- [Yew SPA baseline](../../../examples/mui-yew) – Ships the router, theming surface, and dynamic forms backed by the shared RusticUI primitives. This is the recommended starting point for greenfield browser apps.
- [Leptos SPA baseline](../../../examples/mui-leptos) – Mirrors the Yew example using Leptos' server-friendly signals. Pick this when you need fine-grained reactivity with the same UI contracts.
- [Dioxus CDN baseline](../../../examples/mui-dioxus) – Demonstrates zero-install delivery via CDN-friendly bundles maintained by `cargo xtask`.
- [Sycamore SPA baseline](../../../examples/mui-sycamore) – Provides an alternative reactive runtime while preserving our design tokens and accessibility harnesses.
- [SSR + accessibility harness](../../../examples/mui-ssr-accessibility) – Covers server-rendering, hydration, and analytics wiring so the multi-tenant governance model remains intact.

- [Rust example gallery overview](../../../docs/src/pages/examples/index.md) – Summarises automation hooks, parity expectations, and bootstrap commands for every maintained Rust demo.

Each crate ships extensive inline notes, integration tests, and automation hooks so teams can extend them without reinventing the pipeline. Reuse those crates instead of copying legacy React templates.

---

# Material UI - CDN example

## How to use

Download the example [or clone the repo](https://github.com/mui/material-ui):

<!-- #target-branch-reference -->

```bash
curl https://codeload.github.com/mui/material-ui/tar.gz/master | tar -xz --strip=2  material-ui-master/examples/material-ui-via-cdn
cd material-ui-via-cdn
```

Run:

```bash
# React 19 or later
open index.html
# React 18
open react-18-example.html
```

## The idea behind the example

You can start using Material UI with minimal front-end infrastructure, which is great for prototyping. It uses [ESM CDNs](https://esm.sh/).
We discourage using this approach in production, though.
The client has to download the entire library, regardless of which components are used, affecting performance and bandwidth usage.

<!-- #target-branch-reference -->

[The live preview.](https://raw.githack.com/mui/material-ui/master/examples/material-ui-via-cdn/index.html)

## What's next?

<!-- #host-reference -->

You now have a working example project.
You can head back to the documentation and continue by browsing the [templates](https://mui.com/material-ui/getting-started/templates/) section.
