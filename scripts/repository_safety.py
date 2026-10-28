#!/usr/bin/env python3
"""Layered publication checks for the current tree and reachable Git history."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ALLOWED_TOP = {
    ".github", ".gitignore", "ARCHITECTURE.md", "CODE_OF_CONDUCT.md",
    "CONTRIBUTING.md", "Cargo.lock", "Cargo.toml", "DCO", "GOVERNANCE.md",
    "LICENSE", "LICENSES.md", "NOTICE", "PUBLICATION_REVIEW.md", "README.md", "ROADMAP.md",
    "SECURITY.md", "STATUS.md", "SUPPORT.md", "TRADEMARKS.md",
    "conformance", "contracts", "docs", "examples", "media", "rustfmt.toml",
    "scripts", "sdk", "tools",
}
DENIED_SUFFIXES = {".devkey", ".ivy", ".jks", ".keystore", ".key", ".p12", ".pfx", ".pem", ".signature.json"}
DENIED_PARTS = {"core", "core_ui", "manager", "orchestrator", "target", ".dart_tool"}
TEXT_PATTERNS = {
    "private key": re.compile(r"BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY"),
    "GitHub token": re.compile(r"gh[pousr]_[A-Za-z0-9_]{20,}"),
    "private home path": re.compile(r"/home/[A-Za-z0-9._-]+"),
    "private IPv4 range": re.compile(r"\b(?:10\.\d{1,3}|192\.168|172\.(?:1[6-9]|2\d|3[01]))\.\d{1,3}\.\d{1,3}\b"),
    "email address": re.compile(r"\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b", re.I),
    "internal identity": re.compile(r"ivy-dev-" + r"[A-Za-z0-9_-]+"),
}
SELF_EXCLUDES = {"scripts/repository_safety.py", "scripts/repository-safety.sh"}
EXPECTED_COMMIT_EMAIL = "36369125+julianra@users.noreply.github.com"
# These exact, GitHub-verified web-flow commits predate this exception. Binding
# the allowance to their immutable commit IDs keeps future GitHub noreply
# committer addresses subject to the normal fail-closed check.
ALLOWED_COMMITTER_EMAILS_BY_COMMIT = {
    "7351a69f438603f51709a62968cd189a0ef3efc3": "noreply@github.com",
    "d3dce01d30aedd9b6678244420d91568072551f4": "noreply@github.com",
}


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True, errors="replace")


def forbidden_path(path: str) -> str | None:
    pure = Path(path)
    if pure.parts and pure.parts[0] not in ALLOWED_TOP:
        return "not in top-level publication allowlist"
    if any(part.lower() in DENIED_PARTS for part in pure.parts):
        return "contains a proprietary, internal or generated path component"
    lower = path.lower()
    if any(lower.endswith(suffix) for suffix in DENIED_SUFFIXES):
        return "forbidden key, package or signature artefact"
    return None


def scan_blob(label: str, path: str, data: bytes, failures: list[str]) -> None:
    if path in SELF_EXCLUDES or b"\0" in data:
        return
    text = data.decode("utf-8", errors="replace")
    for description, pattern in TEXT_PATTERNS.items():
        if pattern.search(text):
            failures.append(f"{label}:{path}: possible {description}")


def scan_tree(commit: str, failures: list[str]) -> None:
    label = "current" if commit == "HEAD" else commit[:12]
    for path in git("ls-tree", "-r", "--name-only", commit).splitlines():
        reason = forbidden_path(path)
        if reason:
            failures.append(f"{label}:{path}: {reason}")
            continue
        try:
            data = subprocess.check_output(["git", "show", f"{commit}:{path}"], cwd=ROOT)
        except subprocess.CalledProcessError:
            failures.append(f"{label}:{path}: cannot inspect blob")
            continue
        scan_blob(label, path, data, failures)


def scan_media(failures: list[str]) -> None:
    for path in (ROOT / "media").rglob("*"):
        if not path.is_file() or path.suffix.lower() == ".md":
            continue
        if path.suffix.lower() != ".svg":
            failures.append(f"current:{path.relative_to(ROOT)}: unreviewed media format")
            continue
        text = path.read_text(errors="replace").lower()
        if "<title" not in text or "<desc" not in text:
            failures.append(f"current:{path.relative_to(ROOT)}: SVG lacks title/description")
        if "<script" in text or re.search(r"(?:href|src)=[\"']https?://", text):
            failures.append(f"current:{path.relative_to(ROOT)}: active or remote SVG content")


def scan_commit_emails(history_head: str, failures: list[str]) -> None:
    for line in git("log", history_head, "--format=%H%x09%ae%x09%ce").splitlines():
        commit, author_email, committer_email = line.split("\t")
        if author_email != EXPECTED_COMMIT_EMAIL:
            failures.append(f"{commit[:12]}: unexpected author email {author_email}")
        allowed_committer_email = ALLOWED_COMMITTER_EMAILS_BY_COMMIT.get(
            commit, EXPECTED_COMMIT_EMAIL
        )
        if committer_email != allowed_committer_email:
            failures.append(f"{commit[:12]}: unexpected committer email {committer_email}")


def main() -> int:
    failures: list[str] = []
    history_head = os.environ.get("IVY_PUBLIC_HISTORY_HEAD", "HEAD")
    current = {
        path
        for path in git("ls-files", "--cached", "--others", "--exclude-standard").splitlines()
        if (ROOT / path).is_file()
    }
    for path in sorted(current):
        reason = forbidden_path(path)
        if reason:
            failures.append(f"current:{path}: {reason}")
        else:
            scan_blob("current", path, (ROOT / path).read_bytes(), failures)
    for commit in git("rev-list", history_head).splitlines():
        scan_tree(commit, failures)
    scan_media(failures)
    scan_commit_emails(history_head, failures)
    if failures:
        print("repository safety checks failed:", file=sys.stderr)
        for item in sorted(set(failures)):
            print(f"- {item}", file=sys.stderr)
        return 1
    print("repository safety: OK (allowlist, HEAD history, commit emails, privacy, proprietary paths, media)")
    print("warning: automated checks are not publication approval; exact-commit human and legal review remain required")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
