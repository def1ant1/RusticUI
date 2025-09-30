# InputBase developer guide

<p class="description">Centralize analytics, hydration, and migration flows for the new `InputBase` primitive so every framework adapter inherits the same guarantees.</p>

The headless [`InputState`](https://docs.rs/rustic-ui-headless/latest/rustic_ui_headless/input_base/struct.InputState.html) powers every Material input in RusticUI.
`InputBase` is the thinnest visual shell around that state machine, yet it is also the component that instrumentation and platform teams integrate with first.
This guide distills the engineering practices we apply across the repository so downstream applications can reuse them without starting from scratch.

## Architectural snapshot

- **Shared state machine** – `InputBase` composes the `InputState` builder which already emits analytics events for changes, commits, and resets.
  Every adapter (React, Yew, Leptos, Sycamore, Dioxus, SSR renderers) simply mirrors those analytics attributes into deterministic `data-rustic-*` hooks.
- **Composable renderers** – the renderer exports (for example `rustic_ui_material::input_base::render_input_base_html`) serialize attributes using the same helper that theme-aware DOM components call.
  This guarantees byte-for-byte parity between the HTML written during SSR and the nodes hydrated on the client.
- **Automation-first styling** – the themed helper prepends a `data-component="rustic-input-base"` marker alongside granular flags such as `data-rustic-input-base-dirty` and `data-rustic-input-base-selection-start`.
  Playwright/Cypress test suites can glob on those markers instead of volatile class names.

## Analytics and automation hooks

`InputBase` mirrors the underlying `InputState` analytics buffer into DOM attributes so that automation dashboards stay trustworthy.
The [`forms-input-base-*` examples](/material-ui/guides/input-base/#reference-blueprints) render both the interactive component and the raw SSR markup, making it easy to inspect attributes like:

- `data-rustic-input-base-dirty`, `data-rustic-input-base-visited`, and `data-rustic-input-base-focused` – debuggable mirrors of the state machine flags.
- `data-rustic-input-base-selection-start` / `-end` – emitted whenever a selection is tracked, allowing cursor analytics or accessibility tooling to assert caret behaviour.
- `data-rustic-input-base-status-message` and the legacy `data-status-message` alias – provide human-readable error summaries for test snapshots.
- `data-analytics-id` plus `data-rustic-input-base-analytics-id` – unify product analytics and QA selectors.

When you build a wrapper (for example to add icons or adornments) keep forwarding the `InputState` handle instead of copying the value into bespoke fields.
That makes the existing automation hooks “just work”.

## Server-side rendering checklist

The Material renderer writes a deterministic `<input>` tag that already contains all automation markers.
To keep hydration lossless:

1. Render using `render_input_base_html` (or the framework helper) on the server.
2. Pass the exact same `InputState` snapshot to the client and call `InputState::set_value_silently` if you rehydrate from persisted data.
3. Replay analytics events after hydration by draining `InputState::drain_analytics()` so dashboards only see runtime interactions.
4. Ensure your SSR shell includes the generated status element IDs (for example `ssr-status-input-base`) so `aria-describedby` never breaks.

The new example bootstrap scripts generate an `ssr.html` artifact per framework, complete with hydration notes that spell out the automation namespace.
Use them as regression fixtures when updating styles or toggling new attributes.

## Migration steps

To migrate from bespoke input wrappers or deprecated slot props:

1. Replace hand-rolled state with `InputState::controlled` or `InputState::uncontrolled` depending on whether the parent owns the value.
2. Remove old `components` / `componentsProps` usage; the Rust renderer expects `slots` / `slotProps` semantics and already exposes strongly typed props per framework.
3. Forward analytics IDs using `InputBaseProps::analytics_id` so dashboards stay correlated after the migration.
4. Validate SSR parity by running `cargo xtask examples --group forms --release` and diffing the resulting `target/forms-input-base/*/ssr.html` snapshots.
5. Update automation tests to target the shared `data-rustic-input-base-*` selectors – the examples document each attribute that ships out of the box.

## Reference blueprints

The repository now ships enterprise-ready examples for each supported renderer under `examples/forms-input-base-*`:

- `forms-input-base-yew`
- `forms-input-base-leptos`
- `forms-input-base-sycamore`
- `forms-input-base-dioxus`

Each example:

- Wires both controlled and uncontrolled flows.
- Surfaces analytics events and automation selectors in the UI so QA engineers can copy/paste selectors into their suites.
- Ships with a `scripts/bootstrap.sh` helper that produces SSR snapshots and hydration harnesses via `cargo run --bin bootstrap`.

## Verification commands

Run the following to validate a migration locally:

```bash
cargo fmt
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
cargo xtask examples --group forms --release
```

These commands mirror the automation we run in CI (see the workspace `CHANGELOG.md` entry for additional wasm checks).
