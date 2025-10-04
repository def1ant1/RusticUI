# Selection Controls (React + WebAssembly)

This example demonstrates how RusticUI composes Rust state machines, telemetry pipelines, and a
React front-end to deliver enterprise-grade selection controls that can be fully automated. The
project is structured so that developers and CI/CD systems can build, test, and deploy the demo with
minimal manual intervention.

## Prerequisites

Install the following tools before running commands:

- [Rust toolchain](https://www.rust-lang.org/tools/install) with the `wasm32-unknown-unknown` target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) 0.12+
- [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen) (installed by `wasm-pack`)
- Node.js 20+ and npm 9+
- [`just`](https://github.com/casey/just) (optional but recommended for automation)
- Chrome (for Playwright smoke tests)

## Project Layout

```
examples/selection-controls-react/
├── Cargo.toml          # Rust crate manifest exporting wasm bindings
├── README.md           # This document
├── Justfile            # Task runner orchestrating Rust + Node workflows
├── package.json        # npm scripts for building/testing the React surface
├── src/                # Rust sources with exhaustive inline documentation
└── web/                # React + Vite application consuming the wasm package
```

## Automation with `just`

The example ships with a `Justfile` so CI and developers can execute the entire workflow with a
single command.

```sh
just bootstrap       # Install npm dependencies and ensure wasm target is present
just build           # Build Rust (host + wasm) and the React bundle
just test            # Run Rust tests, wasm-bindgen tests, Jest unit tests, and Playwright smoke tests
just dev             # Launch wasm-pack in watch mode alongside the Vite dev server
just automation-smoke # Invoke the shared selection-controls-smoke.sh harness for CI reuse
just automation-serve # Start the central serve helper (override port via SELECTION_CONTROLS_REACT_PORT)
```

## Manual Commands

When `just` is unavailable you can rely on the npm scripts directly:

```sh
npm install          # Install JS dependencies in `examples/selection-controls-react`
npm run build:wasm   # Compile the Rust crate to WebAssembly
npm run build:web    # Build the React bundle after wasm output exists
npm run test         # Execute Rust, wasm, Jest, and Playwright smoke tests
npm run test:e2e     # Programmatic Playwright harness via selection-controls-playwright.mjs
npm run dev          # Concurrent wasm-pack watcher + Vite dev server
```

## Testing Strategy

The example provides multiple layers of assurance:

- **Rust (host)** – Ensures telemetry ordering and invariants are deterministic.
- **Rust (wasm)** – Uses `wasm-bindgen-test` for smoke testing under the browser runtime.
- **React unit tests** – Validates hydration and telemetry rendering with Jest + Testing Library.
- **Playwright** – Exercises the full stack in a headless Chromium session via
  `examples/scripts/selection-controls-playwright.mjs`, ensuring automation hooks
  fire as expected.

Each layer feeds the same telemetry delegate, guaranteeing parity across environments.

## Analytics & Automation Hooks

Telemetry delegates emit structured JSON payloads with sequence numbers, timestamps, and source
classifications. React registers listeners that pipe the events into the UI and can easily route
them to observability platforms (e.g., OpenTelemetry, Segment) or robotic process automation
systems.

## CI Integration

The npm scripts are designed for CI servers:

- `npm run build:web` – Generates both wasm artifacts and the web bundle suitable for publishing.
- `npm run test:wasm` – Executes wasm-bindgen tests headlessly in Chrome.
- `npm run test:web` – Chains Jest and Playwright tests after wasm compilation.
- `npm run test:e2e` – Delegates to `examples/scripts/selection-controls-playwright.mjs`
  so CI, local developers, and `cargo xtask selection-controls` all share the same
  orchestration logic.

Combine the scripts in pipelines as needed; the `Justfile` shows a canonical sequence.

## Troubleshooting

- Ensure `wasm-pack` is on your `PATH`. The automation scripts fail fast with descriptive errors if
  it is missing.
- If Playwright cannot launch a browser, install dependencies with `npx playwright install`.
- Clear stale wasm output via `npm run clean` when switching between debug and release builds.

Happy hacking! The source code is intentionally annotated so new teammates can understand the
end-to-end flow without reading additional docs.
