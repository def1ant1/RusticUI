# MUI System (Rust)

This crate provides the low level layout and theming primitives that power the
Material UI ecosystem in Rust.  Components target modern frameworks like
[`yew`](https://yew.rs), `leptos`, `dioxus` and `sycamore` and favor compile‑time
safety over runtime configuration.

## Usage

```rust
use mui_system::{Box, Stack, style_props, ThemeProvider, Theme};
# #[cfg(feature = "yew")]
# fn render() -> yew::Html {
let theme = Theme::default();
html! {
    <ThemeProvider theme={theme}>
        <Stack spacing={Some("8px".into())} justify_content={Some("center".into())}>
            <Box sx={style_props!{ padding: "4px" }}>{"Item"}</Box>
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

### Responsive props

Every layout primitive understands breakpoint aware values through the
`Responsive<T>` helper. Supply an `xs` baseline and optionally override any of
the larger breakpoints (`sm`, `md`, `lg`, `xl`). The active viewport width is
pulled from the current browser context and resolved automatically before the
style string is emitted.

```rust
use mui_system::{
    container::build_container_style,
    grid::build_grid_style,
    r#box::{build_box_style, BoxStyleInputs},
    responsive::Responsive,
    stack::{build_stack_style, StackDirection},
    Theme,
};

let theme = Theme::default();
let columns = Responsive { xs: 4, sm: Some(8), md: Some(12), lg: None, xl: Some(16) };
let span = Responsive { xs: 4, sm: Some(6), md: Some(6), lg: Some(8), xl: Some(12) };
let grid_styles = build_grid_style(900, &theme.breakpoints, Some(&columns), Some(&span), None, None, "");

let max_width = Responsive { xs: "100%".into(), sm: Some("640px".into()), md: Some("960px".into()), lg: Some("1200px".into()), xl: Some("1440px".into()) };
let container_styles = build_container_style(1280, &theme.breakpoints, Some(&max_width), "padding:24px;");

let spacing = Responsive { xs: "4px".into(), sm: Some("8px".into()), md: Some("16px".into()), lg: None, xl: Some("32px".into()) };
let stack_styles = build_stack_style(1000, &theme.breakpoints, Some(StackDirection::Row), Some(&spacing), None, Some("space-between"), "");

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
        sx: "border-radius:8px;",
    },
);

assert!(grid_styles.contains("width:50%;"));
assert!(container_styles.contains("max-width:1200px;"));
assert!(stack_styles.contains("gap:16px;"));
assert!(box_styles.contains("font-size:18px;"));
```

The helper builders above are available to integration tests as well, keeping
the breakpoint logic centralised and encouraging automation over manual styling
rules. Framework adapters (Yew, Leptos, etc.) invoke the same functions under
the hood so behaviour is identical at runtime.

## Theming and global styles

Material Design defaults are baked directly into the crate so that a working
experience is available out-of-the-box:

```rust
use mui_system::theme_provider::{material_theme, material_theme_with_optional_overrides};

let theme = material_theme();
assert_eq!(theme.spacing(2), 16);
assert_eq!(theme.palette.background_default, "#fafafa");
assert_eq!(theme.typography.font_family, "Roboto, Helvetica, Arial, sans-serif");

// Optional overrides generated via `#[derive(Theme)]` merge with the defaults.
#[derive(mui_styled_engine::Theme)]
struct PaletteOnly {
    palette: Option<PaletteOverride>,
}

struct PaletteOverride {
    primary: String,
}

impl From<PaletteOverride> for mui_system::theme::Palette {
    fn from(value: PaletteOverride) -> Self {
        Self { primary: value.primary, ..Self::default() }
    }
}

let merged = material_theme_with_optional_overrides(Some(PaletteOnly {
    palette: Some(PaletteOverride { primary: "#123456".into() }),
}));
assert_eq!(merged.palette.primary, "#123456");
// Unspecified fields inherit the canonical Material tokens.
assert_eq!(merged.typography.font_family, theme.typography.font_family);
```

Framework adapters expose a [`CssBaseline`](./src/theme_provider.rs) component
that injects the canonical Material reset using `css_with_theme!` so palette and
typography overrides flow into the global styles automatically:

```rust
# #[cfg(feature = "yew")]
use mui_system::theme_provider::{CssBaseline, ThemeProvider, material_theme};

# #[cfg(feature = "yew")]
# fn render() -> yew::Html {
html! {
    <ThemeProvider theme={material_theme()}>
        <CssBaseline />
        // application...
    </ThemeProvider>
}
# }
```

To keep documentation, code samples and automation in sync run the helper task
whenever defaults change:

```bash
cargo xtask generate-theme
```

The command serialises `material_theme()` into `crates/mui-system/templates`
which downstream tooling can consume as a golden template.

## Legacy JavaScript Package

The original `packages/mui-system` directory from the upstream project has been
**archived**.  All new development happens in this Rust crate which offers the
same API surface with stronger typing and zero runtime dependencies.  Consumers
are encouraged to migrate and report any missing features.

## Testing

Unit tests cover layout math, theming and WebAssembly compatibility.  Run the
suite with:

```bash
cargo test -p mui-system
wasm-pack test --node crates/mui-system
```

## Contributing

The crate aims to be heavily documented so that enterprise teams can build on
it with confidence.  Contributions that further automate repetitive styling
tasks via macros or code generation are especially welcome.
