# ADR 0004 — PostgreSQL authority, Redis live state and Parquet results

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-2–D-3, D-17, D-19–D-20, D-28, D-30, D-36, D-38–D-39, D-48, D-56, D-65.
- **Related draft requirements:** PR #4 FR-4, FR-11, FR-16, FR-19–FR-20; NFR-1, NFR-5–NFR-6, NFR-8–NFR-10, NFR-12.
- **Companions:** [data model](0002-network-and-scenario-data-model.md), [execution](0005-execution-topology.md), [topology and source snapshot](../topology.md).

## Context and authority

Weather-year outputs multiply by assets, quantities, intervals and sweep members.
Transactional scenario/job metadata and analyst scans have different access patterns.
D-36 already fixes retention: summaries forever, pinned full results retained,
other full-resolution results deleted after 90 days. Hardware cost does not justify
reducing the model (D-2); efficient layouts avoid unnecessary copies and contention.

ASSUMPTION (Q-O-2/Q-O-3): All storage boundaries, maintenance periods and licence
paths below are proposed defaults in the [companion questionnaire](../owner-questionnaire.md).
PostgreSQL 18 and Redis themselves are owner choices (D-3); Valkey is an alternative,
not an authorised substitution. This ADR grants no licence exception or permission
to install a runtime database in the coding container (D-65).

## Proposed placement

| Store | Durable role / contents | Explicit boundary |
|---|---|---|
| PostgreSQL 18 | Tenant/owner scope, entity aliases and dated revisions, provenance/rights, release/scenario/run manifests, summaries, job/outbox/leases, audit and checkpoint pointers | Sole transactional authority; bulk payloads referenced by immutable digest/key |
| PostGIS extension | Versioned asset/zone geometry, spatial indexes, viewport/region queries and spatial joins for ingest/map projections | Geometry does not determine electrical connectivity; approve package/version and GPL obligations before adoption |
| Redis 8 | Stream notifications, bounded progress/recent-event replay, worker presence, rate-limit counters and query caches | Rebuildable acceleration; never sole job, checkpoint, result, tenancy or pin authority |
| Parquet in filesystem/object storage | Full-resolution entity/interval results, large weather/input tables, immutable checkpoints/diagnostic bundles where suitable | Private data plane; manifest-driven visibility, no directory listing as a commit protocol |
| PostgreSQL query projections | Run/market/technology rollups, selected time-series and map-ready slices backed by result manifests | Bounded/rebuildable; not all variables for all buses for all years |

Use Arrow-compatible typed columns for interchange and Rust DataFusion/Polars for
bounded Parquet queries, as already considered in stack candidates. Do not introduce
a Python/Julia runtime or a second analytical server for P1. SQL and API results
always expose product, units, interval duration, validity, rule version and native
currency. Object keys and cache keys include owner scope and immutable run/version
identity; a guessed hash must never bypass authorisation.

Choose **no TimescaleDB initially**, and propose PostGIS for spatial work. Native
PostgreSQL partitioning/indexes cover bounded projections and metadata; Parquet
covers bulk scans. This is a workload-fit argument, not a claim that native tables
outperform hypertables. Retention partitions must allow per-run pin exceptions:
never drop a time partition containing pinned data merely because it is old.

Store one immutable result dataset per run/schema, with deterministic logical
partition keys such as quantity family, delivery date block and asset bucket.
Choose physical partition/file sizes after target measurements; avoid per-asset
tiny files and do not blindly partition every low-cardinality field. Project/filter
by column, time and asset before loading; bound API query bytes, memory and rows.
Keep schema/units/orientation metadata in the manifest, not only in filenames.
Compression codec/row-group sizing is versioned and benchmarked rather than promised.

## Publication and consistency across PostgreSQL and object storage

There is no cross-store transaction. A worker writes to an attempt-specific private
staging area; the controller checks checksums, schema, coverage, numerical validity
and the current fencing token (ADR 0005). Files are immutable once published.

On a filesystem, fsync data and containing directories and use same-filesystem
atomic rename. On object storage, upload immutable keys and verify complete readable
objects/checksums before publication; do not rely on atomic directory rename or
LIST order. Record an immutable manifest naming every key, digest, size and logical
interval. Then one PostgreSQL transaction advances the checkpoint pointer and inserts
the result manifest/summary/coverage. Only database-referenced objects are visible
through the API. A uniqueness constraint on run/committed interval and a conditional
lease-token update reject stale or duplicate completion.

An object-store success followed by database failure leaves an invisible orphan,
not a completed result. A database row must never reference a pending upload.
Reconciliation verifies committed references and repairs derived caches/projections;
missing/corrupt objects surface integrity failure, never synthetic zero results.
Temporary objects expire only after the attempt/lease is no longer active and a
proposed 24-hour orphan grace. Compaction publishes a replacement manifest before
retiring old files; live readers/checkpoints/pins keep their references valid.

Local durable disk and S3-compatible object storage implement the same small
immutable-blob interface. S3 protocol compatibility alone is not a consistency or
backup guarantee; qualify the selected provider's write/read/delete semantics.
Choose a maintained store with reviewed licence terms later, rather than mandating
a particular self-hosted object-store product here.

## Retention, pinning and recovery

| Class | Proposed policy |
|---|---|
| Summaries and provenance | Indefinite under D-36; retain scenario/rule/input hashes, coverage, costs/energy, validity, gaps and rerun limitations after deleting detail |
| Pinned run | Keep full results and required checkpoints; archive legally retainable pinned inputs, manifests and compatible build identity; no automatic expiry |
| Unpinned full results | Expire 90 days after terminal completion, including retained partial results; running jobs are not aged out; retry/resume creates a tracked attempt/run relationship |
| Reconstructible query projections | Refresh/evict as needed; full-resolution copies follow the parent run's pin/90-day policy, even if stored in SQL or cache |
| Completed unpinned checkpoints | Keep while needed for an active/resumable run, then expire with that run's full results; never delete the last usable active checkpoint |
| Redis progress/cache | Proposed 24-hour recent-event horizon and 1-hour query-cache TTL; durable status read/replay falls back to PostgreSQL |
| Attempt logs/diagnostics | Proposed 90 days after terminal state unless pinned or retained for a recorded investigation; secrets/private payloads excluded from logs |
| Inputs | Keep while referenced by active runs, retained results, pins or a curated release retention policy; rights restrictions override archiving and must be disclosed |

Deletion is mark-and-sweep from durable manifest references, with scope checks and
object reference counts/reader grace, not prefix deletion of everything under a
dataset hash. Shared input objects are not removed while another pin/run/release
needs them. Pin and expiration race in one PostgreSQL transaction: a pin acquired
before a deletion tombstone protects the data; a later request reports detail already
unavailable. Unpinning an already-old run schedules collection after the same grace,
with the proposed effect disclosed to the user.

Use PostgreSQL backups plus WAL/PITR, and independently backed-up blob manifests
and data. Proposed backup window: 30 days with a daily recorded recovery point;
no RPO/RTO or HA SLA is claimed. Backups are access-controlled recovery copies,
not an alternate user-accessible result archive. Tombstones survive restoration and
are reapplied before access is enabled, so expired details do not reappear; backup
copies age out within the documented window. P4 deletion/legal obligations require
a separate policy and may override ordinary user retention (D-39).

Test restoration of a consistent database/blob recovery point, corrupt/missing
blobs, pin/GC races and Redis total loss after baseline. Redis AOF/replication may
improve restart speed; they never replace the PostgreSQL outbox or checkpoint.
Separate a no-eviction queue/live-state instance from an evictable cache when cache
pressure could discard queue keys; backpressure/reconciliation is still required.

## Licence implications and upgrade boundaries

The repository's [licence inventory](../../research/dependency-licences.md) is
supporting research, not deployment approval. Primary release texts were rechecked
on 2026-11-13; observed versions below are evidence snapshots, not installation pins.

- **PostgreSQL 18:** permissive PostgreSQL licence [S1]. Review packaged components
  and upgrades; PostgreSQL is the accepted major line, not a requirement to run 18.0.
- **PostGIS:** GPL-2.0-or-later in the researched 3.7.0 source/package [S2]. Ordinary
  separate-server SaaS use does not create AGPL-style network-source obligations
  for the application. Conveying PostGIS, modifications or covered combinations in
  an appliance/container distribution has GPL obligations. Review those artifacts
  and the exact PG18-compatible build before activation; a network boundary alone
  is not universal licence clearance.
- **TimescaleDB:** 2.32.0 source separates Apache-2.0 portions from TSL code [S3].
  PG18 support began in 2.23.0 [S4]; a compatible source release does not establish
  availability of a particular image/package. Core hypertables/basic time_bucket
  differ from TSL compression/columnstore, continuous aggregates and automated
  retention. TSL is not an unrestricted open-source/permissive option for every
  feature. Its qualifying value-added application grant can cover an energy SaaS,
  subject to substantial non-database value, restrictions on customer schema changes,
  notices and other conditions; it does not generally permit managed Timescale
  DBaaS. Review actual endpoints/terms, not the label “SaaS.” No adoption now.
- **Redis 8:** the 8.10.3 licence offers a choice of AGPLv3, RSALv2 or SSPLv1 [S5].
  Proposed Q-O-3 path: unmodified private Redis under AGPLv3, independent protocol
  client, with source/notice/distribution and any combined-work assessment recorded.
  AGPL §13 concerns remote interaction with a modified covered program; it does
  not automatically relicense every independent API client. RSALv2 restricts
  specified third-party service/product uses; SSPLv1's service-source conditions
  are broader than a permissive licence. Neither is approved by choosing Redis.
  No customer database endpoint or arbitrary Redis command proxy is proposed.
- **Valkey:** 9.1.3 core is BSD-3-Clause [S6], with bundled-code notices to audit.
  It is the simpler permissive alternative, but needs an owner decision amending
  D-3 and tests of Streams/recovery/ACL/client semantics. Protocol ancestry is not
  proof of feature parity; avoid Redis modules and unneeded recent commands.

D-56's research-only dataset permission does not waive software/service obligations.
The chosen service/image/version/licence path and native dependencies must enter
the approved deployment inventory before packaging or exposure (PR #4 NFR-8).

## Alternatives and evidence for the choice

| Option | Strength | Cost / condition for reconsideration |
|---|---|---|
| Everything in PostgreSQL | One backup/query/transaction model | Row/index/WAL volume and bulk scan contention; benchmark if output inventory proves small enough |
| PostgreSQL + TimescaleDB | Mature time bucketing, time-series policies and aggregates | Extension upgrade/feature-licence boundary; adopt only if measured retained SQL workloads miss targets and PG18 artifacts/terms pass review |
| ClickHouse analytical service | Strong sustained columnar aggregates/concurrency | Another distributed store and ingestion/consistency path; consider for measured P3/P4 analytics demand |
| PostgreSQL + Parquet (chosen) | Transactions plus portable columnar bulk data, cheap selective projections | Cross-store publication/backup/GC and scan latency must be engineered and measured |
| Redis as durable result/job authority | Low-latency access | Loss/replay/eviction semantics and no cross-store commit; decline for scientific provenance and job completion |
| Valkey replacing Redis | Permissive core and familiar protocol | Best licence simplicity, but D-3 approval and command/operational conformance are outstanding |

PostgreSQL documents both partitioning and queue-like SKIP LOCKED use [S7]; Parquet
documents columnar representation [S8]. These establish mature mechanisms, not this
workload's measured performance. After baseline, qualify export/query latency, WAL
growth, recovery, retention and pinning on the D-65 target at least at the stated
5900X/128 GB/consumer-NVIDIA baseline. No database or heavy benchmark runs here.

## Primary evidence

- S1 — PostgreSQL licence: https://www.postgresql.org/about/licence/
- S2 — PostGIS 3.7.0 GPL text: https://raw.githubusercontent.com/postgis/postgis/3.7.0/COPYING ; source notice: https://raw.githubusercontent.com/postgis/postgis/3.7.0/liblwgeom/lwgeom_geos.c
- S3 — TimescaleDB 2.32.0 boundary: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/LICENSE ; TSL §§2.1, 3.10: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/LICENSE-TIMESCALE
- S4 — PG18 compatibility history: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/CHANGELOG.md
- S5 — Redis 8.10.3 licence choices/texts: https://raw.githubusercontent.com/redis/redis/8.10.3/LICENSE.txt
- S6 — Valkey 9.1.3 notices: https://raw.githubusercontent.com/valkey-io/valkey/9.1.3/COPYING
- S7 — PostgreSQL 18 partitioning: https://www.postgresql.org/docs/18/ddl-partitioning.html ; queue locking: https://www.postgresql.org/docs/18/sql-select.html#SQL-FOR-UPDATE-SHARE
- S8 — Parquet overview: https://parquet.apache.org/docs/overview/
