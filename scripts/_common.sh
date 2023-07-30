#!/bin/bash
set -euo pipefail

# Import common utilities, before Hermit is activated:
source "$(dirname "${BASH_SOURCE[0]}")/_utils.sh"

# Activate the Hermit environment:
eval "$(_print_hermit_env)"

# _CARGO_CLI_ENV_VARS_ (keep In Sync)
export CARGO="${REPO_ROOT}/bin/cargo"
export RUSTC="${REPO_ROOT}/bin/rustc"
export RUSTFMT="${REPO_ROOT}/bin/rustfmt"
export RUSTUP="${REPO_ROOT}/bin/rustup"

# Use the repository's Rust version:
rustup default "$RUST_VERSION"
