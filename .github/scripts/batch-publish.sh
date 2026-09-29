#!/usr/bin/env bash
set -euo pipefail

manifest=${1:?Usage: batch-publish.sh <release-output.json> [--verify-tags]}
shift
plan_file=$(mktemp)
trap 'rm -f "$plan_file"' EXIT
python3 .github/scripts/crate-publish-plan.py "$manifest" --check-external "$@" > "$plan_file"

sparse_index_path() {
  local name=${1,,}
  name=${name//_/-}
  case ${#name} in
    1) printf '1/%s' "$name" ;;
    2) printf '2/%s' "$name" ;;
    3) printf '3/%s/%s' "${name:0:1}" "$name" ;;
    *) printf '%s/%s/%s' "${name:0:2}" "${name:2:2}" "$name" ;;
  esac
}

version_is_live() {
  local name=$1 version=$2 response status body
  response=$(curl --silent --show-error --location --max-time 20 \
    --write-out $'\n%{http_code}' "https://index.crates.io/$(sparse_index_path "$name")") || return 2
  status=${response##*$'\n'}
  if [[ "$status" == 404 ]]; then return 1; fi
  if [[ "$status" != 200 ]]; then echo "Index returned HTTP $status for $name" >&2; return 2; fi
  body=${response%$'\n'*}
  python3 -c 'import json,sys
try:
    entries = [json.loads(line) for line in sys.stdin if line.strip()]
    if not entries or any(not isinstance(entry.get("vers"), str) for entry in entries):
        raise ValueError("invalid sparse index shard")
except (json.JSONDecodeError, ValueError) as error:
    print(f"Invalid sparse index: {error}", file=sys.stderr)
    sys.exit(2)
version = sys.argv[1]
sys.exit(0 if any(entry["vers"] == version and not entry.get("yanked", False) for entry in entries) else 1)' \
    "$version" <<< "$body"
}

wait_until_live() {
  local name=$1 version=$2 attempt
  for attempt in {1..20}; do
    if version_is_live "$name" "$version"; then return 0; fi
    sleep 15
  done
  return 1
}

count=0
while IFS=$'\t' read -r name version; do
  count=$((count + 1))
  echo "[$count] $name $version"
  if version_is_live "$name" "$version"; then
    echo "SKIP $name $version: live in sparse index"
    continue
  else
    status=$?
    if ((status != 1)); then echo "Registry lookup failed for $name; stopping" >&2; exit 1; fi
  fi
  # A failed publish is actionable. Do not poll or retry an invalid manifest.
  cargo publish --package "$name" --allow-dirty --no-verify
  if ! wait_until_live "$name" "$version"; then
    echo "Publish returned but $name $version is not live after 5 minutes" >&2
    exit 1
  fi
  echo "LIVE $name $version"
  # Conservative registry pacing also works when the previous run spent its burst.
  sleep 65
done < "$plan_file"
echo "Verified $count release entries"
