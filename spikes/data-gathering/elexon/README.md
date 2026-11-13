# Elexon 2024 data spike

Raw captures stay outside git, by default in `$HOME/energy-data/raw/elexon`.
Set `DATA_DIR` to use another location. Python 3.12 or later, Bash, curl,
gzip and the pinned streaming JSON dependency are required:

```sh
python3 -m venv /path/to/elexon-venv
/path/to/elexon-venv/bin/python -I -m pip install -r spikes/data-gathering/elexon/requirements.txt
export PYTHON=/path/to/elexon-venv/bin/python
export DATA_DIR=/home/njotob/energy-data/raw/elexon
bash spikes/data-gathering/elexon/fetch.sh all
"$PYTHON" -I spikes/data-gathering/elexon/verify_raw.py \
  --workers 2 --output /path/to/verification.json
"$PYTHON" -I spikes/data-gathering/elexon/generate_reports.py
"$PYTHON" -I spikes/data-gathering/elexon/test_tools.py
```

All JSON readers stream records, including gzip inputs. The independent verifier
retains duplicate keys for one daily file at a time; REMIT keys span all months.
Two workers fit the 8 GB verification machine. It checks exact payload duplicates
and dataset-specific keys, preserving nullable Elexon IDs alongside National Grid
IDs and EICs. Notification revisions and segments are separate records.

The downloader validates existing captures before reusing them, validates new
responses before atomic replacement, and propagates concurrent failures. It
preserves existing valid reference snapshots; remove a reference capture explicitly
if a new current snapshot is wanted. Download times were not recorded for the
original capture, and filesystem timestamps are not treated as retrieval evidence.

Report generation verifies checksum indexes by default and fails on missing,
additional or modified inputs. Use `--refresh-checksums` only after investigating
and intentionally accepting a new capture; this explicitly replaces the integrity
baseline. A first collection also needs that flag to create its indexes. Generated
reports preserve their generation timestamp for byte-stable reruns; use
`--generated-at <ISO-8601>` when deliberately recording a new audit. An alternate
`--output-dir` must already exist and contain the checksum indexes.

A matching checksum proves fidelity to the recorded capture, not completeness of
the source. Read `COVERAGE.md` and the dated verification review before analysis:
provider gaps, event-stream coverage limits and a retained REMIT duplicate matter.
The verifier writes measured findings even when it exits nonzero for discrepancies;
a successful integrity check alone is not a certificate of a rectangular panel.

Contains BMRS data © Elexon Limited copyright and database right 2024.
Current reference snapshot: Contains BMRS data © Elexon Limited copyright and database right 2026.
[Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/).
