#!/usr/bin/env python3
"""Fail closed when a registry dependency has no declared licence."""

import json
import subprocess
import sys

metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--format-version", "1"]))
missing = []
inventory = []
for package in metadata["packages"]:
    source = package.get("source") or "local"
    if source.startswith("registry+"):
        licence = package.get("license")
        inventory.append((package["name"], package["version"], licence))
        if not licence:
            missing.append(f"{package['name']} {package['version']}")

for name, version, licence in sorted(inventory):
    print(f"{name} {version}: {licence}")
if missing:
    print("registry dependencies without a declared licence: " + ", ".join(sorted(missing)), file=sys.stderr)
    raise SystemExit(1)
print(f"dependency licence metadata: OK ({len(inventory)} registry packages; human legal review still required)")
