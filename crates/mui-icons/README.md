# mui-icons

The `mui-icons` crate provides auto-generated bindings to multiple SVG icon
sets for Rust front-end frameworks. Icon sets are organized under
`icons/<set>/` and converted into memoized Rust functions and macros at build
time.

## Adding a new icon set

1. Create a new directory under `icons/` (for example `icons/material`).
2. Drop `.svg` files for the icons into that directory.
3. Run the centralized automation task:
   ```bash
   cargo xtask icon-update
   ```
   This command downloads the upstream Material set, keeps existing sets in
   place, and invokes the `update_features` helper inside this crate. The helper
   scans every `icons/<set>/` folder, rewrites the `[features]` manifest with the
   correct `set-<set>` and `icon-<set>-<name>` entries, and ensures new sets are
   automatically wired into the top-level `all-icons` aggregate.
4. Build or test the crate as usual. The build script validates SVG syntax,
   minifies it and generates Rust functions/macros automatically.

### Working with multiple icon families

The generator treats every folder under `icons/` as an independent family. This
allows you to maintain separate Material, Filled, Outlined, or custom corporate
sets side-by-side without hand-editing `Cargo.toml`. Re-running `cargo xtask
icon-update` after adding or removing icons keeps the manifest sorted and
deterministic so CI diffs stay reviewable.

## Feature flags

Each icon is gated behind an opt-in feature named `icon-<set>-<icon>`. Enabling
`set-<set>` pulls in all icons for that set, while the default `all-icons`
feature aggregates every available icon across every set for convenience. The
`update_features` helper regenerates these lists in alphabetical order each time
new SVGs are introduced, eliminating manual bookkeeping even as the number of
icon families grows.

For production builds where binary size matters, disable the default feature and
select only the icons your application needs:

```toml
[dependencies]
mui-icons = { version = "0.1", default-features = false, features = ["icon-material-10k_24px"] }
```

## Maintenance

The `cargo xtask icon-update` command downloads upstream icon assets, syncs the
`mui-icons-material` crate, and then re-runs the local `update_features`
generator. The process guarantees that both crates stay aligned and that the
workspace maintains a scalable, fully managed workflow for large icon
libraries.
