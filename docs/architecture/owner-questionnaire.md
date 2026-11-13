# Architecture owner questionnaire — ADRs 0002/0004/0005/0006

Date: 2026-11-13. Four **answered product choices**, citing D-67–D-70.
The [canonical questionnaire](../requirements/02-questionnaire.md#o-architecture-product-choices-pr-17)
registers these IDs. This companion supplies rationale/options for that same queue,
not a second set of competing defaults. The mapped questions are answered only to
the scope of those decisions. Alternatives below are retained as design history,
not pending choices. Engineering protocols are
proposed ADR defaults under D-22, not additional owner questions.

## Q-O-1 — Honest historical backtest and repeatability claims

- [x] **Answered → D-70; answers Q-D-5** for the reproducibility/claim contract; historical data gaps
  still use the existing data-gap process, not a new scope waiver.
- **Owner answer (D-70):** label historical backtests **reconstruction** when source
  publication times/vintages cannot prove the knowledge cut. Permit historical research
  with that label; reserve **as-of** for evidence-backed vintages. Pin inputs, solver
  seeds/thread counts/versions and persisted state; disclose that MIP reruns/resumes
  promise declared numerical tolerances, not bit-identical schedules or prices.
- Alternatives: reject backtests lacking historic vintages (less usable research);
  require qualified bit identity for every accepted backend/platform (restricts solver
  capabilities and still does not establish historical information availability).
- [ADR 0002](adr/0002-network-and-scenario-data-model.md) and
  [ADR 0005](adr/0005-execution-topology.md) define the evidence and execution contract.
  Based on D-19/D-25/D-35/D-56; publication evidence is never invented.

## Q-O-3 — Valkey default, Redis-compatible alternative

- [x] **Answered → D-67; answers Q-I-5** and refines Q-I-1/D-3’s former Redis baseline.
- **Owner answer (D-67):** Valkey (BSD-3-Clause core); application clients use only
  the shared Redis protocol. Redis is a compatible alternative with a separately
  recorded licence/deployment assessment. P1 job dispatch stays in PostgreSQL.
- Alternatives: Redis 8 with assessed AGPLv3/RSALv2/commercial terms. Its independent
  client does not automatically become copyleft, but the service/package assessment
  is more complex. Neither choice excuses bundled-code notice or compatibility checks.
- [ADR 0004](adr/0004-storage-topology.md); D-3/D-17/D-22/D-67. The owner answer
  selects the service; storage engineering details still require ADR acceptance.

## Q-O-5 — Customer market code deferred until P4

- [x] **Answered → D-69; answers Q-J-2 for customer market-code uploads before P4**; other plug-in types
  and the P4 extension/isolation decision remain in that existing question's scope.
- **Owner answer (D-69):** no customer-uploaded market code before P4. Offer first-party,
  reviewed compiled Rust modules and typed versioned TOML parameters. Consider
  third-party code only with P4 tenant isolation, resource and capability qualification.
- Alternatives: introduce a sandboxed WASM/process extension earlier (adds runtime,
  security and API qualification); a declarative-only interface (less algorithmic power).
  Arbitrary native library loading is not proposed.
- [ADR 0006](adr/0006-market-design-plugin-model.md); D-15/D-20/D-39/D-44. This does
  not reduce the end-state obligation to model GB/I-SEM and alternative markets.

## Q-O-6 — Scheduled forecasts preempt research in P2

- [x] **Answered → D-68; product policy**, related to Q-H-4 concurrency and Q-C-4 live feeds.
  It does not answer target concurrency counts or availability/HA (Q-H-6).
- **Owner answer (D-68):** P2+ forecast runs preempt research/sweep runs. Prefer a committed
  checkpoint boundary when it meets the deadline; otherwise kill/reap and discard the
  uncommitted window, resuming research later. Qualified reserved forecast capacity
  is another way to enforce this priority. A day-ahead forecast never queues behind
  a long research run merely because it arrived later.
- Alternatives: dedicated forecast hardware only (capacity must actually be available);
  FIFO with no preemption (can miss the forecasting product's gate/deadline).
- [ADR 0005](adr/0005-execution-topology.md); D-8/D-25/D-35/D-44. Priority is not a
  guarantee against missing feeds or insufficient solve capacity.

## Deprecated IDs and engineering disposition

Q-O-2 is **deprecated**, not renumbered: PostgreSQL authority, Parquet layout,
publication, pin/GC/recovery and no initial TimescaleDB are D-22 engineering proposals
in ADR 0004. This ADR supplies an engineering recommendation to existing Q-I-6;
optional P1 PostGIS remains consistent with Q-C-6 and the baseline. Neither existing
product question is silently marked answered by accepting operational timings.

Q-O-4 is **deprecated**, not renumbered: independent worker supervision, PostgreSQL
SKIP LOCKED/leases, physical fencing, resource budgets and transport adapters are
D-22 engineering proposals in ADR 0005. Q-H-4/Q-H-6 keep their existing product scope.
The D-48 hardware floor is an ASSUMPTION pending the D-65 inventory clarification
tracked under Q-H-7, not an additional architecture owner choice.

Lease durations, N-hour staging sweeps, backup windows, partition sizes, canonical
encoding and solver configuration are reviewed with the ADRs and baseline. They are
not extra owner questions. All four ADRs remain Proposed; no implementation gate opens.
