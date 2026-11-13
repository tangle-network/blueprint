#!/usr/bin/env bash
set -euo pipefail

manifest=${1:?Usage: batch-publish.sh <release-output.json> [--verify-tags]}
shift
plan_file=$(mktemp)
trap 'rm -f "$plan_file"' EXIT
python3 .github/scripts/crate-publish-plan.py "$manifest" --check-external "$@" > "$plan_file"

version_is_live() {
  python3 - "$1" "$2" <<'PY'
import sys
sys.path.insert(0, ".github/scripts")
from verify_release_tags import ReleaseVerificationError, fetch_index
try:
    shard = fetch_index(sys.argv[1])
except ReleaseVerificationError as error:
    print(error, file=sys.stderr)
    sys.exit(2)
sys.exit(0 if shard and shard.get(sys.argv[2]) is False else 1)
PY
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
  # cargo can resolve a dependency published moments ago from an index copy cached before it (run 43361716372,
  # over 2 min after the check above saw it); index.crates.io caches for max-age=600, so retry only that for 10 min.
  for attempt in {1..10}; do
    if output=$(cargo publish --package "$name" --allow-dirty --no-verify 2>&1); then break; fi
    printf '%s\n' "$output" >&2
    if ((attempt == 10)) || [[ $output != *"failed to select a version for the requirement"* ]]; then exit 1; fi
    sleep 60
  done
  printf '%s\n' "$output"
  if ! wait_until_live "$name" "$version"; then
    echo "Publish returned but $name $version is not live after 5 minutes" >&2
    exit 1
  fi
  echo "LIVE $name $version"
  # Conservative registry pacing also works when the previous run spent its burst.
  sleep 65
done < "$plan_file"
echo "Verified $count release entries"
