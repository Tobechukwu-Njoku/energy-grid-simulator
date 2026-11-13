"""Bounded-memory readers for complete JSON arrays and BMRS data envelopes."""
import gzip
import ijson


def rows_from(path, compressed=None):
    if compressed is None:
        compressed = path.suffix == ".gz"
    with gzip.open(path, "rb") if compressed else path.open("rb") as source:
        first = source.read(1)
        while first and first.isspace():
            first = source.read(1)
        if first not in (b"[", b"{"):
            raise ValueError("expected a JSON array or data envelope")
        source.seek(0)
        # Exhausting items validates the complete JSON and gzip trailer, including
        # anything following the array. No raw file or JSON array is materialised.
        try:
            for row in ijson.items(source, "item" if first == b"[" else "data.item", use_float=True):
                if not isinstance(row, dict) or not row:
                    raise ValueError("expected nonempty record objects")
                yield row
        except ijson.JSONError as error:
            raise ValueError(f"invalid JSON: {error}") from error
