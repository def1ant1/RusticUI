# Selection control verification matrix

Enterprise selection controls span both the Rust crates (responsible for SSR and telemetry wiring) and the Joy TypeScript adapters. The test additions in this change set close coverage gaps so that telemetry identifiers, automation IDs, and SSR output remain deterministic across frameworks.

## Test coverage overview

- **Rust SSR + telemetry contract** – `descriptor_merges_telemetry_and_remains_deterministic` validates that the Material descriptor keeps telemetry IDs stable, emits deterministic SSR, and cooperates with the shared `instrument_render` telemetry hooks.【F:crates/rustic-ui-material/tests/selection_control.rs†L1-L129】
- **Joy SSR determinism** – The Joy Select suite now snapshots the server-rendered markup for telemetry attributes, ensuring SSR output stays stable even when analytics hooks are configured via slot props.【F:packages/mui-joy/src/Select/Select.test.tsx†L1-L40】【F:packages/mui-joy/src/Select/Select.test.tsx†L400-L475】
- **Joy telemetry propagation** – A controlled harness asserts that change events preserve the analytics and automation datasets, confirming that enterprise instrumentation survives the hand-off from Joy’s `useSelect` state machine to consumer callbacks.【F:packages/mui-joy/src/Select/Select.test.tsx†L413-L475】

## One-command local reproduction

Run the full selection-control regression matrix from the workspace root:

```bash
pnpm install --frozen-lockfile
pnpm selection:verify
```

`selection:verify` performs the entire CI routine in series:

1. `pnpm lint` → `cargo xtask fmt --check` + `cargo xtask clippy` so Rust style drifts are caught alongside web changes.【F:package.json†L8-L14】
2. `pnpm run selection:ci` → `cargo xtask selection-controls`, which runs the Rust descriptor suites and the Joy SSR/telemetry smoke tests in one call.【F:package.json†L12-L14】【F:crates/xtask/src/main.rs†L41-L70】【F:crates/xtask/src/main.rs†L227-L255】
3. `pnpm --filter docs run build` to ensure the Next.js docs (our Storybook-equivalent for selection demos) still compile to `docs/export/` for Netlify and Vercel.【F:package.json†L12-L14】
4. `cargo xtask build-docs` regenerates the Rust mdBook so API docs and telemetry notes stay in sync with the binaries.【F:package.json†L12-L14】

Skip portions of the matrix when debugging specific layers:

```bash
# Rust-only validation
cargo xtask selection-controls --skip-web

# Web-only validation (requires pnpm install)
cargo xtask selection-controls --skip-rust
```

## CI and deployment integration

- **GitHub Actions** – The new `Selection Controls Matrix` job inside `rust-ci.yml` provisions Node + Rust toolchains, installs mdBook, and executes `pnpm selection:verify`. Any instability in the Rust or Joy suites will fail this gate before merge.【F:.github/workflows/rust-ci.yml†L1-L200】
- **Netlify** – The build command now shells through `pnpm selection:verify`, guaranteeing that docs deploys always include fresh selection-control telemetry checks before the static export is produced.【F:netlify.toml†L1-L9】
- **Vercel** – `buildCommand` mirrors Netlify so preview environments inherit the same safety rails without bespoke scripting.【F:vercel.json†L1-L8】

## Monitoring for regressions

- GitHub Actions logs expose the lint/test/doc build timing under the `Selection Controls Matrix` job—use this to track test duration and catch flaky behaviour early.
- Netlify and Vercel both execute the same composite command; watch their build dashboards for timing spikes or repeated retries to spot regressions in the SSR/storybook exports before release.
- When the Joy telemetry test fails, the Mocha output surfaces the captured dataset so you can diff the expected analytics IDs against the emitted payload, avoiding guesswork during triage.

Keeping the entire workflow on one reproducible command prevents manual drift between local development, CI, and hosting providers.
