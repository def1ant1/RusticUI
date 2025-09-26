# MUI System (Rust)

This crate provides the low level layout and theming primitives that power the
Material UI ecosystem in Rust.  Components target modern frameworks like
[`yew`](https://yew.rs), `leptos`, `dioxus` and `sycamore` and favor compile‑time
safety over runtime configuration.

## Usage

```rust
use rustic_ui_system::{Box, Stack, ThemeProvider, Theme};
use serde_json::json;
# #[cfg(feature = "yew")]
# fn render() -> yew::Html {
let theme = Theme::default();
html! {
    <ThemeProvider theme={theme}>
        <Stack
            spacing={Some("8px".into())}
            justify_content={Some("center".into())}
            align_items={Some("center".into())}
        >
            <Box sx={json!({
                "padding": "4px",
                "background-color": "#f5f5f5",
                "border-radius": "4px",
                "transition": "opacity 150ms ease-in-out",
            })}>{"Item"}</Box>
        </Stack>
    </ThemeProvider>
}
# }
```

Enable the desired front‑end framework via Cargo features:

```toml
mui-system = { version = "0.1", features = ["yew"] }
```

Available features include `yew`, `leptos`, `dioxus` and `sycamore`.

## Automation-first style helpers

The [`style`](./src/style.rs) module exposes a comprehensive suite of helper
functions that emit canonical `prop:value;` declarations. They remove the need
to hand-maintain ad-hoc strings across components while keeping everything
friendly to automation. Each helper is thoroughly documented in code with the
expected units or keywords.

```rust
use rustic_ui_system::{
    animation, column_gap, display, grid_template_columns, row_gap, transform,
};

let css = vec![
    display("grid"),
    grid_template_columns("repeat(12, 1fr)"),
    row_gap("16px"),
    column_gap("24px"),
    transform("scale(1.02)"),
    animation("fade-in 500ms ease-in forwards"),
]
.into_iter()
.collect::<String>();
assert!(css.contains("grid-template-columns:repeat(12, 1fr);"));
```

The helpers can be composed manually as above or alongside the
[`style_props!`](./src/macros.rs) macro for ad-hoc declarations. Layout
builders such as [`Stack`](./src/stack.rs), [`Grid`](./src/grid.rs),
[`Box`](./src/box.rs) and [`Container`](./src/container.rs) now exclusively rely
on these helpers so the behaviour remains identical between component adapters
and test harnesses.

## Portal orchestration

The [`portal`](./src/portal.rs) module centralises how floating surfaces are
rendered during SSR. Use `PortalMount::popover` to generate deterministic anchor
and container markup:

```rust
use rustic_ui_system::portal::PortalMount;

let mount = PortalMount::popover("orders-popover");
let anchor_html = mount.anchor_html(); // <span data-portal-anchor="orders-popover" ...>
let detached_container = mount.wrap("<ul id=\"orders-list\">...</ul>");
```

Adapters emit `anchor_html` next to the trigger and append the detached container
after the host markup. Client frameworks inspect the `data-portal-*` metadata
once lifecycle hooks fire to mount the floating surface into `document.body`
without duplicating markup. Because the portal IDs derive from automation IDs
the resulting DOM is easy to target in QA automation regardless of hosting
framework.

### Responsive props

Every layout primitive understands breakpoint aware values through the
`Responsive<T>` helper. Supply an `xs` baseline and optionally override any of
the larger breakpoints (`sm`, `md`, `lg`, `xl`). The active viewport width is
pulled from the current browser context and resolved automatically before the
style string is emitted.

```rust
use rustic_ui_system::{
    container::{build_container_style, ContainerStyleInputs},
    grid::{build_grid_style, GridStyleInputs},
    r#box::{build_box_style, BoxStyleInputs},
    responsive::Responsive,
    stack::{build_stack_style, StackDirection, StackStyleInputs},
    Theme,
};

let theme = Theme::default();
let columns = Responsive { xs: 4, sm: Some(8), md: Some(12), lg: None, xl: Some(16) };
let span = Responsive { xs: 4, sm: Some(6), md: Some(6), lg: Some(8), xl: Some(12) };
let grid_styles = build_grid_style(
    900,
    &theme.breakpoints,
    GridStyleInputs {
        columns: Some(&columns),
        span: Some(&span),
        justify_content: None,
        align_items: Some("center"),
        sx: None,
    },
);

let max_width = Responsive { xs: "100%".into(), sm: Some("640px".into()), md: Some("960px".into()), lg: Some("1200px".into()), xl: Some("1440px".into()) };
let container_styles = build_container_style(
    1280,
    &theme.breakpoints,
    ContainerStyleInputs {
        max_width: Some(&max_width),
        sx: Some(&serde_json::json!({
            "padding": "24px",
            "box-shadow": "0 2px 8px rgba(0,0,0,0.15)",
        })),
    },
);

let spacing = Responsive { xs: "4px".into(), sm: Some("8px".into()), md: Some("16px".into()), lg: None, xl: Some("32px".into()) };
let stack_styles = build_stack_style(
    1000,
    &theme.breakpoints,
    StackStyleInputs {
        direction: Some(StackDirection::Row),
        spacing: Some(&spacing),
        align_items: Some("center"),
        justify_content: Some("space-between"),
        sx: Some(&serde_json::json!({
            "align-items": "center",
            "gap": "24px",
        })),
    },
);

let font_size = Responsive { xs: "14px".into(), sm: None, md: Some("16px".into()), lg: Some("18px".into()), xl: None };
let margin = Responsive::from(String::from("8px"));
let padding = Responsive::from(String::from("16px"));
let width = Responsive::from(String::from("100%"));
let box_styles = build_box_style(
    1100,
    &theme.breakpoints,
    BoxStyleInputs {
        margin: Some(&margin),
        padding: Some(&padding),
        font_size: Some(&font_size),
        font_weight: None,
        line_height: None,
        letter_spacing: None,
        color: None,
        background_color: None,
        width: Some(&width),
        height: None,
        min_width: None,
        max_width: None,
        min_height: None,
        max_height: None,
        position: None,
        top: None,
        right: None,
        bottom: None,
        left: None,
        display: Some("flex"),
        align_items: Some("center"),
        justify_content: Some("space-between"),
        sx: Some(&serde_json::json!({
            "border-radius": "8px",
            "background-color": "#fff",
            "overflow": "hidden",
            "transition": "opacity 150ms ease-in-out",
        })),
    },
);

assert!(grid_styles.contains("width:50%;"));
assert!(container_styles.contains("max-width:1200px;"));
assert!(stack_styles.contains("gap:24px;"));
assert!(box_styles.contains("font-size:18px;"));
assert!(box_styles.contains("overflow:hidden;"));
```

The helper builders accept lightweight `*StyleInputs` descriptors so framework
adapters and test harnesses can forward borrowed `Responsive<T>` handles without
cloning. This mirrors how enterprise design systems centralise layout rules—one
place produces the responsive map, and every consumer resolves values through
the shared automation shown above.

The helper builders above are available to integration tests as well, keeping
the breakpoint logic centralised and encouraging automation over manual styling
rules. Framework adapters (Yew, Leptos, etc.) invoke the same functions under
the hood so behaviour is identical at runtime.

### JSON-first `sx` overrides

Every component now accepts `sx: Option<serde_json::Value>` instead of raw style
strings. This keeps overrides declarative and lets us rely on `rustic_ui_utils::deep_merge`
to combine user supplied JSON with the generated defaults. The merge step avoids
hand written string concatenation and means that properties defined by the
component (such as responsive `padding`) can be overridden without losing the
rest of the style cascade.

```rust
use rustic_ui_system::container::{build_container_style, ContainerStyleInputs};
use serde_json::json;

let theme = rustic_ui_system::Theme::default();
let merged = build_container_style(
    1440,
    &theme.breakpoints,
    ContainerStyleInputs {
        max_width: None,
        sx: Some(&json!({
            "width": "90%",
            "background-color": "#fafafa",
            "box-shadow": "0 2px 8px rgba(0,0,0,0.1)",
        })),
    },
);
assert!(merged.contains("width:90%;"));
```

The [`style`](./src/style.rs) module exposes helpers for common properties so
automation can stay type safe. Recent additions include typography and layout
builders such as `font_size`, `font_weight`, `line_height`, `width`, `height`,
`min_width`, `background_color`, `border_radius`, `box_shadow`, `position`,
`top` and `left`. Prefer these helpers alongside JSON `sx` payloads to minimise
manual style plumbing in enterprise codebases.

## Themed element helpers

`rustic_ui_system::themed_element` demonstrates how the styling engine, theme and
accessibility attributes interact. Each adapter now renders an accessible
`<input type="text">` decorated with a deterministic BEM class (for example
`mui-themed-input--outlined`). The scoped class generated by `css_with_theme!`
injects additional declarations driven by the active theme:

* **Colour and typography** follow `theme.palette.text_primary` and the default
  typography ramp so inputs match surrounding Material content without extra
  configuration.
* **Padding and radius** fall back to `theme.spacing(1)` and
  `theme.joy.radius`, providing comfortable spacing alongside familiar rounded
  corners.
* **Focus feedback** uses `theme.joy.focus.thickness` and the primary palette to
  paint an accessible focus ring that still respects corporate colour palettes.
* **Overrides** can be appended via `style_overrides`, letting consumers inject
  bespoke CSS for unique situations while the theme-synchronised defaults remain
  in charge.

When consumers supply optional ARIA metadata or a debounce window, the helpers
route everything through `rustic_ui_utils::collect_attributes`. That guarantees
attributes are merged in a predictable order (critical for server rendering) and
that assistive technologies always see the intended `aria-label` alongside a
declarative `data-debounce-ms` attribute for instrumentation.

## Theming and global styles

Material Design defaults are baked directly into the crate so that a working
experience is available out-of-the-box. The palette exposes explicit light and
dark schemes which can be targeted individually by automation:

```rust
use rustic_ui_system::theme_provider::{
    material_theme, material_theme_dark, material_theme_light, material_theme_with_optional_overrides,
};

let theme = material_theme();
assert_eq!(theme.spacing(2), 16);
assert_eq!(theme.palette.light.background_default, "#fafafa");
assert_eq!(theme.palette.dark.background_default, "#121212");
assert_eq!(theme.typography.font_family, "Roboto, Helvetica, Arial, sans-serif");

let light = material_theme_light();
let dark = material_theme_dark();
assert_eq!(light.palette.initial_color_scheme, rustic_ui_system::theme::ColorScheme::Light);
assert_eq!(dark.palette.initial_color_scheme, rustic_ui_system::theme::ColorScheme::Dark);

// Optional overrides generated via `#[derive(Theme)]` merge with the defaults.
#[derive(rustic_ui_styled_engine::Theme)]
struct PaletteOnly {
    palette: Option<PaletteOverride>,
}

struct PaletteOverride {
    primary: String,
}

impl From<PaletteOverride> for rustic_ui_system::theme::Palette {
    fn from(value: PaletteOverride) -> Self {
        let mut palette = Self::default();
        palette.light.primary = value.primary.clone();
        palette.dark.primary = value.primary;
        palette
    }
}

let merged = material_theme_with_optional_overrides(Some(PaletteOnly {
    palette: Some(PaletteOverride { primary: "#123456".into() }),
}));
assert_eq!(merged.palette.light.primary, "#123456");
// Unspecified fields inherit the canonical Material tokens.
assert_eq!(merged.typography.font_family, theme.typography.font_family);
```

Framework adapters expose a [`CssBaseline`](./src/theme_provider.rs) component
that injects the canonical Material reset using `css_with_theme!` so palette and
typography overrides flow into the global styles automatically. The baseline now
declares both `color-scheme` and `data-mui-color-scheme` selectors so dark mode
is available even before JavaScript hydrates:

```rust
# #[cfg(feature = "yew")]
use rustic_ui_system::theme_provider::{
    CssBaseline, ThemeProvider, material_css_baseline_from_theme, material_theme_light,
};

# #[cfg(feature = "yew")]
# fn render() -> yew::Html {
let theme = material_theme_light();
let mut css = material_css_baseline_from_theme(&theme);
css.push_str("/* append enterprise global overrides here */");
html! {
    <ThemeProvider theme={theme}>
        <CssBaseline additional_css={Some(css)} />
        // application...
    </ThemeProvider>
}
# }
```

For runtime toggles call the `use_material_color_scheme` hook (Yew/Leptos) which
returns a handle exposing `set`, `toggle` and `apply_to` helpers. The hook keeps
the `<html data-mui-color-scheme>` attribute aligned with application state and
automatically honours `prefers-color-scheme: dark` on first render so server
rendered pages and static exports match user expectations without extra code.

To keep documentation, code samples and automation in sync run the helper task
whenever defaults change:

```bash
cargo xtask generate-theme [--overrides fixtures/material_overrides.json] [--format json|toml]
```

Key behaviours to rely on in enterprise automation pipelines:

* **Deterministic multi-scheme output** – The task emits one theme file per
  scheme (`material_theme.light.json`, `material_theme.dark.json`, etc.) and
  a matching CSS baseline (`material_css_baseline.<scheme>.css`). Additional
  schemes declared in override fixtures are appended automatically so teams can
  introduce high-contrast or brand-specific palettes without custom scripts.
* **Format negotiation** – The default output is prettified JSON. Pass
  `--format toml` to generate TOML templates for tooling that prefers
  configuration files, or keep the default for downstream bundlers that consume
  JSON.
* **Layered overrides** – Supply a JSON or TOML fixture via `--overrides` to
  merge shared tokens (`typography`, spacing, etc.) and per-scheme deltas under
  `schemes.light`, `schemes.dark`, and additional keys. The command removes the
  historical single-file artefacts before writing the new set so stale files
  never linger in CI workspaces.

### Joy override workflow

Joy UI specific tokens live under [`Theme::joy`](./src/theme.rs) and are exposed
via the strongly typed [`JoyTheme`](./src/theme.rs) struct. Enterprises can
override these values without rewriting CSS by using the new builder helpers:

```rust
use rustic_ui_system::theme::{JoyTheme, Theme};

let theme = Theme::with_joy_overrides(
    JoyTheme::builder()
        .radius(10)
        .focus_thickness(3)
        .focus_palette_reference("success")
        .shadow_surface("0 12px 32px rgba(15, 23, 42, 0.25)")
        .build(),
);

// The same overrides can be layered onto an existing theme instance.
let mut runtime_theme = Theme::default();
runtime_theme.apply_joy_overrides(
    JoyTheme::builder().focus_outline_template("{thickness}px dashed {color}").build(),
);
```

The automation helpers surfaced on [`JoyTheme`](./src/theme.rs) expose
`automation_comments` and `json_template` so tooling can document the tokens
inline. Running `cargo xtask generate-theme --joy` now produces dedicated Joy
fixtures (`joy_theme.light.json`, `joy_theme.dark.json`, and a
`joy_theme.template.json` snapshot) alongside the Material outputs. Each fixture
carries the `joy` namespace plus an `automation` block containing structured
comments and the canonical JSON template so centralised configuration systems can
stay in lockstep with the Rust defaults.

Downstream tooling should treat the generated artifacts in
`crates/rustic-ui-system/templates` as the golden source of truth. Load the theme file
that matches the active colour scheme and pair it with the corresponding CSS
baseline during application startup or static site builds. Integration tests in
[`crates/xtask/tests/generate_theme.rs`](../../crates/xtask/tests/generate_theme.rs)
exercise the end-to-end pipeline—including override parsing and per-scheme
serialization—so enterprise teams can rely on the command in repeatable
deployment workflows without hand-tuned steps.

## Legacy JavaScript Package

The original `packag../rustic-ui-system` directory from the upstream project has been
**archived**.  All new development happens in this Rust crate which offers the
same API surface with stronger typing and zero runtime dependencies.  Consumers
are encouraged to migrate and report any missing features.

## Testing

Unit tests cover layout math, theming and WebAssembly compatibility.  Run the
suite with:

```bash
cargo test -p mui-system
wasm-pack test --node crates/rustic-ui-system
```

## Contributing

The crate aims to be heavily documented so that enterprise teams can build on
it with confidence.  Contributions that further automate repetitive styling
tasks via macros or code generation are especially welcome.
