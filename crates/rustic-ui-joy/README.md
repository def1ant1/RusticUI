# mui-joy

Rust-first bindings for the Joy UI design language. The crate mirrors the
structure of Material UI while exposing Joy-specific tokens such as neutral,
danger, success, warning, and info palettes plus radius controls and focus
outlines. Every prop definition is framework agnostic so Yew, Leptos, Dioxus,
and Sycamore adapters share the same API surface and analytics hooks.

## Installation

```bash
cargo add mui-joy --features yew
# or
cargo add mui-joy --features leptos
```

`mui-joy` deliberately keeps features granular so applications only compile the
adapters they need:

| Feature    | Purpose                                                                 | Typical consumers                |
|------------|-------------------------------------------------------------------------|----------------------------------|
| `yew`      | Enables the concrete Yew components plus `yew::Properties` derives.      | SPA/CSR apps built with Yew.     |
| `leptos`   | Implements the Leptos marker trait so downstream crates can validate prop compatibility inside `view!` templates. | SSR/Hydrate apps using Leptos.   |
| `dioxus`   | Derives `dioxus::Props` on every generated prop struct.                  | Dioxus CSR/SSR renderers.        |
| `sycamore` | Derives `sycamore::Props` on every generated prop struct.                | Sycamore SPA or islands apps.    |

The prop macros (`joy_props!` and `joy_component_props!`) always emit plain Rust
structs so feature flags only influence which derive macros are attached. This
keeps CI fast and eliminates manual duplication when new adapters are added.

## Framework snippets

### Yew

```rust
use mui_joy::{Button, ButtonProps, Color, Variant};
use yew::prelude::*;

#[function_component(SaveButton)]
fn save_button() -> Html {
    html! {
        <Button
            ..ButtonProps {
                color: Color::Primary,
                variant: Variant::Solid,
                label: "Approve deployment".into(),
                onclick: Callback::noop(),
                throttle_ms: Some(400),
                disabled: false,
            }
        />
    }
}
```

### Leptos

```rust
use leptos::*;
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};
use mui_system::theme::Theme;

#[component]
pub fn JoyTag(theme: Theme) -> impl IntoView {
    let style = resolve_surface_tokens(&theme, Color::Neutral, Variant::Soft)
        .compose([("padding", "6px 12px".to_string())]);
    view! { <span style=style>{"Production window"}</span> }
}
```

### Dioxus

```rust
use dioxus::prelude::*;
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};
use mui_system::theme::Theme;

fn Chip(theme: &Theme) -> String {
    resolve_surface_tokens(theme, Color::Danger, Variant::Soft)
        .compose([("padding", "8px 14px".to_string())])
}
```

### Sycamore

```rust
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};
use sycamore::prelude::*;

#[component]
fn AlertBadge<G: Html>(cx: Scope) -> View<G> {
    let theme = mui_system::theme::Theme::default();
    let style = resolve_surface_tokens(&theme, Color::Primary, Variant::Solid)
        .compose([("padding", "8px 16px".to_string())]);
    view! { cx, span(style=style) { "Joy automation green" } }
}
```

Each snippet is a distilled version of the [cross-framework Joy workflow
examples](../../examples) which demonstrate full automation pipelines and shared
state management.

## Automated color selection

`Color::ALL` and [`Color::as_str`](./src/macros.rs) expose the complete Joy
palette so enterprise teams can drive documentation, visual regression tests,
and configuration UIs from a single source of truth. The helper works across all
renderers because the inline styles are produced by
[`helpers::resolve_surface_tokens`](./src/helpers/mod.rs):

```rust
use mui_joy::helpers::resolve_surface_tokens;
use mui_joy::{Color, Variant};
use mui_system::Theme;

fn surface_swatches(theme: &Theme) -> Vec<String> {
    Color::ALL
        .iter()
        .map(|color| {
            let tokens = resolve_surface_tokens(theme, *color, Variant::Soft);
            format!("{} => {:?}", color.as_str(), tokens.background)
        })
        .collect()
}
```

Downstream adapters (Yew, Leptos, Dioxus, Sycamore) rely on the same helper, so
tests can assert that background and border styles shift for every color without
duplicated lookup tables.

## Joy theme automation

The Joy design tokens surfaced via [`Theme::joy`](../rustic-ui-system/src/theme.rs)
now expose a dedicated automation API. Use
[`JoyTheme::builder`](../rustic-ui-system/src/theme.rs) to customise radius, focus, or
shadow tokens and serialize the result with `Theme::with_joy_overrides`.

When generating fixtures, pass `--joy` to the Material theming task:

```bash
cargo xtask generate-theme --joy
```

The command emits `joy_theme.<scheme>.json` and `joy_theme.template.json`
alongside the Material outputs. Each JSON blob includes the `joy` namespace,
the automation comments returned by `JoyTheme::automation_comments()`, and the
canonical template from `JoyTheme::json_template()`. These metadata fields make
it trivial to hydrate CMS systems or downstream SDKs without duplicating
documentation.

## Examples

The repository hosts comprehensive workflow demos under `examples/`:

- [`joy-yew`](../../examples/joy-yew) – Trunk-powered CSR build showcasing Joy
  components driven by the shared workflow machine.
- [`joy-leptos`](../../examples/joy-leptos) – Signal-driven SSR/CSR hybrid that
  reuses the identical automation hooks.
- [`joy-dioxus`](../../examples/joy-dioxus) – `rsx!` templates backed by the
  central workflow state.
- [`joy-sycamore`](../../examples/joy-sycamore) – fine-grained Sycamore signals
  consuming the same machine without duplicating business logic.

All demos depend on [`joy-workflows-core`](../../examples/joy-workflows-core), a
crate that centralises the state machine, analytics identifiers, and design
tokens so every adapter stays in lockstep.

## Parity automation

The Joy adapters participate in the workspace parity pipeline. Run the inventory
scanner to refresh coverage metrics and CI fixtures:

```bash
cargo xtask joy-parity
```

The command emits an inventory report at
[`docs/joy-component-parity.md`](../../docs/joy-component-parity.md) and is
executed automatically in CI. The report lists every Joy component alongside the
React baseline so teams can track adoption progress and enforce automation
parity across frameworks.
