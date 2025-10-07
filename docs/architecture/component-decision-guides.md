# Component Decision Guides: Box, Container, Grid & Beyond

RusticUI ships a family of layout primitives that all expose the same headless
state machines and adapter surfaces, yet they target different layout problems.
This guide explains when to reach for each component, how their automation hooks
behave, and the tooling you can run to keep parity guarantees intact across
React, Yew, Leptos, Dioxus, and Sycamore adapters.

## Choosing the right primitive

| Scenario | Recommended primitive | Why it fits |
| --- | --- | --- |
| Apply theme-aware spacing, backgrounds, or analytics hooks around arbitrary content | [`Box`](#box) | Provides the thinnest wrapper over [`render_box`](../../crates/rustic-ui-material/src/box.rs) so adapters simply forward `BoxState` snapshots without touching layout logic. |
| Constrain page width or switch between fluid/fixed breakpoints | [`Container`](#container) | Builds on [`render_container`](../../crates/rustic-ui-material/src/container.rs) to emit deterministic max-width + density attributes that line up with Material Design breakpoints. |
| Compose responsive two-dimensional layouts | [`Grid`](#grid) | Delegates to [`render_grid`](../../crates/rustic-ui-material/src/grid.rs) for breakpoint-aware column counts, gaps, and dense layouts, keeping SSR and hydration output identical. |
| Arrange one-dimensional flows with predictable gaps/dividers | [`Stack`](#stack) | Leverages [`render_stack`](../../crates/rustic-ui-material/src/stack.rs) so adapters only choose direction/density while analytics markers stay centralized. |
| Hide or reveal content at specific breakpoints without branching adapters | [`Hidden`](#hidden) | Wraps [`render_hidden`](../../crates/rustic-ui-material/src/hidden.rs) so automation identifiers and breakpoints stay aligned with the grid system. |

## Box

`Box` should be your default wrapper when you need theme-aware spacing,
backgrounds, or automation identifiers without imposing layout semantics. The
component consumes a [`BoxState`](../../crates/rustic-ui-headless/src/box.rs)
and forwards it through [`render_box`](../../crates/rustic-ui-material/src/box.rs)
so every adapter returns the same inline style string and `data-rustic-*`
markers.【F:crates/rustic-ui-material/src/box.rs†L108-L224】

```rust
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use rustic_ui_headless::r#box::{BoxRole, BoxState, BoxTokens};
use rustic_ui_material::render_box;

let tokens = BoxTokens {
    padding: ResponsiveValue::from(String::from("16px")),
    margin: ResponsiveValue::new(String::from("0"))
        .with_override(Breakpoint::Lg, String::from("auto")),
    background: ResponsiveValue::from(String::from("var(--surface)")),
};
let state = BoxState::new(tokens, BreakpointConfig::material()).with_role(BoxRole::Region);
let render = render_box(&state);
assert!(render.inline_style().contains("--rustic_ui_box_padding"));
```

:::info Automation
Run `cargo test -p rustic-ui-material --test layout_renderers -- material_box`
to confirm the SSR snapshot stays aligned with the adapter wrappers whenever you
change padding tokens or automation identifiers. Pair the snapshot refresh with
`cargo xtask parity-report --check` so CI blocks merges when the adapter
dashboard drifts from the regenerated renderer output.
:::

## Container

Use `Container` when you need responsive gutters or fixed-width layouts. The
adapter delegates to [`render_container`](../../crates/rustic-ui-material/src/container.rs),
which emits max-width breakpoints, density toggles, and automation IDs that are
shared across every framework integration.【F:crates/rustic-ui-material/src/container.rs†L94-L220】

```rust
use rustic_ui_headless::container::{ContainerRole, ContainerState, ContainerTokens};
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use rustic_ui_material::render_container;

let tokens = ContainerTokens {
    max_width: ResponsiveValue::new(String::from("540px"))
        .with_override(Breakpoint::Md, String::from("960px")),
    padding_inline: ResponsiveValue::new(String::from("16px"))
        .with_override(Breakpoint::Lg, String::from("32px")),
};
let state = ContainerState::new(tokens, BreakpointConfig::material())
    .with_role(ContainerRole::Presentation)
    .fixed(true);
let render = render_container(&state);
assert!(render.inline_style().contains("--rustic_ui_container_padding_inline"));
```

:::tip Automation
`cargo test -p rustic-ui-material --test layout_renderers -- material_container`
keeps the container snapshot (including `data-breakpoint-*` hooks) synchronized
across adapters. Pair it with `cargo xtask parity-report --check` before landing
changes so the docs parity dashboard and CI guardrail stay in lockstep.
:::

## Grid

Reach for `Grid` when you need responsive, multi-column layouts that collapse or
densify across breakpoints. The adapters call
[`render_grid`](../../crates/rustic-ui-material/src/grid.rs), guaranteeing that
column counts, gaps, and automation attributes remain identical across SSR and
client renderers.【F:crates/rustic-ui-material/src/grid.rs†L92-L220】

```rust
use rustic_ui_headless::grid::{GridState, GridTokens};
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use rustic_ui_material::render_grid;

let tokens = GridTokens {
    columns: ResponsiveValue::new(2)
        .with_override(Breakpoint::Sm, 4)
        .with_override(Breakpoint::Lg, 6),
    column_gap: ResponsiveValue::new(String::from("16px"))
        .with_override(Breakpoint::Md, String::from("24px")),
    row_gap: ResponsiveValue::from(String::from("32px")),
};
let state = GridState::new(tokens, BreakpointConfig::material()).dense(true);
let render = render_grid(&state);
assert!(render.inline_style().contains("--rustic_ui_grid_column_gap"));
```

:::warning Automation
The same `layout_renderers` suite exposes `material_grid` snapshots. Execute
`cargo test -p rustic-ui-material --test layout_renderers -- material_grid`
after adjusting breakpoints so the recorded HTML/CSS stays current.
:::

## Stack

`Stack` targets one-dimensional flows such as button groups or media cards. It
uses [`render_stack`](../../crates/rustic-ui-material/src/stack.rs) to apply
responsive direction changes, spacing, and optional dividers—all while stamping
consistent automation hooks.【F:crates/rustic-ui-material/src/stack.rs†L83-L210】
This keeps React/Yew/Leptos adapters thin wrappers that simply switch between
horizontal and vertical layouts based on the shared state.

```rust
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use rustic_ui_headless::stack::{StackDirection, StackRole, StackState, StackTokens};
use rustic_ui_material::render_stack;

let tokens = StackTokens {
    direction: ResponsiveValue::new(StackDirection::Vertical)
        .with_override(Breakpoint::Lg, StackDirection::Horizontal),
    gap: ResponsiveValue::from(String::from("12px")),
    divider: ResponsiveValue::new(Some(String::from("1px solid var(--border)"))),
};
let state = StackState::new(tokens, BreakpointConfig::material()).with_role(StackRole::List);
let render = render_stack(&state);
assert!(render.inline_style().contains("--rustic_ui_stack_gap"));
```

## Hidden

When you need to hide content at specific breakpoints without branching adapter
logic, `Hidden` wraps [`render_hidden`](../../crates/rustic-ui-material/src/hidden.rs)
and emits the same deterministic automation hooks that power the grid system.【F:crates/rustic-ui-material/src/hidden.rs†L68-L182】
This keeps SSR snapshots aligned with hydration regardless of which framework is
rendering the tree.

:::info Automation
Hidden participates in the same layout renderer tests. Run
`cargo test -p rustic-ui-material --test layout_renderers -- material_hidden`
when changing breakpoint logic, followed by `cargo xtask parity-report`
locally and `cargo xtask parity-report --check` in CI to refresh the docs
tables and parity checks automatically.
:::

## Keep the docs in sync

All of these primitives surface in the cross-adapter parity dashboard. After
updating any of them:

1. Refresh the markdown via `cargo xtask parity-report` so
   [`adapter-parity.md`](./adapter-parity.md) reflects the latest adapter
   coverage, then run `cargo xtask parity-report --check` to confirm no drift
   remains.
2. Run the targeted `layout_renderers` tests listed above to ensure SSR snapshots
   and automation hooks stay deterministic.
3. Commit both the source changes and regenerated docs so CI parity guards stay
   green.
