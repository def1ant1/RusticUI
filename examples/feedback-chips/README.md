# Feedback chips — automation centric blueprint

The feedback chips example provisions dismissible and read-only chip markup for
all supported frameworks. The bootstrap binary removes repetitive wiring by
emitting SSR shells, hydration snippets, and automation metadata in a single
step so teams can focus on analytics and observability instead of plumbing.

## Quick start

```bash
# From the repository root
cargo run --bin bootstrap --manifest-path examples/feedback-chips/Cargo.toml
```

The command produces `target/feedback-chips/<framework>/` directories with:

- `dismissible.html` – hydrated chip with trailing action and hover automation
  metadata.
- `read-only.html` – static chip variant sharing the same typography and theming
  overrides.
- `hydrate.rs` – framework-specific starter showing how to mount both variants
  under a shared theme provider.

## Automation contracts

Both chip variants expose deterministic `data-*` hooks:

- `data-automation-id="feedback-chip"` for the dismissible instance.
- `data-automation-id="feedback-chip-static"` for the read-only instance.
- `data-dismissible` toggles to `true`/`false` depending on the variant allowing
  QA suites to assert hover affordances.

The helper returns the themed `Theme` so SSR shells and hydration roots can share
spacing, palette, and typography overrides.

## Validating the story

Execute the library tests to ensure the generated markup remains stable:

```bash
cargo test --manifest-path examples/feedback-chips/Cargo.toml
```

The test suite confirms each framework variant emits the same automation hooks,
preventing regressions when the upstream `rustic_ui_headless` state machines evolve.
