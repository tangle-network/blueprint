#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Run x402 restricted auth dry-run with delegated caller signature.

Required:
  --service-id <u64>
  --job-index <u32>
  --caller-private-key <hex>

Optional:
  --gateway-url <url>       (default: http://127.0.0.1:8402)
  --body <string>           (default: {})
  --body-file <path>        (alternative to --body)
  --nonce <string>          (default: x402-<unix-secs>-<random>)
  --expiry <unix-secs>      (default: now + 300)
  --caller <address>        (derived from private key if omitted)
  --rpc-url <url>           (optional direct isPermittedCaller parity check)
  --tangle-contract <addr>  (required with --rpc-url for parity check)

Examples:
  scripts/x402-auth-dry-run.sh \
    --service-id 1 \
    --job-index 1 \
    --caller-private-key "$CALLER_PK" \
    --body '{"input":"hello"}'

  scripts/x402-auth-dry-run.sh \
    --gateway-url http://127.0.0.1:8402 \
    --service-id 1 \
    --job-index 1 \
    --caller-private-key "$CALLER_PK" \
    --rpc-url http://127.0.0.1:8545 \
    --tangle-contract 0xYourTangleContract
USAGE
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: missing required command: $1" >&2
    exit 1
  fi
}

GATEWAY_URL="http://127.0.0.1:8402"
SERVICE_ID=""
JOB_INDEX=""
BODY="{}"
BODY_FILE=""
NONCE=""
EXPIRY=""
CALLER_PK=""
CALLER=""
RPC_URL=""
TANGLE_CONTRACT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --gateway-url)
      GATEWAY_URL="$2"
      shift 2
      ;;
    --service-id)
      SERVICE_ID="$2"
      shift 2
      ;;
    --job-index)
      JOB_INDEX="$2"
      shift 2
      ;;
    --body)
      BODY="$2"
      shift 2
      ;;
    --body-file)
      BODY_FILE="$2"
      shift 2
      ;;
    --nonce)
      NONCE="$2"
      shift 2
      ;;
    --expiry)
      EXPIRY="$2"
      shift 2
      ;;
    --caller-private-key)
      CALLER_PK="$2"
      shift 2
      ;;
    --caller)
      CALLER="$2"
      shift 2
      ;;
    --rpc-url)
      RPC_URL="$2"
      shift 2
      ;;
    --tangle-contract)
      TANGLE_CONTRACT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$SERVICE_ID" || -z "$JOB_INDEX" || -z "$CALLER_PK" ]]; then
  echo "error: --service-id, --job-index, and --caller-private-key are required" >&2
  usage
  exit 1
fi

require_cmd cast
require_cmd curl

if [[ -n "$BODY_FILE" ]]; then
  BODY="$(cat "$BODY_FILE")"
fi

if [[ -z "$EXPIRY" ]]; then
  EXPIRY="$(( $(date +%s) + 300 ))"
fi

if [[ -z "$NONCE" ]]; then
  NONCE="x402-$(date +%s)-${RANDOM}"
fi

if [[ -z "$CALLER" ]]; then
  CALLER="$(cast wallet address --private-key "$CALLER_PK")"
fi

BODY_HASH="$(cast keccak "$BODY")"
BODY_HASH_NO_PREFIX="${BODY_HASH#0x}"
PAYLOAD="x402-authorize:${SERVICE_ID}:${JOB_INDEX}:${BODY_HASH_NO_PREFIX}:${NONCE}:${EXPIRY}"
SIG="$(cast wallet sign --private-key "$CALLER_PK" "$PAYLOAD")"

ENDPOINT="${GATEWAY_URL%/}/x402/jobs/${SERVICE_ID}/${JOB_INDEX}/auth-dry-run"

echo "== x402 delegated auth dry-run =="
echo "endpoint: ${ENDPOINT}"
echo "caller:   ${CALLER}"
echo "nonce:    ${NONCE}"
echo "payload:  ${PAYLOAD}"
echo ""

HTTP_OUTPUT="$(
  curl -sS -X POST "$ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "X-TANGLE-CALLER: ${CALLER}" \
    -H "X-TANGLE-CALLER-SIG: ${SIG}" \
    -H "X-TANGLE-CALLER-NONCE: ${NONCE}" \
    -H "X-TANGLE-CALLER-EXPIRY: ${EXPIRY}" \
    --data-binary "$BODY" \
    -w '\n%{http_code}'
)"

HTTP_BODY="$(printf '%s' "$HTTP_OUTPUT" | sed '$d')"
HTTP_CODE="$(printf '%s' "$HTTP_OUTPUT" | tail -n1)"

echo "http_status: ${HTTP_CODE}"
echo "response:"
printf '%s\n' "$HTTP_BODY"

if [[ -n "$RPC_URL" || -n "$TANGLE_CONTRACT" ]]; then
  if [[ -z "$RPC_URL" || -z "$TANGLE_CONTRACT" ]]; then
    echo "error: --rpc-url and --tangle-contract must be provided together" >&2
    exit 1
  fi
  echo ""
  echo "== direct on-chain parity check =="
  PARITY_RESULT="$(cast call --rpc-url "$RPC_URL" "$TANGLE_CONTRACT" \
    "isPermittedCaller(uint64,address)(bool)" "$SERVICE_ID" "$CALLER")"
  echo "isPermittedCaller(${SERVICE_ID}, ${CALLER}) => ${PARITY_RESULT}"
fi
