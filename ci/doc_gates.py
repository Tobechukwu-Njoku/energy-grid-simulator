#!/usr/bin/env python3
"""Documentation gates (ADR 0001 profile B0): local links and traceability.

Checks, over files tracked by Git:
  links        every relative Markdown link points at an existing file or directory
  decisions    decision IDs in the decisions log are unique and contiguous
  references   every D-<n> cited in requirements, roadmap and ADRs exists
  requirements every FR-/NFR- section has a unique ID and cites at least one decision
  allocation   every FR-/NFR- is allocated in the milestone table
  adrs         ADR files are numbered NNNN-title.md with a matching title and Status

Usage: python3 ci/doc_gates.py [--root PATH]
Exit status is non-zero if any check fails. Needs only Python 3 and Git.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REQUIREMENTS = Path("docs/requirements/04-requirements.md")
DECISIONS = Path("docs/requirements/03-decisions-log.md")
ROADMAP = Path("docs/requirements/06-roadmap.md")
ADR_DIR = Path("docs/architecture/adr")

LINK = re.compile(r"(?<!!)\[[^\]]*\]\(\s*<?([^)\s>]+)>?(?:\s+\"[^\"]*\")?\s*\)")
FENCE = re.compile(r"^\s*(```|~~~)")
DECISION_REF = re.compile(r"\bD-(\d+)\b")
DECISION_ROW = re.compile(r"^\|\s*D-(\d+)\s*\|")
REQUIREMENT_HEADING = re.compile(r"^###\s+((?:N?FR)-(\d+))\b")
RANGE = re.compile(r"\b(N?FR)-(\d+)\s*[–—-]\s*(?:N?FR-)?(\d+)\b")
SINGLE = re.compile(r"\b(N?FR)-(\d+)\b")
ADR_NAME = re.compile(r"^(\d{4})-[a-z0-9]+(?:-[a-z0-9]+)*\.md$")


def tracked_markdown(root: Path) -> list[Path]:
    output = subprocess.run(
        ["git", "-C", str(root), "ls-files", "-z", "--", "*.md"],
        check=True,
        capture_output=True,
    ).stdout
    return [Path(name) for name in output.decode().split("\0") if name]


def prose_lines(text: str):
    """Yield (line number, line) outside fenced code, with code spans removed.

    A code span may continue onto following lines of the same paragraph.
    """
    fenced = False
    in_span = False
    for number, line in enumerate(text.splitlines(), 1):
        if FENCE.match(line):
            fenced = not fenced
            continue
        if fenced:
            continue
        if not line.strip():
            in_span = False
        kept = []
        for index, part in enumerate(line.split("`")):
            if index:
                in_span = not in_span
            if not in_span:
                kept.append(part)
        yield number, "".join(kept)


def check_links(root: Path, files: list[Path]) -> list[str]:
    errors = []
    for path in files:
        for number, line in prose_lines((root / path).read_text(encoding="utf-8")):
            for target in LINK.findall(line):
                if re.match(r"^[a-z][a-z0-9+.-]*:", target, re.I) or target.startswith("#"):
                    continue
                relative = target.split("#", 1)[0].split("?", 1)[0]
                if not relative:
                    continue
                base = root if relative.startswith("/") else (root / path).parent
                resolved = (base / relative.lstrip("/")).resolve()
                if not resolved.exists():
                    errors.append(f"{path}:{number}: broken link to {target}")
    return errors


def decision_ids(root: Path) -> tuple[list[int], list[str]]:
    ids, errors = [], []
    for number, line in enumerate((root / DECISIONS).read_text(encoding="utf-8").splitlines(), 1):
        match = DECISION_ROW.match(line)
        if match:
            ids.append(int(match.group(1)))
    seen = set()
    for value in ids:
        if value in seen:
            errors.append(f"{DECISIONS}: duplicate decision D-{value}")
        seen.add(value)
    if ids and sorted(seen) != list(range(1, max(seen) + 1)):
        missing = sorted(set(range(1, max(seen) + 1)) - seen)
        errors.append(f"{DECISIONS}: missing decision IDs {missing}")
    return ids, errors


def check_references(root: Path, known: set[int]) -> list[str]:
    errors = []
    sources = [REQUIREMENTS, ROADMAP, *sorted((root / ADR_DIR).glob("*.md"))]
    for source in sources:
        path = source if not source.is_absolute() else source.relative_to(root)
        for number, line in enumerate((root / path).read_text(encoding="utf-8").splitlines(), 1):
            for value in DECISION_REF.findall(line):
                if int(value) not in known:
                    errors.append(f"{path}:{number}: cites unknown decision D-{value}")
    return errors


def requirement_sections(root: Path) -> dict[str, list[str]]:
    sections: dict[str, list[str]] = {}
    current = None
    for line in (root / REQUIREMENTS).read_text(encoding="utf-8").splitlines():
        match = REQUIREMENT_HEADING.match(line)
        if match:
            current = match.group(1)
            if current in sections:
                raise ValueError(f"{REQUIREMENTS}: duplicate requirement {current}")
            sections[current] = []
        elif line.startswith("## ") or line.startswith("### "):
            current = None
        elif current:
            sections[current].append(line)
    return sections


def check_requirements(sections: dict[str, list[str]]) -> list[str]:
    errors = []
    if not sections:
        errors.append(f"{REQUIREMENTS}: no FR-/NFR- sections found")
    for requirement, body in sections.items():
        source = [line for line in body if "Rationale / source:" in line]
        if not source or not DECISION_REF.search(" ".join(source)):
            errors.append(f"{REQUIREMENTS}: {requirement} cites no decision in 'Rationale / source:'")
    return errors


def milestone_allocation(root: Path) -> set[str]:
    allocated: set[str] = set()
    for line in (root / REQUIREMENTS).read_text(encoding="utf-8").splitlines():
        if not re.match(r"^\|\s*M\d", line):
            continue
        cells = line.split("|")
        if len(cells) < 3:
            continue
        allocation = cells[2]
        for kind, start, end in RANGE.findall(allocation):
            allocated.update(f"{kind}-{n}" for n in range(int(start), int(end) + 1))
        allocated.update(f"{kind}-{n}" for kind, n in SINGLE.findall(allocation))
    return allocated


def check_adrs(root: Path) -> list[str]:
    errors = []
    for path in sorted((root / ADR_DIR).glob("*.md")):
        relative = path.relative_to(root)
        match = ADR_NAME.match(path.name)
        if not match:
            errors.append(f"{relative}: name must be NNNN-lower-case-title.md")
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        if not lines or not lines[0].startswith(f"# ADR {match.group(1)} "):
            errors.append(f"{relative}: first line must be '# ADR {match.group(1)} — title'")
        if not any(re.match(r"^- \*\*Status:\*\* \S", line) for line in lines[:10]):
            errors.append(f"{relative}: missing '- **Status:**' line near the top")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", type=Path, default=Path.cwd())
    root = parser.parse_args().root.resolve()

    results: dict[str, list[str]] = {}
    files = tracked_markdown(root)
    results["links"] = check_links(root, files)
    ids, results["decisions"] = decision_ids(root)
    results["references"] = check_references(root, set(ids))
    try:
        sections = requirement_sections(root)
        results["requirements"] = check_requirements(sections)
        missing = sorted(set(sections) - milestone_allocation(root), key=lambda r: (r[0], int(r.split("-")[1])))
        results["allocation"] = [f"{REQUIREMENTS}: {r} is not allocated to a milestone" for r in missing]
    except ValueError as error:
        results["requirements"] = [str(error)]
        results["allocation"] = ["skipped: requirement sections could not be read"]
    results["adrs"] = check_adrs(root)

    failed = False
    for name, errors in results.items():
        print(f"{'FAIL' if errors else 'PASS'}: {name}")
        for error in errors:
            print(f"  {error}")
        failed |= bool(errors)
    print(f"checked {len(files)} Markdown files")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
