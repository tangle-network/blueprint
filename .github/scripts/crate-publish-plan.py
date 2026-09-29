#!/usr/bin/env python3
"""Validate a release manifest and order its crates by workspace dependencies."""

from __future__ import annotations

import argparse
import heapq
import json
from pathlib import Path
import subprocess
import sys


def plan(releases: list[dict], metadata: dict) -> list[tuple[str, str]]:
    members = set(metadata['workspace_members'])
    packages = {p['name']: p for p in metadata['packages'] if p['id'] in members}
    targets: dict[str, str] = {}
    for release in releases:
        if not isinstance(release, dict):
            raise ValueError('release entry must be an object')
        name, version = release.get('package_name'), release.get('version')
        if not isinstance(name, str) or not isinstance(version, str) or not name or not version:
            raise ValueError('each release needs a package_name and version')
        if name in targets:
            raise ValueError(f'duplicate release: {name}')
        package = packages.get(name)
        if not package or package.get('publish') == []:
            raise ValueError(f'{name} is not a publishable workspace package')
        if package['version'] != version:
            raise ValueError(f'{name}: workspace version {package["version"]} differs from release {version}')
        targets[name] = version
    if not targets:
        raise ValueError('release manifest is empty')

    graph = {name: set() for name in targets}
    dependents = {name: set() for name in targets}
    for name in targets:
        for dep in packages[name]['dependencies']:
            dependency = dep['name']
            # Path-only dev dependencies are not part of a published crate.
            if dep.get('kind') == 'dev' and dep.get('path') and dep.get('req') in (None, '*'):
                continue
            if dependency in targets:
                graph[name].add(dependency)
                dependents[dependency].add(name)
    ready = [name for name, deps in graph.items() if not deps]
    heapq.heapify(ready)
    ordered = []
    while ready:
        name = heapq.heappop(ready)
        ordered.append((name, targets[name]))
        for dependent in sorted(dependents[name]):
            graph[dependent].remove(name)
            if not graph[dependent]:
                heapq.heappush(ready, dependent)
    if len(ordered) != len(targets):
        raise ValueError(f'workspace dependency cycle: {sorted(set(targets) - {n for n, _ in ordered})}')
    return ordered


def check_external_dependencies(releases: list[dict], metadata: dict) -> None:
    from verify_release_tags import fetch_index

    members = set(metadata['workspace_members'])
    packages = {p['name']: p for p in metadata['packages'] if p['id'] in members}
    targets = {release['package_name'] for release in releases}
    external = set()
    for name in targets:
        for dep in packages[name]['dependencies']:
            if dep.get('kind') == 'dev' and dep.get('path') and dep.get('req') in (None, '*'):
                continue
            dependency = dep['name']
            if dependency in packages and dependency not in targets:
                external.add((dependency, packages[dependency]['version']))
    for name, version in sorted(external):
        shard = fetch_index(name)
        if shard is None or version not in shard or shard[version]:
            raise ValueError(f'{name} {version}: required workspace dependency is not live')


def validate_tags(releases: list[dict], source: str) -> None:
    subprocess.run(['git', 'merge-base', '--is-ancestor', source, 'HEAD'], check=True)
    for release in releases:
        expected_tag = f"{release['package_name']}-v{release['version']}"
        if release.get('tag') != expected_tag:
            raise ValueError(f'{release["package_name"]}: expected tag {expected_tag}')
        actual = subprocess.check_output(['git', 'rev-list', '-n', '1', expected_tag], text=True).strip()
        if actual != source:
            raise ValueError(f'{expected_tag}: tag resolves to {actual}, expected {source}')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('manifest', type=Path)
    parser.add_argument('--verify-tags', action='store_true')
    parser.add_argument('--check-external', action='store_true')
    args = parser.parse_args()
    payload = json.loads(args.manifest.read_text())
    releases = payload['releases'] if isinstance(payload, dict) else payload
    if args.verify_tags:
        if not isinstance(payload, dict) or not isinstance(payload.get('source_sha'), str):
            raise ValueError('tag verification requires source_sha')
        validate_tags(releases, payload['source_sha'])
    metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--format-version', '1', '--no-deps'], text=True))
    ordered = plan(releases, metadata)
    if args.check_external:
        check_external_dependencies(releases, metadata)
    for name, version in ordered:
        print(f'{name}\t{version}')
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f'publish plan refused: {error}', file=sys.stderr)
        sys.exit(1)
