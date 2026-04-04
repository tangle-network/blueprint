#!/usr/bin/env bash
#
# Detect which workspace crates changed in a PR and output the minimal
# set of crates to test (changed crates + their transitive dependents).
#
# Usage: ./changed-crates.sh [BASE_REF]
#   BASE_REF defaults to origin/main
#
# Output: JSON array of crate names to stdout. All logs go to stderr.

set -euo pipefail

BASE="${1:-origin/main}"

log() { echo "[changed-crates] $*" >&2; }

# --- 1. Get changed files ---

CHANGED_FILES="$(git diff --name-only "$BASE"...HEAD 2>/dev/null || git diff --name-only "$BASE" HEAD)"

if [ -z "$CHANGED_FILES" ]; then
  log "No changed files detected."
  echo "[]"
  exit 0
fi

FILE_COUNT="$(echo "$CHANGED_FILES" | wc -l | tr -d ' ')"
log "Changed files ($FILE_COUNT):"
echo "$CHANGED_FILES" | head -20 >&2
[ "$FILE_COUNT" -gt 20 ] && log "... and $((FILE_COUNT - 20)) more"

# --- 2. Check for workspace-wide triggers ---

FULL_TEST=false
while IFS= read -r file; do
  case "$file" in
    Cargo.toml|Cargo.lock|rust-toolchain.toml|rust-toolchain|.github/*)
      FULL_TEST=true
      log "Workspace-wide trigger: $file -> full test"
      break
      ;;
  esac
done <<< "$CHANGED_FILES"

# --- 3. Collect cargo metadata ---

WORKSPACE_ROOT="$(git rev-parse --show-toplevel)"

TMPDIR_WORK="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_WORK"' EXIT

log "Running cargo metadata (no-deps)..."
cargo metadata --format-version 1 --no-deps 2>/dev/null > "$TMPDIR_WORK/meta-no-deps.json"

if [ "$FULL_TEST" = "false" ]; then
  log "Running cargo metadata (full, for dep graph)..."
  cargo metadata --format-version 1 2>/dev/null > "$TMPDIR_WORK/meta-full.json"
else
  echo '{}' > "$TMPDIR_WORK/meta-full.json"
fi

echo "$CHANGED_FILES" > "$TMPDIR_WORK/changed-files.txt"

# --- 4. Map files to crates, compute transitive dependents ---

python3 - "$FULL_TEST" "$WORKSPACE_ROOT" "$TMPDIR_WORK" <<'PYEOF'
import json
import os
import sys
from collections import defaultdict, deque

full_test = sys.argv[1] == "true"
workspace_root = sys.argv[2]
tmpdir = sys.argv[3]

def log(msg):
    print(f"[changed-crates] {msg}", file=sys.stderr)

# Load no-deps metadata for crate name <-> path mapping.
with open(os.path.join(tmpdir, "meta-no-deps.json")) as f:
    meta_no_deps = json.load(f)

workspace_members = set(meta_no_deps.get("workspace_members", []))

# Build: relative dir -> crate_name
crate_dirs = {}
all_crates = []

for pkg in meta_no_deps["packages"]:
    # Only workspace crates (manifest under workspace root, not workspace-hack).
    if not pkg["manifest_path"].startswith(workspace_root):
        continue
    name = pkg["name"]
    if name == "workspace-hack":
        continue
    manifest_dir = os.path.dirname(pkg["manifest_path"])
    rel_dir = os.path.relpath(manifest_dir, workspace_root)
    crate_dirs[rel_dir] = name
    all_crates.append(name)

log(f"Workspace crates: {len(all_crates)}")

# Full test: output everything.
if full_test:
    log("Full test triggered -- outputting all crates.")
    print(json.dumps(sorted(all_crates)))
    sys.exit(0)

# Load changed files.
with open(os.path.join(tmpdir, "changed-files.txt")) as f:
    changed_files = [line.strip() for line in f if line.strip()]

# Map changed files to crates (longest-prefix match).
sorted_dirs = sorted(crate_dirs.keys(), key=len, reverse=True)

changed_crates = set()
for fpath in changed_files:
    for d in sorted_dirs:
        if fpath.startswith(d + "/") or fpath == d:
            changed_crates.add(crate_dirs[d])
            break

if not changed_crates:
    log("No workspace crates changed (docs-only or non-crate files).")
    print("[]")
    sys.exit(0)

log(f"Directly changed crates ({len(changed_crates)}): {sorted(changed_crates)}")

# Build reverse dependency graph from full metadata.
with open(os.path.join(tmpdir, "meta-full.json")) as f:
    meta_full = json.load(f)

ws_crate_names = set(all_crates)

def pkg_id_to_name(pkg_id):
    """Extract crate name from a cargo metadata package ID.

    Formats:
      path+file:///path#name@version  -> name
      path+file:///path#version       -> directory name (last path component)
      registry+https://...#name@ver   -> name
      name version (registry+...)     -> name (legacy format)
    """
    if "#" in pkg_id:
        fragment = pkg_id.split("#", 1)[1]
        if "@" in fragment:
            return fragment.split("@", 1)[0]
        path_part = pkg_id.split("#", 1)[0]
        return path_part.rstrip("/").rsplit("/", 1)[-1]
    return pkg_id.split(" ", 1)[0]

# crate_name -> set of workspace crates that directly depend on it
reverse_deps = defaultdict(set)
for node in meta_full.get("resolve", {}).get("nodes", []):
    node_name = pkg_id_to_name(node["id"])
    if node_name not in ws_crate_names:
        continue
    for dep in node.get("deps", []):
        dep_name = pkg_id_to_name(dep["pkg"])
        if dep_name in ws_crate_names:
            reverse_deps[dep_name].add(node_name)

# BFS from changed crates to find all transitive dependents.
to_test = set(changed_crates)
queue = deque(changed_crates)
while queue:
    crate = queue.popleft()
    for dependent in reverse_deps.get(crate, []):
        if dependent not in to_test:
            to_test.add(dependent)
            queue.append(dependent)

added = to_test - changed_crates
if added:
    log(f"Transitive dependents ({len(added)}): {sorted(added)}")

log(f"Total crates to test: {len(to_test)}")
print(json.dumps(sorted(to_test)))
PYEOF
