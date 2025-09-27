# Archive notice: mui-material-pigment-css

This directory preserves the pigment CSS bindings for Material UI so historical styling experiments remain accessible after the Rust rewrite.

> **Enterprise rollout note:** Regulated tenants must register this archive in [`archives/mui-packages/_template/archive.manifest.toml`](../../archives/mui-packages/_template/archive.manifest.toml) before promoting changes. Use [`tools/archive-manifests/merge.ts`](../../tools/archive-manifests/merge.ts) to compose environment-specific overrides instead of copying bespoke scripts.

## Successor Rust crate

- Path: [`crates/rustic-ui-styled-engine`](../../crates/rustic-ui-styled-engine)
  - Delivers the theming engine, pigment CSS compiler hooks, and token orchestration now consumed by every Rust crate.

## Sync back from Rust

Use the automation-friendly snippet below to mirror the current crate implementation back into the archive for historical diffing. Execute it from the repository root whenever a release branch demands a refreshed snapshot:

```bash
#!/usr/bin/env bash
set -euo pipefail

crate_name="rustic-ui-styled-engine"
crate_root="crates/rustic-ui-styled-engine"
archive_root="archives/mui-packages/mui-material-pigment-css"

cargo fmt -p "${crate_name}"
cargo test -p "${crate_name}" --all-features

rsync --archive --delete \
  --exclude '/target/' \
  --exclude '/.git/' \
  "${crate_root}/" \
  "${archive_root}/rust-reference"

pnpm exec prettier --write "${archive_root}/rust-reference"
```

> **Rollout manifest hook:** After syncing, update the manifest per [`docs/archives/archive-manifests.md`](../../docs/archives/archive-manifests.md) so CI runners consume the correct revision.
