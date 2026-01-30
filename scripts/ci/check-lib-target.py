#!/usr/bin/env python3
"""Check if a Cargo package has a lib target."""
import json
import os
import sys

def main():
    cargo_metadata = os.environ.get("CARGO_METADATA")
    pkg = os.environ.get("PKG")

    if not cargo_metadata or not pkg:
        print("false")
        return

    data = json.loads(cargo_metadata)
    for package in data["packages"]:
        if package["name"] == pkg:
            has_lib = any("lib" in target["kind"] for target in package["targets"])
            print("true" if has_lib else "false")
            return

    print("false")

if __name__ == "__main__":
    main()
