#!/usr/bin/env python3
"""Run the shared message-policy fixtures; needs only Python, Perl and Git."""
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

HOOKS = Path(__file__).resolve().parents[1]
FIXTURES = json.loads((HOOKS / 'tests/fixtures.json').read_text())


def run_hook(hook, message):
    result = subprocess.run([str(hook), str(message)], capture_output=True)
    assert not list(message.parent.glob(message.name + '.strip.*')), 'temporary file leaked'
    return result


with tempfile.TemporaryDirectory(prefix='message policy ') as directory:
    root = Path(directory)
    hooks = root / 'hooks with spaces'
    shutil.copytree(HOOKS, hooks)
    message = root / 'message with spaces'
    for fixture in FIXTURES:
        raw = bytes.fromhex(fixture['input_hex']) if 'input_hex' in fixture else fixture['input'].encode()
        expected = bytes.fromhex(fixture['expected_hex']) if 'expected_hex' in fixture else fixture['expected'].encode()
        message.write_bytes(raw)
        result = run_hook(hooks / 'commit-msg', message)
        assert (result.returncode != 0) == (fixture['exit'] != 0), (fixture['name'], result.stderr)
        assert message.read_bytes() == expected, (fixture['name'], message.read_bytes(), expected)
        if 'stderr' in fixture:
            assert fixture['stderr'].encode() in result.stderr, (fixture['name'], result.stderr)

    patterns = hooks / 'attribution-patterns.regex'
    original = patterns.read_bytes()
    message_bytes = b'fix: pattern-file robustness\n\nGenerated with GPT6\n'
    for variant in [original + b'\n  \n# comment\n', original.replace(b'\n', b'\r\n')]:
        patterns.write_bytes(variant)
        message.write_bytes(message_bytes)
        assert run_hook(hooks / 'commit-msg', message).returncode == 0
        assert message.read_bytes() == b'fix: pattern-file robustness\n\n'
    for variant in [original + b'.*\n', original + b'[\n', original + b'\xff\n', b'# no rules\n\n']:
        patterns.write_bytes(variant)
        message.write_bytes(message_bytes)
        assert run_hook(hooks / 'commit-msg', message).returncode != 0
        assert message.read_bytes() == message_bytes, 'failed pattern load rewrote message'
    patterns.write_bytes(original)

    repo = root / 'scratch repo'
    repo.mkdir()

    def git(*args, check=True):
        return subprocess.run(['git', '-C', str(repo), *args], check=check, capture_output=True)

    git('init', '-q')
    git('config', 'user.name', 'Policy Tester')
    git('config', 'user.email', 'tester@example.org')
    git('config', 'core.hooksPath', str(hooks))
    git('config', 'commit.gpgsign', 'false')
    for merge in [False, True]:
        if merge:
            (repo / '.git/MERGE_HEAD').write_bytes(git('rev-parse', 'HEAD').stdout)
        message.write_text('fix: preserve human trailers\n\nGenerated with GPT6\nCo-authored-by: Jane Doe <jane@google.com>\n')
        git('-c', 'commit.cleanup=verbatim', 'commit', '--allow-empty', '-F', str(message))
        committed = git('log', '-1', '--format=%B').stdout
        assert b'Generated with' not in committed and b'Jane Doe' in committed
    message.write_text('Generated with GPT6\n\nBody\n')
    assert git('commit', '--allow-empty', '-F', str(message), check=False).returncode != 0
    git('config', 'i18n.commitEncoding', 'ISO-8859-1')
    message.write_bytes(b'fix: caf\xe9 encoding\n')
    assert git('commit', '--allow-empty', '-F', str(message), check=False).returncode != 0
    assert message.read_bytes() == b'fix: caf\xe9 encoding\n'

print(f'PASS: {len(FIXTURES)} fixtures, pattern-file failures/CRLF/blanks, Git commits/merge/encoding')
