#!/usr/bin/env python3
"""Idempotent downloader and verifier for 2024 all-island Ireland (I-SEM) market prices.

Downloads SEMOpx ex-ante auction prices and SEMO balancing market imbalance settlement
prices into raw storage outside git per project provenance rules (D-56).

Requires Python 3.11+. No secrets or authentication credentials required.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import time
import tomllib
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
MANIFEST_PATH = SCRIPT_DIR / "manifest.toml"


def compute_sha256(path: Path) -> str:
    """Compute SHA-256 hash of a file."""
    hasher = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(65536):
            hasher.update(chunk)
    return hasher.hexdigest()


def validate_file(entry: dict, target_path: Path) -> tuple[int, str]:
    """Validate that an existing file matches the pinned bytes and sha256."""
    size = target_path.stat().st_size
    digest = compute_sha256(target_path)
    if size != entry["bytes"] or digest != entry["sha256"]:
        raise ValueError(
            f"Integrity mismatch for {entry['filename']}:\n"
            f"  Expected: {entry['bytes']} bytes, SHA-256: {entry['sha256']}\n"
            f"  Observed: {size} bytes, SHA-256: {digest}"
        )
    return size, digest


def download_dataset(
    entry: dict,
    target_path: Path,
    max_retries: int = 3,
    chunk_size: int = 65536,
    pause_sec: float = 1.0,
) -> tuple[int, str]:
    """Download a dataset if not already present, with retry and integrity validation."""
    if target_path.exists():
        return validate_file(entry, target_path)

    target_path.parent.mkdir(parents=True, exist_ok=True)
    part_path = target_path.with_suffix(target_path.suffix + ".part")
    headers = {"User-Agent": "energy-grid-simulator-semo-spike/1.0"}

    for attempt in range(max_retries):
        try:
            req = urllib.request.Request(entry["source_url"], headers=headers)
            with urllib.request.urlopen(req, timeout=120) as response:
                digest = hashlib.sha256()
                size = 0
                with part_path.open("wb") as stream:
                    while chunk := response.read(chunk_size):
                        stream.write(chunk)
                        digest.update(chunk)
                        size += len(chunk)
            observed_sha256 = digest.hexdigest()
            if size != entry["bytes"] or observed_sha256 != entry["sha256"]:
                raise ValueError(
                    f"Integrity check failed after download for {entry['filename']}:\n"
                    f"  Expected {entry['bytes']} bytes / {entry['sha256']}\n"
                    f"  Got {size} bytes / {observed_sha256}"
                )
            part_path.replace(target_path)
            time.sleep(pause_sec)
            return size, observed_sha256
        except Exception as error:
            part_path.unlink(missing_ok=True)
            if isinstance(error, ValueError) or attempt + 1 == max_retries:
                raise RuntimeError(
                    f"Failed to fetch {entry['filename']} from {entry['source_url']}: {error}"
                ) from error
            wait_time = 2 ** (attempt + 1)
            print(f"  [WARN] Download failed: {error}. Retrying in {wait_time}s...")
            time.sleep(wait_time)

    raise RuntimeError(f"Exhausted retries downloading {entry['filename']}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Download and verify 2024 Ireland SEMO / SEMOpx market price datasets."
    )
    parser.add_argument(
        "--target-dir",
        type=Path,
        default=None,
        help="Target storage directory (defaults to storage_root from manifest.toml).",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="List planned downloads without performing network transfers or writes.",
    )
    parser.add_argument(
        "--verify-only",
        action="store_true",
        help="Verify checksums of existing files without downloading any missing files.",
    )
    args = parser.parse_args()

    if not MANIFEST_PATH.is_file():
        print(f"ERROR: Manifest file not found at {MANIFEST_PATH}", file=sys.stderr)
        return 1

    with MANIFEST_PATH.open("rb") as stream:
        manifest = tomllib.load(stream)

    datasets = manifest.get("datasets", [])
    if not datasets:
        print("ERROR: Manifest has no datasets.", file=sys.stderr)
        return 1

    storage_root = (
        args.target_dir.expanduser().resolve()
        if args.target_dir
        else Path(manifest["meta"]["storage_root"]).expanduser().resolve()
    )

    repository_root = SCRIPT_DIR.parents[2]
    if storage_root.is_relative_to(repository_root):
        print(
            f"ERROR: Target directory {storage_root} must be strictly outside the git repository {repository_root} (D-56).",
            file=sys.stderr,
        )
        return 1

    print(f"Repository Root : {repository_root}")
    print(f"Raw Storage Dir : {storage_root}")
    print(f"Manifest Path   : {MANIFEST_PATH}")
    print(f"Total Datasets  : {len(datasets)}")

    if args.dry_run:
        print("\n--- Dry Run Targets ---")
        for entry in datasets:
            target = storage_root / entry["relative_path"]
            status = "EXISTS" if target.is_file() else "MISSING"
            print(f"[{status}] {entry['filename']} ({entry['bytes']:,} bytes)")
            print(f"       Source: {entry['source_url']}")
            print(f"       Target: {target}")
            print(f"       Licence: {entry['licence_status']} ({entry['licence_name']})")
        return 0

    storage_root.mkdir(parents=True, exist_ok=True)
    total_bytes = 0

    print("\n--- Processing Datasets ---")
    for idx, entry in enumerate(datasets, 1):
        target = storage_root / entry["relative_path"]
        print(f"[{idx}/{len(datasets)}] {entry['filename']}")
        if args.verify_only:
            if not target.is_file():
                print(f"  FAILED: File not found at {target}", file=sys.stderr)
                return 1
            try:
                size, digest = validate_file(entry, target)
                print(f"  VERIFIED: {size:,} bytes, SHA-256: {digest}")
                total_bytes += size
            except Exception as e:
                print(f"  VERIFICATION ERROR: {e}", file=sys.stderr)
                return 1
        else:
            try:
                size, digest = download_dataset(entry, target)
                print(f"  OK: {size:,} bytes, SHA-256: {digest}")
                total_bytes += size
            except Exception as e:
                print(f"  DOWNLOAD/VALIDATION ERROR: {e}", file=sys.stderr)
                return 1

    print(f"\nAll {len(datasets)} datasets verified successfully ({total_bytes:,} bytes).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
