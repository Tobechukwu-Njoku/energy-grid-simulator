# Independent review of PR #17: architecture ADRs 0002, 0004, 0005 and 0006

_Reviewer: Claude Opus 5.5 (independent review under D-54) · 2026-11-13_
_Scope: PR #17 head `254e0e1f` (`t3/design-adrs-0002-0006`), compared with `main` at `87d57af` (merge base)._
_Context: PR #4 head `dbc6299` (ADR 0001, ADR 0003, draft `04-requirements.md`, `02-questionnaire.md`)._

PR #17 adds six documents: ADR 0002 (data model), ADR 0004 (storage), ADR 0005 (execution), ADR 0006
(market plug-ins), `docs/architecture/topology.md` and `docs/architecture/owner-questionnaire.md`.
No PR #17 file was edited. Line references are to the PR head.

## Verdict

**Must-fix: four items, then merge-ready.** The four ADRs are careful, honest about what is unmeasured,
and consistent with D-1…D-65 and with PR #4's ADR 0001/0003 on almost every point. The core choices are
right: PostgreSQL as the single transactional authority, immutable Parquet for bulk results, no
TimescaleDB, fenced leases with checkpoints at the advance boundary, compiled-in Rust market modules
and a bitemporal data model. The licence analysis is accurate; I checked the primary texts.

The must-fix items are gaps, not wrong choices. The topology has no path for live data ingestion or
scheduled forecast runs, and nothing gives a forecast priority over a research run. So the P2
short-term forecasting goal (D-25, roadmap P2) cannot be added "without a rewrite" as drawn. The
questionnaire also duplicates questions that are already open. One planning assumption cites an
owner statement that is not in the decisions log. The retention and resume rules can also produce a
"completed" run that is missing intervals.

### Must-fix items

1. **M-1:** Add the ingestion path, the scheduled-run trigger and forecast-priority admission to the topology and ADR 0005.
2. **M-2:** Remove duplicate open questions: Q-O-2/3/4/5 overlap Q-C-6, Q-I-5, Q-I-6, Q-H-4 and Q-J-2.
3. **M-3:** Rewrite the "owner clarification" planning floor as a cited `ASSUMPTION:` (rules 1 and 7).
4. **M-4:** Make resume consistent with 90-day expiry. This endorses CodeRabbit's major finding.

## What I checked and how

| Check | Method | Result |
|---|---|---|
| Decision consistency | Read all six files against `03-decisions-log.md` D-1…D-65 and `06-roadmap.md` | Consistent except for the gaps in M-1 and M-3. See the table below |
| PR #4 consistency | Read PR #4 ADR 0001, ADR 0003, FR-11/15/16, NFR-1/5/6/8/9/10/12 and Q-N table at `dbc6299` | Pipeline, 48 h/24 h windows, 2 s/5 s cancel, 96 GiB ceiling, run states and isolation all match. Crate ownership needs reconciling (S-7) |
| PR #4 pin drift | `git diff 1698c4be dbc6299` over `docs/architecture`, `04-requirements.md`, `02-questionnaire.md` | Only the ADR 0001 commit-msg-hook section changed. No effect on PR #17 |
| Question IDs | Searched PR #4's `02-questionnaire.md` for every cited Q-ID and for `Q-O-` | All cited IDs exist. No `Q-O-` collision. Overlaps found (M-2) |
| Licence claims | Fetched the cited primary texts (Redis 8.10.3 `LICENSE.txt`, TimescaleDB 2.32.0 `LICENSE`, `tsl/LICENSE-TIMESCALE`, `CHANGELOG.md`, PostGIS 3.7.0 `COPYING`, Valkey 9.1.3 `COPYING`) | All URLs resolve (HTTP 200). Redis 8 is RSALv2/SSPLv1/AGPLv3 at the user's choice ✓. TSL §2.1(b)/§3.10 Value Added Products prohibit users from modifying the database schema ✓. Changelog: "v2.23 … full PostgreSQL 18 support" ✓ |
| Storage volumes | Order-of-magnitude sizing (below) | The PostgreSQL + Parquet split is sound. Sizing is missing from the ADR (S-3) |
| Failure modes | Traced crash, lease expiry, Redis loss, PG loss, blob loss, cancel/complete race, resume, restore | Mostly sound. Gaps: M-4, S-2, S-5 |
| D-52 | PR diff, three commit messages, PR body | No `Co-Authored-By`, "Generated with", 🤖 or model names |
| Protected files | `git diff --name-status` | Six added files only. `STATUS.md`, the decisions log and PR #4 files untouched ✓ |
| Bot review | CodeRabbit inline comments | Two findings. I agree with both: M-4 (major) and the pre-first-checkpoint case in S-2 |

## Consistency with decisions

| Decision | Verdict |
|---|---|
| D-2 performance, D-48 hardware | ✓ Concurrency is limited by correctness and measurement, not cost. Fidelity is never traded for headroom (ADR 0005 l.140–145) |
| D-3 PostgreSQL 18 + Redis | ✓ Both retained. ✗ The framing "Redis is retained because D-3 selects it" for *dispatch* over-reads D-3 (S-1) |
| D-8 map, batch, API | ✓ One shared API. Sweeps arrive in P2 (matches FR-15.AC3/Q-N-10) |
| D-10/D-20 Rust | ✓ No Python/Julia runtime. DataFusion/Polars for Parquet queries |
| D-13/D-16 GPU, pluggable solvers | ✓ Capability-checked, no silent fallback, consistent with ADR 0003 |
| D-15 pluggable markets | ✓ ADR 0006. Separate `gb`/`isem` identities, P3 zonal/nodal, and the D-15 end state is not relabelled as an exclusion |
| D-17 SaaS, licence-clean | ✓ Owner scope in every key from the first schema. ~ The Redis AGPL path is defensible, but D-17 favours Valkey (S-4) |
| D-19 canonical TOML | ✓ ADR 0002 keeps the original bytes plus a semantic hash and resolved lock |
| D-21 1-minute resolution | ✓ Product calendars are separate from model intervals. ~ Storage effect of 30× more rows is not sized (S-3) |
| D-22 autonomy | ~ Two of the five owner questions are engineering detail the agents can adopt (S-9) |
| D-25 forecasting | ✓ The knowledge-cut model (ADR 0002 l.73–86) is exactly what credible forecast backtests need. ✗ No live-ingest or scheduled-run path (M-1) |
| D-35 Monte Carlo | ✓ Sweep parent and children with per-child seeds. ~ Cross-member reductions are not placed (S-8) |
| D-36 retention | ✓ Summaries kept forever, pins honoured, 90-day expiry. ✗ Resume-after-expiry gap (M-4) |
| D-38 FX | ✓ Native-currency ledgers, one numeraire conversion (ADR 0006 l.108–114) |
| D-39 tenancy deferred | ✓ RLS, schema-per-tenant and dedicated deployments all stay open |
| D-44 phasing | ✓ No P3/P4 work pulled forward |
| D-55/D-59 MVP physics and BM | ✓ DC plus proxies. Cost-based redispatch, no bid replay claim |
| D-56 provenance | ✓ Exactly two tags. Unknown is treated as research-only. Lineage is checked over all ancestors |
| D-57 prices | ✓ Each reference series keeps its own definition. Market Index is not relabelled day-ahead |
| D-60 all GB 132 kV | ✓ Stated (ADR 0002 l.15) |
| D-62 ingest | ✗ The production ingest role (ADR 0001 `crates/ingest`) is absent from the topology (M-1) |
| D-65 container vs target | ✓ Nothing runs in the container. ✗ Cites an unrecorded "owner clarification" (M-3) |

## Must-fix findings

### M-1: The topology has no ingestion, trigger or priority path, so P2 forecasting needs a redesign

`topology.md` l.13–63 shows clients → API → jobs → workers. It has no component that *acquires* data.
Yet ADR 0001 has `crates/ingest`, D-62 says Claude writes production ingest code, and P2 adds
"short-term forecasting with live feeds" (roadmap). Three things are missing:

- **Ingestion role and trust boundary.** The diagram has no outbound network egress. External sources
  include Elexon Insights, NESO, SEMO/SEMOpx, ENTSO-E and weather forecasts. Ingest needs credentials
  (the ENTSO-E token in STATUS) and writes immutable releases through the same manifest protocol as
  results (ADR 0004 l.56–67). Its acquisition time feeds ADR 0002's knowledge time. Without it, the
  ADR 0002 bitemporal model has no writer.
- **Scheduled triggers.** Day-ahead and intraday forecasts are created by a clock, not by an analyst.
  The job model needs a durable schedule (forecast origin → run) that is idempotent per origin.
- **Admission priority.** ADR 0005 starts with "one active simulation" (l.140). NFR-5 allows 48 h for
  a weather year. A day-ahead forecast submitted during a weather-year run would wait up to two days
  and miss gate closure. The P2 row (ADR 0005 l.150, topology l.97) mentions sweeps and fairness but
  not deadlines.

**Fix (documentation only, no P1 scope added):**

1. Add an ingest role and its egress boundary to the diagram and to ADR 0005's role table.
2. Add a P2 line to both phase tables: "deadline-class forecast jobs, durable schedules, and
   preemption of research runs at a committed checkpoint boundary or a reserved forecast slot".
   ADR 0005's checkpoint design already makes preemption cheap: a research run loses at most one
   uncommitted window.
3. Record an `ASSUMPTION:` and a question for the owner: "In P2, do forecast runs take priority over
   research/sweep runs? Recommended: yes, preempt at a checkpoint."

### M-2: Duplicate open questions with different defaults

The companion questionnaire asks again about items that are already open in `02-questionnaire.md`.
Some of the existing items carry different defaults:

| New | Already open | Conflict |
|---|---|---|
| Q-O-2 (PostGIS, no Timescale, Parquet) | Q-I-6 (TimescaleDB *or* ClickHouse/Parquet+DuckDB), Q-C-6 (★ PostGIS) | Q-I-6 stays open with a different option set |
| Q-O-3 (Redis licence path / Valkey) | Q-I-5 (Redis 8 or Valkey) | Same question, two IDs |
| Q-O-4 (one active simulation) | Q-H-4 (concurrency), Q-H-6 (availability) | Q-O-4 implicitly answers both |
| Q-O-5 (no user-uploaded market code) | Q-J-2 (user plug-ins) | Q-O-5 implicitly answers Q-J-2 for market rules |

AGENTS.md rule 2 says to deprecate rather than renumber. **Fix:** give each Q-O entry a line such as
"On disposition, this answers Q-I-5; mark Q-I-5 `[x]` → D-n". Alternatively, answer the old IDs
directly and drop the new ones. **Merge condition for the coordinator:** AGENTS.md makes
`02-questionnaire.md` the register of *every* open question. Register the surviving Q-O items there
when PR #4 lands, as the PR body proposes, so the owner sees one queue.

### M-3: An unrecorded owner statement is used as the planning floor

`topology.md` l.7–9 and ADR 0005 l.17–21 say that "the task's owner clarification sets a planning
floor of the 5900X/128 GB/consumer-NVIDIA setup". That clarification is not in the decisions log or
in STATUS. D-48 records the 5900X as the owner's *Linux machine*. D-65 says the target system's
details are still to come. Rule 1 says never to invent a decision. Rule 7 says chat history is not
project state.

**Fix:** reword to "ASSUMPTION: plan against D-48's 5900X/128 GB until the D-65 target is specified".
If the owner really did state a floor, the coordinator should log it as a decision first.

### M-4: Resume can produce a "completed" run with expired intervals (CodeRabbit, major)

ADR 0004 l.89 expires an unpinned run's results, *including retained partial results*, 90 days after
it becomes terminal. But l.91 keeps checkpoints "while needed for an active/resumable run". ADR 0005
l.104 allows a manual resume of a partial or cancelled run from its parent checkpoint. Suppose a run
is resumed on day 100. Its first windows' Parquet files are gone, the new attempt commits the rest,
and coverage ends with a gap. That breaks FR-11.AC1 ("exactly one result per delivery interval") and
NFR-9.AC2 (the annual completeness assertion).

**Fix:** pick one rule and state it in both ADRs. Recommended: a partial or cancelled run's checkpoint
expires together with its results under the same clock. Resume first verifies that every previously
committed interval's objects still exist. If they do not, it refuses and offers a new run from that
checkpoint's *state*, which has a new run ID and its own coverage.

## Should-fix findings

**S-1: Keep Redis off the P1 dispatch path (ADR 0005 l.60–97, l.181; topology l.88–90).** D-3 puts
Redis in the stack. It does not require Redis to carry job dispatch. The ADR admits that PostgreSQL-only
dispatch is "simpler for long jobs" and that Redis is used only "because D-3 selects it". With jobs
lasting hours, notification latency is irrelevant. Redis Streams adds an outbox relay, a
duplicate-delivery path and a second "Redis-down" dispatch mode, and each needs failure tests.
Recommendation: in P1, claim jobs with PostgreSQL `SKIP LOCKED` and wake workers with
`LISTEN/NOTIFY`. Use Redis for live progress, recent events, presence, rate limits and caches, which
still satisfies D-3. Keep Streams as a measured P3 option. At minimum, correct the framing so that
Q-O-4 does not present D-3 as the reason.

**S-2: Make the worker host a separately supervised service in P1 and define lease ownership.**
ADR 0005 l.40 makes the worker host a *child process* of the API/orchestrator. Under a default systemd
or Compose supervisor, restarting or redeploying the API kills every running multi-hour simulation.
Checkpoints limit the loss, but routine API deploys should not interrupt science. Run the worker host
as its own unit from P1; the cost is small, and it is also the P2 shape. Three related gaps:

- State whether the worker host or the controller renews the lease (l.78–81).
- Say what kills a still-running worker when its lease is lost.
- Define recovery before the first checkpoint (CodeRabbit minor). Recommended: commit a genesis
  checkpoint after the stage-0 pre-pass, holding the pre-pass result, so resume never re-runs the
  annual LP with a different outcome.

**S-3: Add a sizing table and place the inputs explicitly (ADR 0004).** The ADR's case rests on volume
(l.11) but gives no numbers. My order-of-magnitude estimate assumes ~5,000 buses, ~8,000 branches and
~1,500 units. These are ASSUMPTIONS pending the D-31 inventory. A 30-minute year has 17,520 intervals.

| Workload | Values (long format) | PostgreSQL row-per-value (~100 B incl. index) | Parquet (compressed, to be measured) |
|---|---|---|---|
| One weather-year run, 30 min (6 unit, 2 branch, 2 bus quantities) | ≈ 0.6 B | ≈ 60 GB | low single-digit GB |
| Same at 1 minute (D-21) | ≈ 18 B | ≈ 1.8 TB | tens of GB |
| 40-weather-year Monte Carlo sweep, 30 min (D-35, P2) | ≈ 25 B | ≈ 2.5 TB | ≈ 50–150 GB |
| BOD input, ≈ 55 M rows/yr; ~24 years of history | ≈ 1.3 B rows | ≈ 130 GB | a few GB per year |
| P2 distribution (buses ×10–100) | ×10–100 | not viable | viable with partitioning |

Conclusion: the split is **sound**. PostgreSQL row-per-value is fine for one P1 run but fails at sweep
and 1-minute scale. Parquet keeps both cheap. D-36's 90-day expiry bounds the unpinned working set
(P1 runs sequentially, so about 45 runs per 90 days, roughly 100–150 GB). Three additions are needed:

- State that bulk *inputs* use Parquet as their canonical form, with the PostgreSQL manifest pointing
  to them. These include BOD/PN/BOA, metered data and demand. ADR 0002 l.27 ("store validated releases
  relationally") reads as if they live in PostgreSQL.
- Add `sweep member` to the logical partition key so that cross-member reads (LOLE per bus across
  years) do not scan every file.
- Note that gridded weather (ERA5) may suit Zarr/NetCDF better than Parquet. Decide that in the data
  spike, not here.

**S-4: Reconsider the Q-O-3 default.** The AGPLv3 path is legally sound for unmodified, separately run
Redis, and the ADR's reading of §13 is correct. But D-17 asks for licence-clean dependencies, and
D-33's funders and clients will run due diligence where AGPL is always a flag. The ADR already limits
the application to commands that Valkey shares, so switching costs nothing now and more later. I
recommend that Q-O-3 present Valkey (BSD-3-Clause) as the recommended default, as an explicit
amendment of D-3, and keep AGPL Redis as the alternative. This is the owner's call. Add one licence
note: TimescaleDB's TSL features are generally unavailable on hyperscaler managed PostgreSQL. This
supports "no Timescale" and keeps P4 hosting options open.

**S-5: Clarify "topology events split the solve horizon" (ADR 0002 l.69–71).** GB has thousands of
planned outages a year. If every switching or outage event split a rolling window, ADR 0003's 48-hour
look-ahead would break into fragments. The intent appears to be one topology state per interval
inside a window, with a sparse factorisation or PTDF cached per distinct topology state. Say that,
and leave window boundaries to ADR 0003.

**S-6: Simplify numeric canonicalisation (ADR 0002 l.139–140).** "Decimal coefficients normalised
without lossy floating-point parsing" conflicts with "use maintained TOML libraries" and with the TOML
specification, which defines floats as IEEE-754 binary64. Hash the binary64 value, using shortest
round-trip formatting or raw bits, and reject NaN and ±inf as planned. Decimal exactness is only
needed for money, and money is not authored as TOML floats. If decimal is kept, name the dependency
(for example `toml_edit` raw representations) and add golden vectors for `0.1` and `0.10`.

**S-7: Reconcile crate ownership with ADR 0001.** ADR 0001 l.43 gives `crates/engine` "orchestration,
worker lifecycle". ADR 0005 l.47–48 gives leases, retries and publication to "service job
orchestration" but names no crate. No crate is named for the storage or blob adapter either. Name
them (for example `crates/jobs` and `crates/store`), or say which existing crate holds them. Then
amend ADR 0001's engine row so that the two ADRs do not both claim the worker lifecycle.

**S-8: Place sweep reductions and define how pins cascade.** Cross-member aggregates such as LOLE/EUE
(D-35) and paired market comparisons (P3) scan billions of values. They should run as worker jobs that
publish a summary manifest. They should not run as "bounded analytics" inside the control service,
which ADR 0005 l.44 keeps free of scans. Also state whether pinning a sweep parent pins its children.
Under D-36, unpinned children otherwise expire at 90 days while the parent summary remains.

**S-9: Trim the owner questions to the owner-level choices.** Q-O-2 and Q-O-4 ask the owner to accept
engineering protocols such as 60 s leases, a 24 h orphan grace and partition keys. D-22 delegates that
level of detail to the agents, and ADR acceptance already covers it. The owner-level content is:

- Q-O-3: Redis versus Valkey, which amends D-3.
- From Q-O-1: "backtests are labelled *reconstruction* unless publication-time evidence exists". This
  changes which forecast-accuracy claims can be made to funders.
- From Q-O-5: "customers cannot upload market code; only typed parameters". This answers Q-J-2 and
  shapes the SaaS offer.
- From M-1: forecast priority.

The defaults are otherwise sensible.

**S-10: Restore and GC protocol (ADR 0004 l.96–110).** "Tombstones survive restoration" needs a
mechanism. A PostgreSQL PITR restore to time T loses tombstone rows written after T. A tombstone
journal is needed outside PostgreSQL, for example an append-only log in the blob store. In the other
direction, a restore to T can reference objects that GC removed after T. Delay physical blob deletion
by at least the 30-day backup window, or include blob versions in the backup. Either way, state the
restore order: database, then tombstone replay, then a blob reference check, then access.

## Minor notes

- n-1: Use UUIDv7 for entity, run and attempt IDs for B-tree locality. The ADR says only "opaque UUID"
  (ADR 0002 l.33).
- n-2: Observability (metrics and traces) has no place in the topology. Logs are covered by retention.
  Put it in the deployment ADR later.
- n-3: Label the diagram node "Redis-protocol store (Redis 8 per D-3)" so that a Q-O-3 decision does
  not require editing the diagram.
- n-4: P2 adds Ipopt-class AC/NLP solves (D-44). These fit the same native-worker and capability model.
  One sentence in ADR 0005 would confirm that.
- n-5: In P2 the API publishes forecasts to paying customers (D-41), so the single control-plane
  failure domain (topology l.85) becomes a delivery risk. Link Q-H-6 to the P2 row.

## What is good

- ADR 0002's three times (effective, publication, acquisition) and the rule that "unknown publication
  history blocks an as-of claim" are the right foundation for credible forecast backtests (D-23, D-33).
- ADR 0005 is precise about guarantees: at-least-once notification, fenced idempotent publication and
  no exactly-once claim. Checkpoints commit the advance boundary, not the look-ahead end (FR-11.AC1).
  Lease expiry does not free RAM or VRAM until the process is confirmed dead.
- ADR 0004 rejects TimescaleDB for workload-fit reasons, not as a benchmark claim. It separates
  software licences from dataset rights (D-56). The publication protocol (stage, verify, then one
  database transaction) closes the cross-store gap correctly.
- ADR 0006's fidelity inventory (`implemented`/`proxy`/`deferred`/`unsupported`) prevents P1 proxies
  from being sold as the D-15 end state. The single coordinated solve avoids double-allocating
  interconnector capacity.
- Every operational number is labelled as a proposal, and nothing claims a measurement.
