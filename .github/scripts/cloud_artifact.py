#!/usr/bin/env python3
"""Cargo-dist extension: build the same CLI with its cloud-only feature set."""

import hashlib
from pathlib import Path
import subprocess
import shutil
import sys
import tarfile

from check_cloud_distribution import check

ROOT = Path(__file__).resolve().parents[2]
TARGETS = ("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu")
OUTPUT = ROOT / "target/cloud-artifacts"


def digest(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def verify():
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for target in TARGETS:
        archive = OUTPUT / f"cargo-tangle-cloud-{target}.tar.xz"
        # Native jobs upload files; cargo-dist downloads them into its distribution directory.
        for filename in (archive.name, archive.name + ".sha256"):
            incoming = ROOT / "target/distrib" / filename
            if incoming.exists():
                shutil.copyfile(incoming, OUTPUT / filename)
        expected = (archive.with_suffix(archive.suffix + ".sha256")).read_text().split()
        assert expected == [digest(archive), archive.name]
        with tarfile.open(archive, "r:xz") as source:
            name = f"cargo-tangle-cloud-{target}/cargo-tangle"
            assert source.getmember(name).isfile(), name


def build(target):
    assert target in TARGETS, f"unsupported cloud artifact target: {target}"
    version = subprocess.check_output(["rustc", "-vV"], text=True)
    assert f"host: {target}\n" in version, "cloud artifacts use native release runners"
    subprocess.run([
        "cargo", "build", "--locked", "-p", "cargo-tangle", "--bin", "cargo-tangle",
        "--profile", "dist", "--target", target, "--target-dir", str(ROOT / "target/cloud-only"),
        "--no-default-features", "--features", "cloud-only",
    ], cwd=ROOT, check=True)
    binary = ROOT / f"target/cloud-only/{target}/dist/cargo-tangle"
    check(binary)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    stem = f"cargo-tangle-cloud-{target}"
    archive = OUTPUT / f"{stem}.tar.xz"
    with tarfile.open(archive, "w:xz") as out:
        for source, filename in ((binary, "cargo-tangle"), (ROOT / "cli/README.md", "README.md")):
            out.add(source, arcname=f"{stem}/{filename}")
    checksum = digest(archive)
    archive.with_suffix(archive.suffix + ".sha256").write_text(f"{checksum}  {archive.name}\n")
    print(f"built {archive.name}; sha256 {checksum}")


if __name__ == "__main__":
    if sys.argv[1:] == ["verify"]:
        verify()
    elif len(sys.argv) == 3 and sys.argv[1] == "build":
        build(sys.argv[2])
    else:
        raise SystemExit("usage: cloud_artifact.py build <target> | verify")
