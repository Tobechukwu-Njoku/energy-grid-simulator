# Focused re-check of PR #17, round 2: topology ADRs 0002, 0004, 0005 and 0006

_Reviewer: Claude Opus 5.5 (independent re-check under D-54) · 2026-11-13_
_Scope: PR #17 head `d99ceabb` (`t3/design-adrs-0002-0006`), merge base with `main` `959a243`._
_Items checked: [Claude review](claude-review-pr17-2026-11-13.md) (M-1…M-4, S-1…S-10),
[Gemini adversarial review](gemini-adversarial-pr17-2026-11-13.md) (ADV17-01…14), both
CodeRabbit inline comments, and the author's response
(`response-pr17-2026-11-13.md` on the PR branch)._
_PR #4 context: the pinned snapshot `dbc6299` that PR #17 cites, and the current PR #4 head
`52fda46` (round 3)._

This is not a second full review. I checked whether each item is resolved in the text at
`d99ceabb`, whether the revision introduced regressions, and whether it now contradicts
D-1…D-65 or PR #4. Line references are to `d99ceabb`.

## Verdict

**Must-fix: one item (R-1). After that, the PR is merge-ready.**

All four original must-fix items, all ten should-fix items, all 14 adversarial findings
and both CodeRabbit comments are resolved in the ADR text, not just in the response.
I found no contradiction with D-1…D-65. The revision did not regress any earlier finding.

The one remaining defect is a deduplication gap of the kind M-2 was meant to close. PR #17
recommends ★ Valkey under Q-O-3. The PR #4 snapshot that PR #17 itself pins
(`dbc6299`) proposes the opposite default in four places, and PR #17 does not
acknowledge any of them. If both PRs merge as they stand, the owner will see two
opposing recommendations on the same subject. Q-O-1 overlaps PR #4's Q-N-6 in the same
way, but there the two defaults agree. The fix is a few lines of mapping text.

### R-1 (must-fix): Map Q-O-3 and Q-O-1 to PR #4's Q-N-8, NFR-8/NFR-10 and Q-N-6

| PR #4 text (present at `dbc6299` and `52fda46`) | PR #17 position |
|---|---|
| Q-N-8 proposed default: "Retain Redis/PostgreSQL 18; no unapproved Valkey or TimescaleDB adoption" | Q-O-3 ★ Valkey; ADR 0004 title and table name Valkey as the live-state store |
| NFR-8.AC4: "no Valkey substitution is authorised by this draft" | ADR 0004 l.19–22 recommends Valkey |
| NFR-10 statement "PostgreSQL 18 plus Redis infrastructure"; NFR-10.AC4 "ADR 0004 remains deferred" | ADR 0004 now proposes the store roles |
| ADR 0001 l.53: "PostgreSQL 18 and Redis remain the baseline" | Topology diagram l.32 labels the node Valkey |
| Q-N-6 proposed default, "relates Q-D-5": manifest-level reproducibility within NFR-2 tolerances, not identical MILP schedules | Q-O-1 "answers Q-D-5" with the same repeatability policy |

None of these is a contradiction with an owner decision. Q-O-3 explicitly gates the
Valkey change on owner disposition and does not rewrite D-3, and that is correct. The
problem is that the questionnaire would hold two proposed defaults with opposite answers,
one of which (Q-N-8) agents may adopt under D-22. The PR #17 sentence "PR #4 Q-N
accuracy/data-gap choices keep their existing scope" (topology l.143) does not cover
Q-N-6 or Q-N-8.

**Fix (PR #17 files only; PR #4 files are not edited):**

1. In the Q-O-3 entries (`02-questionnaire.md` section O and `owner-questionnaire.md`),
   add: "On disposition, also resolves the Redis/Valkey clause of PR #4 Q-N-8 and the
   matching NFR-8.AC4, NFR-10 and ADR 0001 wording. Until then D-3's Redis stands, and
   ADR 0004's Valkey row is conditional on this answer."
2. In ADR 0004 (l.19–22), state the same condition.
3. In the Q-O-1 entries, add: "The repeatability half matches PR #4 Q-N-6. One
   disposition covers both, and Q-N-6 is not a separate owner question."
4. Add these mappings to the coordinator handoff (topology l.171–181 and the response's
   integration section), so that the PR #4 text is updated at integration.

Once these lines are in, a spot check of the diff is enough. A new full review is not needed.

## Owner questions: count, deduplication and ★ defaults

| Check | Result |
|---|---|
| Count | ✓ Exactly four open choices: Q-O-1, Q-O-3, Q-O-5 and Q-O-6. Q-O-2 and Q-O-4 are deprecated, not renumbered (AGENTS.md rule 2) |
| ★ default | ✓ All four carry **★ Recommended default** in the canonical `02-questionnaire.md` section O. The companion `owner-questionnaire.md` says "Recommended default" without the ★ glyph (nit, n-1) |
| Registered in the canonical queue | ✓ Section O of `02-questionnaire.md`, with back-references from Q-D-5, Q-I-5 and Q-J-2 |
| Deduplicated within the repository's own questionnaire | ✓ Q-O-1 answers Q-D-5; Q-O-3 answers Q-I-5; Q-O-5 answers Q-J-2 for pre-P4 market code; Q-O-6 is new and scoped against Q-H-4, Q-H-6 and Q-C-4 |
| Deduplicated against PR #4 | ✗ Q-O-3 and Q-N-8 have conflicting defaults; Q-O-1 and Q-N-6 overlap with the same default (R-1) |
| Owner-level, not engineering detail (S-9) | ✓ All four are product or claim choices. Leases, sweeper period, backups and encodings are D-22 engineering defaults |

Coordinator note: PR #4's questionnaire header declares its own active owner queue
(Q-N-4, and Q-N-3 conditionally). Together with PR #17, the owner would face up to six
open items. Q-O-5 (a P4 policy) and Q-O-6 (a P2 policy) do not block the P1 baseline,
so the coordinator can sequence them after the P1 questions if the owner wants a
shorter queue. This is not a PR #17 defect.

## Must-fix items from round 1

| Item | Status | Evidence at `d99ceabb` |
|---|---|---|
| M-1 ingest, schedule trigger, forecast priority | ✓ Resolved | Ingest role with allowlisted egress and scoped credentials: diagram l.27–29 and l.71–72, ADR 0005 role table l.47, l.78–85. Durable origin-idempotent P2 scheduler: ADR 0005 l.87–93. Preemption at a timely checkpoint, or kill/reap and replay the uncommitted window: ADR 0005 l.95–104. P2 rows in both phase tables: topology l.123, ADR 0005 l.239. `ASSUMPTION:` plus new Q-O-6. Live feeds stay in P2; only job-model fields land in P1 (no D-44 pull-forward) |
| M-2 duplicate questions | ◐ Resolved within the repository; cross-PR gap remains (R-1) | Q-O-2 and Q-O-4 deprecated; each surviving Q-O entry names the question it answers |
| M-3 unrecorded planning floor | ✓ Resolved | `ASSUMPTION:` citing D-48/D-65 at topology l.8–10, ADR 0005 l.17–23 and ADR 0004 l.245–246. No "owner clarification" text remains. One wording issue in S-b below |
| M-4 resume after 90-day expiry | ✓ Resolved | One rule in both ADRs: ADR 0004 l.136 and l.141–149; ADR 0005 l.205–215. Checkpoints expire with results. A resume hold through the deletion state machine stops GC racing the checks. Every prior interval object and the whole checkpoint chain are verified. On failure, a new run starts from the original initial state. Completion requires exact horizon coverage. This is stricter than my round-1 suggestion and is correct for FR-11.AC1 and NFR-9.AC2 |

## Should-fix items, CodeRabbit and adversarial findings

| Item | Status | Where |
|---|---|---|
| S-1 PostgreSQL dispatch in P1 | ✓ | ADR 0005 l.108–114: SKIP LOCKED, NOTIFY as a hint plus scans, no Streams relay |
| S-2 separate supervisor and lease owner | ✓ | ADR 0005 l.34–38: separate unit. l.123–126: supervisor renews and watchdog stops the tree first. Genesis checkpoint: l.159–167 |
| S-3 sizing and input placement | ✓ | ADR 0004 l.60–80. Labelled `ASSUMPTION:`, sweep member in the partition key at l.52, Zarr/NetCDF left to the data spike. ADR 0002 l.30–33 no longer implies bulk inputs live in PostgreSQL |
| S-4 Valkey default | ✓ | Q-O-3 ★ Valkey. Hyperscaler TSL note at ADR 0004 l.207–208. Cross-PR mapping missing (R-1) |
| S-5 topology events | ✓ | ADR 0002 l.74–77: topology per interval and factors cached per topology state; no window fragmentation |
| S-6 numeric canonicalisation | ✓ | ADR 0002: binary64 as fixed-width bytes, −0 normalised, exact rational unit factors rounded once, money as typed decimal strings, golden vectors |
| S-7 crate ownership | ✓ | ADR 0005 l.51–59 places lifecycle and `store` inside `crates/engine`. This now agrees with ADR 0001 l.43 ("Orchestration, worker lifecycle"), so there is no dual claim |
| S-8 sweep reductions and pin cascade | ✓ | Reductions as admitted worker jobs: ADR 0005 l.61–65 and l.243–244. Parent pin covers existing and future children: ADR 0004 l.163–167 |
| S-9 owner-level queue | ✓ | See the owner-question table above |
| S-10 restore and GC | ✓ | ADR 0004 l.169–180: external append-only deletion journal before physical deletion, 30-day blob versions, stated restore order, incomplete journal blocks reopening |
| CodeRabbit: expiry and resume | ✓ | Same as M-4 |
| CodeRabbit: first checkpoint | ✓ | Genesis checkpoint with null predecessor; the pre-pass is not rerun once genesis exists |
| ADV17-01 Redis AGPL | ✓ | Default changed to Valkey. The AGPL analysis is kept and stated accurately |
| ADV17-02 MIP divergence | ✓ | ADR 0005 l.188–203, ADR 0002 l.177–185. Execution identity is pinned and the same warm-start policy applies at every boundary, so an uninterrupted run and a resumed run follow the same rule. No bit-identity promise. This matches PR #4 NFR-1.AC2/AC3 and Q-N-6 |
| ADV17-03 zombie worker OOM | ✓ | ADR 0005 l.127–138: confirm death or kill/reap before reclaim, then re-admit against actual headroom. Reservations are never overlapped beyond the RAM budget |
| ADV17-04 orphan blobs | ✓ | ADR 0004 l.87–120: registered staging prefix, pending manifest, pre-registered promotion keys, staging-only LIST sweeper |
| ADV17-05 repair exhaustion | ✓ | ADR 0006 l.85–107. Agrees with PR #4 round 3 (see below) |
| ADV17-06 pin/expiry race | ✓ | ADR 0004 l.151–159: row-locked state machine, tombstone committed before any deletion |
| ADV17-07 fd-passing IPC | ✓ | ADR 0005 l.67–74: versioned envelopes and blob references; fd passing is a local adapter |
| ADV17-08 analytics in control process | ✓ | ADR 0005 l.61–65; topology l.47 and l.82–85 |
| ADV17-09 as-of infeasibility | ✓ | ADR 0002 l.91–95: unknown publication history gives a **reconstruction** label, not a rejection. The owner-facing claim is in Q-O-1 |
| ADV17-10 hash drift | ✓ | Same as S-6 |
| ADV17-11 PostGIS | ✓ Rejection is sound | The licence reasoning is correct. One authority wording issue: S-a below |
| ADV17-12 cooperative cancel | ✓ | ADR 0005 l.219–224: cooperation depends on capability; the watchdog enforces the hard bound |
| ADV17-13 schema migration | ✓ | ADR 0002 l.169–175: versioned Arrow projection adapters; migrations produce new derived datasets |
| ADV17-14 single active run | ✓ | ADR 0005 l.228–248: P1 admission only; independent children run concurrently in P2 |

## Consistency with D-1…D-65 and PR #4

- **Decisions.** I re-checked the revised text against D-2, D-3, D-15, D-17, D-22,
  D-25, D-35, D-36, D-39, D-44, D-48, D-56, D-60, D-62 and D-65 and found no
  contradiction. The Valkey proposal is gated by Q-O-3 and the decisions log is not
  rewritten (D-3). Checkpoint expiry and the sweep pin cascade are consistent with D-36.
  P1 gains only job-model fields for priority and preemption. Live feeds and scheduling
  stay in P2 (D-25, D-44). ADR 0006 l.100–101 forbids disabling 132 kV rows (D-60).
- **PR #4 round-3 fallback.** PR #4 head `52fda46` now contains the round-3 policy:
  ADR 0003 l.52–64, FR-21.AC4 and the Q-N-5 row. It allows up to three repairs, then one
  bounded network-aware recovery MILP. If that fails, it keeps the last fully checked
  boundary and marks the run failed/partial, with no security slack. ADR 0006 l.85–107
  and ADR 0005 l.147–150 agree with this. The integration caveats at topology
  l.153–156 and ADR 0006 l.88–89 ("the pinned snapshot still says…") are now out of date
  (n-2). One wording difference is harmless: ADR 0006 calls the fallback a "joint
  UC/redispatch solve", while ADR 0003 freezes the recovered commitment and reruns
  stage 2 as an LP. The ADR 0003 form is the more specific one; follow it at integration.
- **PR #4 lifecycle and states.** External states (queued, running, completed, partial,
  cancelled, failed), the 96 GiB/80% ceiling, the one-active-simulation assumption in
  NFR-5 and the 2 s/5 s cancellation bounds all match.
- **Merge mechanics.** PR #17 merges cleanly into `main`. It conflicts with PR #4 head
  `52fda46` in `02-questionnaire.md` (both PRs edit the Q-D-5 line and nearby entries).
  This is expected; the coordinator should resolve it together with R-1.

## Remaining should-fix and nits (non-blocking)

- **S-a (rule 1 wording).** ADR 0004 l.43 says "retain *baseline* PostGIS", and the
  response's ADV17-11 row says "The owner's baseline includes it". No decision records
  PostGIS. It is the ★ default of open Q-C-6 and a stack candidate. Reword both to
  "Q-C-6's recommended PostGIS (optional in P1)". The design does not change.
- **S-b (tracking under an answered ID).** The D-65 target-inventory clarification is
  tracked by annotating Q-H-7, which is marked `[x]`, so it will not appear as open.
  PR #4 round 3 tracks the same item under Q-N-7. Point to Q-N-7 or D-65 instead of
  annotating an answered question. This keeps one tracker and avoids reopening Q-H-7's
  status.
- n-1: Add the ★ glyph to the four "Recommended default" lines in
  `owner-questionnaire.md`, to match the canonical file.
- n-2: Update the PR #4 pin to `52fda46` and remove the "still says repair exhaustion
  fails" caveats, or reduce them to the stage-2 wording note above.
- n-3: The response gives no disposition for round-1 minor notes n-1…n-5 (UUIDv7,
  observability, node label, Ipopt fit, Q-H-6 link for P2 delivery). The node label is
  in effect done (topology l.32). The rest can wait for later ADRs, but list them once
  in the response so that they are not lost.
- n-4: ADR 0005 l.209 is a single overlong line, unlike the wrapping used elsewhere.
- n-5: The CodeRabbit check on `d99ceabb` was still pending when I checked
  (`gh pr checks 17`).

## What I checked and how

| Check | Method | Result |
|---|---|---|
| Item resolution | Read all eight changed files at `d99ceabb`; `git diff 254e0e1 d99ceab` for ADRs 0002 and 0006 | All round-1 items resolved in text; R-1 is the only open item |
| Question queue | Read section O and the cross-references in `02-questionnaire.md`, `owner-questionnaire.md`, and PR #4's questionnaire at `dbc6299` and `52fda46` | Four ★ entries; cross-PR overlaps found (R-1) |
| PR #4 consistency | `git show` of PR #4 ADR 0001, ADR 0003, `04-requirements.md` and `02-questionnaire.md` at `dbc6299` and `52fda46`; searched for Redis, Valkey, repair, fallback, crate rows and reproducibility | Fallback, states and crates agree; Redis/Valkey defaults conflict (R-1) |
| Authority claims | Searched the decisions log, vision, AGENTS.md and stack candidates for PostGIS; searched PR #17 for "owner clarification" and "baseline" | M-3 fixed; S-a found |
| Local links | Script over the eight files: relative paths and heading anchors | 41 links, 0 broken (the response's count is correct) |
| D-52 | Commit messages `origin/main..d99ceab`, PR diff, PR body | No attribution trailers, footers or badges |
| Protected files | Three-dot `git diff --name-status origin/main...d99ceab` | `STATUS.md` and the decisions log are untouched; eight documentation files changed |
| Mergeability | `gh pr view 17`; `git merge-tree` against `main` and against PR #4 `52fda46` | Clean against `main`; expected conflict with PR #4 in `02-questionnaire.md` |
