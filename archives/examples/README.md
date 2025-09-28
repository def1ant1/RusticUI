# Archived React examples catalog

This directory warehouses the historical Material UI React blueprints that previously lived under `examples/material-ui-*`. Each subfolder now contains an archive notice plus links to the Rust-first successors (`examples/mui-*` crates). We retain the original sources strictly for reference when porting patterns into the WebAssembly stack.

## Governance

- **Read-only snapshots.** Do not modify the archived React code paths except when annotating the README. Ports and experiments should happen in scratch directories before being codified into new `rustic-ui-*` crates.
- **Automation-first replacements.** All successors expose `cargo xtask` hooks, `wasm-bindgen` integration tests, and CI-ready manifests so large teams can scale without bespoke scripts.
- **Traceability.** The README in each archive points to the maintained blueprint and documents any deviations uncovered during the migration.

When a React example requires modernization, start from the mapped Rust crate and only consult these archives to understand any missing ergonomic affordances.
