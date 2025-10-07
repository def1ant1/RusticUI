# RusticUI `cargo xtask` Automation Audit

> **Objective:** catalogue automation gaps in the Rust-first xtask binary so upcoming work can focus on production-ready scaffolding, dev server orchestration, and accessibility coverage. This review synthesises the current command surface in `crates/xtask/src` and calls out concrete hooks contributors can extend without hand-written scripts.

## Scope of review

- `Commands::NewComponent` generator (`crates/xtask/src/main.rs`).
- Scaffolding helpers in `new_component.rs` (template enumeration + writers).
- Dev server harness (`Commands::Dev` + `DevCommandPlan` in `dev.rs`).
- Accessibility routines exposed through `Commands::AccessibilityAudit` / `Commands::AccessibilityNightly` (`main.rs` + `accessibility` module).

## High-level findings

| Area | Observed coverage | Missing workflows | Recommended entry points |
| --- | --- | --- | --- |
| Component scaffolding | Generates Rust + TypeScript + docs stubs via `build_template_specs` and `write_template`. | No manifest wiring, feature-gate updates, or automated adapter registration. Templates lack Playwright/test harness hooks. | Extend `Commands::NewComponent` branch in `main.rs` and augment helpers in `new_component.rs` with manifest-mutating utilities. |
| Dev server orchestration | Launches `pnpm --dir docs run dev` and `cargo run -p rustic-docs --bin rustic-docs-server --features ssr` with shared logging. | No dependency bootstrap, liveness probes, or auto-restart on file changes outside the two services. Missing `pnpm install` guardrails and Tailwind/token rebuild triggers. | Add orchestration layers around `DevCommandPlan` in `dev.rs` plus pre-flight checks in `DevArgs` handling to warm caches and assert prerequisites. |
| Accessibility audits | Markdown-focused parser that scans docs directories for heading/order issues. | No axe-core/Playwright sweeps, no severity classification, and no diffable baseline for CI. Lacks integration with docs hot reload harness to audit rendered HTML. | Introduce browser automation helpers alongside `accessibility::run`, wire new subcommands off `Commands::AccessibilityAudit`, and persist JSON fixtures for gating CI.

## Detailed recommendations

### 1. Scaffolding gaps (`Commands::NewComponent`)

Current generator logic enumerates templates in `build_template_specs` and writes files verbatim via `write_template`. The pipeline stops short of registering the new component with runtime modules or front-end packages:

- **Module exports:** Neither `crates/rustic-ui-material/src/lib.rs` nor the headless crate receive automatic `pub mod` insertions, so contributors still edit the root modules by hand after scaffolding. Implement a manifest writer that injects entries before the telemetry/mod helper sections; re-use the `TemplateContext` naming to minimise string drift.
- **Cargo + feature wiring:** The generator does not toggle feature flags or add test targets to `Cargo.toml`. Add helper routines that locate the `[features]` table in `crates/rustic-ui-material/Cargo.toml` and append gated modules, ensuring enterprise adopters can scaffold components without editing manifests manually.
- **Front-end adapter integration:** The TypeScript template only drops a `RusticAdapter.tsx`. We still need automatic export updates (e.g., `packages/mui-material/src/index.ts` and storybook/story factory scaffolds). Introduce an enum variant in `TemplateAudience` (e.g., `AllWithStories`) that triggers additional file mutations and re-run `pnpm lint` via a follow-up xtask command so the generated code is CI ready.
- **Verification automation:** No post-generation smoke test exists. Add an optional `--verify` flag to `Commands::NewComponent` that invokes `cargo fmt`, `cargo test -p <component crate> -- --ignored`, and `pnpm --dir docs lint` to enforce consistency immediately after scaffolding.

Actionable next steps:

1. Extend the `Commands::NewComponent` arm in `main.rs` to parse new flags (`--register`, `--verify`).
2. Introduce a `manifest` helper module under `crates/xtask/src` that mutates `lib.rs`, feature sets, and package exports using the existing workspace-aware paths from `TemplateSpec`.
3. Wire verification runs through the shared `run(Command)` helper so CI logs stay uniform.

### 2. Dev server orchestration gaps (`Commands::Dev`)

The current harness spawns two long-running processes and streams logs to `target/logs/dev.log`. To fully support distributed enterprise teams we need extra automation around setup and resiliency:

- **Dependency pre-flight:** `DevCommandPlan::docs` assumes `pnpm install` already ran. Insert a pre-flight step that checks `docs/node_modules` freshness and, if missing/stale, executes `pnpm --dir docs install` before launching the dev server.
- **Port and health checks:** The orchestrator fails fast only after a child process exits. Add async health probes that poll `http://docs_host:docs_port` and `gallery_host:gallery_port`, surfacing readiness markers and auto-retrying transient failures.
- **Auto-restart + watch integration:** File watchers for design token regeneration (`rustic_ui_design_tokens::ArtifactBundleBuilder`) and Tailwind builds are out-of-band. Introduce a lightweight supervisor around `DevCommandPlan` that watches `packages/`, `docs/src/`, and token artifacts, triggering targeted rebuild commands when relevant files change.
- **Composable targets:** Teams often want to include Playwright smoke tests or API mock servers. Add new variants to `DevArgs` (e.g., `--with-playwright`, `--with-api`) and extend `plan_commands` so additional `DevCommandPlan` builders can register more services without rewriting the harness.

Implementation hooks:

1. Update the `Commands::Dev` branch in `main.rs` to route new flags into `DevArgs`.
2. Expand `DevCommandPlan` with lifecycle callbacks (`pre_spawn`, `post_spawn`) and integrate a Tokio runtime so file watching and health checks run concurrently with the child processes.
3. Persist readiness + restart state in the existing log writer to keep CI visibility consistent.

### 3. Accessibility automation gaps (`Commands::AccessibilityAudit`)

`accessibility::run` currently walks Markdown files, enforcing simple heading checks. Production-grade coverage needs to incorporate rendered HTML, interactive widgets, and severity tracking:

- **Browser-driven audits:** Add a Playwright- or axe-core-powered pass that loads docs routes through the same URLs the dev harness serves, capturing WCAG violations that static Markdown parsing misses (e.g., focus traps, ARIA attributes).
- **Configurable severity gates:** Extend the JSON manifest handled by `AccessibilityConfig` with severity levels and allow `Commands::AccessibilityAudit` to fail only when thresholds are crossed (e.g., block deploy on `critical`, warn on `moderate`). Persist summaries alongside `target/logs/accessibility.json` for CI trend analysis.
- **Baseline diffs:** Store previous findings and compute diffs so nightly audits only fail on regressions. This enables incremental hardening without blocking merges on existing debt.
- **Integration with dev harness:** Teach the dev orchestrator to optionally launch audits (`cargo xtask accessibility-audit --watch`) whenever docs MDX files change, minimising manual verification.

Implementation hooks:

1. Create an `accessibility::browser` submodule that wraps Playwright via `Command` helpers, returning enriched findings (URL, severity, snippet).
2. Extend `AuditMode` with additional variants (`Interactive`, `Full`) and update the `Commands::AccessibilityNightly` match arm to orchestrate the multi-phase scan.
3. Serialize findings through `serde_json` and reuse `run_async_task` to parallelise Markdown + browser audits under a single command.

---

By addressing the gaps above, the xtask binary can become the single orchestration entry point for scaffolding new components, running resilient dev stacks, and enforcing accessibility standards—all without falling back to ad-hoc shell scripts.
