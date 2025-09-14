# mui-joy

Experimental Joy UI component library for Rust.

This crate mirrors the `mui-material` crate but targets the Joy design
language.  It showcases how additional design tokens such as `neutral` and
`danger` palette colors or corner `radius` can be modeled in Rust.
Shared enums and macros keep component props consistent and reduce manual
boilerplate when authoring new widgets.

## Differences from Material

* Extra palette colors: `neutral` and `danger`.
* Joy specific tokens grouped under `Theme::joy` such as `radius` and
  `focus_thickness` which have no Material equivalents.
* Component variants `solid`, `soft`, `outlined`, and `plain` shared across
  multiple components via macros.

## Usage

```rust
use mui_joy::{Button, Card, Chip, Color, Variant};

let button = html! { <Button label="Save" color={Color::Primary} /> };
```

## Migration from Material

Material components use `color`/`variant` patterns but the available values
are different. Joy's `Color` adds `Neutral` and `Danger`, while `Variant`
expands to `Soft` and `Plain`. When moving from Material simply switch to the
`Color` and `Variant` enums exported by this crate and reference any Joy
specific tokens through `Theme::joy`.

## Examples

```bash
cargo run -p mui-joy --example button --features yew
cargo run -p mui-joy --example card --features yew
```
