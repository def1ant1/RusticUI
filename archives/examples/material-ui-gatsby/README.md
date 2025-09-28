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

Each crate ships extensive inline notes, integration tests, and automation hooks so teams can extend them without reinventing the pipeline. Reuse those crates instead of copying legacy React templates.

---

# Material UI - Gatsby example

## How to use

Download the example [or clone the repo](https://github.com/mui/material-ui):

<!-- #target-branch-reference -->

```bash
curl https://codeload.github.com/mui/material-ui/tar.gz/master | tar -xz --strip=2  material-ui-master/examples/material-ui-gatsby
cd material-ui-gatsby
```

Install it and run:

```bash
npm install
npm run develop
```

## The idea behind the example

<!-- #host-reference -->

The project uses [Gatsby](https://github.com/gatsbyjs/gatsby), which is a static site generator for React.
It includes `@mui/material` and its peer dependencies, including [Emotion](https://emotion.sh/docs/introduction), the default style engine in Material UI.
If you prefer, you can [use styled-components instead](https://mui.com/material-ui/integrations/interoperability/#styled-components).

## What's next?

<!-- #host-reference -->

You now have a working example project.
You can head back to the documentation and continue by browsing the [templates](https://mui.com/material-ui/getting-started/templates/) section.
