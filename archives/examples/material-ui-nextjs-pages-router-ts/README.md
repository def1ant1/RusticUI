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

# Material UI - Next.js Pages Router example in TypeScript

## How to use

Download the example [or clone the repo](https://github.com/mui/material-ui):

<!-- #target-branch-reference -->

```bash
curl https://codeload.github.com/mui/material-ui/tar.gz/master | tar -xz --strip=2  material-ui-master/examples/material-ui-nextjs-pages-router-ts
cd material-ui-nextjs-pages-router-ts
```

Install it and run:

```bash
npm install
npm run dev
```

or:

<!-- #target-branch-reference -->

[![Edit on StackBlitz](https://developer.stackblitz.com/img/open_in_stackblitz.svg)](https://stackblitz.com/github/mui/material-ui/tree/master/examples/material-ui-nextjs-pages-router-ts)

[![Edit on CodeSandbox](https://codesandbox.io/static/img/play-codesandbox.svg)](https://codesandbox.io/p/sandbox/github/mui/material-ui/tree/master/examples/material-ui-nextjs-pages-router-ts)

## The idea behind the example

**Note:** This example is set up to use the Next.js Pages Router.
As of Next.js 13.4, the newer App Router pattern is stable.
We recommend starting new projects with the [Material UI with Next.js (App Router) example](https://github.com/mui/material-ui/tree/master/examples/material-ui-nextjs-ts) unless you need (or prefer) the Pages Router.

<!-- #host-reference -->

The project uses [Next.js](https://github.com/vercel/next.js), which is a framework for server-rendered React apps.
It includes `@mui/material` and its peer dependencies, including [Emotion](https://emotion.sh/docs/introduction), the default style engine in Material UI.
If you prefer, you can [use styled-components instead](https://mui.com/material-ui/integrations/interoperability/#styled-components).

## The link component

<!-- #target-branch-reference -->
<!-- #host-reference -->

The [example folder](https://github.com/mui/material-ui/tree/HEAD/examples/material-ui-nextjs-pages-router-ts) provides an adapter for the use of [Next.js's Link component](https://nextjs.org/docs/pages/api-reference/components/link) with Material UI.
More information [in the documentation](https://mui.com/material-ui/integrations/routing/#next-js-pages-router).

## What's next?

<!-- #host-reference -->

You now have a working example project.
You can head back to the documentation and continue by browsing the [templates](https://mui.com/material-ui/getting-started/templates/) section.
