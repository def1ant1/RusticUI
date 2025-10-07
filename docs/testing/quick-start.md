# Quick-start automation verification

The quick-start gallery and getting-started guide ship automation-ready scaffolds
for every supported framework. This playbook documents the prerequisites,
commands, and caching strategies required to exercise the verification harness
before submitting changes.

Pair these steps with the [cross-suite coverage dashboard](coverage-overview.md)
when preparing release branches. The dashboard captures whether the Rust,
TypeScript, accessibility, and visual regression suites ran, so quick-start
owners can confirm their scaffolds remain inside the enterprise guardrails
before sign-off.【F:docs/testing/coverage-overview.md†L1-L72】

## Prerequisites

> **Managed option:** Opening the repository in the RusticUI devcontainer or a
> GitHub Codespace provisions everything in this list automatically—Rust,
> Node 20 + pnpm, Playwright Chromium bundles, and the xtask verification
> guards. Review `.devcontainer/devcontainer.json` for the cache layout and
> post-create automation sequence before mirroring custom environments.【F:.devcontainer/devcontainer.json†L1-L52】【F:.devcontainer/scripts/post-create.sh†L1-L38】

1. Install the latest stable Rust toolchain and add the `wasm32-unknown-unknown`
   target: `rustup target add wasm32-unknown-unknown`.
2. Install framework launchers used by the scaffolds you plan to touch:
   - [Trunk](https://trunkrs.dev/) for the Yew, Leptos, and Sycamore flows.
   - [`dx`](https://dioxuslabs.com/learn/0.4/cli) or the Dioxus CLI for Dioxus
     demos.
   - Node.js 20+ plus pnpm for the React selection controls harness.
3. Provision optional tooling that enables the deeper smoke checks triggered by
   the harness—`wasm-pack`, Playwright (Chromium + dependencies), and the
   workspace `just` command—so the verification steps can run without falling
   back to `--skip-checks` mode.【F:docs/src/pages/getting-started/quick-start.md†L9-L136】

> Tip: export `CARGO_TARGET_DIR`, `TRUNK_HOME`, `DX_CACHE_DIR`, and
> `PLAYWRIGHT_BROWSERS_PATH` to shared cache directories (or rely on the
> repository defaults). Persisting those paths in CI and on development
> workstations avoids rebuilding the same scaffolds and browser binaries on every
> iteration.【F:CONTRIBUTING.md†L54-L87】【F:docs/src/pages/getting-started/quick-start.md†L9-L88】

## Run the automated checks

1. From the repository root execute:

   ```bash
   cargo xtask quick-start
   ```

   The command shells through each scaffold published in the quick-start guide,
   runs the documented follow-up checks (for example, `just test` or
   `trunk test`), and writes transcripts to
   `target/logs/quick-start.log`.【F:crates/xtask/src/main.rs†L206-L233】【F:crates/xtask/src/main.rs†L3078-L3296】

## Scaffold new components during onboarding

- Use `cargo xtask new-component <Name>` with `--dry-run` to preview the generated Rust modules, TypeScript telemetry helpers,
  docs stubs, and placeholder tests. The command prints every file path so teams can agree on automation identifiers before
  landing code. Drop the dry-run flag once the plan looks correct and fill in the TODO markers emitted by the templates.
- When iterating on docs or gallery content, launch `cargo xtask dev`. The harness boots the Next.js docs site and Leptos
  example gallery together, writing live output to `target/logs/dev.log` so reviewers can inspect hot-reload sessions alongside
  code changes.
- Codespaces automatically starts the harness after attach. Re-run `cargo xtask
  dev --dry-run` from the `RusticUI · dry-run dev harness` task palette entry if
  you need to capture a transcript without long-running processes.【F:.devcontainer/codespaces.json†L5-L34】

2. Inspect `target/logs/quick-start.log` after the run completes. Each scaffold
   prints a bounded header, the exact commands executed, and a success/failure
   trailer so reviewers can audit changes without rehydrating the environment.
   Attach the relevant excerpts to your pull request when the output informs the
   review (for example, when updating StackBlitz snapshots or adjusting
   automation IDs).【F:crates/xtask/src/main.rs†L3097-L3288】

3. If you encounter a missing dependency (for example, Playwright browsers are
   unavailable on an air-gapped workstation), rerun the command with
   `--skip-checks` to confirm the bootstrap scripts still provision the
   workspace. Document the limitation in your pull request summary so CI reruns
   the full verification path.【F:crates/xtask/src/main.rs†L206-L233】

## Interpreting failures

- **Bootstrap errors** typically indicate a missing prerequisite or stale cache.
  Re-run the scaffold command printed in the log to reproduce locally, then
  update the getting-started guide if new prerequisites emerged.
- **Check failures** surface when downstream smoke tests (for example, Playwright
  suites or `cargo test`) detect behavioural drift. Apply fixes in the
  corresponding example, re-run `cargo xtask quick-start`, and commit the log
  excerpt alongside code changes when it clarifies reviewer expectations.
- **Timeouts** often stem from cold Playwright caches or first-time Trunk
  compilations. Prime the caches noted above, then re-run the harness to confirm
  the issue disappears.

Keep this playbook close to the quick-start gallery when updating docs or
scaffolds—following the automation-first workflow ensures enterprise adopters can
bootstrap production-ready experiences with repeatable guardrails.【F:docs/src/pages/examples/quick-start-gallery.md†L1-L24】【F:docs/src/pages/getting-started/quick-start.md†L1-L136】
