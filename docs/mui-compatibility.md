# `compat-mui` Feature Guide

The RusticUI workspace exposes an opt-in `compat-mui` Cargo feature on every renamed crate. Enabling the feature
reintroduces the legacy `mui_*` identifiers as deprecated re-exports so existing codebases can migrate incrementally
without breaking builds. The shims are intentionally temporary—they emit compiler warnings today and will be removed
once the ecosystem completes the rename to the `rustic_ui_*` namespace.

## Enabling the shims

Add the new crate names to your `Cargo.toml` and enable the compatibility flag while you migrate module paths:

```toml
[dependencies]
rustic-ui-system = { version = "0.1", features = ["compat-mui"] }
rustic-ui-material = { version = "0.1", features = ["compat-mui", "yew"] }
rustic-ui-icons = { version = "0.1", default-features = false, features = ["compat-mui", "set-material"] }
```

With the feature active you may continue importing the old identifiers for a short period:

```rust
use rustic_ui_material::mui_material::Button;
use rustic_ui_system::mui_system::ThemeProvider;
```

Because the re-exports are marked `#[deprecated]`, the compiler surfaces warnings that highlight exactly where new
imports are required. Treat the warnings as your migration punch list.

## Migration plan

1. **Flip dependencies to the `rustic-ui-*` crates** – Update `Cargo.toml` to reference the new package names. Add the
   `compat-mui` feature while you update source code imports.
2. **Run `cargo fix --allow-dirty --allow-staged`** – Rust can automatically rewrite many paths from
   `mui_*` to `rustic_ui_*`. The compatibility shim ensures the project keeps building while you apply the fixes.
3. **Audit compiler warnings** – Re-run `cargo check` with `-D warnings` to guarantee no deprecated aliases remain.
4. **Disable `compat-mui`** – Once the build is warning-free, remove the feature flag from your dependency entries. Your
   project is now fully migrated to the RusticUI namespace.

## Removal timeline

The compatibility shims are intended for short-lived transitions. They will be maintained for the next few releases while
community crates update. Expect the aliases to be removed ahead of the 1.0 milestone—plan migrations accordingly and
subscribe to the workspace changelog for exact dates.

