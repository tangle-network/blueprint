#!/usr/bin/env python3
"""Check if a Cargo package has a lib target."""
import json
import sys

def main():
    if len(sys.argv) < 3:
        print("Usage: check-lib-target.py <metadata-file> <package-name>", file=sys.stderr)
        print("false")
        return

    metadata_file = sys.argv[1]
    pkg = sys.argv[2]

    try:
        with open(metadata_file, 'r') as f:
            data = json.load(f)
    except (IOError, json.JSONDecodeError) as e:
        print(f"Error reading metadata: {e}", file=sys.stderr)
        print("false")
        return

    for package in data["packages"]:
        if package["name"] == pkg:
            has_lib = any("lib" in target["kind"] for target in package["targets"])
            print("true" if has_lib else "false")
            return

    print("false")

if __name__ == "__main__":
    main()
