"""Adversarial checks for integrity, streaming validation and resumable downloads."""
import contextlib
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
from stream_rows import rows_from

spec = importlib.util.spec_from_file_location('reports', ROOT / 'generate_reports.py')
reports = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reports)


class ToolsTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.raw = self.base / 'raw'
        self.raw.mkdir()
        self.bin = self.base / 'bin'
        self.bin.mkdir()
        self.env = dict(os.environ, DATA_DIR=str(self.raw), PYTHON=sys.executable,
                        PATH=str(self.bin) + os.pathsep + os.environ['PATH'],
                        MAX_RETRIES='1', CONCURRENCY='2')
        self.bmunits = self.raw / 'bmunits' / 'bmunits.json'
        self.bmunits.parent.mkdir()
        self.valid = b'[{"nationalGridBmUnit":"UNIT1","elexonBmUnit":null,"eic":null}]'
        self.bmunits.write_bytes(self.valid)

    def fake_curl(self, body, status=0):
        script = self.bin / 'curl'
        script.write_text('#!/bin/bash\n' + body + f'\nexit {status}\n')
        script.chmod(0o755)

    def fetch(self, target='bmunits'):
        return subprocess.run(['bash', str(ROOT / 'fetch.sh'), target],
                              env=self.env, capture_output=True, text=True)

    def validate(self, path, compressed=False):
        return subprocess.run([sys.executable, '-I', str(ROOT / 'validate_response.py'),
                               str(path), str(compressed).lower()], capture_output=True)

    def test_stream_validation_rejects_bad_json_and_gzip(self):
        for payload in (b'[{"x":1}', b'[]garbage', b'{"error":"unavailable"}',
                        b'[1]', b'[]', b'[{"settlementDate":"wrong"}]'):
            self.bmunits.write_bytes(payload)
            self.assertNotEqual(self.validate(self.bmunits).returncode, 0)
        for payload in (gzip.compress(self.valid)[:-4], gzip.compress(self.valid) + b'junk'):
            self.bmunits.write_bytes(payload)
            self.assertNotEqual(self.validate(self.bmunits, True).returncode, 0)
        self.bmunits.write_bytes(gzip.compress(self.valid))
        self.assertEqual(self.validate(self.bmunits, True).returncode, 0)
        self.bmunits.write_bytes(b'{"metadata":{},"data":' + self.valid + b'}')
        self.assertEqual(list(rows_from(self.bmunits)), json.loads(self.valid))

    def test_cache_reuse_is_byte_and_timestamp_idempotent(self):
        self.fake_curl('exit 99')
        before = (self.bmunits.read_bytes(), self.bmunits.stat().st_mtime_ns)
        for _ in range(2):
            result = self.fetch()
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((self.bmunits.read_bytes(), self.bmunits.stat().st_mtime_ns), before)

    def test_invalid_cache_is_replaced_only_with_valid_response(self):
        self.bmunits.write_bytes(b'broken')
        self.fake_curl('while [[ "$1" != "-o" ]]; do shift; done\n'
                       'shift\nprintf \'[{"nationalGridBmUnit":"UNIT1","elexonBmUnit":null,"eic":null}]\' > "$1"\nprintf 200')
        result = self.fetch()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.bmunits.read_bytes(), self.valid)
        self.assertEqual(list(self.bmunits.parent.glob('*.tmp.*')), [])
        self.bmunits.write_bytes(b'broken')
        self.fake_curl('while [[ "$1" != "-o" ]]; do shift; done\nshift\nprintf \'{"error":"bad"}\' > "$1"\nprintf 200')
        self.assertNotEqual(self.fetch().returncode, 0)
        self.assertEqual(self.bmunits.read_bytes(), b'broken')
        self.assertEqual(list(self.bmunits.parent.glob('*.tmp.*')), [])

    def test_concurrent_worker_failure_is_propagated(self):
        # Use the actual function definitions, with a two-day fixture calendar.
        # Put the harness beside helpers so SCRIPT_DIR remains correct.
        harness = self.base / 'harness'
        harness.mkdir()
        for name in ('validate_response.py', 'stream_rows.py'):
            shutil.copy(ROOT / name, harness / name)
        source = (ROOT / 'fetch.sh').read_text().split('TARGET="${1:-all}"')[0]
        source += '\nget_dates_2024() { printf "2024-01-01\\n2024-01-02\\n"; }\nfetch_system_prices\n'
        script = harness / 'fetch.sh'
        script.write_text(source)
        self.fake_curl('printf 503')
        result = subprocess.run(['bash', str(script)], env=self.env, capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(list((self.raw / 'system_prices').iterdir()), [])

    def test_report_checksums_fail_closed_and_reports_repeat(self):
        output = self.base / 'output'
        output.mkdir()
        index = output / 'bmunits_sha256.txt'
        index.write_text(hashlib.sha256(self.valid).hexdigest() + '  bmunits.json\n')
        original = reports.RAW_BASE, reports.OUTPUT_DIR, reports.DATASETS_CONFIG
        self.addCleanup(lambda: self.restore_reports(original))
        reports.RAW_BASE, reports.OUTPUT_DIR = self.raw, output
        reports.DATASETS_CONFIG = [reports.DATASETS_CONFIG[0], dict(reports.DATASETS_CONFIG[6],
            id='remit', pattern='remit/*.json')]
        remit = self.raw / 'remit'
        remit.mkdir()
        payload = b'[{"mrid":"1","messageType":"UnavailabilitiesOfElectricityFacilities","revisionNumber":1}]'
        (remit / '2024-01.json').write_bytes(payload)
        (output / 'remit_sha256.txt').write_text(hashlib.sha256(payload).hexdigest() + '  2024-01.json\n')
        saved_argv = sys.argv
        self.addCleanup(setattr, sys, 'argv', saved_argv)
        sys.argv = ['generate_reports.py', '--output-dir', str(output)]
        with contextlib.redirect_stdout(io.StringIO()):
            reports.main()
            before = {p.name: p.read_bytes() for p in output.iterdir()}
            reports.main()
        self.assertEqual(before, {p.name: p.read_bytes() for p in output.iterdir()})
        cfg = reports.DATASETS_CONFIG[0]
        for change in ('modified', 'added', 'missing'):
            self.bmunits.write_bytes(self.valid)
            if change == 'modified':
                self.bmunits.write_bytes(self.valid + b' ')
            elif change == 'added':
                (self.bmunits.parent / 'extra.json').write_bytes(self.valid)
            else:
                self.bmunits.unlink()
            with self.assertRaises(ValueError), contextlib.redirect_stdout(io.StringIO()):
                reports.analyze_dataset(cfg)
            self.assertEqual(index.read_bytes(), before[index.name])
            (self.bmunits.parent / 'extra.json').unlink(missing_ok=True)

    @staticmethod
    def restore_reports(original):
        reports.RAW_BASE, reports.OUTPUT_DIR, reports.DATASETS_CONFIG = original


if __name__ == '__main__':
    unittest.main()
