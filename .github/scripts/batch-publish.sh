#!/bin/bash
set -euo pipefail

# This script publishes Rust crates in topological dependency order while
# respecting crates.io's rate limit.
#
# Rate limit: burst=30 (immediate), then 1 request per minute
# See: https://github.com/rust-lang/crates.io/blob/master/src/middleware/app.rs
#
# Key: crates are sorted so leaf dependencies publish first. Without this,
# a crate that depends on blueprint-std will fail if blueprint-std hasn't
# been published yet, because cargo publish --no-verify still checks that
# all dependencies resolve on the registry.

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
if ! packages_json=$(jq -r '.[].package_name' "$RELEASE_JSON" 2>&1); then
    echo "Error: Failed to parse JSON file"
    echo "$packages_json"
    exit 1
fi

if [[ -z "$packages_json" ]]; then
    echo "No packages to publish"
    exit 0
fi

# Build a set of packages to publish (from release-plz output)
declare -A publish_set
while IFS= read -r pkg; do
    [[ -z "$pkg" ]] && continue
    publish_set["$pkg"]=1
done <<< "$packages_json"

if ((${#publish_set[@]} == 0)); then
    echo "No packages to publish"
    exit 0
fi

echo "Resolving topological publish order for ${#publish_set[@]} package(s)..."

# Use cargo metadata to get dependency graph and sort topologically.
# This ensures leaf crates (no internal deps) publish first.
topo_order=()
if command -v cargo &>/dev/null; then
    # Get all workspace members and their internal dependencies
    metadata=$(cargo metadata --format-version 1 --no-deps 2>/dev/null || true)
    if [[ -n "$metadata" ]]; then
        # Extract workspace member names in topological order:
        # For each package, count how many OTHER workspace packages it depends on.
        # Sort by count ascending (fewest deps first = leaves first).
        topo_order_raw=$(echo "$metadata" | python3 -c "
import json, sys

meta = json.load(sys.stdin)
workspace_members = set()
pkg_map = {}

# Collect workspace member package IDs and names
for pkg in meta['packages']:
    pkg_id = pkg['id']
    if any(pkg_id.startswith(m.rsplit('#', 1)[0]) or pkg['name'] in m for m in meta.get('workspace_members', [])):
        workspace_members.add(pkg['name'])
        pkg_map[pkg['name']] = pkg

# For simple topological sort: count internal deps per package
dep_counts = {}
for name in workspace_members:
    pkg = pkg_map.get(name)
    if not pkg:
        dep_counts[name] = 0
        continue
    internal_deps = 0
    for dep in pkg.get('dependencies', []):
        if dep['name'] in workspace_members:
            internal_deps += 1
    dep_counts[name] = internal_deps

# Sort: fewest internal deps first (leaves), most last (root crates like sdk)
for name in sorted(dep_counts.keys(), key=lambda n: (dep_counts[n], n)):
    print(name)
" 2>/dev/null || true)

        if [[ -n "$topo_order_raw" ]]; then
            while IFS= read -r pkg; do
                # Only include packages that release-plz wants to publish
                if [[ -n "${publish_set[$pkg]:-}" ]]; then
                    topo_order+=("$pkg")
                fi
            done <<< "$topo_order_raw"
        fi
    fi
fi

# Fallback: if topo sort failed, use original order
if ((${#topo_order[@]} == 0)); then
    echo "Warning: topological sort unavailable, using original order"
    mapfile -t topo_order <<< "$packages_json"
fi

# Verify we didn't lose any packages
if ((${#topo_order[@]} != ${#publish_set[@]})); then
    echo "Warning: topo sort has ${#topo_order[@]} packages but release-plz wants ${#publish_set[@]}"
    # Add any missing packages at the end
    for pkg in "${!publish_set[@]}"; do
        found=0
        for ordered in "${topo_order[@]}"; do
            [[ "$ordered" == "$pkg" ]] && found=1 && break
        done
        ((found == 0)) && topo_order+=("$pkg")
    done
fi

total_packages=${#topo_order[@]}

echo "Found $total_packages package(s) to publish (topologically ordered)"
echo ""
echo "Publish order:"
for ((i=0; i<total_packages; i++)); do
    echo "  $((i+1)). ${topo_order[$i]}"
done
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

# Track failures for retry
declare -a failed_packages=()

# Publish packages in topological order
for ((i=0; i<total_packages; i++)); do
    package="${topo_order[$i]}"
    echo "========================================="
    echo "[$((i+1))/$total_packages] Publishing $package"
    echo "========================================="

    # Run cargo publish — treat "already exists" as success
    output=$(cargo publish --package "$package" --allow-dirty --no-verify 2>&1)
    exit_code=$?
    if [[ $exit_code -eq 0 ]]; then
        echo "✓ Successfully published $package"
    elif echo "$output" | grep -q "already exists"; then
        echo "✓ $package already published (skipped)"
    else
        echo "⚠ First attempt failed for $package, waiting 30s and retrying..."
        echo "$output" | tail -3
        sleep 30
        output=$(cargo publish --package "$package" --allow-dirty --no-verify 2>&1)
        exit_code=$?
        if [[ $exit_code -eq 0 ]]; then
            echo "✓ Successfully published $package (retry)"
        elif echo "$output" | grep -q "already exists"; then
            echo "✓ $package already published (skipped)"
        else
            echo "✗ Failed to publish $package after retry"
            echo "$output" | tail -5
            failed_packages+=("$package")
            echo "Continuing with remaining packages..."
        fi
    fi

    # Delay logic:
    # - During burst (packages 1-29): small 5s delay to let registry index update
    # - After burst: 60s delay for rate limit
    if ((i < BURST_SIZE - 1)); then
        sleep 5
    elif ((i >= BURST_SIZE - 1 && i < total_packages - 1)); then
        echo ""
        echo "Rate limit: waiting $POST_BURST_DELAY seconds before next publish..."
        sleep "$POST_BURST_DELAY"
        echo ""
    fi
done

echo ""
echo "========================================="
if ((${#failed_packages[@]} > 0)); then
    echo "⚠ Published $((total_packages - ${#failed_packages[@]}))/$total_packages packages"
    echo ""
    echo "Failed packages:"
    for pkg in "${failed_packages[@]}"; do
        echo "  ✗ $pkg"
    done
    echo ""
    echo "Re-run this script or publish manually: cargo publish --package <name> --allow-dirty --no-verify"
    exit 1
else
    echo "✓ Successfully published all $total_packages packages!"
fi
echo "========================================="
