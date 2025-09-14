# mui-icons-material

This crate auto-generates Rust bindings for Material Design SVG icons.

## Generation process

A build script scans the local [`material-icons/`](material-icons) directory for
`.svg` files. Each file is parsed with [`usvg`](https://crates.io/crates/usvg)
for validation and minification, then transformed into a memoized Rust function
via [`quote`](https://crates.io/crates/quote). A `material_icon!` macro maps icon
names to these functions.

## Custom icon sets

Add or remove SVG files from `material-icons/` and rebuild; the bindings update
automatically. This provides a scalable way to manage large icon sets without
manual wiring.

## Updating icons

To sync with the upstream Material Design repository run:

```bash
make icons
```

The `update_icons` utility downloads Google's latest SVGs, refreshes the
`material-icons/` directory and rewrites the crate's feature flags so each icon
can be enabled individually. Subsequent `cargo build` or `cargo test` invocations
will regenerate the Rust bindings automatically.
