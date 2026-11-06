#!/usr/bin/env python3
"""Exercise the cloud-only Cargo profile and the resulting public CLI process."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile


def check(binary=None):
    tree = subprocess.run([
        "cargo", "tree", "--locked", "-p", "cargo-tangle", "--no-default-features",
        "--features", "cloud-only", "--edges", "normal,build", "--prefix", "none",
        "--format", "{p}",
    ], check=True, capture_output=True, text=True)
    names = {line.split()[0] for line in tree.stdout.splitlines() if line.strip()}
    forbidden = {name for name in names if name in {
        "cargo-generate", "blueprint-cargo-generate", "testcontainers",
    } or (name.startswith("blueprint-") and ("anvil" in name or "testing-utils" in name))}
    assert not forbidden, f"development tools in production graph: {sorted(forbidden)}"
    if binary:
        binary = str(Path(binary).resolve(strict=True))
        with tempfile.TemporaryDirectory(prefix="cargo-tangle-cloud-proof-") as folder:
            for args in (["--help"], ["cloud", "--help"], ["tangle", "cloud", "--help"]):
                result = subprocess.run([binary, *args], cwd=folder, text=True, capture_output=True, check=True)
                assert "cloud" in result.stdout.lower(), result.stdout
                if args == ["--help"]:
                    for omitted in ("blueprint", "delegator", "operator", "harness", "attest"):
                        assert not any(line.lstrip().startswith(omitted + " ") for line in result.stdout.splitlines())
            result = subprocess.run([binary, "blueprint", "--help"], cwd=folder, text=True, capture_output=True)
            assert result.returncode != 0 and result.stdout == "", result
            assert "unrecognized subcommand" in result.stderr, result.stderr
    print(json.dumps({"profile": "cloud-only", "production_packages": len(names),
                      "development_tools": sorted(forbidden), "binary_exercised": bool(binary)}))


if __name__ == "__main__":
    check(sys.argv[1] if len(sys.argv) > 1 else None)
