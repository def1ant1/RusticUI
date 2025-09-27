# Archive manifest tooling

Utilities that understand `archive.manifest.toml` live in this directory. They provide a single
entrypoint for validation, override layering, and documentation generation so enterprise automation
can process every archive manifest the same way.

## Planned utilities

- **`schema.toml`** – Machine-readable description of valid keys and types. Downstream pipelines can
  pull this file to validate manifests without duplicating logic.
- **`merge.ts`** – Helper script that merges `archive.manifest.toml` with optional
  `archive.manifest.override.toml` files to support environment-specific overrides.
- **`lint.ts`** – Static checks that ensure command strings reference approved scripts and that
  required fields (build/test/publish, sync sources, crate relationships) are present.

## Usage guidelines

1. **Keep manifests source-of-truth** – Tooling should read instructions from the manifest rather
   than inferring behavior from directory structure.
2. **Prefer reusable scripts** – When commands need to change, update or extend the shared scripts in
   `scripts/` instead of adding ad-hoc shell commands to manifests.
3. **Fail fast** – Validation scripts should exit non-zero when required sections are missing. This
   allows CI systems to block misconfigured archives before they reach production workflows.

Refer to `docs/archives/archive-manifests.md` for authoring guidance and adoption playbooks.
