#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

export RUN_TNT_E2E="${RUN_TNT_E2E:-1}"

echo "RUN_TNT_E2E=${RUN_TNT_E2E}"
echo
echo "Running the Anvil-backed end-to-end test for the incredible-squaring blueprint..."
echo "This exercises the BlueprintHarness, seeded service ID 0, and the XSQUARE job."
echo

(
  cd "${REPO_ROOT}"
  cargo test -p incredible-squaring-blueprint-lib --test anvil -- --nocapture
)
