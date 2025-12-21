#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
snapshot_path="${ANVIL_SNAPSHOT_PATH:-"${repo_root}/crates/chain-setup/anvil/snapshots/localtestnet-state.json"}"
broadcast_path="${TNT_BROADCAST_PATH:-"${repo_root}/crates/chain-setup/anvil/snapshots/localtestnet-broadcast.json"}"

if [[ ! -f "${snapshot_path}" ]]; then
  echo "::error::Anvil snapshot missing at ${snapshot_path}" >&2
  exit 1
fi

if [[ ! -f "${broadcast_path}" ]]; then
  echo "::error::Anvil broadcast missing at ${broadcast_path}" >&2
  exit 1
fi

python3 - <<PY
import json, sys
for path in ("${snapshot_path}", "${broadcast_path}"):
    try:
        with open(path, "r", encoding="utf-8") as handle:
            json.load(handle)
    except Exception as exc:
        print(f"::error::failed to parse {path}: {exc}", file=sys.stderr)
        sys.exit(1)
PY

echo "Anvil snapshot/broadcast fixtures verified."
