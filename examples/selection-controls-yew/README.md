# Selection Controls with Yew

Enterprise telemetry now wraps the checkbox, switch, and radio demos so both
controlled and uncontrolled ownership models emit deterministic analytics spans
before user callbacks execute.  The crate exposes a `SelectionControlsDemo`
component for CSR, an SSR harness for CI smoke tests, and reusable automation
utilities that downstream teams can embed when auditing hydration order.

## Quick start

| Workflow | Command |
| --- | --- |
| Format + dual-target build | `just` |
| Serve with Trunk | `just serve` |
| Run host + wasm checks | `just check` |
| Execute smoke tests | `just smoke` |
| CI automation bundle | `just ci-smoke` or `./scripts/ci-smoke.sh` |
| Shared smoke harness | `just automation-smoke` |

The `Cargo.toml` defines aliases so manual invocations remain terse:

- `cargo host-check` → `cargo check --all-targets`
- `cargo wasm-check` → `cargo check --target wasm32-unknown-unknown`
- `cargo host-test` → `cargo test --all-targets`
- `cargo wasm-test` → `cargo test --target wasm32-unknown-unknown -- --nocapture`

## Building for the host

```bash
# Render SSR fragments and verify telemetry wiring.
cargo run -p selection-controls-yew

# Compile + test with exhaustive commentary-enabled smoke tests.
cargo host-check
cargo host-test
```

`cargo run` prints each SSR snapshot so reviewers can diff `data-automation-id`
and `data-rustic-analytics-id` attributes during code review.

## Building for `wasm32-unknown-unknown`

```bash
# Validate the wasm build without starting a browser.
cargo wasm-check

# Execute wasm-bindgen tests in headless mode.
cargo wasm-test

# Launch the demo with Trunk + Yew hydration telemetry.
just serve
```

`Trunk.toml` installs build hooks that format sources, run the wasm check, and
execute the wasm tests before emitting bundles.  CI mirrors the same workflow
via `scripts/ci-smoke.sh`.

## Telemetry expectations

Every control registers the following identifiers and event ordering:

1. `TelemetryHooks::on_render` logs the analytics/automation IDs **before** any
   change/focus handlers execute, guaranteeing observability spans wrap the
   entire hydration cycle.
2. `TelemetryHooks::on_focus_transition`, `on_state_change`, and
   `on_commit_ack` project structured payloads into the shared
   `TelemetryRecorder`, which writes both to `gloo_console` (in browsers) and to
   stdout (on the host).
3. Telemetry delegates always emit their structured events prior to the public
   callbacks (`change`, `focus`, `blur`, `key`), so analytics receives ground
   truth even when consumers mutate state.
4. SSR markup includes deterministic `data-automation-id` values derived from
   `automation.selection-controls.<channel>`.

Host tests (`checkbox_telemetry_precedes_change_handler` and
`radio_keyboard_sequence_is_tracked`) and wasm tests (`checkbox_events_order_matches_browser_expectations`
and `radio_keyboard_events_are_chronological`) assert this ordering so future
refactors cannot regress instrumentation.

## CI + automation

`./scripts/ci-smoke.sh` delegates to the shared
[`examples/scripts/selection-controls-smoke.sh`](../scripts/selection-controls-smoke.sh)
helper so every automation pathway—`just automation-smoke`, `cargo xtask
selection-controls`, and the Playwright harness—shares the exact same
command graph. The script is idempotent and safe to run locally before
opening a pull request.

## Next steps

- Wire additional selection controls (e.g., segmented buttons) into the same
  telemetry harness once adapters land in `rustic-ui-material`.
- Extend the recorder with structured exports (JSONL) when downstream analytics
  platforms require native ingestion.
