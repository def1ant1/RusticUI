#!/usr/bin/env bash
# Compile API docs and the mdBook guide in one step.
# Usage: ./scripts/build-rust-docs.sh
set -e

cargo doc --no-deps --all
mdbook build docs/rust-book
