"""Locations shared by the one-off inspections; tracked samples are read-only."""

import os
from pathlib import Path


ROOT = Path(__file__).resolve().parent
RAW = Path(os.environ.get("SPIKE_RAW_DIR", ROOT / "raw")).expanduser().resolve()
OUTPUT = Path(os.environ.get("SPIKE_OUTPUT_DIR", ROOT / "output")).expanduser().resolve()
SAMPLES = ROOT / "samples"
if OUTPUT == SAMPLES or SAMPLES in OUTPUT.parents:
    raise ValueError("SPIKE_OUTPUT_DIR must be outside the read-only samples directory")


def available(name):
    if (RAW / name).is_file():
        return True
    print(f"SKIP missing optional input: {RAW / name}")
    return False
