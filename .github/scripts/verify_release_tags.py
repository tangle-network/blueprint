#!/usr/bin/env python3
"""Fail a release loudly when a tag exists but the crate is not on crates.io.

release-plz decides a crate is "already published" by looking at git tag
existence. A run that dies mid-batch leaves tags behind without ever reaching
crates.io, and the next run then skips those crates and exits green. The result
is a release whose tags all exist but whose crates are unpinnable.

`batch-publish.sh` already checks the sparse index before publishing, so this
does not duplicate that work: it is the post-condition check. After the batch
step returns, every crate in the release set must resolve in the sparse index.
Anything else exits non-zero and names the tag to delete.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import urllib.error
import urllib.request

SPARSE_INDEX = "https://index.crates.io"
USER_AGENT = "tangle-blueprint-release-verify (hello@tangle.tools)"
DEFAULT_TIMEOUT = 20


def normalize(name: str) -> str:
    """crates.io path form: lowercase, underscores become hyphens."""
    return name.strip().lower().replace("_", "-")


def sparse_index_path(name: str) -> str:
    """The sparse index shard layout: 1/n, 2/n, 3/n/n, else nn/nn/n."""
    name = normalize(name)
    length = len(name)
    if length == 1:
        return f"1/{name}"
    if length == 2:
        return f"2/{name}"
    if length == 3:
        return f"3/{name[0]}/{name}"
    return f"{name[:2]}/{name[2:4]}/{name}"


def parse_index(body: str) -> dict[str, bool]:
    """Map every version in a sparse index shard to its yanked flag."""
    versions: dict[str, bool] = {}
    for line in body.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            entry = json.loads(line)
        except json.JSONDecodeError:
            # A truncated final line means the shard is still propagating.
            # Treat the shard as incomplete rather than as "not published".
            continue
        if "vers" not in entry:
            continue
        versions[entry["vers"]] = bool(entry.get("yanked", False))
    return versions


def fetch_index(name: str, timeout: int = DEFAULT_TIMEOUT) -> dict[str, bool] | None:
    """Fetch one crate's index shard. None means the crate is not on crates.io."""
    url = f"{SPARSE_INDEX}/{sparse_index_path(name)}"
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return parse_index(response.read().decode("utf-8", "replace"))
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise
    except urllib.error.URLError:
        return None


def workspace_versions() -> dict[str, str]:
    """Every workspace member's name -> version, as cargo sees it."""
    raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"], text=True
    )
    meta = json.loads(raw)
    members = meta.get("workspace_members", [])
    versions: dict[str, str] = {}
    for package in meta["packages"]:
        if any(package["id"].startswith(m.rsplit("#", 1)[0]) or package["name"] in m for m in members):
            versions[package["name"]] = package["version"]
    return versions


def release_names(path: str) -> list[str]:
    with open(path, encoding="utf-8") as handle:
        payload = json.load(handle)
    names: list[str] = []
    for release in payload:
        name = release.get("package_name")
        if name:
            names.append(name)
    return names


def unresolved(
    names: list[str],
    versions: dict[str, str],
    fetch=None,
) -> list[tuple[str, str, str]]:
    """Return (name, version, reason) for each crate that is not resolvable."""
    # Resolved at call time, not as a default argument, so tests can patch the
    # module-level fetcher.
    fetch = fetch or fetch_index
    problems: list[tuple[str, str, str]] = []
    for name in names:
        version = versions.get(name)
        if version is None:
            problems.append((name, "?", "no version in cargo metadata"))
            continue
        shard = fetch(name)
        if shard is None:
            problems.append((name, version, "not on crates.io"))
        elif version not in shard:
            problems.append((name, version, "version absent from sparse index"))
        elif shard[version]:
            problems.append((name, version, "version is yanked"))
    return problems


def report(problems: list[tuple[str, str, str]]) -> str:
    lines = [
        "",
        "=" * 72,
        f"RELEASE VERIFICATION FAILED: {len(problems)} crate(s) tagged but not publishable",
        "=" * 72,
    ]
    for name, version, reason in problems:
        lines.append(f"  ✗ {name} {version}  ({reason})")
    lines += [
        "",
        "A tag exists for these crates but crates.io has no matching version, so",
        "consumers cannot resolve them and the next release-plz run will skip them",
        "as 'already published'.",
        "",
        "Recover by deleting the orphan tags, then re-running the publish:",
        "",
    ]
    for name, version, reason in problems:
        if version != "?":
            lines.append(f"  git push origin :refs/tags/{normalize(name)}-v{version}")
    lines += [
        "",
        "Tag deletion must go through the API; the local pre-push mergeability",
        "guard rejects tag deletions.",
        "=" * 72,
    ]
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "release_json",
        nargs="?",
        default="release-output.json",
        help="release-plz output written by release-plz.yml",
    )
    args = parser.parse_args(argv)

    names = release_names(args.release_json)
    if not names:
        print("No packages in release set, nothing to verify")
        return 0

    problems = unresolved(names, workspace_versions())
    if not problems:
        print(f"✓ All {len(names)} released crate(s) resolve in the sparse index")
        return 0

    print(report(problems), file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
