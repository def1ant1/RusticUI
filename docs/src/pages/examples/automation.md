# Automation blueprints

The automation example group coordinates the new headless utilities and Material
renderers across frameworks so QA pipelines can verify click-away, focus trap,
and telemetry behaviour without bespoke harnesses.

## Bootstrapping and verification

1. Run `cargo xtask examples --group automation --release` from the repository
   root. The task compiles every automation example for native and `wasm32` targets
   while emitting SSR snapshots, hydration manifests, and telemetry event streams.
2. Inspect the generated `target/rustic-ui-automation/automation-events.ndjson`
   file. Each record captures click-away dismissals, focus trap transitions, and
   snackbar queue updates so enterprise monitoring stacks can replay the exact
   lifecycle.【F:crates/rustic-ui-material/README.md†L268-L275】
3. Review the headless diagnostics documented in the utility README to confirm
   focus traps, click-away listeners, and telemetry batching are behaving as
   expected across adapters before promoting the change to production.【F:crates/rustic-ui-headless/README.md†L241-L305】

## Observability hooks

- Material adapters expose `TelemetrySubscriber` props and automation attribute
  helpers so the same telemetry payloads can flow to browser consoles, server
  logs, or CI reporters without framework-specific glue code.【F:crates/rustic-ui-material/README.md†L255-L275】
- When a test uncovers inconsistent focus handling, enable the diagnostics
  described in the headless README and rerun the automation task to capture the
  full transition timeline alongside the telemetry events.【F:crates/rustic-ui-headless/README.md†L253-L290】

## Related resources

- [Headless utility rationale and troubleshooting](../../../../crates/rustic-ui-headless/README.md#architectural-rationale-for-the-new-utility-suite)
- [Material adapter architecture and observability](../../../../crates/rustic-ui-material/README.md#architectural-rationale-for-the-new-renderers-and-adapters)
- [Example gallery overview](./index.md)
