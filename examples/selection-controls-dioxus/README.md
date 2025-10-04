# Selection Controls with Dioxus

This example turns the README snippet into a fully instrumented Dioxus
crate. The showcase renders the checkbox, switch, and radio group in the
same order as the documentation while bolting Rustic UI telemetry hooks
into both the render lifecycle and the structured delegates exposed by
`rustic-ui-material`.

The library exports helper functions that:

* construct a `VirtualDom` with deterministic telemetry recording so host
  and WebAssembly tests can validate hydration ordering.
* expose smoke-test helpers that emit a canonical telemetry cycle without
  duplicating event wiring logic.
* launch desktop and web runners so automation suites execute the same
  instrumentation pipeline the design system expects in production.
* render the checkbox, switch, and radio controls directly via `rsx!`
  using the headless state machines, keeping the example compatible while
  the upstream Dioxus adapters continue to evolve.

## Developer workflows

The `Justfile` centralises the commands developers and CI runners need.
All recipes call `examples/scripts/ensure-example-toolchain.sh` so the
Rust targets and supporting tools are present before any build starts.

```shell
just run-desktop     # build + launch the native window with telemetry logging
just run-web         # serve the wasm build via the Dioxus CLI
just build-desktop   # compile without launching (handy for CI cache priming)
just build-web       # emit the wasm bundle for static hosting
just test-host       # run the native telemetry smoke tests
just test-wasm       # execute wasm-bindgen tests in headless Chrome
```

> **Tip:** the `scripts/bootstrap.sh` helper powers CI smoke tests. It
> checks both the host and wasm pipelines, mirroring what the
> documentation recommends for local validation.

## Telemetry wiring

The Rust module in `src/lib.rs` encapsulates the telemetry wiring inside
a reusable harness:

* `TelemetryHooks` emit render events with analytics and automation IDs
  per control (`checkbox`, `switch`, `radio`, and the nested
  `radio.component`).
* Console logging mirrors the structured delegates so operators can tail
  stdout while the recorder stores the same payloads for assertions.
* `simulate_telemetry_cycle` replays a representative change cycle,
  making it trivial for tests—or other examples—to verify that the
  logging contract remains intact.

Host tests assert the render order via `VirtualDom::rebuild` while wasm
smoke tests validate that hydration triggers the nested radio telemetry.
Both suites reuse the same `TelemetryRecorder`, guaranteeing parity
across execution environments.

## Building for WebAssembly

The Dioxus CLI reads `dx.json` to discover how to compile the wasm
artifacts. Running `dx build --config dx.json` produces bundles in
`dist/`; `dx serve --config dx.json` spins up a local dev server using
hydration. The `web` Cargo feature ensures the crate pulls in
`dioxus-web`, `wasm-bindgen`, and the associated tests.

## Desktop execution

Desktop builds enable the `desktop` feature which activates the
`dioxus-desktop` renderer. `run_desktop` boots the virtual DOM through
`dioxus_desktop::launch::launch_virtual_dom`, applying the same telemetry
hooks used elsewhere. Developers can run the binary directly via
`cargo run --manifest-path Cargo.toml --features desktop` or lean on the
`just run-desktop` recipe.

## Testing strategy

* `tests/desktop.rs` covers hydration ordering and verifies that all
  telemetry delegates fire with change payloads.
* `tests/web.rs` (gated behind `wasm_bindgen_test`) exercises the wasm
  path to assert that hydration captures the nested radio component
  telemetry. Execute it via `just test-wasm`.

The README’s original snippet now lives as executable code with exhaustive
inline commentary in `src/lib.rs`, ensuring engineers can read, run, and
extend the instrumentation without searching through multiple files.
