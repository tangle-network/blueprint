#!/usr/bin/env bash
set -euo pipefail

# Smoke-test helper for the Blueprint MPP ingress.
#
# POSTs an unpaid request to /mpp/jobs/{service_id}/{job_index}, captures
# the WWW-Authenticate: Payment header(s), and pretty-prints the parsed
# challenge so an operator can confirm that the gateway is wired and that
# the challenge is conformant with the IETF Payment HTTP Authentication
# Scheme.
#
# This is the MPP analog of `scripts/x402-auth-dry-run.sh`. It does NOT
# build a real Authorization: Payment credential or settle a payment —
# that requires a wallet implementation. Use the `examples/x402-blueprint`
# integration tests as a reference for full credential roundtrip.

usage() {
  cat <<'USAGE'
Smoke-test the Blueprint MPP /mpp/jobs/... ingress.

Required:
  --service-id <u64>
  --job-index <u32>

Optional:
  --gateway-url <url>       (default: http://127.0.0.1:8402)
  --body <string>           (default: empty)
  --body-file <path>        (alternative to --body)
  --raw                     Print the raw HTTP response without parsing.

Example:
  scripts/mpp-challenge.sh --service-id 1 --job-index 0

  scripts/mpp-challenge.sh \
    --gateway-url http://127.0.0.1:8402 \
    --service-id 1 \
    --job-index 1 \
    --body '{"input":"hello"}'

The script asserts:
  - HTTP status is 402 Payment Required
  - Content-Type is application/problem+json (RFC 9457)
  - At least one WWW-Authenticate: Payment header is present
  - Each challenge advertises method="blueprintevm" and intent="charge"
  - The Problem Details type URI starts with https://paymentauth.org/problems/
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
BODY=""
BODY_FILE=""
RAW=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --gateway-url)   GATEWAY_URL="$2"; shift 2 ;;
    --service-id)    SERVICE_ID="$2"; shift 2 ;;
    --job-index)     JOB_INDEX="$2"; shift 2 ;;
    --body)          BODY="$2"; shift 2 ;;
    --body-file)     BODY_FILE="$2"; shift 2 ;;
    --raw)           RAW=1; shift ;;
    -h|--help)       usage; exit 0 ;;
    *)               echo "error: unknown argument: $1" >&2; usage; exit 1 ;;
  esac
done

if [[ -z "$SERVICE_ID" || -z "$JOB_INDEX" ]]; then
  echo "error: --service-id and --job-index are required" >&2
  usage
  exit 1
fi

require_cmd curl
require_cmd jq

if [[ -n "$BODY_FILE" ]]; then
  BODY="$(cat "$BODY_FILE")"
fi

ENDPOINT="${GATEWAY_URL%/}/mpp/jobs/${SERVICE_ID}/${JOB_INDEX}"

echo "== MPP challenge smoke-test =="
echo "endpoint: ${ENDPOINT}"
echo ""

# -i to capture headers, -s for silent. We need both the headers and the
# body, so we use a tempfile for the body and let curl print headers.
HEADERS_FILE="$(mktemp)"
BODY_FILE_OUT="$(mktemp)"
trap 'rm -f "$HEADERS_FILE" "$BODY_FILE_OUT"' EXIT

HTTP_CODE="$(
  curl -sS -X POST "$ENDPOINT" \
    -D "$HEADERS_FILE" \
    -o "$BODY_FILE_OUT" \
    --data-binary "$BODY" \
    -w '%{http_code}'
)"

if [[ "$RAW" == "1" ]]; then
  echo "-- raw headers --"
  cat "$HEADERS_FILE"
  echo "-- raw body --"
  cat "$BODY_FILE_OUT"
  echo ""
  exit 0
fi

echo "http_status: ${HTTP_CODE}"

if [[ "$HTTP_CODE" != "402" ]]; then
  echo "FAIL: expected HTTP 402 Payment Required, got ${HTTP_CODE}" >&2
  echo "" >&2
  echo "-- response body --" >&2
  cat "$BODY_FILE_OUT" >&2
  exit 2
fi

CONTENT_TYPE="$(grep -i '^content-type:' "$HEADERS_FILE" | sed 's/.*: *//' | tr -d '\r\n' || true)"
echo "content_type: ${CONTENT_TYPE}"
if [[ "$CONTENT_TYPE" != application/problem+json* ]]; then
  echo "FAIL: expected application/problem+json (RFC 9457), got ${CONTENT_TYPE}" >&2
  exit 2
fi

CHALLENGES=()
while IFS= read -r line; do
  CHALLENGES+=("$line")
done < <(grep -i '^www-authenticate:' "$HEADERS_FILE" | sed 's/.*: *//' | tr -d '\r')

if [[ ${#CHALLENGES[@]} -eq 0 ]]; then
  echo "FAIL: no WWW-Authenticate headers in 402 response" >&2
  exit 2
fi

echo "challenges_emitted: ${#CHALLENGES[@]}"
echo ""

i=0
for c in "${CHALLENGES[@]}"; do
  i=$((i + 1))
  echo "-- challenge ${i} --"
  echo "  raw: ${c}"
  if [[ "$c" != Payment* ]]; then
    echo "  FAIL: challenge is not in Payment scheme" >&2
    exit 2
  fi
  if ! echo "$c" | grep -q 'method="blueprintevm"'; then
    echo "  FAIL: challenge does not advertise method=\"blueprintevm\"" >&2
    exit 2
  fi
  if ! echo "$c" | grep -q 'intent="charge"'; then
    echo "  FAIL: challenge does not advertise intent=\"charge\"" >&2
    exit 2
  fi
  # Pretty-print the parameters as a list of key=value pairs
  echo "$c" | sed -n 's/Payment //p' | tr ',' '\n' | sed 's/^ */  /'
done

echo ""
echo "-- problem details body --"
jq '.' "$BODY_FILE_OUT"

PROBLEM_TYPE="$(jq -r '.type // empty' "$BODY_FILE_OUT")"
if [[ -z "$PROBLEM_TYPE" ]]; then
  echo "FAIL: response body has no .type field" >&2
  exit 2
fi
if [[ "$PROBLEM_TYPE" != https://paymentauth.org/problems/* ]]; then
  echo "FAIL: .type does not point at https://paymentauth.org/problems/, got ${PROBLEM_TYPE}" >&2
  exit 2
fi

echo ""
echo "OK: gateway is wired correctly. ${#CHALLENGES[@]} challenge(s) issued."
