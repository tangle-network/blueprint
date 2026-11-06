#!/usr/bin/env python3
"""Cargo-dist extension: build the same CLI with its cloud-only feature set."""

import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

from check_cloud_distribution import check

ROOT = Path(__file__).resolve().parents[2]
TARGETS = {"x86_64-unknown-linux-gnu": 62, "aarch64-unknown-linux-gnu": 183}
# Not target/distrib: cargo-dist deletes its planned outputs there before running the verify hook.
OUTPUT = ROOT / "target/cloud-artifacts"
VERIFIED = ROOT / "target/verified-cloud-artifacts"


def digest(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def inspect_archive(archive, target, execute=False):
    with tarfile.open(archive, "r:xz") as source:
        member = source.getmember(f"cargo-tangle-cloud-{target}/cargo-tangle")
        assert member.isfile() and member.size > 64, "missing CLI executable"
        for notice in ("LICENSE-MIT", "LICENSE-APACHE"):
            assert source.getmember(f"cargo-tangle-cloud-{target}/{notice}").isfile(), "missing license notice"
        with source.extractfile(member) as binary:
            header = binary.read(20)
            assert header[:6] == b"\x7fELF\x02\x01", "expected little-endian ELF64"
            assert int.from_bytes(header[18:20], "little") == TARGETS[target], "wrong ELF architecture"
            if execute:
                with tempfile.TemporaryDirectory(prefix="cargo-tangle-cloud-extracted-") as folder:
                    path = Path(folder) / "cargo-tangle"
                    with path.open("wb") as out:
                        out.write(header)
                        shutil.copyfileobj(binary, out)
                    path.chmod(0o700)
                    check(path)


def verify(targets, source):
    VERIFIED.mkdir(parents=True, exist_ok=True)
    # Verified paths hold only outputs of a run in which every input passed.
    for target in targets:
        assert target in TARGETS, "unsupported cloud artifact target"
        for suffix in (".tar.xz", ".tar.xz.sha256"):
            (VERIFIED / f"cargo-tangle-cloud-{target}{suffix}").unlink(missing_ok=True)
    files = []
    for target in targets:
        archive = source / f"cargo-tangle-cloud-{target}.tar.xz"
        checksum = archive.with_suffix(archive.suffix + ".sha256")
        assert checksum.read_text().split() == [digest(archive), archive.name], "checksum mismatch"
        inspect_archive(archive, target)
        files.extend((archive, checksum))
    for path in files:
        shutil.copyfile(path, VERIFIED / path.name)


def build(target):
    assert target in TARGETS, f"unsupported cloud artifact target: {target}"
    version = subprocess.check_output(["rustc", "-vV"], text=True)
    assert f"host: {target}\n" in version, "cloud artifacts use native release runners"
    metadata = subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT)
    target_dir = Path(json.loads(metadata)["target_directory"]) / "cloud-only"
    subprocess.run([
        "cargo", "build", "--locked", "-p", "cargo-tangle", "--bin", "cargo-tangle",
        "--profile", "dist", "--target", target, "--target-dir", str(target_dir),
        "--no-default-features", "--features", "cloud-only",
    ], cwd=ROOT, check=True)
    binary = target_dir / target / "dist/cargo-tangle"
    OUTPUT.mkdir(parents=True, exist_ok=True)
    stem = f"cargo-tangle-cloud-{target}"
    archive = OUTPUT / f"{stem}.tar.xz"
    with tarfile.open(archive, "w:xz") as out:
        files = ((binary, "cargo-tangle"), (ROOT / "cli/README.md", "README.md"))
        files += tuple((ROOT / name, name) for name in ("LICENSE-MIT", "LICENSE-APACHE"))
        for source, filename in files:
            out.add(source, arcname=f"{stem}/{filename}")
    checksum = digest(archive)
    archive.with_suffix(archive.suffix + ".sha256").write_text(f"{checksum}  {archive.name}\n")
    inspect_archive(archive, target, execute=True)
    used = sum(path.stat().st_size for path in target_dir.rglob("*") if path.is_file())
    # The standard dist build runs next on the same runner disk.
    shutil.rmtree(target_dir)
    print(f"built and executed {archive.name}; sha256 {checksum}; removed {used} byte target")


if __name__ == "__main__":
    if sys.argv[1:] == ["verify"]:
        verify(TARGETS, OUTPUT)
    elif len(sys.argv) == 4 and sys.argv[1] == "verify":
        verify((sys.argv[2],), Path(sys.argv[3]))
    elif len(sys.argv) == 3 and sys.argv[1] == "build":
        build(sys.argv[2])
    else:
        raise SystemExit("usage: cloud_artifact.py build <target> | verify [<target> <source-dir>]")
