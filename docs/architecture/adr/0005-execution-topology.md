# ADR 0005 — A Rust control service with isolated workers and durable jobs

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-2–D-3, D-8, D-10, D-13, D-16–D-21, D-24, D-28–D-29, D-35–D-36, D-39, D-43–D-44, D-48, D-55, D-59, D-65.
- **Related draft requirements:** PR #4 FR-4, FR-6–FR-11, FR-15–FR-16, FR-18; NFR-1–NFR-2, NFR-4–NFR-10, NFR-12.
- **Companions:** [data model](0002-network-and-scenario-data-model.md), [storage](0004-storage-topology.md), [market modules](0006-market-design-plugin-model.md), [topology/source snapshot](../topology.md).

## Context and authority

Numerical runs take much longer than HTTP requests. Native solver failure, memory
pressure, cancellation and rolling-state continuity need explicit process boundaries.
Resource cost is not the constraint (D-2), but correctness, responsiveness and recovery
still are. A microservice count is not evidence of simulation throughput.

D-65 separates the coding container from the runtime. Target specifications are TBD;
the task's owner clarification sets a planning floor of the D-48 5900X (12 cores/
24 threads), 128 GB RAM and consumer NVIDIA setup. It does not identify the container
as that machine, guarantee an installed GPU model or change PR #4's CPU qualification
workload. Inventory the actual target and reconcile NFR-5/ADR 0003 wording before
baseline acceptance; do not benchmark, build heavily or start databases here.

ASSUMPTION (Q-O-4): The process layout, queue/recovery protocol and all operational
defaults below are proposals in the [companion questionnaire](../owner-questionnaire.md).
PR #4's rolling, numerical, private-access and cancellation policies remain proposed
Q-N defaults, not newly accepted requirements. No P1 distributed sweep, public
deployment or P4 isolation decision is pulled forward by this design.

## Proposed process and module boundaries

Adopt the coordinator's modular monolith **for the control plane**. One Rust service
composes API, scenario/catalogue access, job orchestration, result publication and
bounded analytics. Use explicit Rust module/crate interfaces and database ownership;
do not turn every crate in ADR 0001 into a separately deployed service.

| Role | P1 process boundary and responsibility | Later scaling boundary |
|---|---|---|
| Web UI / Rust CLI/SDK | Calls shared API; canonical TOML/scenario contracts | Static web hosting and API replicas; no direct database/queue access |
| Rust API/orchestrator | Private ingress, validation, durable jobs/outbox, leases, cancellation and manifest publication | Same build with API/controller roles; replicas coordinate through DB leases |
| Worker host | Private Rust child process; loads immutable inputs, builds sparse models, executes rolling pipeline and independently checks residuals | Standalone worker-host role on remote nodes with scoped control channel |
| Native solver workers | Subprocesses owned/reaped by worker host; one native session/handle owner, bounded versioned IPC | Resource-labelled CPU/GPU/licensed pools, with the same solver contract |
| PostgreSQL/Redis/blob store | Separate runtime services/durable volumes | Dedicated/managed nodes, replication/backups and object storage after qualification |

The worker host keeps matrix construction and scans off the API runtime. Reuse
ADR 0001's domain/scenario/network/market/engine/API crates; role entrypoints are
packaging proposals, not a new workspace or code in this PR. The scientific rolling
pipeline lives in engine/commitment/redispatch/pricing modules, while service job
orchestration owns retries/leases/publication. No native solver handle lives in the
API process or is shared across async threads (ADR 0003).

Use bounded local IPC with control messages and checksummed Arrow/file descriptors
for large inputs/results; do not serialise a matrix into Redis or execute one RPC
per coefficient. Local artifact access is confined to the run's workspace. Remote
hosts later exchange the same coarse-grained task/result envelopes over authenticated
transport, using authorised blob references rather than host-local paths. gRPC is
an option then, not a required internal microservice mesh for P1.

## PostgreSQL jobs, Redis delivery and fencing

Keep durable jobs, attempt history, runnable dependencies, cancellation intent,
leases and checkpoints in PostgreSQL. Redis Streams delivers small notifications
and live progress; it is not the authoritative state machine. Use a maintained
Streams client, PostgreSQL transactions and SKIP LOCKED [S1], not a custom broker.

1. Validate and authorise the frozen scenario/manifest. In one PostgreSQL transaction,
   create a run/job and an outbox event. A client idempotency key is unique within
   owner scope; reuse with different input fails. Record the selected backend and
   capability check; unavailable solvers never silently fall back.
2. A relay claims outbox rows, emits a notification containing scope/job/event IDs
   to Redis, then marks delivery. A crash between send and mark may duplicate it.
   Consumer groups use pending/reclaim/acknowledgement primitives [S2]. Notification
   loss or stream trimming is repaired by scanning durable runnable jobs/outbox.
3. A controller atomically claims an eligible PostgreSQL job, increments a monotonic
   fencing token and records attempt/lease expiry. A second consumer of a duplicate
   cannot start it. Acknowledge Redis after the durable claim or confirmed obsolete
   message, not as proof of successful simulation. Jobs waiting for capacity remain
   durable and discoverable independently of Redis pending entries.
4. Start an authorised worker host. Proposed lease: 60 seconds, renewed every
   10 seconds using database time and the current token; actual timings need failure
   tests. A watchdog remains responsive while native solvers block. Publication,
   heartbeat and completion require matching scope/job/attempt/token and a live lease.
5. A recovery loop reclaims expired leases with a new token and resumes from the
   last committed checkpoint. A late/stalled old worker may still compute but cannot
   publish. Stop its local process group and reserve resources conservatively until
   death is confirmed; lease expiry alone does not free physical RAM/VRAM/licences.

This provides at-least-once notification and conditional, idempotent publication,
not exactly-once execution. The PostgreSQL outbox closes the database/queue dual-write
gap; attempt-specific immutable staging plus DB publication closes the result-store
gap (ADR 0004). No critical lock or completion signal depends only on a Redis key.

On Redis failure, the controller can use the same durable runnable-job scan with
bounded backoff, while live events degrade to polling PostgreSQL. This is an explicit
dispatch mode with identical claims, not a second job semantics. On PostgreSQL failure,
accept no new claims/publication; workers pause at a safe boundary or stop when their
lease cannot be renewed. On blob-store failure, do not advance the checkpoint until
results are durable. Recovery may recompute the uncommitted window.

Externally retain PR #4 states: queued, running, completed, partial, cancelled and
failed. Internally distinguish attempts, retry-wait, cancel-requested, abandoned
leases and validation failures without changing a completed run's scientific status.
Transient infrastructure errors get proposed at most three attempts with bounded
backoff. Invalid data, infeasible models and exhausted UC repair are reported, not
retried blindly. Manual resume records a new attempt and its parent checkpoint.

## Checkpoint and cancellation semantics

A checkpoint commits the **advance boundary**, never the look-ahead end. For ADR
0003's proposed 48 h look-ahead/24 h advance, each durable window commit includes:

- Committed interval coverage, result manifest and numerical validity/certificates;
  original/repaired stage identities, repair count, pricing policy and diagnostics.
- Generator on/off and elapsed state, output, ramp/start history, storage/hydro SoC
  and energy budgets, reserve obligations and all intertemporal market state.
- Annual/seasonal pre-pass identity and boundary targets/water values, dated topology
  cursor, market-rule versions, seeds/RNG state where used and input/model hashes.
- Engine/checkpoint schema, solver build/options/platform identity, attempt/token
  and the hash of the preceding committed checkpoint.

Write/verify immutable artifacts first, then atomically advance the checkpoint and
coverage pointers in PostgreSQL. Unique interval identities prevent duplicate committed
output. Resume only when semantic inputs, state schema and required build/solver
compatibility match; otherwise start a new run or use a separately reviewed migration.
A solver LP basis/MIP start is an optional version-specific acceleration, never the
only durable state or a portable mid-solve checkpoint. A crash during native solve
restarts that window from committed physical state.

Preserve ADR 0003's cooperative cancellation contract: proposed two-second solver
grace and five-second total native process-group kill/reap bound. The API first
persists cancellation intent; worker-host control IPC and its watchdog remain
responsive even during a solve. A supervisor can kill/reap the whole host/solver
tree if the host hangs. Never make a raw native-handle call concurrently from the
API. Race cancellation versus completion through a conditional database transition.
Only already validated and durably committed intervals survive; cancelled/partial
status and retained coverage remain visible. No final checkpoint fabricates the
missing pricing stage, complete-year result or zero unserved energy.

## Resource admission, batches and the path to P4

Start with one active simulation and PR #4's proposed 96 GiB/80%-of-RAM process-tree
ceiling. Reserve capacity for PostgreSQL, OS and API based on actual total use; if
that ceiling leaves insufficient headroom, relocate infrastructure or lower admitted
concurrency, not model fidelity. CPU thread/solver-token/VRAM reservations are explicit;
12 physical/24 logical cores do not justify 24 concurrent MILPs. GPUs are optional
and selected only after installed-device/library/licence checks and real measurements.

| Phase | Deployment and work allocation | New evidence needed |
|---|---|---|
| P1 | Private single runtime node, Compose/system services, one service and isolated worker processes; CLI batch with two independent runs may execute sequentially | Crash/cancel/restart, full-network year, API responsiveness and backup/restore on actual target |
| P2 | Durable sweep parent plus immutable child scenarios; add worker hosts/nodes when measured work requires it | Bounded lazy expansion, fair CPU/RAM admission, stable per-child seeds, aggregate progress and failed-member policy |
| P3 | Multiple worker pools/API replicas and shared private object storage; optional controller role separation | Concurrent claim/fencing, input locality, backpressure and analytics throughput under zonal/scenario comparisons |
| P4 | Authenticated tenant admission, quotas/audit, hardened workers, validated data isolation and independent scale | Threat model, isolation choice, cross-tenant negative tests, licence/hosting clearance, availability/recovery objectives |

Batch parents track member identities/statuses and coverage; cancelling a parent
persists cancellation for unfinished members. Identical input hashes do not authorise
cross-owner result reuse. Sweep children with independent parameters/weather years
can run in parallel. Windows within one run carry commitment/energy state sequentially;
parallel time blocks require ADR 0003's explicit reconciled-boundary algorithm and
bias evidence. More machines do not make that dependency disappear.

From the first schema, carry an owner/tenant scope through IDs, unique/foreign keys,
jobs/outbox, cache entries, object access and logs. P1 has one local owner; no public
tenancy claim. P4 may choose shared-schema row-level security, schema/database-per-
tenant or dedicated deployments (D-39). Keep SQL repository access and blob/queue
adapters behind scope-aware interfaces so that choice changes adapters/deployment,
not market/network/solver contracts. RLS is defence in depth, not a decision already
made; migrations and operational hardening are still substantial work.

Before external exposure, authenticate users/workers, authorise every referenced
manifest/object, allocate short-lived scoped worker access, isolate resource pools,
and validate cancellation/lease/result access across tenants. Never accept arbitrary
native plug-ins, arbitrary SQL, Redis commands or host paths from scenario input.
Do not promise “no rewrite” means no migration: domain contracts and task/result
envelopes survive, while tenancy enforcement, scheduling and operations grow in P4.

## Challenge to the recommendation and alternatives

| Alternative | What it improves | Why it is not the proposed starting point |
|---|---|---|
| All work in the API process | Lowest IPC overhead and deployment count | Native crashes/hangs and long CPU work threaten API and cancellation; conflicts with ADR 0003 isolation |
| PostgreSQL-only dispatch | One durable control store, simpler recovery | A strong alternative for long jobs; retain Redis because D-3 selects it and it supports live state. Extra Streams/outbox complexity must be measured and justified, not assumed faster |
| API + separate orchestration microservice from P1 | Independent releases/failure domains | Extra service versioning/network failures without evidence of separate control-plane load; preserve extraction boundary |
| NATS JetStream / durable workflow engine | Broker redelivery or mature timers/workflow tooling | Additional runtime/licence/SDK/version qualification. Consider when orchestration/retry complexity dominates; no claim that a Rust workflow SDK is production-ready without evaluation |
| Cluster scheduler from P1 | Placement, quotas and standard recovery | More operations; useful when multiple worker nodes/tenant isolation require it, not a prerequisite for one runtime node |
| Modular control service + worker processes (chosen) | Shared domain/API semantics, solver containment, later horizontal job distribution | Shared control-service outage and DB dependency; requires explicit outbox/fencing/checkpoints and bounded analytics |

The choice is supported by ADR 0003's native lifecycle and rolling dependencies,
D-20's Rust boundary, and PostgreSQL/Redis documented primitives. It has **no measured
throughput advantage yet**. Reconsider service extraction if API latency or controller
recovery fails under the qualified workload; reconsider dispatch technology if durable
scan/outbox load or operational complexity becomes the bottleneck. Do not add services
just to spend unconstrained hardware resources.

Before adoption: Q-O-4 disposition, runtime inventory and agreement with PR #4's final
contracts. After baseline: inject duplicate/lost notifications, Redis reset, database
outage, stale leases, worker/host crash, partial upload, checkpoint corruption and
cancel/complete races; verify one committed interval set and honest partial status.
Then qualify full-network year and concurrent API use under the existing gate rules.
No heavy validation is claimed by this documentation proposal.

## Primary evidence

- S1 — PostgreSQL 18 queue-like SKIP LOCKED and its consistency limits: https://www.postgresql.org/docs/18/sql-select.html#SQL-FOR-UPDATE-SHARE
- S2 — Redis 8.10.3 command contracts: [XREADGROUP](https://raw.githubusercontent.com/redis/redis/8.10.3/src/commands/xreadgroup.json), [XACK](https://raw.githubusercontent.com/redis/redis/8.10.3/src/commands/xack.json), [XAUTOCLAIM](https://raw.githubusercontent.com/redis/redis/8.10.3/src/commands/xautoclaim.json).
- PR #4 ADR 0001 workspace and ADR 0003 solver/rolling contract: [pinned source context](../topology.md#design-context-and-evidence).
