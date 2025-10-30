#!/bin/bash
set -euo pipefail

# This script publishes Rust crates while respecting crates.io's rate limit.
# Rate limit: burst=30 (immediate), then 1 request per minute
# See: https://github.com/rust-lang/crates.io/blob/master/src/middleware/app.rs
#
# Strategy:
# - Publish first 30 crates immediately (using the burst allowance)
# - Wait 60 seconds between each subsequent crate (1 per minute rate)

RELEASE_JSON="${1:-}"
BURST_SIZE=30       # Can publish 30 immediately
POST_BURST_DELAY=60 # Must wait 60 seconds between each publish after burst

if [[ -z "$RELEASE_JSON" ]]; then
    echo "Error: No release JSON file provided"
    echo "Usage: $0 <release-output.json>"
    exit 1
fi

if [[ ! -f "$RELEASE_JSON" ]]; then
    echo "Error: File not found: $RELEASE_JSON"
    exit 1
fi

# Extract package names from the release-plz JSON output
# The JSON format is: {"releases": [{"package_name": "...", "version": "...", ...}, ...]}
if ! packages=$(jq -r '.releases[].package_name' "$RELEASE_JSON" 2>&1); then
    echo "Error: Failed to parse JSON file"
    echo "$packages"
    exit 1
fi

if [[ -z "$packages" ]]; then
    echo "No packages to publish"
    exit 0
fi

# Convert to array
mapfile -t package_array <<< "$packages"

# Remove any empty elements
package_array=("${package_array[@]}")

total_packages=${#package_array[@]}

if ((total_packages == 0)); then
    echo "No packages to publish"
    exit 0
fi

echo "Found $total_packages package(s) to publish"
echo ""
echo "crates.io rate limit: burst=$BURST_SIZE immediate, then 1 per minute"
if ((total_packages <= BURST_SIZE)); then
    echo "All packages fit within burst limit - will publish immediately"
else
    post_burst_packages=$((total_packages - BURST_SIZE))
    estimated_time=$((post_burst_packages * POST_BURST_DELAY / 60))
    echo "Will publish first $BURST_SIZE immediately, then $post_burst_packages more at 1/minute"
    echo "Estimated total time: ~$estimated_time minutes"
fi
echo ""

# Publish packages
for ((i=0; i<total_packages; i++)); do
    package="${package_array[$i]}"
    echo "========================================="
    echo "[$((i+1))/$total_packages] Publishing $package"
    echo "========================================="

    # Run cargo publish with the same flags as release-plz
    if cargo publish --package "$package" --allow-dirty --no-verify; then
        echo "✓ Successfully published $package"
    else
        echo "✗ Failed to publish $package"
        exit 1
    fi

    # Delay logic:
    # - During burst (packages 1-29): small 2s delay to avoid hammering
    # - After publishing package 30 onwards: 60s delay before next publish (rate limit kicks in)
    if ((i < BURST_SIZE - 1)); then
        # Still in burst (packages 1-29), small delay between publishes
        sleep 2
    elif ((i >= BURST_SIZE - 1 && i < total_packages - 1)); then
        # Just published package 30+, need to wait for rate limit token before next
        echo ""
        echo "Rate limit: waiting $POST_BURST_DELAY seconds before next publish..."
        sleep "$POST_BURST_DELAY"
        echo ""
    fi
done

echo ""
echo "========================================="
echo "✓ Successfully published all $total_packages packages!"
echo "========================================="
