#!/usr/bin/env python3
"""Validate bilingual documentation parity and internal Markdown links."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LANGS = ("en", "nl")
META = {
    "doc-key": re.compile(r"<!-- doc-key: ([a-z0-9-]+) -->"),
    "parity-version": re.compile(r"<!-- parity-version: ([0-9]+) -->"),
    "sections": re.compile(r"<!-- sections: ([a-z0-9,-]+) -->"),
}
SECTION = re.compile(r"<!-- section: ([a-z0-9-]+) -->")
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
FENCE = re.compile(r"```(bash|sh|rust|yaml|json)\n(.*?)```", re.DOTALL)


def fail(message: str, failures: list[str]) -> None:
    failures.append(message)


def metadata(text: str, path: Path, failures: list[str]) -> dict[str, str]:
    result: dict[str, str] = {}
    for name, pattern in META.items():
        match = pattern.search(text)
        if not match:
            fail(f"{path}: missing {name} metadata", failures)
        else:
            result[name] = match.group(1)
    return result


def paired_docs(failures: list[str]) -> None:
    trees = {
        lang: {path.relative_to(ROOT / "docs" / lang) for path in (ROOT / "docs" / lang).rglob("*.md")}
        for lang in LANGS
    }
    if trees["en"] != trees["nl"]:
        fail(f"language trees differ: en-only={sorted(trees['en'] - trees['nl'])}; nl-only={sorted(trees['nl'] - trees['en'])}", failures)

    outside = [
        path.relative_to(ROOT)
        for path in (ROOT / "docs").rglob("*.md")
        if path.relative_to(ROOT / "docs").parts[0] not in LANGS
    ]
    if outside:
        fail(f"human docs outside docs/en or docs/nl: {sorted(outside)}", failures)

    for relative in sorted(trees["en"] & trees["nl"]):
        en_path = ROOT / "docs" / "en" / relative
        nl_path = ROOT / "docs" / "nl" / relative
        en, nl = en_path.read_text(), nl_path.read_text()
        en_meta = metadata(en, en_path, failures)
        nl_meta = metadata(nl, nl_path, failures)
        if en_meta != nl_meta:
            fail(f"{relative}: parity metadata differs: {en_meta} != {nl_meta}", failures)
        declared = en_meta.get("sections", "").split(",") if en_meta.get("sections") else []
        en_sections, nl_sections = SECTION.findall(en), SECTION.findall(nl)
        if en_sections != declared or nl_sections != declared:
            fail(f"{relative}: section order differs from declared sections", failures)
        if FENCE.findall(en) != FENCE.findall(nl):
            fail(f"{relative}: executable code blocks drifted", failures)


def links(failures: list[str]) -> None:
    for path in ROOT.rglob("*.md"):
        if ".git" in path.parts or "target" in path.parts:
            continue
        for raw in LINK.findall(path.read_text()):
            destination = raw.strip().split()[0].strip("<>")
            if destination.startswith(("http://", "https://", "mailto:", "#")):
                continue
            target = destination.split("#", 1)[0]
            if target and not (path.parent / target).resolve().exists():
                fail(f"{path.relative_to(ROOT)}: broken link {destination}", failures)


def gateways(failures: list[str]) -> None:
    for name in ("README.md", "ARCHITECTURE.md", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md", "SECURITY.md", "SUPPORT.md", "GOVERNANCE.md", "ROADMAP.md", "STATUS.md", "TRADEMARKS.md", "LICENSES.md", "PUBLICATION_REVIEW.md"):
        text = (ROOT / name).read_text()
        if "docs/en/" not in text or "docs/nl/" not in text:
            fail(f"{name}: root gateway must link both languages", failures)


def main() -> int:
    failures: list[str] = []
    paired_docs(failures)
    links(failures)
    gateways(failures)
    if failures:
        print("documentation checks failed:", file=sys.stderr)
        for item in failures:
            print(f"- {item}", file=sys.stderr)
        return 1
    print("documentation checks: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
