# ADR 0004 — PostgreSQL authority, Valkey live state and Parquet results

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-2–D-3, D-17, D-19–D-20, D-22, D-28, D-30, D-36, D-38–D-39, D-48, D-56, D-65.
- **Related draft requirements:** PR #4 FR-4, FR-11, FR-16, FR-19–FR-20; NFR-1, NFR-5–NFR-6, NFR-8–NFR-10, NFR-12.
- **Companions:** [data model](0002-network-and-scenario-data-model.md), [execution](0005-execution-topology.md), [topology and source snapshot](../topology.md).

## Context and authority

Weather-year outputs multiply by assets, quantities, intervals and sweep members.
Transactional scenario/job metadata and analyst scans have different access patterns.
D-36 already fixes retention: summaries forever, pinned full results retained,
other full-resolution results deleted after 90 days. Hardware cost does not justify
reducing the model (D-2); efficient layouts avoid unnecessary copies and contention.

The boundaries and operational periods below are proposed engineering defaults
under D-22. Q-O-2 is deprecated; storage mechanics do not add an owner question.
**Valkey (BSD-3-Clause) is the recommended default**, with Redis a compatible
alternative, through Q-O-3 (answers Q-I-5). This proposes an explicit refinement
of D-3; it does not rewrite the decisions log or record an owner answer. Keep
application clients Redis-protocol-only, without Redis-specific modules/commands.
No runtime database is installed in the coding container (D-65).

## Proposed placement

| Store | Durable role / contents | Explicit boundary |
|---|---|---|
| PostgreSQL 18 | Tenant/owner scope, entity aliases and dated revisions, provenance/rights, release/scenario/run manifests, summaries, jobs/schedules/reservations/leases, audit and checkpoint pointers | Sole transactional authority; bulk payloads referenced by immutable digest/key |
| PostGIS extension | Versioned asset/zone geometry, spatial indexes, viewport/region queries and spatial joins for ingest/map projections | Optional in P1; separate server extension for spatial work, not electrical connectivity; qualify PG18 package/version |
| Valkey (Redis-compatible alternative: Redis) | Bounded progress/recent-event replay, worker presence, rate-limit counters and query caches | Rebuildable acceleration; never sole job, checkpoint, result, tenancy or pin authority |
| Parquet in filesystem/object storage | Full-resolution entity/interval results, large weather/input tables, immutable checkpoints/diagnostic bundles where suitable | Private data plane; manifest-driven visibility, no directory listing as a commit protocol |
| PostgreSQL query projections | Run/market/technology rollups, selected time-series and map-ready slices backed by result manifests | Bounded/rebuildable; not all variables for all buses for all years |

Use Arrow-compatible typed columns for interchange and Rust DataFusion/Polars for
Parquet queries in admitted analytics workers (ADR 0005), as considered in stack
candidates. Heavy scans never run in the API or heartbeat supervisor. Do not introduce
a Python/Julia runtime or a second analytical server for P1. SQL and API results
always expose product, units, interval duration, validity, rule version and native
currency. Object keys and cache keys include owner scope and immutable run/version
identity; a guessed hash must never bypass authorisation.

Choose **no TimescaleDB initially**; retain baseline PostGIS for spatial work,
optional in P1. Its GPL covers the separate PostgreSQL server extension and does
not extend to independent application client code. Native
PostgreSQL partitioning/indexes cover bounded projections and metadata; Parquet
covers bulk scans. This is a workload-fit argument, not a claim that native tables
outperform hypertables. Retention partitions must allow per-run pin exceptions:
never drop a time partition containing pinned data merely because it is old.

Store one immutable result dataset per run/schema, with deterministic logical
partition keys such as sweep member, quantity family, delivery date block and asset
bucket. Keep a member/coverage index for selective cross-member scans.
Choose physical partition/file sizes after target measurements; avoid per-asset
tiny files and do not blindly partition every low-cardinality field. Project/filter
by column, time and asset before loading; bound API query bytes, memory and rows.
Keep schema/units/orientation metadata in the manifest, not only in filenames.
Compression codec/row-group sizing is versioned and benchmarked rather than promised.

## Sizing and bulk input placement

ASSUMPTION: The following review estimates use roughly 5,000 buses, 8,000 branches,
1,500 units, 6 unit/2 branch/2 bus quantities and 17,520 half-hour intervals per
non-leap year. Actual D-31 inventory, file encoding and compression remain unknown;
these are planning orders of magnitude, not measurements or scope limits. Input
inventory/size clarification stays in Q-C-2/Q-C-3 of the canonical questionnaire.

| Workload | Approximate long-format values/rows | PostgreSQL at ~100 B/value including index | Compressed Parquet estimate, unmeasured |
|---|---|---|---|
| One 30-minute weather year | 0.6 billion values | 60 GB | Low single-digit GB |
| Same year at 1 minute (D-21) | 18 billion values | 1.8 TB | Tens of GB |
| 40-member weather-year sweep (P2, D-35) | 25 billion values | 2.5 TB | 50–150 GB |
| BOD, ~55 million rows/year, ~24 years | 1.3 billion rows | 130 GB | A few GB/year |
| P2 distribution expansion | Bus dimension ×10–100 | Row-per-value growth motivates bulk storage | Partition and measure; no capacity guarantee |

Validated bulk BOD/PN/BOA, metered data, demand and suitable time-series inputs use
immutable Parquet as canonical payloads; PostgreSQL owns their manifests, aliases
and typed metadata, not a row for every value. Gridded weather may instead retain
Zarr/NetCDF; the data spike establishes format/access needs before adoption. Preserve
original legally retainable sources and transformation identities (D-56).

## Publication and consistency across PostgreSQL and object storage

There is no cross-store transaction. Results, checkpoints and ingest releases share
one idempotent publication protocol; no file listing serves as a commit signal.

1. Register an attempt/publication ID, token and private **staging prefix** in
   PostgreSQL before upload. Write immutable staging objects, fsync filesystem data
   and directories or finish object uploads, then verify readable bytes/checksums,
   schema, coverage and numerical/rights checks as applicable.
2. Commit an immutable **pending manifest** in PostgreSQL naming staging keys,
   intended final keys, digests, sizes and interval coverage. This commits publication
   intent only; it does not advance visible coverage/checkpoint pointers or mark a
   run completed. Reject stale attempt/lease tokens and duplicate interval ownership.
3. **Promote after the manifest commit.** Use same-filesystem atomic rename or
   idempotent object copy to immutable final keys; verify each final object. Object
   stores need no directory rename. For copies, retain staging sources until final
   verification and database transition; after a rename, recovery checks the recorded
   final key when the staging key is gone. A reconciler resumes a half-promoted
   pending manifest from its recorded keys after a crash.
4. With a current lease/token, atomically mark the manifest available and advance
   result/checkpoint/summary/coverage pointers. Only available manifests are API
   visible. A unique run/interval key and conditional token transition reject stale
   or duplicate completion. Recovery can reconcile an old pending publication but
   cannot bypass current fencing to publish its scientific output.

A database failure after upload leaves invisible staging objects. A failure after
pending-manifest commit leaves recoverable publication intent, never a completed
run with partial promotion. Missing/corrupt available objects are integrity failures,
not zero results; reconciliation verifies references and repairs derived projections.

A periodic sweeper lists **staging prefixes** (and abandoned multipart uploads) to
find objects older than configurable N hours; **N = 24 hours is the proposed D-22
default**. LIST is allowed for reclamation, never scientific visibility. Before
removal, lock the publication row and verify no live attempt, pending promotion,
checkpoint, pin or retained manifest references the key. Expired pending publications
are explicitly abandoned under fencing before cleanup; age alone cannot reap an
active upload. Unregistered orphan prefixes are rechecked against the database;
database unavailability defers deletion. Promotion destinations are pre-registered
in pending manifests, so partially copied final objects are discoverable too.
Compaction publishes a new manifest before retiring old objects/references.

Local durable disk and S3-compatible object storage expose the same immutable-blob
interface. Qualify actual write/read/copy/delete/versioning semantics; S3 protocol
compatibility is not a backup guarantee. Choose the maintained storage product and
review its exact licence later, rather than mandate a self-hosted vendor here.

## Retention, pinning and recovery

| Class | Proposed policy |
|---|---|
| Summaries and provenance | Indefinite under D-36; retain scenario/rule/input hashes, coverage, costs/energy, validity, gaps and rerun limitations after deleting detail |
| Pinned run | Keep full results and required checkpoints; archive legally retainable pinned inputs, manifests and compatible build identity; no automatic expiry |
| Unpinned full results | Expire 90 days after terminal completion, including retained partial results; running jobs are not aged out; retry/resume creates a tracked attempt/run relationship |
| Reconstructible query projections | Refresh/evict as needed; full-resolution copies follow the parent run's pin/90-day policy, even if stored in SQL or cache |
| Terminal unpinned checkpoints | Expire together with their committed interval results, 90 days after terminal state (including partial/cancelled/failed); active run/resume references protect both |
| Valkey/Redis-protocol progress/cache | Proposed 24-hour recent-event horizon and 1-hour query-cache TTL; durable status read/replay falls back to PostgreSQL |
| Attempt logs/diagnostics | Proposed 90 days after terminal state unless pinned or retained for a recorded investigation; secrets/private payloads excluded from logs |
| Inputs | Keep while referenced by active runs, retained results, pins or a curated release retention policy; rights restrictions override archiving and must be disclosed |

**Resume after expiry (same rule as ADR 0005):** first lock the retained run and
acquire a temporary resume hold through the pin/deletion state machine. Then verify
the checkpoint/input chain and every previously committed interval object before
starting work; release the hold on rejection. The hold prevents GC during checks.
Expired, missing or corrupt checkpoints or outputs reject resume. A replacement is a new run from the original validated
initial state, rerunning the pre-pass and **all intervals from the start**, not a
continuation from a surviving later state. Retention/rights can make even that rerun
unavailable. Completion requires exact requested-horizon coverage and verified objects
for every interval; a checkpoint or summary never replaces deleted interval data.

Pins and deletion use a PostgreSQL state machine with row locks/conditional updates:
`retained-unpinned ↔ retained-pinned`, then `retained-unpinned → tombstoned → deleting
→ deleted`. The expiry transaction checks age, pins, active resumes/publications and
reader/reference holds, then **commits the tombstone before any external deletion**.
A pin can only transition a retained row: a tombstoned/deleting/deleted run rejects
pin and resume, even if some physical objects still exist. GC workers act only on
committed tombstones and idempotently retry deletion. A pin that commits first blocks
tombstoning. Unpinning an old run schedules the same grace with its effect disclosed.
There is no read-unpinned/delete-files/write-tombstone path.

Deletion uses manifest reachability and scope/reference counts plus reader grace,
never indiscriminate prefix deletion under a shared dataset hash. Shared input blobs
stay while any retained run/release needs them. A sweep-parent pin atomically pins
its existing children, reduction manifest and required shared inputs; future members
inherit that pin. Unpinning the parent releases only its holds, not independent child
pins. A summary-only retention choice is distinct and does not claim child detail
is pinned. Physical removal and retries cannot race a newly accepted pin.

Use PostgreSQL backups/WAL/PITR plus independently backed-up manifests and versioned
blob recovery copies. Proposed recovery window: 30 days with daily recorded recovery
points; no RPO/RTO or HA SLA is claimed. Keep deleted blob versions for that window,
inaccessible to ordinary users, to permit a consistent recovery point. Before
physical deletion, copy each committed tombstone idempotently into an append-only
external deletion journal, with replication/checkpoint monitoring; deletion waits
for journal durability. Replay includes journal entries after the selected PITR
point and the current authoritative journal, not only the restored database rows.
Restore database and compatible blob versions, replay tombstones, check all remaining
manifest references, then enable access. Missing journal coverage blocks reopening.
Recovery copies age out under the documented policy. P4 legal deletion may override
this ordinary retention/recovery policy and requires an explicit separate decision.

After baseline, test restore with post-PITR tombstones, missing/corrupt blobs, pin/GC/
resume races, partial promotion and complete Valkey loss. Cache replication/AOF may
improve restart speed but replaces neither PostgreSQL authority nor checkpoints.
Separate live telemetry from evictable cache capacity if eviction would lose useful
recent events; dispatch and completion never depend on either instance.

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
  feature. TSL features are generally unavailable on hyperscaler managed PostgreSQL;
  verify hosting support before relying on them. Its qualifying value-added
  application grant can cover an energy SaaS,
  subject to substantial non-database value, restrictions on customer schema changes,
  notices and other conditions; it does not generally permit managed Timescale
  DBaaS. Review actual endpoints/terms, not the label “SaaS.” No adoption now.
- **Valkey (default):** 9.1.3 core is BSD-3-Clause [S6]; audit bundled-code notices
  and pin a reviewed release/image at deployment. Q-O-3 answers Q-I-5 and explicitly
  proposes refining D-3. Keep application clients to the shared Redis protocol and
  qualify cache/progress/ACL/reconnect semantics. No Redis-only modules or commands.
- **Redis 8 (compatible alternative):** 8.10.3 offers AGPLv3, RSALv2 or SSPLv1 [S5].
  Deployment requires a recorded service/image/licence assessment. Separate-server
  protocol access does not automatically relicense independent application clients;
  AGPL network-source duties for modified covered programs and conveyance duties
  still need assessment. RSAL and SSPL are not permissive substitutes. No arbitrary
  command proxy or customer database endpoint is proposed. Choosing Valkey avoids
  these additional core-licence conditions; it is not a claim that all AGPL SaaS use
  contaminates client code.

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
| Cache service as durable result/job authority | Low-latency access | Loss/replay/eviction semantics and no cross-store commit; decline for scientific provenance and job completion |
| Redis instead of default Valkey | Compatible protocol | Requires explicit licence assessment and conformance; not needed for dispatch |

PostgreSQL documents both partitioning and queue-like SKIP LOCKED use [S7]; Parquet
documents columnar representation [S8]. These establish mature mechanisms, not this
workload's measured performance. After baseline, qualify export/query latency, WAL
growth, recovery, retention and pinning on the as-yet-unspecified D-65 target.
ASSUMPTION: D-48's 5900X/128 GB is only a provisional planning floor (Q-H-7),
not the specified target or GPU configuration. No heavy benchmark runs here.

## Primary evidence

- S1 — PostgreSQL licence: https://www.postgresql.org/about/licence/
- S2 — PostGIS 3.7.0 GPL text: https://raw.githubusercontent.com/postgis/postgis/3.7.0/COPYING ; source notice: https://raw.githubusercontent.com/postgis/postgis/3.7.0/liblwgeom/lwgeom_geos.c
- S3 — TimescaleDB 2.32.0 boundary: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/LICENSE ; TSL §§2.1, 3.10: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/LICENSE-TIMESCALE
- S4 — PG18 compatibility history: https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/CHANGELOG.md
- S5 — Redis 8.10.3 licence choices/texts: https://raw.githubusercontent.com/redis/redis/8.10.3/LICENSE.txt
- S6 — Valkey 9.1.3 notices: https://raw.githubusercontent.com/valkey-io/valkey/9.1.3/COPYING
- S7 — PostgreSQL 18 partitioning: https://www.postgresql.org/docs/18/ddl-partitioning.html ; queue locking: https://www.postgresql.org/docs/18/sql-select.html#SQL-FOR-UPDATE-SHARE
- S8 — Parquet overview: https://parquet.apache.org/docs/overview/
