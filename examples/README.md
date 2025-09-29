# RusticUI Example Catalog

RusticUI ships automation-first demos that mirror production-ready pipelines across
frameworks. This catalog groups every example by capability so teams can jump
to the right blueprint, reuse the existing automation hooks, and avoid manual
scaffolding.

## Automation-first setup

- Run `examples/scripts/ensure-example-toolchain.sh <framework> [--wasm] [--ssr]`
  before cloning a demo into CI. The helper validates `cargo`, `rustup`, optional
  WebAssembly targets, and Trunk so bootstrap failures surface immediately.【F:examples/scripts/ensure-example-toolchain.sh†L1-L85】
- Framework-specific READMEs call out `just`, `trunk`, or `dx` tasks. These
  commands double as CI entry points and should be reused instead of inventing
  bespoke scripts.

## Layout blueprints

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `layout-box-leptos` | `cargo run --features ssr --bin layout-box-leptos` streams the full SSR document; `just build-csr` emits the WASM bundle for hydration.【F:examples/layout-box-leptos/README.md†L37-L43】【F:examples/layout-box-leptos/README.md†L23-L33】 | `just test` runs SSR-only checks before the full feature matrix, confirming responsive padding and automation markers stay in sync.【F:examples/layout-box-leptos/README.md†L23-L53】 | `just bootstrap` provisions Rust, `wasm32-unknown-unknown`, Trunk, and shared tooling.【F:examples/layout-box-leptos/README.md†L10-L19】 |
| `layout-grid-yew` | `cargo run --features ssr --bin layout-grid-yew` prints the SSR HTML, while `just build-csr` compiles the WASM build for client hydration.【F:examples/layout-grid-yew/README.md†L47-L55】【F:examples/layout-grid-yew/README.md†L24-L33】 | Tests assert breakpoint resolution plus SSR automation hooks so monitors can diff hydration output.【F:examples/layout-grid-yew/README.md†L59-L66】 | `just bootstrap` aligns the toolchain; `just check` and `just test` mirror CI expectations.【F:examples/layout-grid-yew/README.md†L11-L20】【F:examples/layout-grid-yew/README.md†L70-L80】 |

### Responsive breakpoint summary

- `layout-box-leptos` encodes three panel blueprints. Padding scales from `20px`
  at `xs` to `36px` at `xl`, while max widths progress from `100%` to `880px`
  (`intro`), `900px` (`features`), and `760px` (`compliance`) to maintain
  legibility across devices.【F:examples/layout-box-leptos/src/blueprint.rs†L32-L90】
- `layout-grid-yew` maps hero, feature grid, and CTA sections onto responsive
  spans (e.g., hero `12 → 6` columns between `xs` and `xl`) and allows the CTA to
  extend to 16 columns on ultra-wide screens for balanced composition.【F:examples/layout-grid-yew/src/blueprint.rs†L54-L135】

## Data display surfaces

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `data-display-avatar` | Bootstrapper emits SSR HTML plus hydration stubs for Yew, Leptos, Dioxus, and Sycamore so every runtime shares the same markup.【F:examples/data-display-avatar/README.md†L3-L20】 | Regression tests guarantee the generated automation hooks stay stable across frameworks.【F:examples/data-display-avatar/README.md†L29-L36】 | `cargo run --bin bootstrap --manifest-path examples/data-display-avatar/Cargo.toml` materialises framework directories under `target/`.【F:examples/data-display-avatar/README.md†L8-L20】 |
| `data-display-leptos` | `cargo run --package data-display-leptos --features csr` hydrates the list/table demo; toggling to `--features ssr` streams the server snapshot for parity checks.【F:examples/data-display-leptos/README.md†L8-L18】 | Shared renderers stamp deterministic `data-rustic-*` attributes so QA selectors never drift between SSR and CSR.【F:examples/data-display-leptos/README.md†L20-L23】 | No extra bootstrap script—run the `cargo` commands directly from the repo root.【F:examples/data-display-leptos/README.md†L8-L18】 |
| `data-display-yew` | Ensure the WASM toolchain via `wasm-pack build --target web`, then `cargo run --package data-display-yew` to mirror SSR output locally.【F:examples/data-display-yew/README.md†L9-L14】 | List and table panels expose consistent automation hooks so accessibility and QA scripts stay portable.【F:examples/data-display-yew/README.md†L16-L24】 | The README’s two-command flow doubles as a CI smoke test for Yew adapters.【F:examples/data-display-yew/README.md†L9-L14】 |

## Feedback surfaces

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `feedback-chips` | Generator emits dismissible and read-only chip markup plus hydration stubs for every framework.【F:examples/feedback-chips/README.md†L3-L22】 | Tests assert automation IDs (`data-rustic-chip-id`, `data-dismissible`) remain deterministic across states.【F:examples/feedback-chips/README.md†L24-L45】 | `cargo run --bin bootstrap --manifest-path examples/feedback-chips/Cargo.toml` seeds the `target/feedback-chips` workspace.【F:examples/feedback-chips/README.md†L8-L22】 |
| `feedback-tooltips` | Bootstrapper emits SSR HTML, hydration adapters, and README reminders for all frameworks.【F:examples/feedback-tooltips/README.md†L3-L22】 | `cargo test` verifies portal metadata and `data-rustic-tooltip-*` hooks across renderers.【F:examples/feedback-tooltips/README.md†L27-L45】 | `cargo run --bin bootstrap --manifest-path examples/feedback-tooltips/Cargo.toml` generates the assets under `target/`.【F:examples/feedback-tooltips/README.md†L9-L20】 |

## Joy workflow family (Rust)

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `joy-workflows-core` | Shared crate centralises the state machine, analytics IDs, and Joy design tokens consumed by every adapter.【F:examples/joy-workflows-core/src/lib.rs†L1-L59】 | `cargo test --manifest-path examples/joy-workflows-core/Cargo.toml` keeps deterministic automation bundles in lockstep.【F:examples/shared-dialog-state-core/README.md†L11-L20】 | Add the crate as a dependency; adapters already point at it via relative paths.【F:examples/joy-workflows-core/src/lib.rs†L1-L59】 |
| `joy-yew` | `trunk serve --open` hydrates the workflow; `cargo run --features ssr` prints the server snapshot for parity audits.【F:examples/joy-yew/README.md†L19-L34】 | Analytics IDs flow directly from `joy-workflows-core`, enabling SSR/CSR diffing without new selectors.【F:examples/joy-yew/README.md†L3-L18】 | Install the WASM target, then reuse the README’s Trunk instructions for CSR development.【F:examples/joy-yew/README.md†L19-L27】 |
| `joy-leptos` | `trunk serve --open` powers CSR; `cargo run --features ssr` emits the same lifecycle log as other adapters.【F:examples/joy-leptos/README.md†L19-L36】 | Signals wrap the shared machine so automation hooks (`data-analytics-id`) stay deterministic.【F:examples/joy-leptos/README.md†L3-L17】 | Install the WASM target once and reuse the provided Trunk workflow.【F:examples/joy-leptos/README.md†L21-L27】 |
| `joy-dioxus` | `dx serve --open` builds the WebAssembly bundle; `cargo run --features ssr` outputs the SSR summary for audits.【F:examples/joy-dioxus/README.md†L17-L34】 | `rsx!` templates propagate analytics IDs from the shared machine, keeping automation parity with the other frameworks.【F:examples/joy-dioxus/README.md†L3-L15】 | No extra scripts—run `dx serve` inside the example directory for local development.【F:examples/joy-dioxus/README.md†L19-L25】 |
| `joy-sycamore` | `trunk serve --open` hydrates the workflow; `cargo run --features ssr` prints matching lifecycle output.【F:examples/joy-sycamore/README.md†L18-L35】 | Shared machine plus fine-grained signals maintain identical analytics IDs and journals across adapters.【F:examples/joy-sycamore/README.md†L3-L16】 | Install the WASM target and run the provided Trunk commands to bootstrap CSR builds.【F:examples/joy-sycamore/README.md†L20-L27】 |

## Joy UI React starters

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `joy-ui-cra-ts` | Standard CRA dev server (`npm start`) handles CSR; SSR is not included in this starter.【F:examples/joy-ui-cra-ts/README.md†L12-L33】 | Upstream documentation references highlight how to explore Joy templates after spinning up the project.【F:examples/joy-ui-cra-ts/README.md†L1-L39】 | Download via the documented `curl` command or clone from GitHub, then run `npm install` / `npm start`.【F:examples/joy-ui-cra-ts/README.md†L12-L28】 |
| `joy-ui-nextjs-ts` | Next.js dev server (`npm run dev`) provides CSR/SSR hybrid rendering out of the box.【F:examples/joy-ui-nextjs-ts/README.md†L12-L30】 | README links to Next.js docs and Joy customisation guides for further automation guidance.【F:examples/joy-ui-nextjs-ts/README.md†L31-L39】 | Fetch the template via `curl` or clone, then run `npm install` and `npm run dev`.【F:examples/joy-ui-nextjs-ts/README.md†L12-L28】 |
| `joy-ui-vite-ts` | Vite dev server (`npm run dev`) compiles the Joy starter; SSR is not configured by default.【F:examples/joy-ui-vite-ts/README.md†L12-L32】 | README reiterates that the starter bundles Joy UI peers so additional automation can be layered as needed.【F:examples/joy-ui-vite-ts/README.md†L1-L33】 | Download with the provided `curl` command, run `npm install`, and start the dev server via `npm run dev`.【F:examples/joy-ui-vite-ts/README.md†L12-L28】 |

## Material marketing microsite (Rust)

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `mui-shared` | Provides route descriptors, automation IDs, and themed layout used by every adapter.【F:examples/mui-dioxus/README.md†L5-L63】 | Workspace-wide checks (`cargo fmt`, `cargo clippy`, `cargo test --package mui_shared`, `cargo doc`) keep the shared surface consistent.【F:examples/mui-dioxus/README.md†L74-L109】 | Add as a dependency; all adapters consume it directly.【F:examples/mui-dioxus/README.md†L5-L63】 |
| `mui-yew` | `trunk serve --open` hydrates the site; `cargo run --features ssr` produces HTML for server streaming; `trunk build --release` yields production WASM bundles.【F:examples/mui-yew/README.md†L11-L37】 | `cargo test --package rustic_ui_yew_example` enforces router parity, automation ID determinism, and hydration-safe theming.【F:examples/mui-yew/README.md†L31-L37】 | Follow the README flow to run `trunk` for CSR, `cargo run` for SSR, and `trunk build --release` for production assets.【F:examples/mui-yew/README.md†L11-L25】【F:examples/mui-yew/README.md†L31-L37】 |
| `mui-leptos` | `trunk serve --open` drives CSR; `cargo run --features ssr` streams server HTML; `trunk build --release` emits production bundles.【F:examples/mui-leptos/README.md†L11-L33】 | `cargo test --package rustic_ui_leptos_example` validates automation IDs and hydration-aware theme switching.【F:examples/mui-leptos/README.md†L33-L37】 | Use the README’s Trunk and Cargo commands for dev, SSR, and release workflows.【F:examples/mui-leptos/README.md†L11-L33】 |
| `mui-dioxus` | `npx dioxus-cli@latest serve --platform web` hydrates CSR; `cargo run --features ssr` prints prerender HTML; `npx dioxus-cli@latest build --platform web --release` emits production bundles.【F:examples/mui-dioxus/README.md†L9-L31】 | `cargo test --package rustic_ui_dioxus_example` covers router descriptors, automation IDs, and hydration awareness.【F:examples/mui-dioxus/README.md†L33-L41】 | Run the documented `npx dioxus-cli` commands plus the SSR pipeline for full coverage.【F:examples/mui-dioxus/README.md†L9-L31】 |
| `mui-sycamore` | `trunk serve --open` for CSR, `cargo run --features ssr` for HTML snapshots, and `trunk build --release` for production bundles.【F:examples/mui-sycamore/README.md†L11-L33】 | `cargo test --package rustic_ui_sycamore_example` ensures descriptors and automation hooks stay aligned.【F:examples/mui-sycamore/README.md†L33-L37】 | Follow the README’s command sequence for dev, SSR, and release workflows.【F:examples/mui-sycamore/README.md†L11-L33】 |
| `mui-ssr-accessibility` | `cargo run --manifest-path examples/mui-ssr-accessibility/Cargo.toml` streams the SSR document; combine with any CSR adapter (e.g., `mui-yew`) for hydration; `--release` captures golden HTML snapshots.【F:examples/mui-ssr-accessibility/README.md†L11-L38】 | `cargo test --package rustic_ui_ssr_accessibility` verifies automation IDs and ARIA metadata in the server renderer.【F:examples/mui-ssr-accessibility/README.md†L33-L38】 | Build CSR assets via `examples/mui-yew` (or peers) after generating SSR HTML to validate parity.【F:examples/mui-ssr-accessibility/README.md†L21-L32】 |

## Navigation scaffolds

### Drawers

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `navigation-drawer-yew` | Bootstrap project hydrates via Trunk and demonstrates responsive anchor switching from modal to top-aligned layouts.【F:examples/navigation-drawer-yew/README.md†L13-L55】 | Generated scaffold includes axe-core hooks and deterministic automation attributes for CI pipelines.【F:examples/navigation-drawer-yew/README.md†L13-L60】 | `./examples/navigation-drawer-yew/scripts/bootstrap.sh` followed by `trunk serve --open` inside `target/navigation-drawer-yew-demo`.【F:examples/navigation-drawer-yew/README.md†L20-L26】 |
| `navigation-drawer-leptos` | Bootstrap script deposits a Leptos project showcasing responsive anchors and controlled vs uncontrolled state.【F:examples/navigation-drawer-leptos/README.md†L3-L18】 | Accessibility metadata stays intact across SSR/CSR via shared `DrawerState` helpers.【F:examples/navigation-drawer-leptos/README.md†L3-L18】 | Run `./examples/navigation-drawer-leptos/scripts/bootstrap.sh` then follow the generated README for Trunk commands.【F:examples/navigation-drawer-leptos/README.md†L7-L12】 |
| `navigation-drawer-dioxus` | Script emits documentation plus integration notes for the Dioxus renderer, covering surface/backdrop markup and controlled state toggles.【F:examples/navigation-drawer-dioxus/scripts/bootstrap.sh†L1-L20】 | Automation identifiers (`DrawerState::sync_open`, explicit IDs) are preserved in the generated notes for QA scripting.【F:examples/navigation-drawer-dioxus/scripts/bootstrap.sh†L10-L17】 | Execute `./examples/navigation-drawer-dioxus/scripts/bootstrap.sh` to generate the reference README under `target/`.【F:examples/navigation-drawer-dioxus/scripts/bootstrap.sh†L5-L19】 |
| `navigation-drawer-sycamore` | Script mirrors the shared drawer renderer for Sycamore, ensuring responsive navigation without bespoke styling.【F:examples/navigation-drawer-sycamore/README.md†L1-L6】 | Generated snippets highlight automation-aware markup for Sycamore adapters.【F:examples/navigation-drawer-sycamore/README.md†L1-L6】 | `./examples/navigation-drawer-sycamore/scripts/bootstrap.sh` creates the scaffold in `target/`.【F:examples/navigation-drawer-sycamore/README.md†L1-L6】 |

### Tabs

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `navigation-tabs-yew` | Bootstrap project hydrates via Trunk, wiring `yew-router` and responsive orientation toggles.【F:examples/navigation-tabs-yew/README.md†L9-L86】 | Includes automated axe checks and deterministic automation hooks for parity audits.【F:examples/navigation-tabs-yew/README.md†L15-L33】 | `./examples/navigation-tabs-yew/scripts/bootstrap.sh`, then `trunk serve --open` in the generated project.【F:examples/navigation-tabs-yew/README.md†L24-L28】 |
| `navigation-tabs-leptos` | Script provisions a Leptos SPA that mirrors routing, responsive breakpoints, and accessibility from the Yew demo.【F:examples/navigation-tabs-leptos/README.md†L1-L41】 | Comments and snippets explain how automation IDs persist through SSR and hydration.【F:examples/navigation-tabs-leptos/README.md†L1-L41】 | `./examples/navigation-tabs-leptos/scripts/bootstrap.sh` followed by the generated README instructions (Trunk serve).【F:examples/navigation-tabs-leptos/README.md†L20-L33】 |
| `navigation-tabs-dioxus` | Bootstrap notes keep the Dioxus markup contract aligned with Yew/Leptos for automation parity.【F:examples/navigation-tabs-dioxus/README.md†L1-L7】 | Emphasises shared markup so selectors remain reusable across renderers.【F:examples/navigation-tabs-dioxus/README.md†L1-L7】 | `./examples/navigation-tabs-dioxus/scripts/bootstrap.sh` emits the reference materials under `target/`.【F:examples/navigation-tabs-dioxus/README.md†L1-L7】 |
| `navigation-tabs-sycamore` | Script deposits Sycamore-ready snippets with automation reminders and routing guidance.【F:examples/navigation-tabs-sycamore/README.md†L1-L12】 | Highlights deterministic automation metadata for Playwright/Cypress reuse.【F:examples/navigation-tabs-sycamore/README.md†L1-L12】 | `./examples/navigation-tabs-sycamore/scripts/bootstrap.sh` seeds the documentation bundle.【F:examples/navigation-tabs-sycamore/README.md†L1-L12】 |

## Select menus

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `select-menu-shared` | Centralises async loaders, theming, and HTML renderers used by Leptos and Yew adapters.【F:examples/select-menu-shared/README.md†L1-L32】 | `cargo check --manifest-path examples/select-menu-shared/Cargo.toml` keeps helpers compiling until golden tests arrive.【F:examples/select-menu-shared/README.md†L28-L33】 | No bootstrap script—depend on the crate from framework-specific demos.【F:examples/select-menu-shared/README.md†L1-L32】 |
| `select-menu-leptos` | `trunk serve --open` hydrates the async select; `cargo run --features ssr` captures parity snapshots.【F:examples/select-menu-leptos/README.md†L13-L35】 | Demonstrates toggling disabled options through the headless state so SSR/CSR automation hooks stay aligned.【F:examples/select-menu-leptos/README.md†L17-L35】 | Install WASM tooling once, then reuse the README’s Trunk and Cargo commands.【F:examples/select-menu-leptos/README.md†L9-L35】 |
| `select-menu-yew` | `trunk serve --open` powers CSR; `cargo run --features ssr` writes `ssr.html` for smoke tests.【F:examples/select-menu-yew/README.md†L13-L39】【F:examples/select-menu-yew/README.md†L41-L56】 | Shows how `SelectState` automation hooks (`aria-disabled`, `data-disabled`) stay deterministic via shared renderers.【F:examples/select-menu-yew/README.md†L17-L33】 | Install Trunk/WASM target, run the documented commands, and reuse the automation selectors in CI.【F:examples/select-menu-yew/README.md†L9-L56】 |

## Selection controls

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `selection-controls-yew` | Inline example renders checkbox, switch, and radio HTML via Material adapters for hydration-ready markup.【F:examples/selection-controls-yew/README.md†L1-L34】 | Demonstrates how headless states propagate automation-friendly attributes into the DOM.【F:examples/selection-controls-yew/README.md†L1-L34】 | Embed the snippet directly or adapt it inside your Yew app—no bootstrap script required.【F:examples/selection-controls-yew/README.md†L1-L34】 |
| `selection-controls-leptos` | Leptos component mounts rendered HTML with `view!`, keeping SSR/CSR markup identical.【F:examples/selection-controls-leptos/README.md†L1-L33】 | Highlights how the headless state machines preserve automation and accessibility metadata automatically.【F:examples/selection-controls-leptos/README.md†L1-L33】 | Copy the snippet into a Leptos view; toolchains follow the standard `trunk` workflow when embedded in a project.【F:examples/selection-controls-leptos/README.md†L1-L33】 |
| `selection-controls-dioxus` | Dioxus `rsx!` snippet renders Material HTML while maintaining controlled state and automation markers.【F:examples/selection-controls-dioxus/README.md†L1-L27】 | Ensures hydration via `dangerous_inner_html` keeps automation hooks intact across CSR rebuilds.【F:examples/selection-controls-dioxus/README.md†L1-L27】 | Integrate the snippet into an existing Dioxus project; no standalone bootstrap steps required.【F:examples/selection-controls-dioxus/README.md†L1-L27】 |
| `selection-controls-sycamore` | Sycamore `view!` example injects rendered HTML with deterministic automation metadata.【F:examples/selection-controls-sycamore/README.md†L1-L30】 | Demonstrates how to wire RusticUI renderers into Sycamore while keeping accessibility hooks consistent.【F:examples/selection-controls-sycamore/README.md†L1-L30】 | Paste into a Sycamore component; follow your project’s existing `trunk` or bundler workflow.【F:examples/selection-controls-sycamore/README.md†L1-L30】 |

## Shared dialog state overlays

| Example | Renderer coverage | Automation & test focus | Bootstrap / build commands |
| --- | --- | --- | --- |
| `shared-dialog-state-core` | Provides shared overlay state, automation IDs, and anchor diagrams for every framework blueprint.【F:examples/shared-dialog-state-core/README.md†L1-L20】 | `cargo test --manifest-path examples/shared-dialog-state-core/Cargo.toml` guards deterministic state transitions and exported diagrams.【F:examples/shared-dialog-state-core/README.md†L11-L17】 | Depend on the crate from framework adapters to avoid duplicating validation or focus management.【F:examples/shared-dialog-state-core/README.md†L1-L20】 |
| `shared-dialog-state-yew` | Bootstrap project hydrates via Trunk, demonstrating how to wire `SharedOverlayState` into a Yew SPA with deterministic automation hooks.【F:examples/shared-dialog-state-yew/README.md†L1-L33】【F:examples/shared-dialog-state-yew/README.md†L45-L66】 | Lifecycle journal and automation IDs stream to console/DOM so QA suites can assert behaviour without new selectors.【F:examples/shared-dialog-state-yew/README.md†L1-L33】【F:examples/shared-dialog-state-yew/README.md†L45-L66】 | `./examples/shared-dialog-state-yew/scripts/bootstrap.sh`, then `trunk serve --open` from the generated workspace.【F:examples/shared-dialog-state-yew/README.md†L37-L44】 |
| `shared-dialog-state-leptos` | Trunk-powered bootstrap mirrors the Yew demo while wrapping state in Leptos signals for SSR parity.【F:examples/shared-dialog-state-leptos/README.md†L1-L34】 | Automation journal and anchor diagram keep parity with other frameworks for audits.【F:examples/shared-dialog-state-leptos/README.md†L1-L34】 | `./examples/shared-dialog-state-leptos/scripts/bootstrap.sh` then run `trunk serve --open` inside the generated project.【F:examples/shared-dialog-state-leptos/README.md†L32-L39】 |
| `shared-dialog-state-dioxus` | Dioxus bootstrap showcases the shared overlay state with lifecycle journaling and automation hooks intact.【F:examples/shared-dialog-state-dioxus/README.md†L1-L33】 | Highlights parity-friendly automation attributes and anchor diagrams for QA reuse.【F:examples/shared-dialog-state-dioxus/README.md†L1-L33】 | `./examples/shared-dialog-state-dioxus/scripts/bootstrap.sh`, install `dioxus-cli` if needed, then `DX_PLATFORM=web dx serve`.【F:examples/shared-dialog-state-dioxus/README.md†L25-L33】 |
| `shared-dialog-state-sycamore` | Sycamore bootstrap keeps automation hooks deterministic while mirroring Yew/Leptos behaviour via shared state.【F:examples/shared-dialog-state-sycamore/README.md†L1-L33】 | Lifecycle journaling and anchor diagrams reinforce parity across frameworks.【F:examples/shared-dialog-state-sycamore/README.md†L1-L33】 | `./examples/shared-dialog-state-sycamore/scripts/bootstrap.sh` then run the generated Trunk workflow.【F:examples/shared-dialog-state-sycamore/README.md†L25-L32】 |

## Additional utility scripts

- `examples/scripts/ensure-example-toolchain.sh` keeps Trunk/WASM/SSR tooling in
  sync across CI jobs and local machines—always invoke it from new automation
  entry points to minimise drift.【F:examples/scripts/ensure-example-toolchain.sh†L1-L85】
- Navigation, dialog, and select-menu bootstrap scripts drop entire workspaces
  under `target/` with exhaustive inline commentary. Reuse those artefacts instead
  of copying snippets by hand to preserve automation hooks and hydration
  expectations.

Use this catalog as the authoritative index when planning demos, parity audits,
or onboarding flows. Every example is engineered to minimise repetitive setup so
teams can focus on enterprise-specific extensions while staying aligned with the
RusticUI automation guarantees.
