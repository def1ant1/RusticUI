# Developer friendly entrypoints for the Rust workspace.
# Each target delegates to `cargo xtask` which encapsulates the
# underlying commands so contributors and CI use the same logic.

.RECIPEPREFIX := >
.PHONY: build fmt clippy deny test wasm-test doc icon-update coverage bench deploy-docs \
        docs-build docs-test docs-package

# Format the entire workspace. Use `cargo xtask fmt --check` in CI.
fmt:
> @cargo xtask fmt

# Lint all crates with Clippy and deny warnings.
clippy:
> @cargo xtask clippy

# Audit dependencies for advisories, yanked crates, and license drift.
deny:
> @cargo xtask deny

# Run the standard test suites for every crate.
test:
> @cargo xtask test

# Execute WebAssembly tests in headless Chrome.
wasm-test:
> @cargo xtask wasm-test

# Generate API documentation.
doc:
> @cargo xtask doc

# Stage documentation + wasm bundles for deployment.
deploy-docs:
> @cargo xtask deploy-docs

# Build the documentation host binary and wasm bundle together.
docs-build:
> @cargo xtask docs-build

# Execute the wasm smoke tests backing the documentation bundle.
docs-test:
> @cargo xtask docs-test

# Assemble deploy-ready documentation artifacts (respects RUSTIC_DOCS_EXPORT_DIR).
docs-package:
> @cargo xtask docs-package

# Refresh Material Design icon bindings used by component crates.
icon-update:
> @cargo xtask icon-update

# Build all crates after ensuring icons are up to date.
build: icon-update
> @cargo build --workspace

# Produce an lcov coverage report using grcov.
coverage:
> @cargo xtask coverage

# Run Criterion benchmarks. The command succeeds even when no benches exist.
bench:
> @cargo xtask bench
