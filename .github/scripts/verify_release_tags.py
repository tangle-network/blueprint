#!/usr/bin/env python3
"""Verify that each released crate version is available from the Cargo index."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys
import urllib.error
import urllib.request

SPARSE_INDEX = "https://index.crates.io"
USER_AGENT = "tangle-blueprint-release-verify (hello@tangle.tools)"
DEFAULT_TIMEOUT = 20


class RegistryLookupError(RuntimeError):
    """The registry could not provide a complete sparse index shard."""


def normalize(name: str) -> str:
    return name.strip().lower().replace("_", "-")


def sparse_index_path(name: str) -> str:
    name = normalize(name)
    length = len(name)
    if length == 1:
        return f"1/{name}"
    if length == 2:
        return f"2/{name}"
    if length == 3:
        return f"3/{name[0]}/{name}"
    return f"{name[:2]}/{name[2:4]}/{name}"


def parse_index(body: str, name: str) -> dict[str, bool]:
    versions: dict[str, bool] = {}
    for line_number, line in enumerate(body.splitlines(), start=1):
        if not line.strip():
            continue
        try:
            entry = json.loads(line)
        except json.JSONDecodeError as error:
            raise RegistryLookupError(
                f"{name} index contains invalid JSON on line {line_number}"
            ) from error
        version = entry.get("vers") if isinstance(entry, dict) else None
        if not isinstance(version, str):
            raise RegistryLookupError(
                f"{name} index entry on line {line_number} has no version"
            )
        versions[version] = bool(entry.get("yanked", False))
    if not versions:
        raise RegistryLookupError(f"{name} index shard is empty")
    return versions


def fetch_index(name: str, timeout: float = DEFAULT_TIMEOUT) -> dict[str, bool] | None:
    """Return None only when crates.io explicitly says the crate is absent."""
    url = f"{SPARSE_INDEX}/{sparse_index_path(name)}"
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return parse_index(response.read().decode("utf-8"), name)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise RegistryLookupError(f"{name} index returned HTTP {error.code}") from error
    except (urllib.error.URLError, TimeoutError, OSError, UnicodeDecodeError) as error:
        reason = getattr(error, "reason", error)
        raise RegistryLookupError(f"{name} index request failed: {reason}") from error


def workspace_versions() -> dict[str, str]:
    try:
        raw = subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"], text=True
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise RegistryLookupError(f"cargo metadata failed: {error}") from error

    metadata = json.loads(raw)
    members = set(metadata.get("workspace_members", []))
    return {
        package["name"]: package["version"]
        for package in metadata.get("packages", [])
        if package.get("id") in members and package.get("publish") != []
    }


def release_names(path: str) -> list[str]:
    with open(path, encoding="utf-8") as handle:
        payload = json.load(handle)
    releases = payload["releases"] if isinstance(payload, dict) else payload
    return [
        release["package_name"]
        for release in releases
        if isinstance(release, dict) and release.get("package_name")
    ]


def tagged_workspace_names(versions: dict[str, str]) -> list[str]:
    try:
        tags = set(subprocess.check_output(["git", "tag", "--list"], text=True).splitlines())
    except (OSError, subprocess.CalledProcessError) as error:
        raise RegistryLookupError(f"git tag listing failed: {error}") from error
    return [
        name
        for name, version in versions.items()
        if f"{normalize(name)}-v{version}" in tags
    ]


def audited_names(names: list[str], versions: dict[str, str]) -> list[str]:
    """Union the manifest's releases with every tagged workspace crate.

    release-plz skips a crate whose tag already exists and reports it as
    "Already published - Tag <tag>" without listing it, so a tag that exists
    with no matching index entry never reaches `names`. On 2026-07-03 that
    left 19 tagged crates off crates.io while the run went green.
    """
    combined = list(names)
    seen = set(names)
    for name in tagged_workspace_names(versions):
        if name not in seen:
            seen.add(name)
            combined.append(name)
    return combined


def verify(
    names: list[str], versions: dict[str, str]
) -> tuple[list[tuple[str, str, str]], list[tuple[str, str, str]]]:
    missing: list[tuple[str, str, str]] = []
    unverified: list[tuple[str, str, str]] = []
    for name in names:
        version = versions.get(name)
        if version is None:
            missing.append((name, "?", "no exact workspace package version"))
            continue
        try:
            shard = fetch_index(name)
        except RegistryLookupError as error:
            unverified.append((name, version, str(error)))
            continue
        if shard is None:
            missing.append((name, version, "crate not found (HTTP 404)"))
        elif version not in shard:
            missing.append((name, version, "version absent from sparse index"))
        elif shard[version]:
            missing.append((name, version, "version is yanked"))
    return missing, unverified


def report_missing(problems: list[tuple[str, str, str]]) -> str:
    lines = [
        "",
        "=" * 72,
        f"RELEASE VERIFICATION FAILED: {len(problems)} crate(s) are not resolvable",
        "=" * 72,
    ]
    for name, version, reason in problems:
        lines.append(f"  - {name} {version}: {reason}")
    lines.extend(
        [
            "",
            "These results came from complete registry responses. Inspect the tags",
            "and release state before taking recovery action.",
        ]
    )
    return "\n".join(lines)


def report_unverified(problems: list[tuple[str, str, str]]) -> str:
    lines = [
        "",
        "=" * 72,
        f"RELEASE VERIFICATION INCOMPLETE: {len(problems)} crate(s) could not be checked",
        "=" * 72,
    ]
    for name, version, reason in problems:
        lines.append(f"  - {name} {version}: {reason}")
    lines.extend(
        [
            "",
            "No tag-removal advice is safe while the registry result is unknown.",
            "Restore registry connectivity, then rerun the verification.",
        ]
    )
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

    try:
        versions = workspace_versions()
        if Path(args.release_json).is_file():
            names = release_names(args.release_json)
        else:
            names = []
        names = audited_names(names, versions)
        if not names:
            print("No tagged workspace releases, nothing to verify")
            return 0
    except (OSError, json.JSONDecodeError, RegistryLookupError) as error:
        print(f"RELEASE VERIFICATION INCOMPLETE: {error}", file=sys.stderr)
        return 2

    missing, unverified = verify(names, versions)
    if unverified:
        print(report_unverified(unverified), file=sys.stderr)
        if missing:
            print(report_missing(missing), file=sys.stderr)
        return 2
    if missing:
        print(report_missing(missing), file=sys.stderr)
        return 1

    print(f"All {len(names)} released crate(s) resolve in the sparse index")
    return 0


if __name__ == "__main__":
    sys.exit(main())
