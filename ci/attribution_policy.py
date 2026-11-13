#!/usr/bin/env python3
"""CI attribution-policy check (D-52, D-61, ADR 0001, NFR-3.AC4).

Scans, with the SAME regex list as the local commit-msg hook, and rejects matches:
  1. every commit message in base..head, merge commits included;
  2. the pull request title and body (re-run on `edited` events);
  3. lines added in base...head, except narrowly exempt policy/fixture paths.

GitHub Actions: reads GITHUB_EVENT_NAME and GITHUB_EVENT_PATH (pull_request or push).
Locally: attribution_policy.py --base REV --head REV [--title T] [--body-file F].

Trusted inputs: the pattern list and exemption list are read from the BASE revision
when it has them, so a pull request cannot weaken the policy that judges it. If the
base lacks them (bootstrap), the head copies are used and a warning says so.

No D-61 bot-summary exemption is implemented yet: the whole body is scanned. That is
stricter than the policy allows; a provenance-verified exemption can be added later
if a bot summary ever matches.
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
PATTERNS = ".githooks/attribution-patterns.regex"
EXEMPTIONS = "ci/attribution-exemptions.txt"
ZERO_SHA = "0" * 40


def git(*args: str) -> bytes:
    return subprocess.run(["git", *args], check=True, capture_output=True).stdout


def file_at(revision: str, path: str) -> bytes | None:
    result = subprocess.run(["git", "show", f"{revision}:{path}"], capture_output=True)
    return result.stdout if result.returncode == 0 else None


def trusted(base: str | None, head: str, path: str) -> tuple[bytes, str]:
    if base:
        content = file_at(base, path)
        if content is not None:
            return content, f"base {base[:12]}"
    content = file_at(head, path)
    if content is None:
        raise SystemExit(f"error: {path} missing at {head}")
    print(f"::warning::bootstrap: {path} is not on the base branch; using the head copy")
    return content, f"head {head[:12]} (bootstrap)"


def commits(base: str | None, head: str) -> list[str]:
    selector = [head] if not base else [f"{base}..{head}"]
    return git("rev-list", *selector).decode().split()


def added_lines(base: str | None, head: str, exempt: list[str]) -> list[tuple[str, int, bytes]]:
    if base:
        diff = git("diff", "--no-color", "--no-ext-diff", "--unified=0", "--no-renames", f"{base}...{head}")
    else:
        diff = git("show", "--no-color", "--no-ext-diff", "--unified=0", "--no-renames", "--format=", head)
    found = []
    path, number, in_header = None, 0, False
    for line in diff.split(b"\n"):
        if line.startswith(b"diff --git "):
            path, in_header = None, True
        elif in_header and line.startswith(b"+++ "):
            name = line[4:].decode("utf-8", "replace")
            path = None if name == "/dev/null" else name.removeprefix("b/")
            if path and any(fnmatch.fnmatchcase(path, rule) for rule in exempt):
                path = None
        elif line.startswith(b"@@"):
            in_header = False
            number = int(line.split(b" ")[2].split(b",")[0].lstrip(b"+"))
        elif not in_header and line.startswith(b"+") and path:
            found.append((path, number, line[1:]))
            number += 1
    return found


def scan(patterns: bytes, items: list[dict]) -> list[dict]:
    with tempfile.NamedTemporaryFile(prefix="patterns-", delete=False) as handle:
        handle.write(patterns)
    try:
        result = subprocess.run(
            ["perl", str(HERE / "attribution-match.pl"), handle.name],
            input=json.dumps(items).encode(),
            capture_output=True,
        )
    finally:
        os.unlink(handle.name)
    if result.returncode != 0:
        raise SystemExit(f"error: matcher failed: {result.stderr.decode(errors='replace')}")
    return [json.loads(line) for line in result.stdout.decode().splitlines() if line]


def item(identifier: str, mode: str, raw: bytes) -> dict:
    return {"id": identifier, "mode": mode, "text_hex": raw.hex()}


def main() -> int:
    parser = argparse.ArgumentParser(description="Reject AI attribution in a commit range and PR metadata.")
    parser.add_argument("--base", help="base revision (exclusive)")
    parser.add_argument("--head", help="head revision")
    parser.add_argument("--title", help="pull request title")
    parser.add_argument("--body-file", type=Path, help="file holding the pull request body")
    options = parser.parse_args()

    base, head = options.base, options.head
    title = options.title
    body = options.body_file.read_bytes() if options.body_file else None
    event_name = os.environ.get("GITHUB_EVENT_NAME")
    if not head and event_name:
        event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text(encoding="utf-8"))
        if event_name == "pull_request":
            base = event["pull_request"]["base"]["sha"]
            head = event["pull_request"]["head"]["sha"]
            title = event["pull_request"]["title"]
            body = (event["pull_request"].get("body") or "").encode()
        elif event_name == "push":
            base = None if event["before"] == ZERO_SHA else event["before"]
            head = event["after"]
        else:
            raise SystemExit(f"error: unsupported event {event_name}")
    if not head:
        parser.error("--head is required outside GitHub Actions")

    patterns, pattern_source = trusted(base, head, PATTERNS)
    exemptions_raw, exemption_source = trusted(base, head, EXEMPTIONS)
    exempt = [
        line.strip()
        for line in exemptions_raw.decode().splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]
    print(f"patterns from {pattern_source}; exemptions from {exemption_source}")

    items, where = [], {}
    for sha in commits(base, head):
        key = f"commit {sha[:12]}"
        items.append(item(key, "message", git("log", "-1", "--format=%B", sha)))
    if title is not None:
        items.append(item("pull request title", "text", title.encode()))
    if body is not None:
        items.append(item("pull request body", "text", body))
    for path, number, raw in added_lines(base, head, exempt):
        key = f"added line {path}:{number}"
        where[key] = number
        items.append(item(key, "text", raw))

    findings = scan(patterns, items)
    for finding in findings:
        location = finding["id"] if finding["id"] in where else f"{finding['id']} line {finding['line']}"
        print(f"::error::{location}: {finding['reason']}: {finding['text']}")
    commit_count = sum(1 for entry in items if entry["id"].startswith("commit "))
    print(f"scanned {commit_count} commit(s), {len(where)} added line(s), "
          f"title={'yes' if title is not None else 'no'}, body={'yes' if body is not None else 'no'}")
    if findings:
        print("FAIL: attribution policy (D-52). Reword the commits, title or body; CI never rewrites them.")
        return 1
    print("PASS: attribution policy")
    return 0


if __name__ == "__main__":
    sys.exit(main())
