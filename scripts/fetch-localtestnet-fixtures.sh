#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${OUT_DIR:-$ROOT_DIR/crates/chain-setup/anvil/snapshots}"
CRATE_NAME="${TNT_FIXTURES_CRATE:-tnt-core-fixtures}"
VERSION="${TNT_FIXTURES_VERSION:-}"
KEEP_TMP="${KEEP_FIXTURE_TMP:-0}"

if [[ -z "$VERSION" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    VERSION="$(curl -s "https://crates.io/api/v1/crates/${CRATE_NAME}" | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data.get("crate", {}).get("newest_version", ""))
PY
)"
  elif command -v python >/dev/null 2>&1; then
    VERSION="$(curl -s "https://crates.io/api/v1/crates/${CRATE_NAME}" | python - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data.get("crate", {}).get("newest_version", ""))
PY
)"
  fi
fi

if [[ -z "$VERSION" ]]; then
  echo "error: TNT_FIXTURES_VERSION is required (unable to detect latest version)" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d -t tnt-fixtures.XXXXXX)"
TARBALL="${TMP_DIR}/${CRATE_NAME}-${VERSION}.crate"
cleanup() {
  if [[ "$KEEP_TMP" != "1" ]]; then
    rm -rf "$TMP_DIR"
  else
    echo "Temporary directory preserved at: $TMP_DIR"
  fi
}
trap cleanup EXIT

echo "Fetching ${CRATE_NAME} ${VERSION} from crates.io..."
curl -sSL "https://crates.io/api/v1/crates/${CRATE_NAME}/${VERSION}/download" -o "$TARBALL"
tar -xzf "$TARBALL" -C "$TMP_DIR"

SRC_DIR="${TMP_DIR}/${CRATE_NAME}-${VERSION}/fixtures"
STATE_SRC="${SRC_DIR}/localtestnet-state.json"
BROADCAST_SRC="${SRC_DIR}/localtestnet-broadcast.json"

if [[ ! -f "$STATE_SRC" || ! -f "$BROADCAST_SRC" ]]; then
  echo "error: fixture files not found in ${CRATE_NAME} ${VERSION}" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
cp "$STATE_SRC" "$OUT_DIR/localtestnet-state.json"
cp "$BROADCAST_SRC" "$OUT_DIR/localtestnet-broadcast.json"

echo "Wrote fixtures to $OUT_DIR"
