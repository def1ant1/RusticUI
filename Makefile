# Central Makefile driving common developer workflows for the Rust
# workspace. These targets wrap cargo commands and provide a single
# entrypoint for CI and local development.
#
# Examples:
#   make build   # compile all crates
#   make test    # run unit tests across the workspace
#   make doc     # generate API documentation

.RECIPEPREFIX := >
.PHONY: build test doc fmt check icons

# Fetch the latest Material Design icon set and update generated bindings.
# This target is idempotent; it removes outdated icons and downloads fresh
# assets so subsequent builds operate on a clean slate.
icons:
> @cargo run -p mui-icons-material --bin update_icons --features update-icons

build: icons
> @cargo build --workspace

# Run the default test suites for all crates. Additional integration tests
# can be wired in here as the project grows.
test:
> @cargo test --workspace

# Produce HTML documentation for all crates in the workspace.
doc:
> @cargo doc --no-deps --workspace

# Format the code base using rustfmt. Keeping formatting consistent allows
# contributors to focus on functionality rather than style.
fmt:
> @cargo fmt --all

# Run a lightweight pre-commit style check. `cargo fmt` verifies formatting
# and `cargo check` ensures everything still builds.
check: fmt
> @cargo check --workspace
