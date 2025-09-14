# mui-joy

Experimental Joy UI component library for Rust.

This crate mirrors the `mui-material` crate but targets the Joy design
language.  It showcases how additional design tokens such as `neutral` and
`danger` palette colors or corner `radius` can be modeled in Rust.

## Differences from Material

* Extra palette colors: `neutral` and `danger`.
* Joy specific tokens grouped under `Theme::joy` like `radius` for rounded
  corners.
* Component variants `solid`, `soft`, `outlined`, and `plain`.

## Examples

```bash
cargo run -p mui-joy --example button --features yew
```
