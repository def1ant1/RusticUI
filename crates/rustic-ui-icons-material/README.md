# mui-icons-material

This crate auto-generates Rust bindings for Material Design SVG icons.

## Generation process

A build script scans the local [`material-icons/`](material-icons) directory for
`.svg` files. Each file is parsed with [`usvg`](https://crates.io/crates/usvg)
for validation and minification, then transformed into a memoized Rust function
via [`quote`](https://crates.io/crates/quote). A `material_icon!` macro maps icon
names to these functions.

## Feature flags

| Feature | Enables | Notes |
|---------|---------|-------|
| `all-icons` | every SVG icon | default; convenient for prototypes but slows compilation |
| `icon-<name>` | a single icon | specify multiple entries for the icons your app uses |
| `update-icons` | maintenance CLI | used only to refresh the icon list |

Every SVG is gated behind an opt-in Cargo feature named `icon-<file name>`. An
umbrella `all-icons` feature pulls in the full set and is enabled by default for
ease of use. To keep compile times and binary sizes minimal in production,
disable the default and enable only the icons your application actually uses:

```toml
[dependencies]
mui-icons-material = { version = "0.1", default-features = false, features = ["icon-10k_24px"] }
```

Additional examples live in the [Cargo feature guide](../../docs/cargo-features.md).

The `update_icons` tool (see below) regenerates the `[features]` block in
`Cargo.toml` so this list stays in sync with the available SVGs.

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
can be enabled individually. Subsequent `cargo build` or `cargo test`
invocations will regenerate the Rust bindings automatically.

To keep the workflow fast on CI and local machines, HTTP metadata is cached in
`target/.icon-cache`. The cache stores the most recent ETag/Last-Modified values
and the archive checksum so repeated runs can skip downloading or rewriting
identical SVGs. When testing unreleased drops or debugging stale assets you can
invoke the binary directly and opt out of the cache layer:

```bash
cargo run -p mui-icons-material --features update-icons --bin update_icons -- \
  --force-refresh
```

The updater also accepts `--source-url <URL>` so enterprises can point at
mirrored archives or pre-approved artifact repositories without editing source
files.
