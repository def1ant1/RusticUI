# Selection Controls with Sycamore

This example promotes the README snippet into a fully fledged Sycamore crate
that mirrors production-ready telemetry wiring. The checkbox, switch, and radio
components use the Rustic UI adapters directly so the telemetry delegates fire
with the exact payloads expected by analytics, automation, and QA tooling.

## Project layout

```
examples/selection-controls-sycamore/
├── Cargo.toml                 # Crate manifest with `sycamore` + `forms` features enabled
├── Justfile                   # Consolidated build / test automation
├── Trunk.toml                 # WebAssembly bundler configuration
├── index.html                 # Minimal Trunk entry point
├── scripts/ci-smoke.sh        # CI-friendly entrypoint invoking host + wasm tests
├── src/
│   ├── lib.rs                 # Telemetry recorder, Sycamore component, SSR helpers
│   └── main.rs                # Host CLI + wasm bootstrap
└── tests/
    ├── host.rs                # SSR smoke tests asserting hydration determinism
    └── wasm.rs                # wasm-bindgen tests verifying event ordering
```

## Developer workflows

All developer workflows are centralised in the [`Justfile`](Justfile). Each
recipe bootstraps the toolchain via
[`examples/scripts/ensure-example-toolchain.sh`](../scripts/ensure-example-toolchain.sh)
so the wasm target, Trunk CLI, and native pipeline are always ready before a
build starts.

```shell
just prepare       # Validate toolchains (Rust host + wasm32 + trunk)
just build-host    # Compile the native binary (`cargo build --all-targets`)
just build-wasm    # Compile the wasm artifact (`cargo build --target wasm32-unknown-unknown`)
just smoke         # Run host + wasm smoke tests in one command
just serve         # Serve the wasm bundle via Trunk with hydration enabled
just ci-smoke      # Helper consumed by CI runners (wraps host + wasm tests)
just automation-smoke # Shared selection-controls-smoke.sh entry point
just automation-serve # Launch the central serve harness (SELECTION_CONTROLS_SYCAMORE_PORT overrides the port)
```

> **Tip:** `Trunk.toml` wires a `pre-build` hook to `just prepare` so local dev
> servers and CI pipelines always confirm the environment before issuing a
> compilation.

## Telemetry wiring

`src/lib.rs` is intentionally verbose so the telemetry contract stays obvious:

* `TelemetryRecorder` captures `RecordedEvent` entries with channel, phase, and
  human-friendly detail strings for analytics dashboards or test assertions.
* `TelemetryChannel::hooks` configures `TelemetryHooks` with render, analytics,
  focus, state-change, commit, and error delegates. Each callback records
  structured log entries while preserving analytics/automation identifiers.
* Adapter delegates (`checkbox_delegate`, `switch_delegate`, `radio_delegate`)
  mirror component-specific telemetry (render → telemetry → handler order).
* `SelectionControlsProps::simulate_nominal_cycle` replays a deterministic change
  sequence so tests (and automation harnesses) can validate event ordering
  without duplicating wiring logic.

Every Sycamore adapter (`SycamoreCheckbox`, `SycamoreSwitch`,
`SycamoreRadioGroup`) receives `TelemetryHooks` **and** delegate closures. That
mirrors the production runtime where render spans emit analytics first, followed
by telemetry delegates and finally consumer callbacks (`record_*` helpers).

## Testing strategy

Two complementary suites keep the example honest across platforms:

* `tests/host.rs` renders the component twice via `render_ssr` to confirm the
  HTML snapshot is deterministic. It then runs `simulate_nominal_cycle` and
  asserts that render events precede telemetry payloads and change handlers.
* `tests/wasm.rs` uses `wasm-bindgen-test` to execute `TelemetryChannel`
  callbacks inside a headless browser environment. The tests assert that render
  hooks fire before telemetry delegates which in turn precede change/key
  handlers—matching the browser contract Sycamore delivers during hydration.

Run both suites locally with `just smoke` or invoke the standalone scripts:

```shell
# Native telemetry smoke test
defaults to host target
cargo test --package selection-controls-sycamore --all-targets

# wasm smoke test (requires wasm32 + wasm-bindgen-test-runner)
cargo test --package selection-controls-sycamore \
  --target wasm32-unknown-unknown
```

## WebAssembly bundling

Trunk drives the WebAssembly development loop. `index.html` provides the mount
point and includes the `data-trunk` link so the bundler automatically compiles
`Cargo.toml`. Launch a local dev server via `just serve`; the `pre-build` hook
ensures toolchains and dependencies are validated once per session.

Hydration is handled in `hydrate_web_app`, which renders the Sycamore component
and immediately replays a telemetry cycle so automation frameworks can assert
ordering without synthesising DOM events. The SSR helper (`render_ssr`) keeps the
markup identical between native tests and wasm hydration, enabling simple string
comparisons for determinism.

## Automation hooks

* `scripts/ci-smoke.sh` is a no-surprises entrypoint for CI runners. It shells
  into the shared
  [`examples/scripts/selection-controls-smoke.sh`](../scripts/selection-controls-smoke.sh)
  helper so every framework uses identical provisioning, logging, and teardown
  semantics.
* `workspace/Cargo.toml` registers the crate as part of the top-level workspace
  so `cargo test --workspace` exercises the new suite automatically.

Together these affordances eliminate repetitive setup for developers and
automation alike—everything funnels through central scripts and reusable helpers
with exhaustive inline notes for future maintainers.
