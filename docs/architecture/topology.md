# Proposed system topology

Date: 2026-11-13. Status: **Proposed**; requirements remain unbaselined. This is an
architecture/handoff document, not permission to implement. The coding container is
for editing/coordination only; databases, solvers and qualification belong on the
separate runtime target, whose specification is still TBD (D-65).

ASSUMPTION: D-48's 5900X/128 GB is a provisional planning floor pending the D-65
target inventory, tracked under Q-H-7 in the canonical questionnaire. It is not a
confirmed target or GPU specification; container resources establish neither.

## Process, data and trust boundaries

```mermaid
flowchart TB
  sources[External operators and weather feeds]
  subgraph clients[Analyst clients]
    web[TypeScript web UI]
    cli[Rust CLI and SDK]
  end
  subgraph target[Separate runtime target - private P1]
    subgraph control[Rust API service]
      api[API and scenario catalogue]
      submit[Submit and cancel intent - bounded metadata reads]
      api --> submit
    end
    subgraph acquisition[Ingest role - crates/ingest]
      ingest[Source adapters - scoped credentials - validation]
    end
    schedule[P2 durable forecast scheduler]
    pg[(PostgreSQL 18 - optional PostGIS)]
    cache[(Valkey - Redis protocol - live state and cache)]
    blobs[(Private staging and immutable object data)]
    subgraph supervision[Independent worker supervisor service]
      jobs[PG claims - priority - reservations - leases]
      watch[Reserved heartbeat and kill-reap watchdog]
      publish[Verify - pending manifest - promote - make available]
      jobs --> watch
    end
    subgraph host[Contained run attempt process tree]
      engine[Rolling pipeline and sparse model build]
      native[Isolated native solvers - HiGHS default]
      validate[Independent residual and coverage checks]
      engine -->|Versioned transport interface| native
      native --> validate
    end
    analytics[Admitted analytics and sweep reduction workers]
    gc[Reconciler and staging sweeper]
    submit -->|Run and cancel transactions| pg
    schedule -->|Origin-idempotent deadline jobs| pg
    jobs -->|SKIP LOCKED - leases - reservations| pg
    pg -.->|NOTIFY hint and durable scan| jobs
    jobs -->|Fenced task - checkpoint or preempt| engine
    watch -->|Contain - kill - confirm death| host
    pg -->|Frozen manifest references| engine
    blobs -->|Versioned inputs| engine
    validate -->|Checksummed staging objects| blobs
    validate -->|Candidate manifests| publish
    publish -->|Pending then available commit| pg
    publish -->|Promote and verify final objects| blobs
    engine -->|Non-authoritative progress| cache
    cache -->|Cache and recent events| api
    ingest -->|Stage validated releases| blobs
    ingest -->|Manifest and knowledge evidence protocol| pg
    pg -->|Qualified jobs and summaries| analytics
    blobs -->|Budgeted scans| analytics
    analytics -->|Staged reduction publication| publish
    gc -->|Publication and tombstone state| pg
    gc -->|Reconcile and reap eligible staging| blobs
  end
  ingest -->|Allowlisted egress only| sources
  sources -->|Acquired data with publication evidence| ingest
  web -->|Shared private API| api
  cli -->|Shared private API| api
```

PostgreSQL is the sole job/metadata authority; P1 dispatch uses SKIP LOCKED/leases
and durable scans, with NOTIFY only a hint. Valkey is the proposed BSD-3-Clause
default; Redis is a compatible alternative. Application clients use only their
shared Redis protocol. Neither service dispatches jobs or owns completion.

The API and worker supervisor have separate service lifecycles from P1. API restarts
do not kill runs. Before lease reclaim, confirm the old attempt tree is dead, then
recheck and reserve memory before starting a successor. Heavy analytics run in
admitted workers outside API/supervisor; heartbeats retain dedicated headroom.

Ingest (`crates/ingest`, D-62) is the acquisition writer for ADR 0002's effective,
publication and acquisition times. It owns external egress/credentials, validates
D-56 lineage and stages immutable releases through ADR 0004's publication contract.
Historical ingest is P1; continuous feeds and a PostgreSQL-backed forecast-origin
scheduler are P2. Missing vintages/deadlines are explicit, not silently replaced.
Forecast priority and checkpoint/preemption fields fit the job model from the start.

Bulk inputs/results and checkpoints use staging → committed pending manifest →
promotion → verified available pointers. APIs see only available manifests. A
periodic N-hour staging sweeper (proposed N=24) uses listings for cleanup, never as
a commit protocol. PostgreSQL pin/tombstone transitions precede object deletion.
Resume verifies all prior interval objects; expired state requires a new run from
the start. No completion is possible with missing requested intervals.

The transport interface uses versioned envelopes and authorised blob references;
local fd passing is an optional adapter. Network transport can use the same engine
contract. Native/licensed/GPU capabilities need explicit qualification, not silent
backend substitution. The UI never directly accesses stores or native workers.

## Why this shape, and what changes as it grows

D-22 delegates engineering defaults to agents; the four ADRs remain Proposed until
accepted. Only four product choices are in the
[owner questionnaire](owner-questionnaire.md), registered in the canonical queue.
No new decision is recorded here.

A modular API keeps domain/interface semantics together. Independent supervision
contains native failures and preserves long runs across API deployments without
turning every crate into a microservice. PostgreSQL-only dispatch avoids a redundant
broker relay; Valkey remains useful for rebuildable live state. Cross-store publication,
physical fencing, checkpointing and backup/GC still have real operational costs.
No throughput or latency advantage is claimed without target qualification.

| Phase | Stable contracts | Additions / remaining evidence |
|---|---|---|
| P1 one target node | Canonical TOML/releases, PG jobs, separately supervised workers, manifest/checkpoint state | Private access, one simulation reservation initially; historical ingest; bounded worker analytics; target inventory/qualification |
| P2 forecasts and sweeps | Same origin/child run identities, physical state, priority/preemption fields and transport interface | Continuous feeds, durable forecast schedules, deadline class preempting research or reserved capacity; authenticated worker nodes and fair research resumption |
| P3 shared services | Market modules, solver IR, immutable formats and task envelopes | API replicas/pools and qualified shared objects; optional control extraction after measurements |
| P4 tenant service | Scope-aware catalogue/jobs/blobs and provenance | Owner-selected isolation, auth/quotas/audit, scoped credentials and availability objectives; decide customer-code policy |

“Without a rewrite” preserves scientific/domain/task contracts as transport and
scheduling adapters evolve. It does not promise free migrations, already implemented
tenancy or deadline success with missing feeds/insufficient compute. On a constrained
host, forecast preemption kills/reaps research before admission; it never overlaps
two reservations beyond the RAM budget.

## ADR navigation and owner disposition

| ADR | Proposed choice | Owner product disposition |
|---|---|---|
| [0002 — Data model](adr/0002-network-and-scenario-data-model.md) | Permanent IDs, dated immutable releases, resolved TOML and explicit numerical identity | Q-O-1 answers Q-D-5 for historical/repeatability claims |
| [0004 — Storage](adr/0004-storage-topology.md) | PostgreSQL 18, optional P1 PostGIS, Valkey live state, Parquet bulk data; no initial TimescaleDB | Q-O-3 answers Q-I-5; Q-O-2 deprecated as engineering detail |
| [0005 — Execution](adr/0005-execution-topology.md) | Separate API/supervisor, PG dispatch, physical fencing, ingest and schedules | New Q-O-6 forecast priority; Q-O-4 deprecated as engineering detail |
| [0006 — Markets](adr/0006-market-design-plugin-model.md) | Trusted Rust modules, distinct GB/I-SEM policies, reported repair fallback and P3 alternatives | Q-O-5 answers Q-J-2 for customer market-code uploads before P4 |

Exactly four open choices remain; stable Q-O-2/Q-O-4 are deprecated, not renumbered.
Existing Q-C-6/Q-I-6, Q-H-4/Q-H-6 and PR #4 Q-N accuracy/data-gap choices keep their
existing scope. Target inventory clarification is tracked in Q-H-7.

## Design context and evidence

Main has been merged for revision; review reports are now local. Scientific context
is pinned to PR #4 branch
`t3/design-review-mvp-requirements-adrs` at
`dbc6299821b8711ca0d8253b27350d1094c0bbfa`, fetched on 2026-11-13. These are evidence
snapshots, not a dependency on merging that branch into this one. The coordinator
has requested alignment with PR #4 round-3 fallback policy; ADR 0006 states the
integration contract. The pinned snapshot still says repair exhaustion fails/partials
a run, so reconcile ADR 0003 at integration rather than claim that revision is already
present. Do not copy PR #4 files into this PR.

| Source | Architectural consequence |
|---|---|
| [Decisions D-1–D-65](../requirements/03-decisions-log.md) | Owner authority; no invented data/tenancy/market decisions |
| [PR #4 proposed requirements](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/blob/dbc6299821b8711ca0d8253b27350d1094c0bbfa/docs/requirements/04-requirements.md) | Dated/as-of inputs, manifests, staged P1 fidelity, retention and honest validation limits |
| [PR #4 ADR 0001](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/blob/dbc6299821b8711ca0d8253b27350d1094c0bbfa/docs/architecture/adr/0001-repo-layout-and-ci-gates.md) | Existing workspace/crate responsibilities and post-baseline quality gates |
| [PR #4 ADR 0003](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/blob/dbc6299821b8711ca0d8253b27350d1094c0bbfa/docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md) | Pre-pass/UC/redispatch/pricing, native isolation, rolling state, explicit capability/cancellation rules |
| [Stack candidates](stack-candidates.md) and [prior art](../research/prior-art.md) | Candidate/reuse evidence only; no automatic adoption, especially OSM/share-alike datasets prohibited by D-56 |
| [Dependency-licence research](../research/dependency-licences.md) | Separate software/service terms and dataset permissions; release/source details in ADR 0004 |

Primary evidence establishes available mechanisms and licence texts, not measured
scale, installed runtime hardware, legal clearance, scientific accuracy or completed
implementation. Architecture qualification plans are deliberately post-baseline.

## Coordinator handoff

Revisions and finding dispositions are recorded in the
[PR #17 response](../requirements/reviews/response-pr17-2026-11-13.md).
The canonical questionnaire now registers the four surviving product choices and
mapped/deprecated IDs. STATUS.md and the decisions log are untouched by instruction;
the coordinator owns the session status update and future owner-decision entries.
No application code was written. The latest PR #4 scientific fallback and crate
ownership wording still need integration reconciliation; this branch does not edit
ADR 0001/0003. Independent review disposition and owner adoption remain prerequisites
to baseline/merge; no runtime qualification is asserted here.
