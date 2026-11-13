# ADR 0005 — Independent Rust worker supervision and PostgreSQL job dispatch

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-2–D-3, D-8, D-10, D-13, D-16–D-22, D-24–D-25, D-28–D-29, D-35–D-36, D-39, D-43–D-44, D-48, D-55, D-59, D-62, D-65, D-67–D-68, D-70.
- **Related draft requirements:** PR #4 FR-4, FR-6–FR-11, FR-15–FR-16, FR-18; NFR-1–NFR-2, NFR-4–NFR-10, NFR-12.
- **Companions:** [data model](0002-network-and-scenario-data-model.md), [storage](0004-storage-topology.md), [market modules](0006-market-design-plugin-model.md), [topology/source snapshot](../topology.md).

## Context and authority

Numerical runs outlive HTTP requests and API deployments. Native solver failure,
memory pressure and rolling-state continuity need explicit process boundaries.
D-25 also requires day-ahead/intraday forecasting in P2: ingestion, durable schedules
and deadline admission must fit these contracts without replacing the job model.
Resource cost is not the constraint (D-2); correctness and responsiveness still are.

ASSUMPTION: Use D-48's Ryzen 5900X (12 cores/24 threads) and 128 GB RAM as a
provisional planning floor until the separate D-65 runtime target is specified.
D-48 describes an available Linux machine, not the agreed simulator target. No GPU
model, VRAM or runtime capacity is inferred. The inventory clarification is tracked
under Q-H-7 in the [canonical questionnaire](../../requirements/02-questionnaire.md).
Reconcile PR #4 NFR-5/ADR 0003 with that inventory before qualification. No heavy
builds, databases or benchmarks run in the coding container (D-65).

The protocols below are proposed engineering defaults under D-22, subject to ADR
acceptance and the requirements baseline. PR #4 numerical/rolling/cancellation
policies remain proposals. Only the product choices in the
[companion questionnaire](../owner-questionnaire.md) need separate owner disposition;
Q-O-4 is deprecated. No application code, P1 distributed sweep or P4 tenancy decision
is authorised here.

## Proposed process and module boundaries

Use a modular Rust API control service and an **independently supervised worker
service from P1**, with separate systemd units or Compose services. The API handles
private ingress, scenarios, submissions, cancellation intent and bounded metadata
reads. Restarting it does not terminate runs. The worker supervisor owns job claims,
lease renewal, resource admission, host/solver lifecycle and fenced publication.
Native solvers never live in the API or share handles across async threads.

| Role | P1 process boundary and responsibility | Later scaling boundary |
|---|---|---|
| Web UI / Rust CLI/SDK | Shared API and canonical TOML contracts | Static hosting/API replicas; no direct store access |
| Rust API | Catalogue, validation, submission/cancel intent, status and small projections | Replicas using the same PostgreSQL authority |
| Worker supervisor | Separate service; claims/heartbeats, process-tree containment, admission, publication and recovery | Per-node supervisors with authenticated control and host fencing |
| Run host and native solvers | Per-attempt cgroup/process tree owned by supervisor; sparse build, rolling engine, isolated native solver subprocesses and independent residual checks | Resource-labelled CPU/GPU/licensed pools |
| Ingest (`crates/ingest`, D-62) | Separate acquisition role: source adapters, validation, immutable releases and publication/acquisition evidence | P2 continuous feeds and weather forecast vintages |
| Analytics/reduction workers | Heavy Parquet scans and sweep reductions outside the API and supervisor | Independent bounded pools and summary manifests |
| PostgreSQL / Valkey / blob store | Separate services/durable volumes | Managed/dedicated nodes, backups and qualified object storage |

Reuse ADR 0001's crate boundaries. Within `crates/engine`, distinguish the service
job/lifecycle module (leases, retries, publication) from the scientific rolling
pipeline; `crates/api` delegates to it rather than owning a second lifecycle.
Storage/immutable-blob adapters live in `crates/engine::store`, behind scope-aware
repository interfaces defined with domain contracts; domain records have no database
client dependency.
These proposed responsibility details refine merged ADR 0001’s engine-orchestration
boundary without changing workspace code. Ingest uses `crates/ingest`, market/network/optimisation retain their
scientific roles, and commitment/redispatch/pricing retain the staged engine contract.

Heavy DataFusion/Polars scans, model construction and LOLE/EUE/cross-member
reductions run as admitted worker jobs, each with a CPU/RAM budget and summary
manifest. The API permits only bounded rows/bytes/time from precomputed projections.
The supervisor's heartbeat/watchdog executor has reserved CPU/RAM and never executes
analytics; analytics capacity cannot consume control/lease headroom.

Define a versioned transport interface from the outset: coarse task/result/control
envelopes and authorised checksummed blob references, independent of kernel handles.
A local Unix transport may optimise large transfers with Arrow/file-descriptor
passing. That is an adapter optimisation, never the domain/task representation or
only way to load data. A network adapter (for example authenticated gRPC plus blob
fetch) carries the same envelopes without changing engine/solver contracts. Bound
messages, validate digests and keep host paths confined to the attempt workspace.
No per-coefficient RPCs or matrices in the cache protocol.

## Ingest and durable scheduled forecasts

Ingest is the only acquisition role with allowlisted external egress (Elexon, NESO,
SEMO/SEMOpx, ENTSO-E and weather providers). Credentials are scoped to that role,
kept outside manifests/logs and never supplied to solvers. Validate units, calendars,
coverage and D-56 rights/lineage before admission. Write to private staging and use
ADR 0004's manifest/verification/promotion protocol. Preserve source publication
proof and actual acquisition time separately (ADR 0002); a revised historical
download does not fabricate a forecast vintage. P1 supports curated historical
releases; continuous live acquisition is P2, not additional P1 scope.

The P2 scheduler stores versioned schedules, timezone/calendar, forecast origin,
knowledge cut, required input vintages, deadline and job class in PostgreSQL.
A unique `(owner, schedule revision, forecast origin)` key creates at most one run
per occurrence transactionally; restart scans recover due occurrences. No in-memory
cron or Valkey notification is the only trigger. A delayed feed produces an explicit
blocked/missed-deadline status, never an undisclosed later knowledge cut or latest
release. New vintages resolve to immutable locks before a run becomes runnable.

D-68 (Q-O-6) requires P2+ forecast runs to preempt research/sweep work. The
checkpoint/replay and reservation mechanics below remain proposed engineering
defaults. Persist
priority class, deadline, preemption intent and resume parent in the same job model
from P1. Select eligible forecasts before research. To meet a deadline, checkpoint
a research run at the next committed advance boundary **only if that is soon enough**;
otherwise stop/reap it, discard its uncommitted window and free its reservation.
It later resumes from its last durable state. Do not wait for a multi-hour window
or annual pre-pass to finish. Reserved forecast capacity is an alternative when
qualified concurrency allows it; two 96 GiB jobs cannot share a 128 GB host.
Qualify forecast admission/kill latency in P2. Priority removes research blocking;
it does not promise timely forecasts if input data or solve capacity is inadequate.

## PostgreSQL dispatch, leases and physical fencing

PostgreSQL owns jobs, schedules, attempts, dependencies, cancellation/preemption
intent, reservations, leases and checkpoints. P1 dispatch uses transactional
`SELECT ... FOR UPDATE SKIP LOCKED` [S1], with `LISTEN/NOTIFY` [S2] as a wake-up
hint and periodic runnable-job scans. Lost notifications cannot lose work. Valkey
(the D-67 default Redis-protocol service; Q-O-3/Q-I-5 answered) holds only rebuildable
progress/recent events, presence, rate limits and cache. No Streams relay, consumer
acknowledgement or cache availability is required to claim a P1 job.

1. Authorise and freeze the scenario/lock; create the run/job transactionally. An
   idempotency key is unique within owner scope and rejects different inputs on
   reuse. Record solver/capability checks; never silently substitute a backend.
2. The worker supervisor locks an eligible row, checks priority/dependencies and
   atomically reserves capacity, claims the job, increments its monotonic token
   and records attempt/lease expiry. Concurrent supervisors serialise per-node
   resource reservations as well as job claims; SKIP LOCKED alone is not admission.
3. Start the contained attempt tree. Proposed lease: 60 seconds, renewed every
   10 seconds by the supervisor using database time and matching attempt/token.
   Publication/completion require the same token and an unexpired lease. A local
   watchdog stops the tree before lease authority is lost when renewal fails.
4. On expiry, mark recovery pending and reject the stale token. **Before reclaiming
   the lease or launching a successor**, the supervisor must confirm the prior
   attempt's cgroup is empty or kill and reap its entire process tree, including
   native descendants. Store death/fencing evidence, then release its reservation.
   A partitioned remote host requires positive host fencing/death confirmation;
   inability to prove death blocks reclaim on that host. A token alone prevents
   stale writes but does not reclaim physical RAM, VRAM or solver licences.
5. Re-run memory/CPU/VRAM/licence admission against actual remaining headroom and
   atomic reservations, then assign a fresh token and recover from a verified
   checkpoint. Never overlap jobs whose reservations exceed the RAM budget, even
   while an old worker is being killed. Cancellation and preemption use this same
   kill/reap and admission sequence.

At-least-once attempts with conditional idempotent publication are promised, not
exactly-once execution. Attempt-specific staging and database publication close
the result-store gap (ADR 0004). No critical lock depends on a cache key.

On Valkey loss, status falls back to PostgreSQL polling; dispatch is unchanged.
On PostgreSQL loss, accept no claims/publication; supervisors stop work when lease
renewal cannot be guaranteed. On blob failure, do not advance the checkpoint.
Transient infrastructure failures get a proposed maximum of three attempts with
bounded backoff. Invalid inputs and genuine model infeasibility are reported rather
than blindly retried. UC↔redispatch non-convergence invokes ADR 0006's documented
fallback; exhausting the repair count alone does not fail the whole run.

Externally retain PR #4 states: queued, running, completed, partial, cancelled and
failed. Internal suspended/retry/recovery states describe attempts without changing
scientific validity. API deployments do not own or kill attempt processes. Manual
resume records its parent attempt/checkpoint and passes all checks below.

## Checkpoints, determinism and resume

Persist a validated initial-state manifest before the pre-pass. It includes exact
initial commitment timers, SoC/budgets, dated inputs, policy and execution identity.
After the annual/seasonal pre-pass, commit a **genesis checkpoint** containing that
initial state plus the pre-pass result/targets, empty interval coverage and a null
predecessor hash. The first advance checkpoint references the genesis hash; later
checkpoints reference their immediate predecessor. If failure occurs before genesis,
rebuild the validated initial state from its manifest and rerun the pre-pass and
first window. Once genesis exists, reuse its persisted pre-pass; no rerun with a
silently different energy signal.

Checkpoints commit the **advance boundary**, never the look-ahead end. For the
proposed 48 h look-ahead/24 h advance, retain:

- Complete committed interval coverage, result manifests, independent numerical
  certificates, original/repaired/fallback stages, pricing policy and diagnostics.
- On/off elapsed timers, output/ramp/start history, storage/hydro SoC and energy
  budgets, reserves and all intertemporal market state.
- Pre-pass identity/targets, topology cursor, rule releases, RNG state and model/input
  hashes; schema, engine/solver/library/platform versions and fixed options.
- Attempt/token and predecessor hash, linking each commit back to genesis.

Use ADR 0004's staging → committed pending manifest → promotion → visible transition;
advance checkpoint/coverage pointers atomically only after final objects verify.
A unique run/interval constraint prevents duplicate committed output. Crash recovery
restarts at the last **committed checkpoint boundary from persisted physical state**,
not an in-memory incumbent or portable mid-MIP branch tree. LP bases/MIP starts are
optional version-specific accelerators. Schema/input/build compatibility is verified;
incompatible execution requires a new run or separately reviewed migration.

The determinism contract fixes solver seeds, thread counts, versions, options,
platform/library identity, row/column ordering and per-child RNG streams. Use the
same documented cold/reconstructed warm-start policy at every boundary, including
uninterrupted execution; unrecorded solver history must not be a scientific input.
Persist reused basis/start artifacts when required by that policy. Record all of
this in execution provenance and distinguish scenario identity from execution identity.

These controls aim at repeatability within a qualified configuration. **Bit identity
is not promised for MIP solutions**, alternative integer optima, parallel solvers,
time-limit termination or cross-platform floating-point execution. A resumed window
can take a different feasible path within the declared MIP gap/tolerances. Independently
check physical feasibility and report objective gaps, tolerances, termination cause,
resume boundary and any non-bit-identical outcome in run provenance. Conservation
and accepted numerical tolerances follow PR #4 NFR-2, not a hash-based guarantee of
identical prices/commitment. Wider scientific sensitivity needs separate evidence.
D-70 answers the corresponding reproducibility claim in Q-O-1/Q-D-5.

**Resume after expiry:** unpinned terminal partial/cancelled/failed checkpoints expire
with their interval results under the same 90-day clock. Before any resume, lock
the retained run and acquire a temporary resume hold through ADR 0004's deletion
state machine. Then verify every committed interval/object and the entire required
checkpoint/input chain; release the hold on rejection. GC cannot race these checks. If a checkpoint or earlier committed output
has expired, is corrupt or is missing, reject resume with an integrity/expiry reason.
Offer a **new run from the original validated initial state, rerunning the pre-pass
and every interval from the start**; do not continue from surviving later state and
call it a complete original-horizon run. Missing retained inputs can also block that
rerun. Completion requires exact requested-horizon coverage and verified objects for
every interval; neither a late resume nor a summary can bridge a gap.

## Cancellation and resource admission

Persist cancellation first. Request a supported native interrupt, allow a proposed
two-second cooperative grace, then kill/reap the whole host/solver tree within the
proposed five-second total target. Cooperation is capability-dependent, not guaranteed
during blocking BLAS/native calls. A separate watchdog enforces the hard fallback;
a stuck/unreapable tree blocks resource reuse and is reported as a failed cancellation
bound. Qualify each adapter after baseline; no timing success is claimed here. The
API never concurrently calls a raw native handle. Race completion versus cancel in
PostgreSQL. Only validated durable intervals survive; partial status stays explicit.

Start P1 with one simulation reservation. Use the smaller of PR #4's proposed 96 GiB
or 80% of measured RAM as a process-tree ceiling, subject to stricter actual headroom
for OS, PostgreSQL, API, supervisor, ingest and analytics. Per-node transactional
reservations plus observed use prevent oversubscription. CPU threads, VRAM and solver
tokens are reserved too; 24 logical cores do not justify 24 simultaneous MILPs.
Relocate services or add capacity rather than lower model fidelity (D-2). Optional
GPU use needs actual device/library/licence qualification.

| Phase | Allocation and deployment | Evidence needed |
|---|---|---|
| P1 | Separate API and supervisor services; one contained simulation initially; historical ingest and bounded analytics jobs; sequential CLI batch allowed | Target inventory, crash/reap/admission, API redeploy survival, expiry/resume, full-network year and restore |
| P2 | Durable sweeps and scheduled forecasts; priority/deadlines and research preemption or reserved forecast capacity; add worker nodes as needed | Feed vintages, schedule deduplication, bounded admission latency, stable child seeds, fair research resumption and reduction jobs |
| P3 | Worker pools/API replicas, qualified shared objects and optional role extraction | Host fencing, network transport, input locality and analytics throughput |
| P4 | Authenticated tenant admission, quotas/audit and hardened isolation | Owner-selected isolation, threat model, scoped credentials and availability/recovery objectives |

Batch parents track identities/coverage/statuses; cancellation cascades to unfinished
members. Sweep reductions are admitted worker jobs publishing their own manifests.
Pinning semantics are explicit in ADR 0004. Independent children can run concurrently
when admitted; within-run windows remain sequential because commitment/energy state
carries forward. Parallel time blocks need ADR 0003's reconciled-boundary algorithm
and bias evidence; extra machines do not remove that dependency.

Carry owner scope through all IDs, foreign keys, jobs, reservations, cache entries,
objects and logs. P1 is private single-owner, not public tenancy. P4 may choose RLS,
schema/database-per-tenant or dedicated deployments (D-39). Scope-aware repository,
blob and transport interfaces preserve the domain contracts, while enforcement and
migrations still need work. Before exposure, authenticate workers/users, authorise
every object, use short-lived scoped credentials and validate cross-tenant denial.
Never accept arbitrary native plug-ins, SQL, cache commands or host paths from TOML.

## Alternatives and qualification

| Alternative | Benefit | Disposition |
|---|---|---|
| All work in API | Few processes | Reject: native failure/scans threaten API and heartbeats |
| PostgreSQL dispatch plus independent supervision (chosen) | One durable claim authority; API deploys preserve runs | Requires physical fencing, reservations and cross-store publication |
| Redis Streams / NATS / workflow engine | Lower-latency delivery or richer timers | Consider only after measured need; no P1 dispatch dependency |
| API and separate controller microservice | Independent scaling | Worker supervision already separate; further control extraction follows evidence |
| Cluster scheduler | Mature placement/quotas | Reconsider for multiple nodes/tenants; not required for P1 |

Before adoption: apply D-67/D-68/D-70, confirm target inventory and review the
integrated contracts. PR #4 head `6c6a9eb` is merged; ADR 0003 now supplies the
shared non-convergence recovery and numerical budgets used by ADR 0006. No repair
policy conflict remains; the crate-responsibility section refines ADR 0001’s engine
boundary without assigning a second lifecycle to the API.
After baseline, inject lost wakeups, cache/database loss, stale leases and zombie
workers, API restarts, partial promotion, checkpoint expiry and cancel/complete races.
Verify complete coverage, fenced resource reuse and responsive heartbeats under
analytics. Then qualify the full-network year on the actual D-65 target. No runtime
or scientific-validation success is claimed by this documentation proposal.

## Primary evidence

- S1 — PostgreSQL 18 queue-like SKIP LOCKED: https://www.postgresql.org/docs/18/sql-select.html#SQL-FOR-UPDATE-SHARE
- S2 — PostgreSQL 18 LISTEN/NOTIFY: https://www.postgresql.org/docs/18/sql-notify.html
- PR #4 ADR 0001 workspace and ADR 0003 staged contract: [pinned source context](../topology.md#design-context-and-evidence).
