#!/usr/bin/env python3
"""Reproduce the pinned network/Irish data inventory outside the repository.

Requires Python 3.11+. The committed manifest is the source of truth for URLs,
hashes, retrieval dates and verified licence metadata.
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


def validate_download(entry: dict, size: int, digest: str) -> None:
    if size != entry["bytes"] or digest != entry["sha256"]:
        raise ValueError(
            f"Integrity mismatch for {entry['relative_path']}; expected "
            f"{entry['bytes']} bytes / {entry['sha256']}, got {size} bytes / {digest}. "
            "Keep the pinned manifest unchanged and review the source before updating it."
        )


def download_file(
    entry: dict,
    target_path: Path,
    max_retries: int = 3,
    chunk_size: int = 65536,
    pause_sec: float = 1.0,
) -> tuple[int, str, str]:
    if target_path.exists():
        with target_path.open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        size = target_path.stat().st_size
        validate_download(entry, size, digest)
        return size, digest, entry["retrieval_date"]

    target_path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = target_path.with_suffix(target_path.suffix + ".part")
    headers = {"User-Agent": "energy-grid-simulator-data-spike/1.0"}

    for attempt in range(max_retries):
        try:
            request = urllib.request.Request(entry["source_url"], headers=headers)
            with urllib.request.urlopen(request, timeout=60) as response:
                digest = hashlib.sha256()
                size = 0
                with temp_path.open("wb") as stream:
                    while chunk := response.read(chunk_size):
                        stream.write(chunk)
                        digest.update(chunk)
                        size += len(chunk)
            sha256 = digest.hexdigest()
            validate_download(entry, size, sha256)
            temp_path.replace(target_path)
            retrieved = datetime.now(timezone.utc).isoformat()
            time.sleep(pause_sec)
            return size, sha256, retrieved
        except Exception as error:
            temp_path.unlink(missing_ok=True)
            if isinstance(error, ValueError) or attempt + 1 == max_retries:
                raise RuntimeError(f"Failed to fetch {entry['relative_path']}: {error}") from error
            wait_time = 2 ** (attempt + 1)
            print(f"  [WARN] Download failed: {error}. Retrying in {wait_time}s...")
            time.sleep(wait_time)

    raise RuntimeError(f"Failed to fetch {entry['relative_path']}")


def generate_manifest_toml(manifest: dict, manifest_path: Path) -> None:
    lines = [
        "# Data Gathering Manifest",
        "# GB & Ireland Electricity Grid Network and Market Datasets",
        "# Raw files stay outside git; licence evidence is recorded per dataset.",
        "",
        "[meta]",
    ]
    lines.extend(
        f"{key} = {json.dumps(value, ensure_ascii=False)}"
        for key, value in manifest["meta"].items()
    )
    for entry in manifest["datasets"]:
        lines.extend(["", "[[datasets]]"])
        lines.extend(
            f"{key} = {json.dumps(value, ensure_ascii=False)}"
            for key, value in entry.items()
        )
    temporary = manifest_path.with_suffix(".toml.part")
    temporary.write_text("\n".join(lines) + "\n", encoding="utf-8")
    temporary.replace(manifest_path)


def main() -> int:
    parser = argparse.ArgumentParser(description="Reproduce pinned GB & Ireland datasets.")
    parser.add_argument("--dry-run", action="store_true", help="List targets without downloading.")
    args = parser.parse_args()

    with MANIFEST_PATH.open("rb") as stream:
        manifest = tomllib.load(stream)
    datasets = manifest["datasets"]
    raw_data_dir = Path(manifest["meta"]["storage_root"]).expanduser().resolve()
    repository_dir = SCRIPT_DIR.parents[2]
    if raw_data_dir.is_relative_to(repository_dir):
        parser.error("Raw data storage must be outside the repository.")
    if manifest["meta"]["dataset_count"] != len(datasets):
        parser.error("Manifest dataset_count does not match its entries.")
    targets = []
    for entry in datasets:
        target = (raw_data_dir / entry["relative_path"]).resolve()
        if not target.is_relative_to(raw_data_dir):
            parser.error(f"Dataset path escapes raw storage: {entry['relative_path']}")
        if entry["licence_status"] not in {"open-commercial", "research-only"}:
            parser.error(f"Invalid licence status: {entry['relative_path']}")
        targets.append(target)
    if len(set(targets)) != len(targets):
        parser.error("Duplicate dataset paths in manifest.")

    print(f"Target raw data root: {raw_data_dir}")
    print(f"Manifest: {MANIFEST_PATH}")
    print(f"Total datasets: {len(datasets)}")
    if args.dry_run:
        for entry in datasets:
            print(f"- [{entry['licence_status']}] {entry['relative_path']} -> {entry['source_url']}")
        return 0

    results = []
    total_volume = 0
    for index, (entry, target) in enumerate(zip(datasets, targets), 1):
        print(f"[{index}/{len(datasets)}] Checking {entry['relative_path']}...")
        try:
            size, digest, retrieved = download_file(entry, target)
        except Exception as error:
            print(f"  ERROR: {error}")
            return 1
        result = dict(entry)
        result.update(bytes=size, sha256=digest, retrieval_date=retrieved)
        results.append(result)
        total_volume += size
        print(f"  OK ({size:,} bytes, sha256: {digest[:12]}...)")

    if results != datasets:
        manifest["datasets"] = results
        generate_manifest_toml(manifest, MANIFEST_PATH)
    print(f"Verified {len(results)} datasets: {total_volume:,} bytes.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
