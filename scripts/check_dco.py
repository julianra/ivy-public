#!/usr/bin/env python3
"""Require a DCO 1.1 sign-off on every commit in a proposed range."""

from __future__ import annotations

import re
import subprocess
import sys

SIGN_OFF = re.compile(r"(?m)^Signed-off-by: .+ <[^<>\s]+@[^<>\s]+>$")


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: check_dco.py <base-commit> <head-commit>", file=sys.stderr)
        return 2
    base, head = sys.argv[1:]
    commits = subprocess.check_output(
        ["git", "rev-list", "--reverse", f"{base}..{head}"], text=True
    ).splitlines()
    failures = []
    for commit in commits:
        message = subprocess.check_output(
            ["git", "show", "-s", "--format=%B", commit], text=True
        )
        if not SIGN_OFF.search(message):
            failures.append(commit)
    if failures:
        print("DCO 1.1 sign-off missing from:", file=sys.stderr)
        for commit in failures:
            print(f"- {commit}", file=sys.stderr)
        print("Use git commit -s and update the pull request.", file=sys.stderr)
        return 1
    print(f"DCO sign-off: OK ({len(commits)} commits)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
