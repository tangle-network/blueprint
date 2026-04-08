#!/usr/bin/env bash
# Quick SSE consumer using curl — works everywhere.
#
# Usage:
#   ./consume_sse.sh http://operator:8080 job-123

set -euo pipefail

BASE_URL="${1:?Usage: $0 <base_url> <job_id>}"
JOB_ID="${2:?Usage: $0 <base_url> <job_id>}"

echo "Connecting to SSE stream for job ${JOB_ID}..."
echo "Press Ctrl+C to stop."
echo

curl -N -s "${BASE_URL}/v1/jobs/${JOB_ID}/events" | while IFS= read -r line; do
  case "$line" in
    data:*)
      data="${line#data: }"
      status=$(echo "$data" | python3 -c "import sys,json; print(json.load(sys.stdin).get('status',''))" 2>/dev/null || echo "")
      echo "[${status:-event}] $data"

      if [ "$status" = "completed" ] || [ "$status" = "failed" ] || [ "$status" = "cancelled" ]; then
        echo
        echo "Job reached terminal state: $status"
        exit 0
      fi
      ;;
    event:*)
      # SSE event type line — logged by the data handler
      ;;
  esac
done
