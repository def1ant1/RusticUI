# Archives

RusticUI carries forward legacy Material UI packages in a Rust-first workspace. The `archives/` tree persists the
JavaScript-era sources so we can reference prior art without forking workflows back to Node-based bundlers. When a
package graduates into the maintained workspace we migrate it into a Rust crate or automation-driven TypeScript bridge
and keep the historical bundle here for provenance.

## Rust-first toolchain reference

The modern toolchain is orchestrated by `cargo xtask` and the root `Makefile`, allowing every contributor and CI job to
invoke a single, typed entrypoint. Prefer these commands over ad-hoc scripting:

- `make fmt` / `make clippy` / `make test` keep code style and linting centralized under Rust's toolchain.
- `make wasm-test` delegates to headless browser coverage so WebAssembly shims stay battle ready.
- `make icon-update` and `make build` encapsulate asset refreshes, ensuring regenerated bindings stay deterministic.

The archived JavaScript packages previously required bespoke Webpack bundles, Storybook builds, and manual publishing.
All of those flows are replaced by Rust-first crates plus automation in `scripts/` (for example
`scripts/migrate-crate-prefix.sh`, `scripts/build.mjs`, and `scripts/validateTypescriptDeclarations.mts`). These scripts
keep any remaining Node-based interop reproducible while avoiding the hand-maintained bundle sprawl that existed before.

## Automation notes for legacy packages

Archived packages should remain inert unless a resurrection is warranted. To minimize manual toil:

- Treat `archives/mui-packages/` as a read-only snapshot of the last upstream state.
- Use the automation in `scripts/` and `tools/` rather than invoking individual bundlers. Many routines already surface as
  `pnpm` scripts (`pnpm build`, `pnpm test`, `pnpm release:*`) that wrap `nx`, `ts-node`, or custom Rust binaries.
- When generating assets or scaffolding code, prefer Rust or TypeScript automation that writes to the canonical crates. Do
  not modify archived files unless you are capturing upstream history.

## Resurrection playbook

When a team needs to resurrect an archived package (for example to study an implementation detail or port functionality to
Rust) follow this repeatable workflow:

1. **Assess fit** – Review the desired feature and confirm that a Rust crate or existing headless component cannot satisfy it.
   The goal is to continue consolidating on Rust primitives.
2. **Stage the sources** – Copy the package from `archives/mui-packages/<package>` into a scratch location (never edit the
   archive in place). Use `rsync --archive --delete archives/mui-packages/<package>/ ./tmp/<package>/` so file metadata stays
   intact for reproducibility.
3. **Migrate identifiers** – Run `scripts/migrate-crate-prefix.sh --with-compat --path ./tmp/<package>` to rewrite legacy
   `mui_*` imports to the current `rustic_ui_*` crates. This keeps the port aligned with the workspace naming scheme.
4. **Transcode build tooling** – Replace legacy bundler configs by leaning on central automation:
   - Prefer generating new Rust crates via `cargo xtask scaffold --component <name>` (see `tools/` for xtask implementations).
   - If a thin TypeScript bridge is required, wire it into the `packages-internal/` workspace and rely on
     `scripts/build.mjs` or `pnpm build --filter <package>` instead of copying Webpack configs.
5. **Rehydrate tests and docs** – Translate historic tests into Rust `#[test]` or WebAssembly integration tests. Use the
   shared automation (`make test`, `make wasm-test`, `pnpm test --filter <package>`) so coverage and snapshots stay
   centralized.
6. **Document the port** – Update the relevant crate README and `docs/` section to capture the migration notes. Include
   pointers back to this archive folder for future reference.
7. **Purge the scratch copy** – Once the functionality lands in the maintained workspace, remove the temporary working
   directory. The canonical JavaScript snapshot continues to live under `archives/` for historical reference.

By following this process the team avoids reintroducing bespoke build pipelines while keeping the historical record close
at hand. Any automation gaps discovered while resurrecting a package should be captured as new `xtask` routines or
scripts, reinforcing the "automation-first" governance model.
