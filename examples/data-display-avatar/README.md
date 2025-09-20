# Data display avatar — shared markup blueprint

This example stitches the chip and tooltip renderers together to produce an
enterprise-friendly avatar widget. A single bootstrap command emits SSR markup
and hydration snippets for Yew, Leptos, Dioxus, and Sycamore so documentation
sites and product surfaces can share a consistent presence indicator.

## Quick start

```bash
# From the repository root
cargo run --bin bootstrap --manifest-path examples/data-display-avatar/Cargo.toml
```

The generator creates `target/data-display-avatar/<framework>/` with:

- `ssr.html` – combined avatar markup (chip + tooltip) ready for server
  rendering.
- `hydrate.rs` – framework specific wiring that reuses the shared theme overrides
  and automation identifiers.

## Automation and theming

The wrapper exposes `data-automation-avatar="avatar-alex"` while the nested chip
and tooltip reuse `avatar-alex` and `avatar-alex-tooltip` respectively. Theme
overrides darken the background and adjust typography so avatars stand out
against dashboards.

## Tests

```bash
cargo test --manifest-path examples/data-display-avatar/Cargo.toml
```

The regression test checks every framework variant for the expected automation
hooks preventing accidental drift between SSR and hydration.
