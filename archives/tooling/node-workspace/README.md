# Archived JavaScript workspace manifests

The files colocated here (`package.json`, `pnpm-workspace.yaml`, `lerna.json`, `nx.json`, and `webpackBaseConfig.js`) represent the former JavaScript/Nx toolchain that powered RusticUI prior to the Rust-first transition.

They are intentionally **read-only** snapshots kept for historical reference, diffing, and incident response. Automation, CI, and contributor tooling **must not** resurrect these manifests in the active workspace. All modern pipelines are mediated via `cargo xtask` (see `README.md` and `CONTRIBUTING.md` at the repository root), and this directory exists solely to preserve provenance for auditors.

When investigations touch documentation flows, defer to the Rust-first helpers: `cargo xtask docs-build` hydrates content, `cargo xtask docs-test` enforces link and lint parity, and `cargo xtask docs-package` seals the shims that the archived JavaScript expects. That trio replaces the legacy `build-docs`/`deploy-docs` npm era.

If you need to inspect legacy npm metadata or webpack defaults, copy the artifacts out of this archive and run experiments in an isolated environment. Do **not** symlink, move, or otherwise surface these files at the repository root.
