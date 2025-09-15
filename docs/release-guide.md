# Release guide

This guide walks maintainers through publishing the workspace crates to
[crates.io](https://crates.io). The process favors automation so releases are
repeatable and low-friction.

## Pre-flight checklist

1. Update each crate's `CHANGELOG.md` with the upcoming changes.
2. Ensure the repository is clean and tests pass:

   ```bash
   cargo xtask test
   ```

3. Validate the package manifests via dry run:

   ```bash
   cargo publish --dry-run -p <crate>
   ```

## Publishing steps

Use [`cargo release`](https://github.com/crate-ci/cargo-release) to bump
versions and tag commits. Release crates independently as they become ready.

### mui-system

```bash
cargo release minor -p mui-system --execute
cargo publish -p mui-system --features "yew leptos dioxus sycamore" --no-default-features
```

### mui-styled-engine

```bash
cargo release minor -p mui-styled-engine --execute
cargo publish -p mui-styled-engine --features "yew leptos dioxus sycamore" --no-default-features
```

### mui-styled-engine-macros

```bash
cargo release minor -p mui-styled-engine-macros --execute
cargo publish -p mui-styled-engine-macros
```

### mui-headless

```bash
cargo release minor -p mui-headless --execute
cargo publish -p mui-headless
```

### mui-material

```bash
cargo release minor -p mui-material --execute
cargo publish -p mui-material --features "yew leptos dioxus sycamore" --no-default-features
```

### mui-icons

```bash
cargo release minor -p mui-icons --execute
cargo publish -p mui-icons
```

### mui-icons-material

```bash
cargo release minor -p mui-icons-material --execute
cargo publish -p mui-icons-material
```

### mui-joy

```bash
cargo release minor -p mui-joy --execute
cargo publish -p mui-joy --features "yew leptos" --no-default-features
```

### mui-lab

```bash
cargo release minor -p mui-lab --execute
cargo publish -p mui-lab
```

### mui-utils

```bash
cargo release minor -p mui-utils --execute
cargo publish -p mui-utils --features web --no-default-features
```

After publishing, push the commit and tag to the repository:

```bash
git push --follow-tags
```

This centralized approach minimizes repetitive manual work and ensures each
crate advertises the proper feature coverage on crates.io.
