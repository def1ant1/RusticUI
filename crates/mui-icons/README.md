# mui-icons

The `mui-icons` crate provides auto-generated bindings to multiple SVG icon
sets for Rust front-end frameworks. Icon sets are organized under
`icons/<set>/` and converted into memoized Rust functions and macros at build
time.

## Adding a new icon set

1. Create a new directory under `icons/` (for example `icons/material`).
2. Drop `.svg` files for the icons into that directory.
3. Update the crate's feature list by running the centralized automation task:
   ```bash
   cargo xtask icon-update
   ```
   This pipeline ensures the `[features]` section in `Cargo.toml` mirrors the
   available icons and keeps repetitive wiring out of version control.
4. Build or test the crate as usual. The build script validates SVG syntax,
   minifies it and generates Rust functions/macros automatically.

## Feature flags

Each icon is gated behind an opt-in feature named `icon-<set>-<icon>`. Enabling
`set-<set>` pulls in all icons for that set, while the default `all-icons`
feature aggregates every available icon for convenience.

For production builds where binary size matters, disable the default feature and
select only the icons your application needs:

```toml
[dependencies]
mui-icons = { version = "0.1", default-features = false, features = ["icon-material-10k_24px"] }
```

## Maintenance

The `cargo xtask icon-update` command downloads upstream icon assets and
regenerates feature declarations, offering a scalable, fully managed workflow
for large icon libraries.
