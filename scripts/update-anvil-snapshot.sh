#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TNT_CORE_PATH="${TNT_CORE_PATH:-$ROOT_DIR/../tnt-core}"
SNAPSHOT_PATH="${ANVIL_SNAPSHOT_PATH:-$ROOT_DIR/crates/chain-setup/anvil/snapshots/localtestnet-state.json}"
ANVIL_PORT="${ANVIL_PORT:-9545}"
ANVIL_URL="http://127.0.0.1:${ANVIL_PORT}"
KEEP_LOGS="${KEEP_SNAPSHOT_LOGS:-0}"
export FOUNDRY_CODE_SIZE_LIMIT="${FOUNDRY_CODE_SIZE_LIMIT:-100000}"

if [[ ! -d "$TNT_CORE_PATH" ]]; then
  echo "error: TNT_CORE_PATH ($TNT_CORE_PATH) does not exist" >&2
  exit 1
fi

mkdir -p "$(dirname "$SNAPSHOT_PATH")"
TMP_STATE="$(mktemp -t anvil-state.XXXXXX.json)"
ANVIL_LOG="$(mktemp -t anvil-log.XXXXXX.txt)"
FORGE_LOG="$(mktemp -t forge-log.XXXXXX.txt)"

cleanup() {
  if [[ -n "${ANVIL_PID:-}" ]]; then
    kill "$ANVIL_PID" >/dev/null 2>&1 || true
    wait "$ANVIL_PID" >/dev/null 2>&1 || true
  fi
  if [[ "$KEEP_LOGS" != "1" ]]; then
    rm -f "$ANVIL_LOG" "$FORGE_LOG"
  else
    echo "Anvil log preserved at: $ANVIL_LOG"
    echo "Forge log preserved at: $FORGE_LOG"
  fi
}
trap cleanup EXIT

echo "Starting Anvil on $ANVIL_URL (logs: $ANVIL_LOG)"
anvil \
  --host 127.0.0.1 \
  --port "$ANVIL_PORT" \
  --base-fee 0 \
  --gas-price 0 \
  --disable-code-size-limit \
  --hardfork cancun \
  --dump-state "$TMP_STATE" \
  >"$ANVIL_LOG" 2>&1 &
ANVIL_PID=$!

for attempt in {1..60}; do
  if cast chain-id --rpc-url "$ANVIL_URL" >/dev/null 2>&1; then
    break
  fi
  if [[ $attempt -eq 60 ]]; then
    echo "error: Anvil failed to start (see $ANVIL_LOG)" >&2
    exit 1
  fi
  sleep 1
done

echo "Running LocalTestnet.s.sol broadcast via forge (logs: $FORGE_LOG)"
pushd "$TNT_CORE_PATH" >/dev/null
if ! forge script script/v2/LocalTestnet.s.sol:LocalTestnetSetup \
  --rpc-url "$ANVIL_URL" \
  --broadcast \
  --slow \
  --legacy \
  --code-size-limit "$FOUNDRY_CODE_SIZE_LIMIT" \
  --skip-simulation \
  >"$FORGE_LOG" 2>&1; then
  echo "forge script exited with a non-zero status; inspect $FORGE_LOG for details" >&2
fi
popd >/dev/null

echo "Stopping Anvil and writing snapshot..."
kill "$ANVIL_PID" >/dev/null 2>&1 || true
wait "$ANVIL_PID" >/dev/null 2>&1 || true

mv "$TMP_STATE" "$SNAPSHOT_PATH"
echo "Snapshot written to $SNAPSHOT_PATH"
echo "Forge log: $FORGE_LOG"
echo "Anvil log: $ANVIL_LOG"
