#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FETCHER="$ROOT_DIR/scripts/fetch-localtestnet-fixtures.sh"
OUT_DIR="${OUT_DIR:-$ROOT_DIR/crates/chain-setup/anvil/snapshots}"

if [[ ! -x "$FETCHER" ]]; then
  echo "error: fixture fetcher not found at $FETCHER" >&2
  exit 1
fi

OUT_DIR="$OUT_DIR" "$FETCHER"
