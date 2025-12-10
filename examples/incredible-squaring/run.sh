#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DEFAULT_TNT_CORE="${REPO_ROOT}/../tnt-core"

if [[ -z "${TNT_CORE_PATH:-}" ]]; then
  if [[ -d "${DEFAULT_TNT_CORE}" ]]; then
    export TNT_CORE_PATH="${DEFAULT_TNT_CORE}"
  else
    echo "error: set TNT_CORE_PATH to a checkout of the tnt-core repo" >&2
    exit 1
  fi
fi

export RUN_TNT_E2E="${RUN_TNT_E2E:-1}"

echo "Using TNT_CORE_PATH=${TNT_CORE_PATH}"
echo "RUN_TNT_E2E=${RUN_TNT_E2E}"
echo
echo "Running the Anvil-backed end-to-end test for the incredible-squaring blueprint..."
echo "This exercises the BlueprintHarness, seeded service ID 0, and the XSQUARE job."
echo

(
  cd "${REPO_ROOT}"
  cargo test -p incredible-squaring-blueprint-lib --test anvil -- --nocapture
)

