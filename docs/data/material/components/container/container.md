---
productId: material-ui
title: React Container component
components: Container, PigmentContainer
githubLabel: 'component: Container'
githubSource: archives/mui-packages/mui-material/src/Container
---

# Container

<p class="description">The container centers your content horizontally. It's the most basic layout element.</p>

While containers can be nested, most layouts do not require a nested container.

{{"component": "@mui/docs/ComponentLinkHeader", "design": false}}

:::tip Architecture reference
See the [Component decision guides](/docs/architecture/component-decision-guides/)
for when to choose `Container` over `Box`, `Grid`, or `Stack`, and consult the
[Adapter parity dashboard](/docs/architecture/adapter-parity/) before enabling a
new framework so you understand current coverage.
:::

## Fluid

A fluid container width is bounded by the `maxWidth` prop value.

{{"demo": "SimpleContainer.js", "iframe": true, "defaultCodeOpen": false}}

```jsx
<Container maxWidth="sm">
```

## Fixed

If you prefer to design for a fixed set of sizes instead of trying to accommodate a fully fluid viewport, you can set the `fixed` prop.
The max-width matches the min-width of the current breakpoint.

{{"demo": "FixedContainer.js", "iframe": true, "defaultCodeOpen": false}}

```jsx
<Container fixed>
```
