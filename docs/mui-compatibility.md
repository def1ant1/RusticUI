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
2. **Run `scripts/migrate-crate-prefix.sh --with-compat`** – The script wraps `cargo fix` so Rust rewrites imports from
   `mui_*` to `rustic_ui_*` automatically while the compatibility shim keeps builds passing.
3. **Disable `compat-mui`** – Remove the feature flag from your dependency entries once the automated rewrites finish.
4. **Verify clean builds** – Execute `scripts/migrate-crate-prefix.sh --verify-clean` (or `cargo xtask clippy`) to deny
   warnings and guarantee no deprecated aliases remain. Your project is now fully migrated to the RusticUI namespace.

## Automation selector migration

The Material automation hooks now follow the `data-rustic-<component>-*` naming convention. Instances of the legacy
`data-automation-*` attributes should be replaced to ensure selectors remain stable across releases.

```bash
# Update attribute names (e.g. data-automation-item -> data-rustic-list-item)
rg --files -g"*.{rs,tsx,ts,js,jsx,html}" -0 | \ 
  xargs -0 sed -i "s/data-automation-/data-rustic-/g"

# Update attribute values (e.g. "wasm-tooltip" -> "rustic-tooltip-wasm-tooltip")
rg --files -g"*.{rs,tsx,ts,js,jsx,html}" -0 | \ 
  xargs -0 sed -i "s/\"\(\w\+-\?\)tooltip\"/\"rustic-tooltip-\1tooltip\"/g"
```

The helper functions in `style_helpers::automation_id` and `style_helpers::automation_data_attr` can also be invoked
directly from application code when bespoke automation hooks are required.

### Automation tips

- `scripts/migrate-crate-prefix.sh` is idempotent and safe to rerun as you migrate individual crates within a monorepo.
- Pair the script with `cargo xtask build-docs` or `make doc` to update API documentation immediately after imports change.
- Use the command under CI so reviewers can trust that every pull request preserves the `rustic_ui_*` identifiers and keeps
  the compatibility shim disabled once migrations complete.

## Removal timeline

The compatibility shims are intended for short-lived transitions. They will be maintained for the next few releases while
community crates update. Expect the aliases to be removed ahead of the 1.0 milestone—plan migrations accordingly and
subscribe to the workspace changelog for exact dates.

