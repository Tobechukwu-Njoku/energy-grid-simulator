#!/usr/bin/env python3
"""Repository policy checks (ADR 0001, NFR-10, NFR-12). Needs only Python 3 and Git.

  unsafe-scope   `unsafe` code may be allowed only inside crates/solver-* adapters
  crate-lints    every workspace crate inherits the workspace lints
  workflows      GitHub workflows are hardened (see docs/process/ci-security.md):
                 - no pull_request_target, workflow_run or self-hosted runners
                   (self-hosted execution is an M1 activation step)
                 - explicit top-level `permissions:` with no write scopes
                 - every `uses:` is pinned to a full commit SHA
                 - checkout never persists credentials
                 - no `${{ github.event.* }}` / `${{ github.head_ref }}` inside `run:`
                 - no secrets other than GITHUB_TOKEN

Exit status is non-zero if any check fails.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ALLOW_UNSAFE = re.compile(r"#!?\[\s*(?:allow|expect|warn)\s*\([^)]*\bunsafe_code\b")
USES = re.compile(r"^\s*-?\s*uses:\s*([^\s#]+)", re.M)
PINNED = re.compile(r"^[\w.-]+/[\w./-]+@[0-9a-f]{40}$")
FORBIDDEN_TRIGGERS = ("pull_request_target", "workflow_run")
UNTRUSTED_EXPRESSIONS = re.compile(
    r"\$\{\{\s*(?:github\.event\.(?:pull_request|issue|comment|review|head_commit|commits)"
    r"|github\.head_ref)"
)
SECRET = re.compile(r"\bsecrets\.(\w+)")


def tracked(*patterns: str) -> list[Path]:
    output = subprocess.run(["git", "ls-files", "-z", "--", *patterns], check=True, capture_output=True).stdout
    return [Path(name) for name in output.decode().split("\0") if name]


def check_unsafe_scope() -> list[str]:
    errors = []
    for path in tracked("*.rs"):
        if path.parts[:1] == ("crates",) and len(path.parts) > 1 and path.parts[1].startswith("solver-"):
            continue
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if ALLOW_UNSAFE.search(line):
                errors.append(f"{path}:{number}: unsafe_code may only be allowed in crates/solver-*")
    for path in tracked("*Cargo.toml"):
        text = path.read_text(encoding="utf-8")
        in_solver = len(path.parts) > 2 and path.parts[0] == "crates" and path.parts[1].startswith("solver-")
        levels = re.findall(r"^\s*unsafe_code\s*=\s*(?:\{[^}]*level\s*=\s*)?\"(\w+)\"", text, re.M)
        if any(level not in ("deny", "forbid") for level in levels) and not in_solver:
            errors.append(f"{path}: unsafe_code must stay deny outside crates/solver-*")
    return errors


def check_crate_lints() -> list[str]:
    errors = []
    for path in tracked("crates/*/Cargo.toml"):
        text = path.read_text(encoding="utf-8")
        if not re.search(r"^\[lints\]\s*\nworkspace\s*=\s*true\s*$", text, re.M):
            errors.append(f"{path}: must contain `[lints]` with `workspace = true`")
    return errors


def strip_comments(text: str) -> str:
    return "\n".join(re.sub(r"(^|\s)#.*$", "", line) for line in text.splitlines())


def check_workflows() -> list[str]:
    errors = []
    workflows = tracked(".github/workflows/*.yml", ".github/workflows/*.yaml")
    if not workflows:
        errors.append(".github/workflows: no workflows found")
    for path in workflows:
        text = strip_comments(path.read_text(encoding="utf-8"))
        for trigger in FORBIDDEN_TRIGGERS:
            if re.search(rf"\b{trigger}\b", text):
                errors.append(f"{path}: `{trigger}` is not allowed (NFR-12)")
        if re.search(r"runs-on:.*self-hosted", text) or re.search(r"^\s*-\s*self-hosted\b", text, re.M):
            errors.append(f"{path}: self-hosted runners are not activated before M1 (NFR-12)")
        if not re.search(r"^permissions:", text, re.M):
            errors.append(f"{path}: needs an explicit top-level `permissions:` block")
        for match in re.finditer(r"^\s*([\w-]+):\s*write\b|write-all", text, re.M):
            errors.append(f"{path}: write permission `{match.group(0).strip()}` needs a reviewed exception")
        for action in USES.findall(text):
            if action.startswith("./"):
                continue
            if not PINNED.match(action):
                errors.append(f"{path}: `{action}` must be pinned to a full commit SHA")
        for block in re.finditer(r"uses:\s*actions/checkout@\S+(?P<rest>(?:\n\s{8,}.*)*)", text):
            if not re.search(r"persist-credentials:\s*false", block.group("rest")):
                errors.append(f"{path}: actions/checkout must set `persist-credentials: false`")
        for number, line in enumerate(text.splitlines(), 1):
            if UNTRUSTED_EXPRESSIONS.search(line) and not re.match(r"^\s*(?!run:)[\w-]+:\s*\$\{\{", line):
                errors.append(f"{path}:{number}: pass untrusted event data through `env:`, not inline")
        for name in SECRET.findall(text):
            if name != "GITHUB_TOKEN":
                errors.append(f"{path}: secret `{name}` is not allowed in hosted CI (NFR-12)")
    return errors


def main() -> int:
    results = {
        "unsafe-scope": check_unsafe_scope(),
        "crate-lints": check_crate_lints(),
        "workflows": check_workflows(),
    }
    for name, errors in results.items():
        print(f"{'FAIL' if errors else 'PASS'}: {name}")
        for error in errors:
            print(f"  {error}")
    return 1 if any(results.values()) else 0


if __name__ == "__main__":
    sys.exit(main())
