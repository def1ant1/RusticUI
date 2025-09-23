# Joy UI workflows across frameworks

The RusticUI workspace ships a shared Joy workflow machine that keeps SSR,
hydration, and analytics semantics in sync across every supported framework. The
`joy-workflows-core` crate centralises the business logic while the adapter
crates (Yew, Leptos, Dioxus, Sycamore) focus purely on rendering.

## Reference demos

- [Joy workflow – Yew](../../../../examples/joy-yew) (`trunk serve --open`)
- [Joy workflow – Leptos](../../../../examples/joy-leptos) (`trunk serve --open`)
- [Joy workflow – Dioxus](../../../../examples/joy-dioxus) (`dx serve --open`)
- [Joy workflow – Sycamore](../../../../examples/joy-sycamore) (`trunk serve --open`)

Each demo consumes the same `JoyWorkflowMachine` and analytics identifiers so QA
pipelines can reuse automation selectors without branching per framework.

## Parity automation

Coverage is enforced by the `joy-parity` inventory scanner:

```bash
cargo xtask joy-parity
```

The task refreshes [`docs/joy-component-parity.md`](../../../joy-component-parity.md)
which CI consumes to guarantee that the Rust adapters match the canonical React
markup. Running the scanner locally before opening a pull request keeps the
workflows in lockstep and prevents regressions from slipping through SSR or
hydration.
