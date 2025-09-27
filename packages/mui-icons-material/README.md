# Archive notice: mui-icons-material

This directory preserves the npm wrapper around Material icons so legacy bundlers can still resolve assets during audits while Rust takes over icon generation.

> **Enterprise rollout note:** Regulated tenants must register this archive in [`archives/mui-packages/_template/archive.manifest.toml`](../../archives/mui-packages/_template/archive.manifest.toml) before promoting changes. Use [`tools/archive-manifests/merge.ts`](../../tools/archive-manifests/merge.ts) to compose environment-specific overrides instead of copying bespoke scripts.

## Successor Rust crate

- Path: [`crates/rustic-ui-icons-material`](../../crates/rustic-ui-icons-material)
  - Provides the streaming SVG ingestion pipeline and build artifacts for Material icons directly in Rust, including deterministic hashing for regulated releases.

## Sync back from Rust

Use the automation-friendly snippet below to mirror the current crate implementation back into the archive for historical diffing. Execute it from the repository root whenever a release branch demands a refreshed snapshot:

```bash
#!/usr/bin/env bash
set -euo pipefail

crate_name="rustic-ui-icons-material"
crate_root="crates/rustic-ui-icons-material"
archive_root="archives/mui-packages/mui-icons-material"

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
