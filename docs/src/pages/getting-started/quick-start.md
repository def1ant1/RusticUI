# Five-minute RusticUI quick start

RusticUI ships automation-first scaffolds so new engineers can deliver audited
surfaces without hand-wiring telemetry, SSR, or accessibility harnesses. This
page mirrors the example gallery's long-form style and points each framework to
the authoritative bootstrap scripts, automation IDs, and validation commands so
you can stand up a reference experience in minutes.【F:docs/src/pages/examples/index.md†L1-L105】

## Global prerequisites

- Prefer the managed devcontainer or GitHub Codespace when onboarding. The
  configuration wires shared caches, installs docs dependencies, and runs both
  `cargo xtask verify-toolchain` and a dry-run of `cargo xtask dev` so you land
  in a verified hot-reload loop on first attach.【F:.devcontainer/devcontainer.json†L1-L52】【F:.devcontainer/scripts/post-create.sh†L1-L38】【F:.devcontainer/codespaces.json†L5-L34】
- Install the Rust toolchain (stable) and add the `wasm32-unknown-unknown`
  target: `rustup target add wasm32-unknown-unknown`.
- Install [Trunk](https://trunkrs.dev/), [Dioxus CLI](https://dioxuslabs.com),
  and [`dx`](https://dioxuslabs.com/learn/0.4/cli) where noted by the examples.
- Ensure Node.js 20+ is available for the React automation harness.
- Run `cargo xtask fmt`, `cargo xtask clippy`, and `cargo xtask test` before
  committing so your quick-start scaffold aligns with workspace CI.【F:docs/src/pages/examples/index.md†L12-L23】【F:crates/xtask/src/main.rs†L59-L133】
- Validate the guide end-to-end with `cargo xtask quick-start`. The harness
  shells through every bootstrap command below, captures transcripts in
  `target/logs/quick-start.log`, and reuses the same workflow that
  `cargo xtask docs-test` executes in CI.【F:crates/xtask/src/main.rs†L150-L233】【F:crates/xtask/src/main.rs†L2100-L2222】
- Use `cargo xtask new-component --dry-run <Name>` to preview the full Rust,
  TypeScript, Storybook, and documentation scaffolds RusticUI generates for new
  surfaces before committing to an implementation. The generator details live in
  [the xtask catalog](../../tooling/xtask.md).【F:crates/xtask/src/main.rs†L134-L205】【F:crates/xtask/src/new_component.rs†L1-L228】【F:docs/tooling/xtask.md†L1-L44】
- When iterating on docs or gallery behaviour, `cargo xtask dev` launches both
  hot-reload servers and records a combined transcript in
  `target/logs/dev.log` so the quick-start scripts mirror production parity.
  Review usage notes in the [xtask catalog](../../tooling/xtask.md).【F:crates/xtask/src/main.rs†L134-L205】【F:crates/xtask/src/dev.rs†L1-L226】【F:docs/tooling/xtask.md†L46-L74】
- Review the [quick-start automation verification guide](../../testing/quick-start.md) for detailed prerequisites, caching tips,
  and log interpretation guidance before modifying any scaffold or gallery CTA.【F:docs/testing/quick-start.md†L1-L96】

## Yew

> **Bootstrap command:** [`./examples/navigation-tabs-yew/scripts/bootstrap.sh`](../../../examples/navigation-tabs-yew/scripts/bootstrap.sh)

The Yew tabs bootstrap seeds a Trunk workspace under `target/` with SSR
snapshots, hydration stubs, and inlined telemetry commentary. Run the script
from the repository root, then open the generated README to follow the
framework-specific wiring guidance.【F:examples/navigation-tabs-yew/README.md†L20-L59】

- `trunk serve --open` hydrates the scaffolded HTML so you can verify SSR/CSR
  parity immediately.【F:examples/navigation-tabs-yew/README.md†L20-L37】
- The scaffold stamps deterministic `data-rustic-*` attributes sourced from the
  shared automation builder, so QA suites consume the same selectors across
  frameworks.【F:examples/navigation-tabs-yew/README.md†L11-L18】【F:examples/mui-shared/src/automation.rs†L120-L214】
- Validate your changes with `cargo xtask examples --group navigation --release`
  to exercise both native and Wasm builds using the central orchestration.
  【F:crates/xtask/src/main.rs†L439-L516】

## Leptos

> **Bootstrap command:** [`./examples/navigation-tabs-leptos/scripts/bootstrap.sh`](../../../examples/navigation-tabs-leptos/scripts/bootstrap.sh)

Executing the Leptos bootstrap emits a documented project under
`target/navigation-tabs-leptos-demo` with router integration, hydration helpers,
and automation IDs wired in. Review the generated README for routing and layout
notes, then start the Trunk dev server to iterate.【F:examples/navigation-tabs-leptos/README.md†L1-L39】

- `trunk serve --open` from the emitted directory mirrors the shipped SSR
  snapshot for accessibility and parity checks.【F:examples/navigation-tabs-leptos/README.md†L20-L28】
- The README highlights how tabs reuse the shared automation builder so QA and
  analytics flows stay deterministic across renderers.【F:examples/navigation-tabs-leptos/README.md†L9-L18】【F:examples/mui-shared/src/automation.rs†L120-L214】
- Fold the crate into CI with `cargo xtask examples --group navigation --release`
  so native and `wasm32` targets compile together.【F:crates/xtask/src/main.rs†L439-L516】

## Dioxus

> **Bootstrap command:** [`./examples/navigation-drawer-dioxus/scripts/bootstrap.sh`](../../../examples/navigation-drawer-dioxus/scripts/bootstrap.sh)

The Dioxus navigation drawer script provisions a workspace with axe-core hooks,
telemetry delegates, and deterministic automation IDs. After bootstrapping, run
`dx serve --open` to hydrate the CSR bundle and inspect the generated
accessibility notes inside the scaffolded README.【F:examples/navigation-drawer-dioxus/README.md†L1-L43】

- The project shares automation selectors with the Rust adapters, ensuring SSR
  snapshots and CSR hydration events emit identical analytics payloads.【F:examples/navigation-drawer-dioxus/README.md†L9-L18】
- Use `cargo xtask examples --group navigation --release` to confirm drawer
  demos compile across targets before committing.【F:crates/xtask/src/main.rs†L439-L516】
- Invoke `examples/scripts/selection-controls-smoke.sh --list-automation` if you
  need the canonical automation manifest while instrumenting adjacent controls;
  the helper echoes the same identifiers consumed by the navigation suites.【F:examples/scripts/selection-controls-smoke.sh†L1-L63】

## Sycamore

> **Bootstrap command:** [`./examples/navigation-drawer-sycamore/scripts/bootstrap.sh`](../../../examples/navigation-drawer-sycamore/scripts/bootstrap.sh)

Sycamore bootstraps mirror the Dioxus flow but lean on Trunk for serving. The
script emits a ready-to-run workspace with inline comments mapping each
automation ID to the shared builder. Follow the generated README for hydration
instructions, then run `trunk serve --open` to validate the SSR snapshot.【F:examples/navigation-drawer-sycamore/README.md†L1-L34】

- Every panel and trigger reuses `mui_shared::AutomationIdBuilder`, matching the
  selectors used by other frameworks and examples.【F:examples/navigation-drawer-sycamore/README.md†L7-L18】【F:examples/mui-shared/src/automation.rs†L120-L214】
- The bootstrap comments call out the telemetry delegate wiring so analytics and
  QA logging stay aligned without manual hooks.【F:examples/navigation-drawer-sycamore/README.md†L18-L34】
- Capture end-to-end coverage through the shared xtask group:
  `cargo xtask examples --group navigation --release`.【F:crates/xtask/src/main.rs†L439-L516】

## React (selection controls)

> **Bootstrap command:** Run [`just bootstrap`](../../../examples/selection-controls-react/Justfile) inside `examples/selection-controls-react`.

The React + WebAssembly selection controls package uses `just` to orchestrate
Node, wasm, and Rust tooling. Running `just bootstrap` installs npm dependencies,
ensures the wasm target exists, and prepares the workspace for iterative
development.【F:examples/selection-controls-react/README.md†L25-L45】

- Follow up with `just dev` or the individual `npm run` scripts described in the
  README to launch the Vite dev server alongside wasm-pack.【F:examples/selection-controls-react/README.md†L37-L72】
- The demo imports the shared `selection-controls-smoke.sh` harness so automation
  IDs and telemetry payloads remain in sync with the Rust-first suites.【F:examples/selection-controls-react/README.md†L73-L105】【F:examples/scripts/selection-controls-smoke.sh†L1-L63】
- CI jobs should invoke `cargo xtask selection-controls --framework react` via
  `npm run test:e2e` to execute the canonical smoke coverage.【F:examples/selection-controls-react/README.md†L82-L109】【F:crates/xtask/src/main.rs†L163-L378】

## Docs and parity follow-up

> **Bootstrap command:** `cargo xtask docs-build`

Once your scaffold is stable, rebuild the docs site so API surfaces and example
galleries reflect the new integration guidance. Pair `cargo xtask docs-build`
with `cargo xtask docs-test` and `cargo xtask docs-package --dry-run` to refresh
the wasm bundle, run the headless smoke tests, and stage review artifacts
without mutating published exports.【F:docs/src/pages/examples/index.md†L24-L33】【F:crates/xtask/src/main.rs†L150-L210】

> **Shared automation IDs:** Reference [`examples/mui-shared/src/automation.rs`](../../../examples/mui-shared/src/automation.rs)
> for the definitive automation contract. Every framework adapter in this guide
> consumes the same builder so selectors remain deterministic across SSR, CSR,
> and telemetry pipelines.【F:examples/mui-shared/src/automation.rs†L50-L214】

Pairing the scaffold commands above with the xtask suite keeps your workspace
aligned with CI and ensures automation selectors stay authoritative without
hand-maintained scripts.
