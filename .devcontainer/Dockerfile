# syntax=docker/dockerfile:1.7

# ----------------------------------------------------------------------------
# RusticUI managed development container
# ----------------------------------------------------------------------------
# The base Rust devcontainer image already includes the stable toolchain,
# rustup, and common build dependencies. We extend it with browser libraries
# required by Playwright/Chromium and pre-install the cross-compilation target
# plus automation CLIs so Codespaces can execute `cargo xtask` flows without
# any manual prep work.
# ----------------------------------------------------------------------------
FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Reuse the default vscode user provided by the upstream devcontainer images so
# bind-mount permissions line up with Codespaces expectations.
ARG USERNAME=vscode

# Centralise frequently reused caches so both local devcontainers and
# GitHub Codespaces persist artefacts across rebuilds. The paths mirror the
# guidance in CONTRIBUTING.md and docs/testing/quick-start.md so new engineers
# inherit deterministic tooling behaviour out of the box.
ENV CARGO_TARGET_DIR=/workspaces/.cargo-target \
    PLAYWRIGHT_BROWSERS_PATH=/workspaces/.cache/ms-playwright \
    PNPM_HOME=/usr/local/share/pnpm \
    PATH="${PNPM_HOME}:${PATH}" \
    RUST_LOG=info

# Install system libraries Playwright needs for headless Chromium runs.
# We intentionally keep the list explicit (instead of relying on install-deps)
# so supply-chain scanners understand the package surface and enterprise teams
# can mirror it in bespoke images.
RUN set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        git \
        libasound2 \
        libatk-bridge2.0-0 \
        libatk1.0-0 \
        libcairo2 \
        libdrm2 \
        libgbm1 \
        libgtk-3-0 \
        libnss3 \
        libpango-1.0-0 \
        libpangocairo-1.0-0 \
        libx11-xcb1 \
        libxcb1 \
        libxcomposite1 \
        libxdamage1 \
        libxfixes3 \
        libxkbcommon0 \
        libxrandr2 \
        xauth \
        xvfb; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*

# Pre-provision the wasm target plus the Rust-native automation helpers that
# CONTRIBUTING.md calls out (mdBook, wasm-pack, cargo-deny, grcov, trunk, just).
# Installing them here keeps the Codespaces bootstrap script fast because
# cargo only needs to pull incremental updates.
RUN set -eux; \
    rustup target add wasm32-unknown-unknown; \
    cargo install --locked \
        cargo-deny \
        grcov \
        just \
        mdbook \
        trunk \
        wasm-pack

# Create directories for shared caches up front so volume mounts declared in
# devcontainer.json do not need to create them as root on first run.
RUN set -eux; \
    mkdir -p /workspaces/.cargo-target; \
    mkdir -p /workspaces/.cache/ms-playwright; \
    mkdir -p /workspaces/.pnpm-store; \
    chown -R "${USERNAME}:${USERNAME}" /workspaces/.cargo-target /workspaces/.cache /workspaces/.pnpm-store || true
