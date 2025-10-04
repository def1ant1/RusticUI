# Release notes – Adapter builders & telemetry orchestration

**Date:** 2025-08-05  \
**Audience:** Product engineering, observability, and platform reliability stakeholders

## Summary

We rolled out end-to-end documentation and inline guidance for the new selection control builder stack:

- README and architecture guides now describe how `SelectionControlAttributes::builder`, `SelectionControlTelemetry`, and `TelemetryHooks` collaborate to deliver immutable descriptors and automation markers across adapters.
- Maintainer-facing rustdoc was expanded so future builder enhancements automatically inherit enterprise telemetry defaults and keep managed identifiers authoritative.
- Stakeholders receive an explicit integration playbook that minimises repetitive adapter wiring while maximising observability parity across SSR, CSR, and multi-framework deployments.

## Impact

- **Automation:** Central platforms can register telemetry delegates and analytics IDs once; builder outputs mirror those settings across React, Yew, Leptos, Sycamore, and bespoke adapters without manual patches.
- **Scalability:** Immutable descriptors and context-rich telemetry payloads reduce divergence between renderers, unblocking horizontal scaling in distributed UI surfaces.
- **Operational excellence:** Enhanced inline guidance and changelog coverage clarify migration steps, testing expectations, and failure handling so release trains remain predictable.

## Required actions

- Review the updated [adapter builders and telemetry orchestration](../../README.md#adapter-builders-and-telemetry-orchestration) section to align custom adapters with the new workflow.
- Platform observability teams should register shared `TelemetryHooks` instances during application bootstrap and rely on the descriptors to surface analytics/focus/change/commit events.
- Ensure CI pipelines invoke `cargo xtask accessibility-audit` alongside existing Rust/wasm smoke tests so documentation drift is caught automatically.

## Next steps

- Monitor telemetry streams for consistent analytics identifiers across frameworks and report anomalies to the platform guild.
- Evaluate extending the builder workflow to input primitives so form telemetry matches the rigor now available for selection controls.
- Continue migrating remaining adapters to the descriptor helpers to eliminate legacy HTML string manipulation.
