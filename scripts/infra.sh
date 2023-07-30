#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/_common.sh"

(
  cd "$REPO_ROOT/crates/infra/cli/"

  cargo run --bin "infra_cli" -- "$@"
)
