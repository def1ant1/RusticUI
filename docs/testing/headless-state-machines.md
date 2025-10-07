# Headless state machine coverage

The property suites that back the headless disclosure and focus primitives now
live alongside the architectural state diagrams so contributors can trace
invariants from documentation to executable tests.  The newest addition expands
the collapsible region coverage with scenario-driven properties that mirror the
controlled/uncontrolled diagrams outlined in
[`docs/architecture/headless-state-machines.md#collapsible-region-state-machine`](../architecture/headless-state-machines.md#collapsible-region-state-machine).

## What the new suite verifies

- **Uncontrolled transitions stay deterministic.**
  `tests/collapsible_region_state.rs` models user-driven expand/collapse/toggle
  flows as a pure boolean latch to guarantee that snapshots, hydration cycles,
  and analytics hooks see the same ordering documented in the diagrams.
- **Controlled consumers must call `sync`.** The controlled-mode property keeps
  `CollapsibleRegionState` frozen until a `sync` arrives, validating that React
  and Leptos integrations can safely render without racing asynchronous
  callbacks.
- **Transition token serialization is stable.** A shadow `BTreeSet` asserts that
  tokens never duplicate and that `is_transitioning()` mirrors the architectural
  expectation that animations complete in a deterministic order.
- **Focus returns remain intact.** The focus-return property walks through
  repeated collapses/expands to guarantee keyboard users always land on the last
  configured trigger.

Each property is annotated with extensive commentary so teams rolling out new
automation or analytics pipelines can extend the suite without reverse-
engineering business rules.  CI runs the tests via `cargo xtask test --examples`
to keep coverage aligned with the workspace’s release-readiness checklist.

After validating the headless properties, run
`cargo xtask coverage-report` to roll their results into the aggregated
dashboard alongside the TypeScript snapshots and axe-core sweeps. The
[cross-suite coverage dashboard](coverage-overview.md) section explains how the
disciplines map to Rust unit/integration, Playwright snapshots, and the
Markdown accessibility audits so the state-machine maintainers can confirm all
pipelines stayed green.【F:docs/testing/coverage-overview.md†L1-L72】
