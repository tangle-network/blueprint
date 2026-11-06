#!/usr/bin/env python3
"""Verify crate availability and source-bound CLI release artifacts."""

from __future__ import annotations

import argparse
import hashlib
import os
import re
import tomllib
import json
from pathlib import Path
import subprocess
import sys
import urllib.error
import urllib.request

SPARSE_INDEX = "https://index.crates.io"
USER_AGENT = "tangle-blueprint-release-verify (hello@tangle.tools)"
DEFAULT_TIMEOUT = 20
CLI_TAG = re.compile(r"cargo-tangle-v[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?(?:\+[A-Za-z0-9.-]+)?")


class ReleaseVerificationError(RuntimeError):
    """A release source or registry response could not be verified."""


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
            raise ReleaseVerificationError(
                f"{name} index contains invalid JSON on line {line_number}"
            ) from error
        version = entry.get("vers") if isinstance(entry, dict) else None
        if not isinstance(version, str):
            raise ReleaseVerificationError(
                f"{name} index entry on line {line_number} has no version"
            )
        versions[version] = bool(entry.get("yanked", False))
    if not versions:
        raise ReleaseVerificationError(f"{name} index shard is empty")
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
        raise ReleaseVerificationError(f"{name} index returned HTTP {error.code}") from error
    except (urllib.error.URLError, TimeoutError, OSError, UnicodeDecodeError) as error:
        reason = getattr(error, "reason", error)
        raise ReleaseVerificationError(f"{name} index request failed: {reason}") from error


def workspace_versions() -> dict[str, str]:
    try:
        raw = subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"], text=True
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise ReleaseVerificationError(f"cargo metadata failed: {error}") from error

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
        raise ReleaseVerificationError(f"git tag listing failed: {error}") from error
    return [
        name
        for name, version in versions.items()
        if f"{normalize(name)}-v{version}" in tags
    ]


def audited_names(names: list[str], versions: dict[str, str]) -> list[str]:
    """Union the manifest's releases with every tagged workspace crate: release-plz omits already-tagged releases, which once left 19 tagged crates unpublished (2026-07-03)."""
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
        except ReleaseVerificationError as error:
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
    lines += ["", "These results came from complete registry responses; inspect tags and release state before recovery."]
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
    lines += ["", "No tag-removal advice is safe while the registry result is unknown; restore connectivity and rerun."]
    return "\n".join(lines)


def github(*args: str, raw: bool = False, timeout: float = 15):
    result = subprocess.run(["gh", *args], capture_output=True, timeout=timeout)
    if result.returncode:
        try:
            if json.loads(result.stdout).get("status") == "404":
                return None
        except (ValueError, AttributeError):
            pass
        raise ReleaseVerificationError(f"GitHub command failed (exit {result.returncode})")
    return result.stdout if raw else json.loads(result.stdout)


def fetch_assets(repo: str, tag: str) -> dict:
    return {row["name"]: row for row in (github("api", f"repos/{repo}/releases/tags/{tag}") or {}).get("assets", [])}


def download_asset(repo: str, asset_id) -> bytes | None:
    return github("api", f"repos/{repo}/releases/assets/{asset_id}", "-H", "Accept: application/octet-stream", raw=True)


def artifact_context(tag: str, plan_path: str):
    if not CLI_TAG.fullmatch(tag or ""):
        raise ReleaseVerificationError("invalid cargo-tangle release tag")
    package = tomllib.loads(Path("cli/Cargo.toml").read_text())["package"]
    if tag != f"cargo-tangle-v{package['version']}":
        raise ReleaseVerificationError("tag and checked-out CLI version differ")
    source = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    tagged = subprocess.check_output(["git", "rev-parse", "--verify", f"refs/tags/{tag}^{{commit}}"], text=True).strip()
    if not re.fullmatch(r"[0-9a-f]{40}", source) or source != tagged:
        raise ReleaseVerificationError("checkout is not the requested tag source")
    plan = json.loads(Path(plan_path).read_text())
    names = plan["artifacts"]
    if plan["announcement_tag"] != tag or not names or any(Path(name).name != name for name in names):
        raise ReleaseVerificationError("invalid artifact plan")
    return source, plan


def artifact_mode(args) -> int:
    repo = os.environ["GITHUB_REPOSITORY"]
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repo):
        raise ReleaseVerificationError("invalid repository")
    if args.artifacts == "dispatch":
        rows = json.loads(Path(args.release_json).read_text())
        rows = rows.get("releases", []) if isinstance(rows, dict) else rows
        if not isinstance(rows, list) or any(not isinstance(row, dict) for row in rows):
            raise ReleaseVerificationError("invalid release output")
        rows = [row for row in rows if row.get("package_name") == "cargo-tangle"]
        tags = set()
        for row in rows:
            version = row.get("version")
            tag = row.get("tag") or (f"cargo-tangle-v{version}" if isinstance(version, str) and version else None)
            if not tag or tag != f"cargo-tangle-v{version}":
                raise ReleaseVerificationError("invalid cargo-tangle release output")
            tags.add(tag)
        if not tags:
            print("No cargo-tangle release to dispatch")
            return 0
        if len(tags) != 1:
            raise ReleaseVerificationError("multiple cargo-tangle releases in one batch")
        tag = tags.pop()
        if not CLI_TAG.fullmatch(tag):
            raise ReleaseVerificationError("invalid dispatch tag")
        record = fetch_assets(repo, tag).get("release-source.json")
        data = download_asset(repo, record["id"]) if record else None
        try:
            published = json.loads(data) if data is not None and record.get("digest") == "sha256:" + hashlib.sha256(data).hexdigest() else None
        except ValueError:
            published = None
        if isinstance(published, dict) and published.get("tag") == tag:
            print(json.dumps({"tag": tag, "dispatch_skipped": "release already records its artifact source", "source_sha": published.get("source_sha")}))
            return 0
        receipt = github("api", "--method", "POST", f"repos/{repo}/actions/workflows/release.yml/dispatches",
                         "-H", "X-GitHub-Api-Version: 2026-03-10", "-f", "ref=main", "-f", f"inputs[tag]={tag}")
        run_id = receipt.get("workflow_run_id") if isinstance(receipt, dict) else None
        server = os.environ.get("GITHUB_SERVER_URL", "https://github.com").rstrip("/")
        api = os.environ.get("GITHUB_API_URL", "https://api.github.com").rstrip("/")
        if type(run_id) is not int or run_id <= 0 or receipt.get("html_url") != f"{server}/{repo}/actions/runs/{run_id}" or receipt.get("run_url") != f"{api}/repos/{repo}/actions/runs/{run_id}":
            raise ReleaseVerificationError("dispatch accepted without a valid run receipt")
        print(json.dumps({"tag": tag, "dispatch_accepted": True, "run_id": run_id, "url": receipt["html_url"]}))
        return 0
    source, plan = artifact_context(args.tag, args.plan)
    if args.artifacts == "record":
        files = {name: hashlib.sha256((Path("target/distrib") / name).read_bytes()).hexdigest()
                 for name in plan["artifacts"]}
        record = {"schema_version": 1, "tag": args.tag, "source_sha": source, "workflow_sha": os.environ["GITHUB_WORKFLOW_SHA"], "sha256": files}
        Path("target/distrib/release-source.json").write_text(json.dumps(record, sort_keys=True) + "\n")
        print(json.dumps({"source_sha": source, "artifacts_recorded": len(files)}))
        return 0
    assets = fetch_assets(repo, args.tag)
    if "release-source.json" not in assets:
        if assets.keys() & plan["artifacts"]:
            raise ReleaseVerificationError("published assets have no source record; release a new version instead of replacing them")
        print("Release artifact set is incomplete")
        return 3
    data = download_asset(repo, assets["release-source.json"]["id"])
    if data is None:
        raise ReleaseVerificationError("published source record is unavailable")
    if assets["release-source.json"].get("digest") != "sha256:" + hashlib.sha256(data).hexdigest():
        raise ReleaseVerificationError("published source record integrity is unverified")
    record = json.loads(data)
    hashes = record.get("sha256", {})
    if record.get("schema_version") != 1 or record.get("tag") != args.tag or record.get("source_sha") != source:
        raise ReleaseVerificationError("published record belongs to a different source or schema; release a new version")
    if not set(plan["artifacts"]).issubset(assets) or set(hashes) != set(plan["artifacts"]):
        return 3
    for name, digest in hashes.items():
        if not re.fullmatch(r"[0-9a-f]{64}", digest) or assets[name].get("digest") != "sha256:" + digest:
            return 3
        checksum = plan["artifacts"][name].get("checksum") or (name + ".sha256" if name + ".sha256" in hashes else None)
        if checksum:
            payload = download_asset(repo, assets[checksum]["id"])
            parts = payload.decode().strip().split(maxsplit=1) if payload is not None else []
            if len(parts) != 2 or parts[0] != digest or parts[1].removeprefix("*") != name:
                return 3
    print(json.dumps({"complete": True, "tag": args.tag, "source_sha": source, "artifacts": len(hashes)}))
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "release_json",
        nargs="?",
        default="release-output.json",
        help="release-plz output written by release-plz.yml",
    )
    parser.add_argument("--artifacts", choices=["check", "record", "dispatch"])
    parser.add_argument("--plan")
    parser.add_argument("--tag")
    args = parser.parse_args(argv)
    if args.artifacts:
        try:
            return artifact_mode(args)
        except (OSError, ValueError, KeyError, TypeError, AttributeError, subprocess.CalledProcessError, subprocess.TimeoutExpired, ReleaseVerificationError) as error:
            print(f"ARTIFACT VERIFICATION INCOMPLETE: {error}", file=sys.stderr)
            return 2

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
    except (OSError, json.JSONDecodeError, ReleaseVerificationError) as error:
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
