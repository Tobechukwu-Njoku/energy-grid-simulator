# D-31 data spike — reproducible inspection, not application code

Research date: **2026-11-12**, revised **2026-11-13**. Start point: main `8b70c67` on branch
`t3/r12-transmission-data-spike`; no earlier cancelled spike was reused.
See [REPORT.md](REPORT.md) for the R-12 verdict and licensing boundaries.

## Reproduce

Requirements: POSIX shell, `curl`, `jq`, `unzip`, Python 3, and `uv`.
The small Python scripts are **one-off file inspections** of XLSX, PSS/E RAW,
CSV, JSON and XML, not an ingest service or supported parser. `openpyxl==3.1.5`
is the only additional parsing dependency. They do not run power flow.

```sh
sh spikes/data/fetch.sh
uv run --with openpyxl==3.1.5 python spikes/data/inspect_tabular.py
```

This downloads sources with identified open terms, plus official metadata/licence
pages, into ignored `raw/` (roughly 80 MB). It skips uncleared network and exchange
data. The tabular inspection now skips missing optional research-only files,
including their manifest entries, so this cleared-only audit works independently.
No account or secret is required for these public downloads.

For the **D-56 non-commercial research network inspection**, not for commercial
use or redistribution of research-only raw/derived network data:

```sh
sh spikes/data/fetch.sh --include-uncleared
unzip -t spikes/data/raw/tytfs-2024.zip
uv run --with openpyxl==3.1.5 python spikes/data/inspect_networks.py
uv run --with openpyxl==3.1.5 python spikes/data/inspect_tabular.py
jq . spikes/data/samples/network-statistics.json
jq . spikes/data/samples/tabular-statistics.json
```

Including uncleared inspection sources brings raw storage to about 135 MB.
The scripts export only aggregate statistics for the uncleared networks; no
Irish/ETYS electrical records or exchange-price/Irish-flow samples are committed.
The Irish physical-bus connectivity projection accounts for partially disconnected
three-winding transformers; it is not an electrical star-equivalent model.
The GB site-collapsed graph is explicitly a diagnostic, not an
approved bus equivalence. No OSM/ODbL data is fetched or used.

`sources.tsv` pins the exact URLs used for successful data downloads. Live files
can be revised in place: `samples/download-manifest.json` preserves the original
byte sizes and SHA-256 values, **not** a promise that a future fetch matches.
The fetcher checks existing SHA-256 pins and **warns, without failing**, on live
source drift. Review differences before re-running inspections, which overwrite
the samples/manifest. The manifest tags downloaded sources `open-commercial` or
`research-only` and records dataset/metadata role and URL; legal/discovery metadata
is conservatively research-only. Commercial clearance stays in
`docs/requirements/TODO-pre-commercial.md` (PC-1–PC-5), not a research-ingest gate.
The manifest also fingerprints the principal licence pages. Failed discovery
requests and page-only investigations are documented in the report, not presented
as successful data downloads. Running inspections again overwrites sample files.

## What is committed

- Scripts, this report, the source URL list, and an **unexecuted** ERA5 request.
- `samples/`: two JSON records or three CSV rows per cleared sampled dataset,
  aggregate inspections and the download manifest. These are excerpts, not full-year
  or operational network models. Each file is below 1 MB.
- No raw downloads, binaries, or reusable uncleared network database.

The full 2024 NESO demand and fuel-mix files were downloaded and audited. Only
one GB day / one BM settlement period was sampled from Elexon. No full BM year,
licensed day-ahead price year, or ERA5 weather year was retrieved. The ERA5 request
targets `reanalysis-era5-single-levels`, January 2024, hourly GB+IE bounding box.
For a full year repeat monthly with valid calendar days (including 29 February).
CDS registration, dataset terms acceptance and credentials are prerequisites; no
local `~/.cdsapirc` was present and no secret was requested or stored.

Subsequent work is cross-referenced, not re-audited, in REPORT.md §4: **PR #10**
inventories all DNO 132-kV sources; **PR #11** gathers network/Irish data;
**PR #12** reports the full-year Elexon download (roughly 4.2 GB, 4.16 GiB stated)
outside git at **`~/energy-data/raw/elexon/`**. These are candidate archives,
not proof of semantic completeness. ENTSO-E I-SEM DAM/flow retrieval still needs
an owner API token; its CC-BY scope has provider exceptions. No token is stored.
The revised ERA5 candidate extends east to 10° and adds precipitation/direct
radiation; its January request remains unexecuted, pending Q-C-12.

## Sample attribution

NESO samples: **Supported by National Energy SO Open Data**.
NGED samples: **Supported by NGED Open Data**.
Elexon historic samples: **Contains BMRS data © Elexon Limited copyright and database right 2024**.
Elexon fleet reference: **Contains BMRS data © Elexon Limited copyright and database right 2026**.
Source links and attribution obligations are listed in REPORT.md and sources.tsv.
Samples are truncated and reformatted; all remain subject to their original
attribution licences. No general repository code licence relicenses these data.
Keep `samples/README.md` with any redistributed sample files; if a file is detached,
carry the applicable attribution and licence reference into its distribution.

Keep licence-page scope checks per dataset: the presence of a licence somewhere
on an operator website does not license every other publication on that site.
