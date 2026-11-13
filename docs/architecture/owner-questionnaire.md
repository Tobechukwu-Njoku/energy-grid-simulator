# Architecture owner questionnaire — ADRs 0002/0004/0005/0006

Date: 2026-11-13. All five items are **open proposals**, not recorded owner answers.
This is a companion to the requirements questionnaire: that file belongs to PR #4
and is intentionally unchanged. Q-O IDs are new stable IDs; existing Q-N IDs and
decisions retain their meaning. The coordinator can register these entries in the
main questionnaire after integration. No baseline, implementation or merge approval
is implied by the recommended defaults.

## Q-O-1 — Immutable, dated data and scenario identity

- [ ] Accept ADR 0002's identity, effective-time/knowledge-time and TOML-lock policy?
- **Recommended default:** persistent internal entity IDs, immutable hashed dataset
  releases and patches, sourced effective dates and separate publication/acquisition
  times; preserve the original TOML plus a semantically canonical resolved lock.
  Missing historical knowledge evidence blocks an as-of forecasting claim, while
  an explicitly labelled reconstruction remains possible. Adopt the detailed schema,
  hashing and migration defaults in [ADR 0002](adr/0002-network-and-scenario-data-model.md).
- Alternatives: mutable latest-state tables and floating scenario references
  (simpler ingest, weak reruns and backtests); full event sourcing (strong history,
  more replay and operational complexity).
- Relates to Q-D-5, Q-N-6/Q-N-9 and D-19/D-35/D-56. Data-spike feasibility and
  individual missing/equivalenced assets still go through Q-N-3; this is no scope waiver.

## Q-O-2 — PostgreSQL/PostGIS and Parquet storage boundary

- [ ] Accept ADR 0004's storage, publication and retention defaults?
- **Recommended default:** PostgreSQL 18 with PostGIS after its service/package
  licence review; no TimescaleDB initially. Keep transactional metadata, summaries
  and a bounded query projection in PostgreSQL; full arrays in immutable Parquet
  behind a filesystem/object-store interface. Adopt the detailed retention, backup,
  garbage-collection and pinning defaults in [ADR 0004](adr/0004-storage-topology.md).
- Alternatives: TimescaleDB for measured SQL time-series needs, or a separate
  columnar database for sustained cross-run analytics; each adds deployment and
  licence/operations work. Plain PostgreSQL without PostGIS avoids that extension's
  packaging obligations but gives up mature spatial indexing and operations.
- Relates to Q-C-6, Q-I-6, Q-N-8/Q-N-9 and D-3/D-36. Keep D-36's indefinite
  summaries and pinned full results; the proposed policies do not shorten them.

## Q-O-3 — Redis licence path and the Valkey alternative

- [ ] Approve the proposed Redis licence path, or explicitly amend D-3 for Valkey?
- **Recommended default:** retain Redis 8 as a private, unmodified, separate service;
  assess and record AGPLv3 compliance as the selected path before runtime packaging
  or exposure. Keep the application independent of Redis-specific modules and
  recent commands. No direct customer Redis access or general cache/database API.
- Alternatives: an assessed RSALv2 path or commercial agreement; owner-approved
  Valkey with BSD-3-Clause core, followed by queue/recovery compatibility tests.
  SSPLv1 is available but is not the recommended path for this service.
- Relates to Q-I-5/Q-N-8 and D-3/D-17. This is a software/service assessment, separate
  from dataset clearance. Until disposition, the architecture is proposed and no
  Redis licence exception or Valkey substitution is approved. See [ADR 0004](adr/0004-storage-topology.md).

## Q-O-4 — Service/worker separation and recovery defaults

- [ ] Accept ADR 0005's modular monolith and durable job/checkpoint protocol?
- **Recommended default:** one Rust API/orchestrator service, isolated native solver
  subprocesses, PostgreSQL job leases/outbox/checkpoint authority, Redis Streams
  delivery and live telemetry. Start with one active simulation on the separate
  runtime target; measure before raising concurrency. Adopt the detailed recovery,
  resource-admission and phased scaling defaults in [ADR 0005](adr/0005-execution-topology.md).
- Alternatives: PostgreSQL-only dispatch (fewer moving parts); an API/orchestrator
  service split with a durable broker/workflow engine (independent scaling and
  stronger built-in workflow features, more failure modes and deployment work).
- Relates to Q-H-4/Q-H-6, Q-N-5/Q-N-10/Q-N-12 and D-8/D-17/D-39/D-65.
  This adds no P1 distributed sweep or public tenancy commitment. Final tenant
  isolation and availability targets remain P4 choices, as D-39 requires.

## Q-O-5 — Trusted, versioned market modules before external plug-ins

- [ ] Accept ADR 0006's compiled-in market modules and explicit fidelity inventory?
- **Recommended default:** trusted Rust modules behind a versioned domain contract,
  separate GB and I-SEM rules/ledgers with dated interconnector coordination, P1
  scheduling/pricing and cost-based balancing proxies, P3 zonal/nodal modules.
  Pin rule/data versions and fail on unsupported capabilities. Adopt the detailed
  composition, result-schema and qualification defaults in [ADR 0006](adr/0006-market-design-plugin-model.md).
- Alternatives: a declarative-only rule DSL (less executable extension power),
  or sandboxed WASM/process plug-ins (stronger extension isolation, additional
  runtime/IPC/schema work). Do not load arbitrary third-party native libraries.
- Relates to Q-J-2, Q-N-1/Q-N-2 and D-15/D-26/D-38/D-44/D-55/D-59.
  End-state market replication remains required by D-15; P1 proxies are labelled
  limitations, never a claim of complete GB or I-SEM market replication.
