# Responsive layout primitives

Migrating from the legacy `mui_*` layout helpers to the RusticUI headless state
machines unlocks deterministic automation and responsive behaviour across every
adapter.

## Mapping legacy props to state machines

1. **Import the headless state** matching your component (`BoxState`,
   `ContainerState`, `GridState`, `StackState`, `HiddenState`, or
   `ImageListState`). Each state accepts a shared
   [`BreakpointConfig`](../../crates/rustic-ui-headless/src/layout.rs) and
   [`ResponsiveValue`](../../crates/rustic-ui-headless/src/layout.rs) so you can
   describe breakpoint overrides in a single place.
2. **Mirror your existing layout tokens** using the fluent constructors on each
   state. For example, replace `Grid`'s `columns={{ md: 3 }}` prop with:

   ```rust
   let tokens = GridTokens {
       columns: ResponsiveValue::new(1).with_override(Breakpoint::Md, 3),
       column_gap: ResponsiveValue::from(String::from("16px")),
       row_gap: ResponsiveValue::from(String::from("24px")),
   };
   let state = GridState::new(tokens, BreakpointConfig::material()).interactive();
   ```
3. **Feed the state into the Material renderer** (`render_grid`, `render_box`,
   etc.) and forward the returned CSS variables and `style` attributes to your
   framework adapter. The integration tests in
   `crates/rustic-ui-material/tests/layout_renderers.rs` snapshot the generated
   markup to keep SSR and hydration in sync.【F:crates/rustic-ui-material/tests/layout_renderers.rs†L1-L126】

## Automation and accessibility hooks

- The `EMPTY_SEGMENTS` constant exported from
  [`style_helpers`](../../crates/rustic-ui-material/src/style_helpers.rs) allows
  you to request automation ids without additional segments while keeping type
  inference unambiguous. Replace ad-hoc `[]` literals with
  `crate::style_helpers::EMPTY_SEGMENTS` when building QA selectors so the code
  compiles cleanly across stable toolchains.【F:crates/rustic-ui-material/src/style_helpers.rs†L1-L120】
- Attribute builders expose the correct `role`, `aria-*`, and `data-*` metadata
  for every primitive. Hidden sections can opt into `data-inert` to block
  pointer/focus interaction while still rendering for assistive technology.
  Snapshot tests in `crates/rustic-ui-headless/tests/layout_primitives.rs` verify
  that each breakpoint emits stable automation hooks.【F:crates/rustic-ui-headless/tests/layout_primitives.rs†L1-L209】

## CI guardrails

Run the standard workspace automation after migrating components:

- `cargo fmt` – formats the Rust sources to keep diffs reviewable.
- `cargo clippy --workspace --all-features` – ensures lint parity across all
  adapters.
- `INSTA_UPDATE=always cargo test -p rustic-ui-headless --test layout_primitives`
  and `INSTA_UPDATE=always cargo test -p rustic-ui-material --test layout_renderers`
  – refresh responsive snapshots and confirm no regressions.
- `cargo xtask build-docs` – rebuilds the Rust book and ensures the new guidance
  ships with the documentation site.【F:crates/xtask/src/main.rs†L780-L832】

Following the sequence above keeps server-rendered markup, hydration output, and
QA automation aligned as you roll out the new layout primitives.
