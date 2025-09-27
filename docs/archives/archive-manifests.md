# Archive manifest playbook

The legacy Material UI packages that live under `archives/mui-packages/` now rely on a lightweight
`archive.manifest.toml` contract. This document explains how to author, extend, and consume those
manifests so enterprise automation remains single-sourced and easy to override.

## Goals

1. **Codify automation per package** – Centralize build, test, and publish commands in the manifest
   so teams never have to chase down bespoke scripts.
2. **Capture provenance** – Track the upstream snapshot (git URL, revision, path) used to populate
   the archive.
3. **Document the migration path** – Link the historical package to its Rust-first successor crates
   so engineers know where to continue development.
4. **Enable overrides** – Provide a single file downstream release pipelines can patch to align with
   enterprise policies (custom registries, alternate test suites, etc.).

## Authoring checklist

Follow this checklist whenever adding a new archive folder:

1. **Copy the template** – Start with `archives/mui-packages/_template/archive.manifest.toml` and
   commit it alongside the archived sources.
2. **Fill in package metadata** – Update the `[package]` block with the real package name, owner,
   and status. The `language` field lets orchestrators pick the correct build container.
3. **Wire the commands** – Point `build`, `test`, and `publish` to the workspace-level scripts. When
   specialized behavior is required, add environment variables or tags to that manifest section
   rather than baking logic into shell scripts.
4. **Record sync sources** – For each upstream location, add a `[[sync.sources]]` entry with the git
   URL, revision, and path. This allows automation to rehydrate the archive on demand.
5. **Map crate relationships** – Populate `[[relationships.crates]]` with the Rust crates that
   replace or depend on the legacy package. Include notes explaining how the crate and archive fit
   together.
6. **Note compatibility requirements** – Update `[relationships.compatibility]` with the semver range
   and any necessary feature flags so downstream QA tools know which environments to emulate.
7. **Review inline comments** – Keep the descriptive comments current. They are consumed by both
   humans and documentation automation to generate onboarding guides.

## Consuming manifests

Automation services read `archive.manifest.toml` files to orchestrate builds without manual wiring.
The following conventions keep the experience consistent:

- **Command delegation** – The `uses` field in each `[commands.*]` block points to the canonical
  script under `scripts/` or `tools/`. Orchestrators can swap this value when routing jobs to
  internal runners, leaving the `run` string untouched.
- **Environment capture** – Use the `env` array to define required environment variables instead of
  referencing ad-hoc wrapper scripts. This keeps environment drift visible in code review.
- **Override layering** – Downstream teams may check in an override manifest (for example,
  `archive.manifest.override.toml`) and merge it with the template using `tools/archive-manifests`
  utilities. Avoid editing automation scripts in place when a manifest tweak will suffice.

## Validation and linting

- Run `pnpm exec prettier --check <manifest>` to ensure the TOML remains formatted according to the
  repository standards.
- Manifest-aware tooling lives under `tools/archive-manifests`. Review that directory for schema
  updates, validation utilities, and additional examples.

Keeping the manifest authoritative dramatically reduces the toil required to reanimate or audit the
legacy packages while preserving room for enterprise-scale automation.
