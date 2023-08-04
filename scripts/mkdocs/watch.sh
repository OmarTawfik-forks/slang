#!/bin/bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/../_common.sh"

# _MKDOCS_WATCH_PORT_ | keep in sync with the port number defined in "$REPO_ROOT/.devcontainer/devcontainer.json"
WATCH_PORT=5353

(
  printf "\n\n🌐 Serving Documentation 🌐\n\n\n"

  cd "$REPO_ROOT/documentation"
  python3 -m pipenv run mkdocs serve --watch-theme --dev-addr "localhost:$WATCH_PORT"
)
