#!/usr/bin/env bash
set -euo pipefail

# ----------------------------------------------------------------------------
# RusticUI post-create bootstrap
# ----------------------------------------------------------------------------
# This script executes inside the devcontainer/Codespace immediately after the
# image is built. It wires pnpm to a shared store, installs the docs workspace
# dependencies, provisions Playwright browsers, and runs the xtask toolchain
# guardrails so every contributor starts from a verified baseline.
# ----------------------------------------------------------------------------

log_step() {
  printf '\n[devcontainer] %s\n' "$1"
}

log_step "Configuring pnpm global store for deterministic caching"
pnpm config set store-dir /workspaces/.pnpm-store --global

log_step "Bootstrapping docs workspace dependencies via pnpm"
# PNPM_DISABLE_LOCKFILE is exported in devcontainer.json so the install avoids
# writing a lockfile into the repo. The docs workspace maintains its own
# manifests under docs/ without touching the Rust-first root workspace.
pnpm --dir docs install --recursive

log_step "Installing Playwright Chromium bundle for wasm smoke tests"
pnpm --dir docs exec playwright install --with-deps chromium

log_step "Verifying Rust + docs toolchain alignment"
cargo xtask verify-toolchain

log_step "Dry-running cargo xtask dev to confirm automation hooks"
cargo xtask dev --dry-run

log_step "Post-create bootstrap completed"
