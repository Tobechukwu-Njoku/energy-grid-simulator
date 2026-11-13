import argparse
import hashlib
import json
from pathlib import Path

from spike_paths import RAW, SAMPLES


def verify(raw, manifest):
    mismatches = 0
    for entry in json.loads(manifest.read_text()):
        path = raw / entry["file"]
        if not path.is_file():
            print(f"SKIP missing optional input: {path}")
            continue
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != entry["sha256"]:
            mismatches += 1
            print(f"HASH MISMATCH: {entry['file']}: expected {entry['sha256']}, got {digest}")
        else:
            print(f"SHA-256 OK: {entry['file']}")
    return mismatches


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Verify existing files against read-only source pins")
    parser.add_argument("--raw-dir", type=Path, default=RAW)
    parser.add_argument("--manifest", type=Path, default=SAMPLES / "download-manifest.json")
    parser.add_argument("--allow-drift", action="store_true", help="Report mismatches but explicitly permit source drift")
    args = parser.parse_args()
    mismatches = verify(args.raw_dir.expanduser(), args.manifest)
    if mismatches and args.allow_drift:
        print(f"ALLOW-DRIFT: {mismatches} mismatch(es); review outputs before accepting new pins")
    raise SystemExit(1 if mismatches and not args.allow_drift else 0)
