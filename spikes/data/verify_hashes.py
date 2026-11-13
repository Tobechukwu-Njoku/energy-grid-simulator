import hashlib
import json
from pathlib import Path


def verify(raw, manifest):
    for entry in json.loads(manifest.read_text()):
        path = raw / entry["file"]
        if not path.exists():
            continue
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != entry["sha256"]:
            print(f"WARNING source drift: {entry['file']}: expected {entry['sha256']}, got {digest}")
        else:
            print(f"SHA-256 OK: {entry['file']}")


if __name__ == "__main__":
    root = Path(__file__).resolve().parent
    verify(root / "raw", root / "samples" / "download-manifest.json")
