# Styled Engine Automation

`StyledEngineProvider` orchestrates style injection for both client and server runtimes.  The snippets below show how to wire it into build pipelines, server‑side rendering (SSR) frameworks, and CI so that style generation, flushing, and auditing happen automatically with zero manual steps.

## Build pipelines

Automate style flushing and SSR output as part of the standard build.  Centralizing the logic in `cargo xtask` keeps the workflow reproducible for every contributor and for CI.

```bash
# Flush the style registry and write critical CSS to disk before packaging
cargo xtask flush-styles

# Pre-generate SSR HTML so deploys can ship static markup
cargo xtask ssr-html --out dist/index.html
```

Example `xtask` implementation:

```rust
// crates/xtask/src/main.rs (excerpt)
#[derive(Subcommand)]
enum Commands {
    /// Flush runtime style caches for deterministic builds.
    FlushStyles,
    /// Render the app to HTML using StyledEngineProvider and emit the file.
    SsrHtml { out: PathBuf },
    /// Scan compiled CSS variables to detect unused or missing entries.
    AuditVars,
}

fn flush_styles() -> Result<()> {
    // Execute the example that calls `StyledEngineProvider::flush_styles()`
    run(Command::new("cargo").args(["run", "-p", "examples/mui-ssr-accessibility", "--example", "flush_styles"]))
}

fn ssr_html(out: PathBuf) -> Result<()> {
    // Render to a file so both CI and local builds share identical HTML
    run(Command::new("cargo").args(["run", "-p", "examples/mui-ssr-accessibility", "--release", "--", out.to_str().unwrap()]))
}

fn audit_vars() -> Result<()> {
    // Placeholder: run a small binary that inspects generated CSS variables
    run(Command::new("cargo").args(["run", "-p", "mui-styled-engine", "--example", "audit_vars"]))
}
```

> **Note:** The commands above are deliberately thin wrappers so they can be reused in any build system.  They ensure a single source of truth for style artifacts and eliminate environment‑specific scripting.

## SSR frameworks

Frameworks like [Leptos](https://leptos.dev) or [Yew](https://yew.rs) can reuse the same tasks.  Wrap your root component with `StyledEngineProvider` and let the `ssr-html` subcommand produce deterministic markup.

```rust
use leptos::*;
use rustic_ui_styled_engine::{StyleRegistry, StyledEngineProvider, Theme};

pub fn app(cx: Scope) -> impl IntoView {
    view! { cx,
        <StyledEngineProvider theme=Theme::default()>/* app components */</StyledEngineProvider>
    }
}
```

Calling `cargo xtask ssr-html` now renders the snippet above to `dist/index.html` so the deployment step can publish it directly.

## Continuous integration

Include the automated tasks in your CI workflow to guarantee that every pull request produces vetted style artifacts.

```yaml
# .github/workflows/ci.yml
steps:
  - uses: actions/checkout@v4
  - uses: actions-rs/toolchain@v1
    with:
      toolchain: stable
  - run: cargo xtask fmt --check
  - run: cargo xtask flush-styles
  - run: cargo xtask ssr-html --out dist/index.html
  - run: cargo xtask audit-vars
```

The CI configuration above executes the exact same commands contributors run locally, ensuring that style caches, SSR output, and variable audits remain consistent across environments without bespoke scripts.
