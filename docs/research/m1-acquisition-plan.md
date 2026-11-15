# M1 acquisition plan: sources without credentials

Publisher pages and catalogue/API documentation checked **2026-11-15 UTC**.
Based on the 54 external resource rows (`SRC-01`–`SRC-50`, including split IDs)
in [m1-data-inventory.md](m1-data-inventory.md). This is an acquisition plan,
not evidence of new acquisition, complete network coverage or M1 acceptance.
No datasets were fetched for this document; only public HTML pages, catalogue
metadata and API specifications were read. Previously recorded sizes come from
committed manifests, not new dataset requests.

## Scope and interpretation

The register covers 42 whole resource rows plus the public portion of `SRC-49`:
**43 entries, 23 open-commercial and 20 research-only**, preserving the inventory's
classifications. Eleven whole resource rows are excluded for credentials; the
gated portion of `SRC-49` is also excluded. `INT-01`/`INT-02` are internal
specifications, not downloads. The appendix accounts for every excluded row.

Unresolved public candidates remain in the register so that a broken link or an
HTTP 403 is not silently treated as an account requirement. **UNVERIFIED** means
the stated location, access, content, cadence, size or rights could not be
confirmed by reading publisher material. A verified catalogue link establishes
that a publisher advertises a resource; it does not establish that the download
works, has the expected contents or reproduces an earlier capture. No dataset
payload endpoint was called to make such a claim.

The governing sources are [D-56 and D-72](../requirements/03-decisions-log.md),
[FR-1–5, FR-19, FR-21 and NFR-1/2](../requirements/04-requirements.md), and
accepted [ADR 0002](../architecture/adr/0002-network-and-scenario-data-model.md)
and [ADR 0004](../architecture/adr/0004-storage-topology.md). Requirements links
below mean the input serves that requirement; acquisition alone does not pass it.
Other requirement IDs identify downstream uses of M1 inputs.

For `research-only`, raw data and restricted derived extracts may **never** be
committed or redistributed (FR-19.AC2). Use private storage outside the checkout,
such as `~/energy-data/raw/`, and commit only legally publishable provenance and
scripts. For `open-commercial`, raw files **may** be committed only within the
resource's grant, with attribution, third-party/personal-data exclusions and
provenance satisfied; the plan recommends keeping bulk raw files outside git too.
All rows remain subject to the absolute OSM/ODbL/share-alike exclusion, including
basemaps and upstream derivatives. Research classification is not a waiver of
access terms or retention restrictions. PC references point to
[TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md).

## Provenance manifest contract

Every resource, archive member and dated API batch needs **C** below plus its row's
profiles. These are required information fields for the M1 manifest design,
not a claim that an implemented schema already exists. Preserve compatibility
with existing `licence_status`, `source_url`, `bytes` and `sha256` fields. Do not
invent acquisition timestamps, hashes, sizes or publication dates before fetching.

| Profile | Fields to record |
|---|---|
| **C — common, every row** | `source_id`, sub-resource ID, publisher, title, publisher-page URL, catalogue/resource ID, requested URL and resolved URL without secrets, query parameters, access-check date/status, source release/version, acquisition timestamp UTC, source publication/revision timestamp and evidence (or explicitly unknown), effective coverage `[from,to)`, format/media type, raw relative path, bytes, SHA-256, archive-member names/hashes, licence name/status (exactly `open-commercial` or `research-only`), terms/evidence URLs and date, attribution text, exclusions/retention/redistribution conditions, raw-commit permission, PC clearance IDs, parent release hashes, transform/mapping/schema versions, derived hashes, missing/imputed coverage, knowledge mode and requirement IDs. Never publish tokens or signed URL query secrets. |
| **N — electrical network/fleet** | Operator/licence area, voltage and asset class, source asset aliases and permanent UUID crosswalk version, snapshot/study year, commissioned/planned/removed status and effective dates, node/GSP/BMU joins, units and electrical bases (kV/MVA, percent/per-unit/ohms), ratings meaning, topology/switching/transformer conventions, coverage numerator/denominator, inferred or absent parameters. |
| **G — geographic** | Coordinate reference system, geometry type, source layer/feature IDs, coordinates' own rights lineage and attribution; separate basemap provenance. Geometry does not establish electrical connectivity. |
| **T — interval observations** | Product/quantity, units, native cadence, source timezone, settlement date/period, UTC interval boundaries, local offset/fold, publication/revision/run identifier, direction/metering convention, expected/observed intervals and dimensions, duplicates/revisions, missing mask, resampling and aggregation policy. |
| **R — rules/proxies** | Document version and clause/table/page, publication and effective dates, jurisdiction/market/product/border, modifications, numerical parameter units, signed branch cut-set or unit-group crosswalk, distinction between operational rule, forecast and outturn. |
| **E — costs/FX** | Commodity/product, units and currency, quote/auction/reference timestamp, delivery/obligation period, price basis (nominal/real and base year), FX direction/numeraire, conversion/resampling/fill version, technology mapping and third-party price rights. |
| **K — calendars** | Jurisdiction, holiday date/type and observance, calendar and tzdata release/hash, `Europe/London`/`Europe/Dublin` mapping version, civil-to-settlement rules, source interval ID/offset/fold, trading-calendar distinction. |
| **W — weather** | Model/product/version, variables/units, grid/CRS/bounds/height, issue/valid time, accumulation convention, native cadence, coverage and missingness, spatial/resampling/power-curve versions and benchmark-only designation. |

All entries also serve **FR-19.AC1–3, FR-4.AC1 and NFR-1.AC1** through C.
T/K entries serve **FR-3.AC1/4 and FR-5.AC3**; N/T/E entries must preserve
original units for **NFR-2.AC2/3**. Row mappings below add the substantive uses.
Three clocks remain separate: source effective time, publication/revision time,
and acquisition time. Unknown historical publication evidence permits a labelled
reconstruction, not an as-of forecast-validation claim (D-70).

## Register conventions

Sizes use decimal MB/kB unless GiB is explicit. **Recorded** means a historical
manifest size; **advertised** means publisher metadata; **UNVERIFIED** means no
verified byte figure. Cadence distinguishes publication updates from observation
resolution. Inventory-only cadence claims are explicitly UNVERIFIED here.
Order codes P0–P7 are defined in the recommended sequence at the end; dependencies
are ingestion/interpretation dependencies unless specifically called a fetch gate.
