#!/usr/bin/env python3
"""Exercise the cloud-only Cargo profile, its public CLI process and cargo-dist's global phase."""

import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib


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


def check_global_phase(dist):
    """Run the pinned cargo-dist global phase: verified archives publish and a bad checksum stops it."""
    from cloud_artifact import OUTPUT, ROOT, TARGETS, VERIFIED, digest

    version = tomllib.loads((ROOT / "cli/Cargo.toml").read_text())["package"]["version"]
    command = [dist, "build", f"--tag=cargo-tangle-v{version}", "--artifacts=global", "--output-format=json"]
    shutil.rmtree(OUTPUT, ignore_errors=True)
    OUTPUT.mkdir(parents=True)
    expected = set()
    for target, machine in TARGETS.items():
        stem = f"cargo-tangle-cloud-{target}"
        archive = OUTPUT / f"{stem}.tar.xz"
        elf = b"\x7fELF\x02\x01" + bytes(12) + machine.to_bytes(2, "little") + bytes(64)
        with tarfile.open(archive, "w:xz") as out:
            for name, data in (("cargo-tangle", elf), ("LICENSE-MIT", b"mit"), ("LICENSE-APACHE", b"apache")):
                info = tarfile.TarInfo(f"{stem}/{name}")
                info.size = len(data)
                out.addfile(info, io.BytesIO(data))
        checksum = OUTPUT / f"{archive.name}.sha256"
        checksum.write_text(f"{digest(archive)}  {archive.name}\n")
        expected |= {archive.name, checksum.name}
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    assert result.returncode == 0, result.stderr
    published = {Path(path).name for path in json.loads(result.stdout)["upload_files"]}
    assert expected <= published, sorted(published)
    checksum.write_text("0" * 64 + f"  {archive.name}\n")
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    assert result.returncode != 0 and "checksum mismatch" in result.stderr, result
    assert not any(VERIFIED.iterdir()), "failed verification left publishable outputs"
    shutil.rmtree(OUTPUT)


if __name__ == "__main__":
    if sys.argv[1:2] == ["global"]:
        check_global_phase(sys.argv[2])
    else:
        check(sys.argv[1] if len(sys.argv) > 1 else None)
