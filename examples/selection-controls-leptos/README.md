# RusticUI Selection Controls — Leptos Edition

This example crate ports the documented checkbox, switch, and radio guidance
into fully wired Leptos components.  Every control surfaces deterministic
telemetry hooks, automation IDs, and change/focus handlers so observability
pipelines stay aligned between SSR and CSR.

## Running the demo

The project relies on [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos)
for an ergonomic SSR + WASM workflow.  Install it once (`cargo install
cargo-leptos`) and then use the bundled [`Justfile`](Justfile) recipes:

```bash
# Build release artifacts for both targets (native + wasm32-unknown-unknown)
just build-all

# Launch the live dev server with SSR + hydration hand-off
just serve
```

Both commands automatically activate the `ssr` feature so telemetry delegates
hydrate with the same automation/analytics identifiers emitted during server
rendering.

## Testing

```bash
# Host tests: SSR snapshots + telemetry sequencing
just test-all
```

`just test-all` runs `cargo test` twice: once on the host (covering SSR
snapshots and delegate ordering) and once with the `wasm32-unknown-unknown`
target to smoke test hydration determinism inside a browser runner.

The WASM suite ensures:

* `data-automation-id` attributes survive hydration.
* `initial-state` telemetry records fire before any `render` events per channel.

## Telemetry expectations

Each control channel advertises stable identifiers:

* **Analytics IDs** – `selection-controls.leptos.<channel>`
* **Automation IDs** – `automation.selection-controls.<channel>`

Telemetry delegates emit the following phases in strict order, which is
validated in both test suites:

1. `initial-state` – snapshot recorded before rendering for CI diffing.
2. `render` – fired from `TelemetryHooks::on_render` during SSR/CSR.
3. `telemetry` – per-control delegate invoked before user callbacks.
4. `change-handler`/`focus`/`blur`/`key` – business callbacks invoked after
   telemetry so state transitions remain deterministic.

Use the [`TelemetryRecorder`](src/lib.rs) utility in your own harnesses to
assert event order or feed data into analytics stacks without re-implementing
wiring logic.

## Project layout

* [`src/lib.rs`](src/lib.rs) – Components, telemetry utilities, and SSR
  descriptors with exhaustive inline documentation.
* [`src/main.rs`](src/main.rs) – Feature-gated entry points for SSR and CSR.
* [`tests/wasm.rs`](tests/wasm.rs) – Browser-driven smoke tests for hydration
  determinism.
* [`Justfile`](Justfile) – One-touch automation for builds, servers, and tests.

The crate is workspace-aware and opts into RusticUI's shared dependency
versions, keeping maintenance automated across the monorepo.
