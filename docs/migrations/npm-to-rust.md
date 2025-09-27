# Migrating from `@mui/*` npm packages to RusticUI crates

<p class="description">Use the automation-first RusticUI toolchain to replace legacy npm dependencies with the `rustic-ui-*` crates while preserving archived JavaScript assets for reference.</p>

RusticUI ships every Material primitive as a Rust crate with opinionated automation around asset bundling, CI, and documentation. This guide walks enterprise teams through a repeatable migration from the deprecated `@mui/*` npm packages to the WebAssembly-ready `rustic-ui-*` crates.

## 1. Inventory existing npm usage

Start by auditing your workspace for `@mui/*` imports. The `scripts/migrate-crate-prefix.sh` helper continues to work during the transition, but your goal is to replace these imports entirely. Capture the current dependency map so you can flip each package to its Rust equivalent in one sweep.

```bash
rg "@mui/" --files-with-matches --hidden --glob "*.{ts,tsx,js,jsx}" > /tmp/mui-usage.txt
```

Use the matrix below to plan your crate substitutions:

| Legacy npm package                | RusticUI crate                 | Notes |
| --------------------------------- | ------------------------------ | ----- |
| `@mui/material`                   | `rustic-ui-material`           | Enable the renderer feature (`yew`, `leptos`, `dioxus`, etc.). |
| `@mui/system`                     | `rustic-ui-system`             | Provides tokens, theming macros, and design-time automation. |
| `@mui/base` / `@mui/core`         | `rustic-ui-headless`           | Headless state machines backing every component. |
| `@mui/icons-material`             | `rustic-ui-icons`              | Pair with `cargo xtask icons-bundle` to refresh SVG payloads. |
| `@mui/styled-engine` / `@mui/styles` | `rustic-ui-styled-engine`    | Exposes `css_with_theme!` and server-friendly style emitters. |

## 2. Update `Cargo.toml` dependencies

Flip each framework package to depend on the RusticUI crates. The example below shows a Yew-focused application that still enables the temporary `compat-mui` feature while the automation rewrites imports. Remove the compatibility flag once `cargo xtask migrate-crate-prefix` completes.

```toml
[workspace.dependencies]
rustic-ui-system = { version = "0.1", features = ["compat-mui"] }
rustic-ui-styled-engine = "0.1"
rustic-ui-headless = "0.1"
rustic-ui-material = { version = "0.1", features = ["yew", "compat-mui"] }
rustic-ui-icons = { version = "0.1", default-features = false, features = ["set-material", "compat-mui"] }
```

> **Note:** `cargo add` understands workspace inheritance. Run `cargo add rustic-ui-material --features yew,compat-mui --workspace` if you prefer incremental edits.

### Framework-specific feature flags

The crates expose dedicated feature flags per renderer. Use the matching block for Leptos or Dioxus projects:

```toml
# Leptos
rustic-ui-material = { version = "0.1", features = ["leptos", "csr"] }
rustic-ui-icons = { version = "0.1", default-features = false, features = ["set-material", "leptos"] }

# Dioxus
rustic-ui-material = { version = "0.1", features = ["dioxus"] }
rustic-ui-icons = { version = "0.1", default-features = false, features = ["set-material", "dioxus"] }
```

Pair these flags with the renderer-specific app crates in the examples directory (`examples/mui-yew`, `examples/mui-leptos`, etc.) to validate SSR and hydration flows before rolling the change into production.

## 3. Automate asset bundling

The Rust-first workflow replaces historical pnpm scripts with typed `cargo xtask` commands. Invoke the icon bundler whenever you refresh upstream SVGs or introduce a new component set:

```bash
cargo xtask icons-bundle --compat
```

- `icons-bundle` pulls the upstream Material icon metadata, normalizes SVGs, and emits Rust-ready lookup tables.
- `--compat` copies the bundle into `archives/assets/icons` so legacy JavaScript pipelines and design tools can continue consuming the archived format during their final migration window.

If your project depended on other static payloads that lived in the npm packages (fonts, locale bundles, codemod templates), fetch them from `archives/mui-packages/`. For example, the Roboto font artifacts remain available under `archives/mui-packages/mui-material/public/static/fonts/`. Copy the required assets into your crate’s `build/` folder and commit them alongside the Rust sources.

```bash
cp -R archives/mui-packages/mui-material/public/static/fonts ./assets/fonts
```

## 4. Wire automation into `build.rs`

Use a `build.rs` hook to make asset generation reproducible. The snippet below invokes `cargo xtask icons-bundle` during build steps and emits deterministic rerun instructions:

```rust
// build.rs
use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build/icon-manifest.toml");
    println!("cargo:rerun-if-changed=archives/mui-packages/mui-material/package.json");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let status = Command::new("cargo")
        .args(["xtask", "icons-bundle", "--out-dir", &out_dir])
        .status()
        .expect("failed to run cargo xtask icons-bundle");

    assert!(status.success(), "icons-bundle generation failed");
}
```

Downstream crates can now load the compiled bundle from `$OUT_DIR` without relying on ad-hoc pnpm scripts.

## 5. Embed UI crates in framework apps

The framework adapters only require a handful of imports once the dependencies flip. Below are minimal bootstraps for Yew, Leptos, and Dioxus projects:

```rust
// Yew (src/main.rs)
use yew::prelude::*;
use rustic_ui_material::button::Button;
use rustic_ui_system::ThemeProvider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ThemeProvider>
            <Button data_rustic_button_id="rustic-button-yew-example">
                {"Launch"}
            </Button>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

```rust
// Leptos (src/main.rs)
use leptos::*;
use rustic_ui_material::button::Button;
use rustic_ui_system::ThemeProvider;

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <Button data_rustic_button_id="rustic-button-leptos-example">
                {"Launch"}
            </Button>
        </ThemeProvider>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
```

```rust
// Dioxus (src/main.rs)
use dioxus::prelude::*;
use rustic_ui_material::button::Button;
use rustic_ui_system::ThemeProvider;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        ThemeProvider {{
            Button {{
                "data-rustic-button-id": "rustic-button-dioxus-example",
                "Launch"
            }}
        }}
    })
}

fn main() {
    dioxus_web::launch(app);
}
```

All three examples rely on the same theming and automation primitives, ensuring consistent automation IDs and telemetry hooks across frameworks.

## 6. Continuous integration template

Replace pnpm-based workflows with `cargo xtask` entry points so CI mirrors the local developer experience.

```yaml
# .github/workflows/rusticui.yml
name: RusticUI
on:
  pull_request:
  push:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Cache cargo
        uses: swatinem/rust-cache@v2
      - name: Install wasm-pack
        run: cargo install wasm-pack --locked
      - name: Format & lint
        run: pnpm lint
      - name: Test workspace
        run: pnpm test
      - name: Bundle icons
        run: cargo xtask icons-bundle --compat
      - name: Build docs
        run: pnpm docs:build
```

## 7. Audit the Rust supply chain

Run RusticUI's new dependency audit before finishing the migration:

```bash
cargo xtask deny
# or
make deny
```

The command wraps `cargo deny check` with verbose logging so regulated teams can capture advisories, yanked crates, and license
drift alongside the rest of the migration evidence. Commit any required `deny.toml` exceptions with rationale comments and mirror
the command in CI to keep nightly releases compliant.

Because every command funnels through `cargo xtask`, the same automation runs locally, in nightly jobs, and inside release pipelines.

## 7. Troubleshooting archived npm dependencies

Some teams still read assets directly from `archives/mui-packages/` while they retire old build tooling. Use the playbook below to keep those flows unblocked during the migration window:

1. **Lock archive versions:** Check in the `archives/mui-packages/**/package.json` files referenced by your build to guarantee deterministic hashes. The archives are immutable once a release ships.
2. **Regenerate caches with xtask:** If bundlers expect compiled artifacts (for example, prebuilt CSS), run `cargo xtask icons-bundle --compat` and `cargo xtask themes-bundle --compat` so the cached outputs stay fresh without pnpm.
3. **Expose read-only mirrors:** Serve the archived JavaScript bundles from your artifact repository instead of rehydrating pnpm workspaces. A simple `rsync` or object storage upload after each release keeps the historical packages discoverable.
4. **Verify TypeScript shims:** The legacy `@mui/*` declaration files remain under `archives/mui-packages/`. Point `tsconfig.json` path mappings at the archive directories until the final TypeScript consumers are rewritten.
5. **Escalate residual imports:** Run `rg "@mui/"` during CI and fail the pipeline when new npm dependencies appear. The regex prevents regressions once the Rust crates power production builds.

## 8. Next steps

- Disable the `compat-mui` features once `scripts/migrate-crate-prefix.sh --verify-clean` reports no deprecated imports.
- Adopt the [`docs/mui-compatibility.md`](../mui-compatibility.md) playbook to stage the migration across large monorepos.
- Share migration wins in the RusticUI RFC board so future releases can fold your automation ideas directly into `cargo xtask`.

By leaning on the centralized automation (`cargo xtask icons-bundle`, `cargo xtask themes-bundle`, and the framework-specific examples), you eliminate bespoke pnpm scripts and gain reproducible, Rust-native asset pipelines.
