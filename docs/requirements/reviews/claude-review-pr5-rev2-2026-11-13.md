# Independent review of revised PR #5: data spike, revision 2

_Reviewer: Claude Opus 5.5 (independent review under D-54) · 2026-11-13_
_Scope: PR #5 head `d214b73b` (`t3/r12-transmission-data-spike`), compared with `main` at `87d57af`._

This review checks that every finding in the earlier PR #5 reviews is resolved. Those reviews are
[claude-review-pr5-2026-11-12.md](claude-review-pr5-2026-11-12.md),
[gemini-adversarial-pr5-2026-11-12.md](gemini-adversarial-pr5-2026-11-12.md),
[triage-gemini-adversarial-pr5-2026-11-12.md](triage-gemini-adversarial-pr5-2026-11-12.md) and the CodeRabbit inline
comments. The author's response is
[response-pr5-2026-11-13.md](response-pr5-2026-11-13.md). The review also checks D-56 compliance, script
reproducibility and secrets, and the accuracy of the data-quality traps. PR #5's files were not edited.

## Verdict

**Must-fix: one item, then merge-ready.** The revision resolves 11 of the 12 earlier document-review findings and
partly resolves the twelfth. It resolves the CodeRabbit numeric finding and ADV5-1, -2 and -4 to -9 as triaged.
D-56 compliance is clean: no raw `research-only` data is in the tree or in the PR history. The scripts reproduce the
committed network statistics exactly. One licence claim is wrong, and it would put the wrong D-56 tag on I-SEM
day-ahead prices. That must be corrected before merge.

### Must-fix items

1. **ENTSO-E free re-use scope is misstated (ADV5-3 resolution).** See M-1 below.

## What I checked and how

| Check | Method | Result |
|---|---|---|
| Reproducibility | Fresh detached worktree at `d214b73b`; ran `fetch.sh --include-uncleared` (43 s), `unzip -t`, then both inspections with `openpyxl==3.1.5` | All steps pass. `network-statistics.json` regenerates **byte-identical**. `tabular-statistics.json` changes only in the REPD block (live-file drift, see m-6). Every committed sample CSV/JSON regenerates identically |
| Hash pinning | Output of `verify_hashes.py` on a fresh fetch | 37 OK, 9 drift warnings (7 licence/catalogue pages, `generation-mix.csv`, `repd.csv`), exit 0. Warn-not-fail works as documented. All five research-only datasets match their pins, including ETYS Appendix B `d5cacf23…5e0c` |
| New raw location | Searched `/home/njotob/energy-data/raw` | Holds the PR #11 gathering, not this spike's `raw/`. `neso/etys-2025-appendix-b-system-technical-data.xlsx` has the same SHA-256 as the spike's pin. The spike scripts read only `spikes/data/raw/`, so I reproduced from a fresh fetch (m-7) |
| Cleared-only run | Copied the spike, deleted the five research-only raw files, ran `inspect_tabular.py` | Passes, but it rewrites the manifest and drops all five research-only pins (m-3) |
| D-56: committed data | `git ls-files spikes/`; `--name-status` of every non-merge PR commit; largest blobs in `origin/main..HEAD` | 23 files under `spikes/data/`, largest blob 60 KB (`REPORT.md`). No raw file was ever added. Samples come only from `open-commercial` sources. ETYS/TYTFS appear only as aggregate counts and component sizes: no bus IDs, names, impedances or ratings |
| D-56: tags | `download-manifest.json` against `sources.tsv` and the licence evidence | 15 `open-commercial` datasets, 5 `research-only` datasets, 26 metadata pages tagged `research-only`. All dataset tags are correct. ENTSO-E is not in the manifest (not retrieved), but the report's stated tag for it is wrong (M-1) |
| Licence spot-checks | Read the fetched raw evidence | NESO constraint datasets: `license_id: ESO`, "NESO Open Data Licence" ✓. NGED LTDS: `wpd-open-data` ✓. ERA5 STAC `CC-BY-4.0` ✓. DSA095: Open, approved 02/12/2025, Security residual Amber ✓. ENTSO-E 18 Oct 2023 list: see M-1 |
| ERA5 template | Against the captured CDS process schema | All 8 variables are in the enum. Area `[61, −12, 48, 10]`, 31 days × 24 h. The 53 × 89 = 4,717 points and 1,325,892,096-byte arithmetic is correct |
| Secrets | Searched tracked `spikes/` files for token/key/password/bearer patterns | None. No `securityToken` value, no `.cdsapirc`, no credential-bearing URL |
| D-52 | Tracked PR files, commit messages and authors, PR body, non-bot comments | No `Co-Authored-By`, "Generated with" or 🤖. No model names outside process docs |
| Protected docs | PR diff | `STATUS.md` and `03-decisions-log.md` are not modified |
| Mergeability | `git merge-tree`; GitHub | Clean; `MERGEABLE`. The branch is 8 commits behind `main` (m-2) |

## Findings

Severity: **must-fix** = do not merge until fixed · **minor** = fix in the same push if convenient, otherwise in a
follow-up.

### Must-fix

**M-1. The ENTSO-E free re-use list does not cover day-ahead prices or scheduled exchanges.**
`spikes/data/REPORT.md:382-388` says the 18 October 2023 list "includes energy prices (TR 12.1.d), scheduled commercial
exchanges (TR 12.1.f) and physical flows (TR 12.1.g)". It then concludes that "`open-commercial` is appropriate" for
I-SEM prices. The §6 register (`REPORT.md:536`) says "I-SEM prices in scope", and the response's ADV5-3 row repeats
the claim.

I read the committed raw evidence, `entsoe-reuse.pdf` (its hash matches the pin). The list has 35 items. It includes
**item 18, Physical flows, 12.1.g**. It does **not** include 12.1.d (day-ahead prices) or 12.1.f (scheduled commercial
exchanges). The list's Transparency Regulation references stop at 12.1.a/b/c, 12.1.g, 13.1.x and 17.x. The captured
legal-terms page (`entsoe-legal.json`) links lists dated 2018-11-08 to 2023-10-18. The 2023 list is the latest, so
no newer list widens the scope. CodeRabbit's inline comment on `REPORT.md:386` is correct on this point. The
omission is plausible: day-ahead prices are supplied by NEMOs (here SEMOpx), and the report already treats SEMOpx
and MID exchange-sourced prices as uncleared.

As written, the report would lead an ingest agent to tag ENTSO-E I-SEM DAM prices `open-commercial`, which breaks
D-56 (rule 10) and D-57's honest-labelling intent.

*Fix:*
- In §4, list only physical flows (12.1.g) as covered by the 2023 list.
- Tag ENTSO-E day-ahead prices (A44) and scheduled exchanges (A09) `research-only`, and track their clearance under
  PC-3 (SEMOpx prices) or PC-5.
- Correct the §6 register row and the response's ADV5-3 row to match.
- Note that the IFA and Nemo Link exclusions also apply to physical flows on those cables.

### Minor

**m-1. ADV5-3 triage outcome.** The access route (A44, EIC `10Y1001A1001A59C`, A11 and A09, token procedure,
half-open UTC year) is accurate and useful. Only the licence conclusion in M-1 is wrong. No further change is needed
beyond M-1.

**m-2. "All three are open PRs" is stale.** `REPORT.md:466`, the response's "Related gathering" section and the R-12
row in `05-risks.md` describe PR #10 and #11 as open. Both merged at 03:39Z and 03:41Z on 13 Nov, one to three
minutes after `d214b73b` (03:38Z). The branch is now 8 commits behind `main`. *Fix:* merge `main` again and say that
#10 and #11 are merged, with their findings still not re-audited here. #12 is still open.

**m-3. A cleared-only run deletes the research-only provenance pins.** `inspect_tabular.py:84-98` rebuilds
`download-manifest.json` only from files present in `raw/`. After a default (cleared-only) fetch, the manifest loses
all five research-only dataset entries (ETYS, both TYTFS files, MID and the I-SEM XML): reproduced, 5 entries → 0. If
someone commits after the documented default reproduction, the D-56 provenance record for those sources disappears.
`verify_hashes.py` then has nothing to check them against. *Fix:* merge with the existing manifest, keeping entries
whose file is absent, or write regenerated pins to a separate file.

**m-4. The DST trap can state the generation-mix time convention.** `REPORT.md:327` asks the reader to "confirm
publisher time convention". The data already answers this. Every 2024 date in `df_fuel_ckan.csv` has 48 rows,
including 31 March and 27 October, and 31 March includes 01:00 and 01:30, which do not exist in Europe/London local
time. The timestamps are therefore UTC/GMT, not local. The NESO demand file uses the 46/50-period settlement-day
calendar. Joining the two files by date silently misaligns them on both clock-change days and offsets every BST
period by one hour. Add this to the clock-change trap at line 395.

**m-5. MID zero-volume wording is understated.** "Some provider records have zero volume" (`REPORT.md:331`). In the
fetched sample, all 49 `N2EXMIDP` records have zero volume and all 49 `APXMIDP` records are non-zero. Say that one
provider's series is entirely zero-volume, so a volume-weighted MID must handle it explicitly.

**m-6. REPD drift under a pinned URL.** The Q2 2026 REPD asset URL now returns 14,968 rows, with 3,162 Operational
(5,219,160 bytes), not the reported 14,657 / 3,132. The pin catches this, as intended. The report's figures stay
valid for the pinned hash. Consider adding "as of the pinned SHA-256" to the REPD row. A second trap: REPD status
values differ only in case ("Planning permission Granted" / "granted" / "Granted"), so normalise case as well as
whitespace.

**m-7. Raw directory is hard-coded.** Both inspectors use `ROOT / "raw"`. Bulk data now lives at `~/energy-data/raw`.
An environment override (for example `SPIKE_RAW_DIR`) would avoid symlinks or re-downloads. The README lists `uv` as
required, but the scripts run with any Python 3 that has `openpyxl==3.1.5`; say that `uv` is optional.

**m-8. Questionnaire convention.** The new Q-C-11, Q-C-12 and Q-M-4 use "**Recommended default:**" rather than the
file's ★ marker (`02-questionnaire.md:5`). Q-C-11 and Q-C-12 are inserted before Q-C-2, so they are out of order.
The IDs are stable, so this is cosmetic. CodeRabbit raised the same point.

## Resolution of earlier findings

### Document review (claude-review-pr5-2026-11-12)

| # | Finding | Status | Evidence |
|---|---|---|---|
| 1 | B depends on A | **Resolved** | §7 row B: "A's source access + assembly/reconciliation, then 4–8 additional person-weeks" |
| 2 | Boundary-zone option / D-55 | **Resolved** | Option F added. D-55 cited in §7 and §8.3. Both NESO constraint datasets confirmed `license_id: ESO` in the raw metadata |
| 3 | Other DNO 132 kV | **Resolved** | §6 "Additional DNO coverage register", §8 gate 2, Scottish 132 kV noted as transmission |
| 4 | A effort optimistic | **Resolved** | Widened to 16–32 pw; D-28 acceptance explicitly excluded |
| 5 | Cleared-only reproduction | **Resolved, with side effect** | Verified passing; see m-3 |
| 6 | No hash verification | **Resolved** | `verify_hashes.py` runs from `fetch.sh`; drift warnings observed, exit 0 |
| 7 | NGED Table 2b / DSA095 / CIM | **Resolved** | 69 rows reproduced; DSA095 contents match the report; E targets the CIM |
| 8 | ERA5 variables | **Resolved** | Precipitation and total-sky direct radiation added; schema-valid; Q-C-12 |
| 9 | Model name in REPORT | **Resolved** | No model names in `spikes/` |
| 10 | Behind main | **Partly resolved** | Merged at 03:38Z; behind again after PR #10/#11 merged (m-2) |
| 11 | Requirements hand-off | **Resolved** | Q-C-11, Q-C-12, Q-M-4, Q-L-7 added; R-12 updated; STATUS left alone as instructed |
| 12 | Double sample write | **Resolved** | `inspect_tabular.py:58-62` writes the 132 kV circuit excerpt once |
| CR | Finite-number check (CodeRabbit) | **Resolved** | `finite_number()` at line 29; 1,138 reproduced |

### Adversarial review and coordinator triage

| ID | Triage | Status | Evidence |
|---|---|---|---|
| ADV5-1 ETYS presumed open | Reject | **Resolved** | §2 states that the duty is not a grant; PC-1 |
| ADV5-2 all DNOs open | Partly accept | **Resolved** | DNO rows UNVERIFIED pending file-level checks; ENWL restriction noted |
| ADV5-3 ENTSO-E | Accept, verify | **Not resolved** | Route correct; licence scope wrong (M-1) |
| ADV5-4 ERA5 / Elexon notices | (not triaged) | **Resolved** | STAC `CC-BY-4.0` confirmed; `samples/README.md` notices |
| ADV5-5 fallbacks | Partly accept | **Resolved** | A widened, B additive, F added, C/E limits stated |
| ADV5-6 data-quality traps | Accept | **Resolved** | §4 traps added; accuracy refinements in m-4 and m-5 |
| ADV5-7 three-winding projection | (not triaged) | **Resolved** | Status→winding mapping `{2:1, 3:2, 4:0}` matches PSS/E STAT semantics; labelled connectivity-only; stats reproduce (2,022 + 1) |
| ADV5-8 ERA5 bbox | (not triaged) | **Resolved** | East bound 10° |
| ADV5-9 sample notices | (not triaged) | **Resolved** | `samples/README.md` beside samples |

## Data-quality trap accuracy

I checked each trap against the fetched data:

- **Clock change:** 31 Mar 2024 = 46 periods, 27 Oct 2024 = 50; 364 × 48 + 46 + 50 = 17,568; no duplicate keys ✓.
  The generation mix uses 48/day UTC (m-4).
- **BOA overlap:** 13 of 65 acceptances for 2024-01-01 SP1 carry `settlementDate` 2023-12-31, starting from 23:02Z ✓.
- **MID inclusive endpoints:** 98 records = 49 × 2 providers, first and last `startTime` at 00:00Z on consecutive days ✓.
  The zero-volume split is in m-5.
- **Five-minute fuel:** 288 records, 00:00Z–23:55Z ✓.
- **I-SEM XML day:** `2023-12-31T23:00Z/2024-01-01T23:00Z` ✓. The report correctly warns that the summer offset differs.
- **Units:** ETYS "% on 100 MVA", PSS/E per-unit and winding bases, and zero RATE-A not meaning zero capacity are stated
  correctly. The PSS/E v33 field indices used (branch ST = 13, transformer STAT = 11, bus BASKV/IDE = 2/3) are correct.
- **ENTSO-E parameters:** A44 / A11 / A09, `BZN|IE(SEM)` EIC and the half-open UTC year are correct.

## Recommendation

Fix M-1 (three text locations: `REPORT.md` §4 and §6, and the response's ADV5-3 row) and merge `main` again (m-2).
After that the PR is merge-ready. m-3 to m-8 can follow in the same push or in an M1 data-ingest follow-up.
