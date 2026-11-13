#!/usr/bin/env python3
"""Tests for the CI attribution check: hook parity and end-to-end range scans.

Run: python3 -m unittest discover -s ci/tests -v   (needs Python 3, Perl and Git)
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HOOK = ROOT / ".githooks" / "commit-msg"
PATTERNS = ROOT / ".githooks" / "attribution-patterns.regex"
MATCHER = ROOT / "ci" / "attribution-match.pl"
DRIVER = ROOT / "ci" / "attribution_policy.py"
FIXTURES = json.loads((ROOT / ".githooks" / "tests" / "fixtures.json").read_text())
ATTRIBUTION_LINE = "Generated with Claude Code"
TRAILER_LINE = "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"


def raw_input(fixture: dict) -> bytes:
    return bytes.fromhex(fixture["input_hex"]) if "input_hex" in fixture else fixture["input"].encode()


def match(items: list[dict], patterns: Path = PATTERNS) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["perl", str(MATCHER), str(patterns)],
        input=json.dumps(items).encode(),
        capture_output=True,
    )


class HookParity(unittest.TestCase):
    """The CI check rejects exactly the messages the hook would change or refuse."""

    def test_every_hook_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            message = Path(directory) / "message"
            items, hook_flags = [], {}
            for fixture in FIXTURES:
                raw = raw_input(fixture)
                message.write_bytes(raw)
                result = subprocess.run([str(HOOK), str(message)], capture_output=True)
                hook_flags[fixture["name"]] = result.returncode != 0 or message.read_bytes() != raw
                items.append({"id": fixture["name"], "mode": "message", "text_hex": raw.hex()})
            result = match(items)
            self.assertEqual(result.returncode, 0, result.stderr)
            flagged = {json.loads(line)["id"] for line in result.stdout.decode().splitlines()}
            for name, hook_flagged in hook_flags.items():
                with self.subTest(fixture=name):
                    self.assertEqual(name in flagged, hook_flagged)
            self.assertTrue(any(hook_flags.values()) and not all(hook_flags.values()))

    def test_bad_pattern_files_fail_closed(self) -> None:
        original = PATTERNS.read_bytes()
        items = [{"id": "x", "mode": "text", "text_hex": b"ordinary text".hex()}]
        with tempfile.TemporaryDirectory() as directory:
            patterns = Path(directory) / "patterns"
            for variant in [original + b".*\n", original + b"[\n", original + b"\xff\n", b"# none\n"]:
                patterns.write_bytes(variant)
                self.assertEqual(match(items, patterns).returncode, 2, variant[-8:])
            patterns.write_bytes(original.replace(b"\n", b"\r\n"))
            self.assertEqual(match(items, patterns).returncode, 0)

    def test_text_mode_ignores_scissors(self) -> None:
        text = f"body\n# ------------------------ >8 ------------------------\n{ATTRIBUTION_LINE}\n"
        message = match([{"id": "m", "mode": "message", "text_hex": text.encode().hex()}])
        body = match([{"id": "b", "mode": "text", "text_hex": text.encode().hex()}])
        self.assertEqual(message.stdout, b"")
        self.assertIn(b'"line":3', body.stdout)


class RangeScan(unittest.TestCase):
    """End-to-end runs of attribution_policy.py over a scratch repository."""

    def setUp(self) -> None:
        self.directory = tempfile.TemporaryDirectory(prefix="attribution scan ")
        self.repo = Path(self.directory.name)
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.name", "Policy Tester")
        self.git("config", "user.email", "tester@example.org")
        self.git("config", "commit.gpgsign", "false")
        (self.repo / ".githooks").mkdir()
        (self.repo / "ci").mkdir()
        shutil.copy(PATTERNS, self.repo / ".githooks" / "attribution-patterns.regex")
        shutil.copy(ROOT / "ci" / "attribution-exemptions.txt", self.repo / "ci" / "attribution-exemptions.txt")
        self.commit("chore: base policy")
        self.base = self.rev("HEAD")

    def tearDown(self) -> None:
        self.directory.cleanup()

    def git(self, *args: str) -> str:
        return subprocess.run(
            ["git", "-C", str(self.repo), *args], check=True, capture_output=True
        ).stdout.decode()

    def rev(self, name: str) -> str:
        return self.git("rev-parse", name).strip()

    def commit(self, message: str) -> None:
        self.git("add", "-A")
        self.git("-c", "core.hooksPath=/dev/null", "commit", "-q", "--allow-empty", "-m", message)

    def write(self, path: str, text: str) -> None:
        target = self.repo / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)

    def run_check(self, *extra: str) -> subprocess.CompletedProcess:
        environment = {key: value for key, value in os.environ.items() if not key.startswith("GITHUB_")}
        return subprocess.run(
            [sys.executable, str(DRIVER), "--base", self.base, "--head", self.rev("HEAD"), *extra],
            cwd=self.repo,
            capture_output=True,
            env=environment,
            text=True,
        )

    def test_clean_range_passes(self) -> None:
        self.write("src/lib.rs", "// ordinary comment\n")
        self.commit("feat: add library")
        result = self.run_check("--title", "feat: add library")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_commit_trailer_fails(self) -> None:
        self.commit(f"feat: add library\n\n{TRAILER_LINE}")
        self.write("src/lib.rs", "// ordinary\n")
        self.commit("fix: later clean commit")
        result = self.run_check()
        self.assertEqual(result.returncode, 1)
        self.assertIn("commit ", result.stdout)

    def test_merge_commit_message_is_scanned(self) -> None:
        self.git("checkout", "-q", "-b", "side")
        self.commit("feat: side")
        self.git("checkout", "-q", "main")
        self.commit("feat: main")
        self.git("-c", "core.hooksPath=/dev/null", "merge", "-q", "--no-ff", "side", "-m", f"merge side\n\n{ATTRIBUTION_LINE}")
        self.assertEqual(self.run_check().returncode, 1)

    def test_title_and_body_fail(self) -> None:
        self.commit("feat: clean")
        self.assertEqual(self.run_check("--title", ATTRIBUTION_LINE).returncode, 1)
        body = self.repo.parent / f"{self.repo.name}-body"
        body.write_text(f"Summary\n\n{ATTRIBUTION_LINE}\n")
        try:
            self.assertEqual(self.run_check("--body-file", str(body)).returncode, 1)
        finally:
            body.unlink()

    def test_added_line_fails_except_in_exempt_paths(self) -> None:
        self.write("docs/process/team.md", f"{ATTRIBUTION_LINE}\n")
        self.commit("docs: policy example")
        self.assertEqual(self.run_check().returncode, 0)
        self.write("src/main.rs", f"// {ATTRIBUTION_LINE}\n")
        self.write("NOTES.md", f"{ATTRIBUTION_LINE}\n")
        self.commit("docs: notes")
        result = self.run_check()
        self.assertEqual(result.returncode, 1)
        self.assertIn("added line NOTES.md:1", result.stdout)

    def test_added_line_resembling_diff_header_is_scanned(self) -> None:
        self.write("NOTES.md", f"++ {ATTRIBUTION_LINE}\n")
        self.commit("docs: notes")
        self.assertEqual(self.run_check().returncode, 0)
        self.write("NOTES.md", f"++ x\n{ATTRIBUTION_LINE}\n")
        self.commit("docs: more notes")
        self.assertEqual(self.run_check().returncode, 1)

    def test_head_cannot_weaken_base_policy(self) -> None:
        self.write(".githooks/attribution-patterns.regex", "^never-matches-anything$\n")
        self.write("ci/attribution-exemptions.txt", "*\n")
        self.write("NOTES.md", f"{ATTRIBUTION_LINE}\n")
        self.commit("chore: weaken policy")
        result = self.run_check()
        self.assertEqual(result.returncode, 1)
        self.assertIn("patterns from base", result.stdout)


if __name__ == "__main__":
    unittest.main()
