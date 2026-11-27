# Response to PR #5 reviews — 13 November 2026

Scope: [PR #5](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/5),
the data spike under D-31. Responses cover the
[document review](claude-review-pr5-2026-11-12.md),
[adversarial review](gemini-adversarial-pr5-2026-11-12.md) and
[coordinator triage](triage-gemini-adversarial-pr5-2026-11-12.md).
No new owner decision or application code is introduced.

## Branch integration and protected documents

- Ran `gh pr checkout 5`. The branch was already bound to another clean worktree,
  so repeated the command there instead of forcing duplicate branch checkouts or
  creating a new branch. Merged `origin/main` and pushed merge **88bebc9**.
- Reproducibility/provenance fixes are committed as **c5753eb** and pushed.
- **No authored edits to STATUS.md or 03-decisions-log.md.** The requested merge
  carries upstream changes in those documents; the revision does not modify
  their contents relative to the merged main. This explicit task restriction
  overrides the normal end-of-session STATUS update rule.

## Revised verdict and owner decisions

**D-56: research can proceed; coverage remains partial.** Publicly downloadable
ETYS and EirGrid/SONI cases are admissible as `research-only` inputs for the
research phase. Commercial permission is not a prerequisite for research ingest.
Only scripts, provenance and non-reconstructive aggregates are committed for
restricted sources; no raw or derived network export is added. Share-alike stays
excluded. Commercial clearance remains in
[TODO-pre-commercial.md](../TODO-pre-commercial.md), **PC-1–PC-5**; no legal grant
is invented and R-12 stays open for coverage/current-topology/electrical QA.

The report now follows **D-55 DC plus proxy constraints**, **D-57 free/minuscule
price sourcing with honest product labels**, **D-59 cost-based redispatch**, and
**D-60 all GB 132 kV**. None authorises a reduced-zone prototype to silently
replace full-network coverage. Questions **Q-C-12, Q-C-13 and Q-M-4** record
unresolved weather, snapshot date and fallback/effort assumptions. Q-C-11 is
answered by D-11/D-32, including NI 275 kV; it does not reopen network scope.
Q-L-7 is marked answered from existing D-57; no decision-log edit is needed.

## Document-review dispositions

| Finding | Resolution |
|---|---|
| 1. B depends on A | Fixed §7: B requires A's access/assembly/reconciliation **plus 4–8 person-weeks**, not a standalone cheaper route. Research-only inputs retain the same commercial-clearance obligations. |
| 2. Missing boundary-zone option / D-55 | Added **F**, indicative 4–6 person-weeks, ~20–40 candidate GB zones plus SEM/HVDC. Independently checked both suggested NESO constraint datasets: `license_id: ESO`, NESO Open Data Licence. Fidelity exclusions and D-60 scope-approval caveat are explicit. |
| 3. Other DNO 132-kV sources | Added UKPN, NPg, ENWL, SP Manweb and SSEN Southern register rows and explicit §8 coverage gates. Their actual file licences/access remain UNVERIFIED here pending independent checks of PR #10. Scottish 132 kV is transmission, not missing DNO data. |
| 4. A effort understated | Widened to **16–32 person-weeks** for data assembly/reconciliation. All estimates explicitly exclude D-28 independent numerical acceptance, external access delays, procurement and engine/UI work. These are assumptions, not promises. |
| 5. Cleared-only reproduction | `inspect_tabular.py` skips optional missing MID/XML and missing manifest sources; isolated cleared-only execution passes without ETYS/Irish restricted datasets. Round 3 also makes DUKES and each network input optional with explicit skip messages; all absent provenance entries are retained. |
| 6. Fetch lacks hash verification | Added `verify_hashes.py`, called by the fetcher. Round 3 makes mismatches fail by default; `--allow-drift` is the explicit override. Inspectors write observed outputs separately and never refresh tracked fixtures/pins. |
| 7. NGED Table 2b / assessment / CIM | Downloaded and counted **69** three-winding records; read **DSA095**, approved 2 December 2025, Open recommendation with amber residual security risk, no stated row-level completeness guarantee. Added exact source URLs/hashes and targeted successor CIM in E. No new raw/binary sample is committed. |
| 8. Hydro/PV weather fields | Added precipitation and total-sky direct solar radiation at surface, labelled candidate inputs, not validated hydro/PV models. Q-C-12 records the assumption and enlarged footprint. |
| 9. Non-process attribution wording | Removed the model name from REPORT.md; no attribution footer/header/trailer is added. |
| 10. Behind main | Merged main as explicitly requested; no rebase/force push. |
| 11. Requirements handoff | Updated R-12 in the risk register and added stable questionnaire IDs/defaults for open assumptions. STATUS and the append-only decisions log are deliberately untouched by authored changes. |
| 12. Double sample write | Removed the generic NGED circuit sample write; only the filtered 132-kV circuit excerpt is emitted. |

The separate inline numeric-coverage finding is also fixed: all R1/X1/B1/Winter/
Summer values must parse as finite numbers. Missing/whitespace/text/NaN/infinity
fixtures are rejected. The historical **1,138** total includes all change statuses;
round 3 separates **907 Unchanged / 53 Updated / 115 New / 63 Removed** circuits.

## Adversarial-review dispositions

| ID | Resolution against evidence and coordinator triage |
|---|---|
| ADV5-1 | **Rejected as triaged.** A presumed-open regulatory duty is not a dataset licence grant. D-56 removes the research blocker; PC-1 still covers commercial ETYS rights. |
| ADV5-2 | **Partly accepted as triaged.** Expanded every relevant DNO coverage row and linked PR #10. Do not inherit the unsupported blanket claim that all 14 DNO areas have permissively licensed 132-kV electrical models; Scotland's ownership boundary, access gates and ENWL terms matter. |
| ADV5-3 | **Accepted, verified with narrower licence scope.** Added I-SEM A44, physical A11 and scheduled A09 inputs, correct EIC, owner-token procedure and a complete-year end bound. The 18 October 2023 CC-BY free-reuse list covers physical flows (12.1.g), **not day-ahead prices (12.1.d) or scheduled exchanges (12.1.f)**. A44/A09 are **research-only**, PC-3/PC-5. A11 physical flows require provider/item checks, including IFA/Nemo exclusions. No complete 2024 series or universal interconnector grant is claimed without authenticated retrieval. |
| ADV5-4 | **Blanket ERA5 objection rejected; notice hygiene improved.** The dataset-attached CC-BY association and captured CDS schema remain the evidence, not an unrelated general Copernicus contract. Added an adjacent sample notice file and explicit detached-distribution guidance for Elexon/NESO/NGED attribution. MID third-party rights remain separate. |
| ADV5-5 | **Partly accepted as triaged.** A widened, B additive, F added, C's connectivity/geography distinction clarified, E explicitly a CIM technical pilot rather than a national MVP. D-56 changes rights timing, not coverage/fidelity. |
| ADV5-6 | **Accepted as triaged.** Added explicit clock-change, market-day, revision/vintage, BOA-overlap and unit/base traps; also effective-dated BMU aliases and REMIT duplicate/revision/as-of handling. These are M1 acceptance checks, not a claim of implemented production ingest tests. |
| ADV5-7 | **Electrical warning accepted; incorrect-component claim rejected.** The one-off script measures physical-bus connectivity, not a power-flow equivalent. Renamed/labelled the terminal projection and explicitly disallowed electrical use. Independently rebuilding a star graph from the same case gives identical physical-bus component sizes **2,022 + 1**. A later electrical importer must retain internal neutral buses, winding impedances/taps/bases; projected edges are not physical circuits or cut-sets. |
| ADV5-8 | **Accepted as a candidate footprint improvement.** East bound extended from 3° to 10°, with eight-variable arithmetic and Q-C-12. No guarantee of all offshore assets or authority to expand neighbouring-market scope is implied. |
| ADV5-9 | **Attribution improvement accepted, mandatory embedded-JSON claim not established.** Notices now sit beside samples with governing licence links and instructions to carry notices when files are detached. No licence requirement to rewrite each raw excerpt into a metadata wrapper was demonstrated. |

## ENTSO-E evidence and access boundary

Official [legal terms and free-reuse list](https://transparencyplatform.zendesk.com/hc/en-us/articles/40921911218961-Legal-Terms-and-Conditions),
[token procedure](https://transparencyplatform.zendesk.com/hc/en-us/articles/12845911031188-How-to-get-security-token)
and [area/EIC list](https://transparencyplatform.zendesk.com/hc/en-us/articles/15885757676308-Area-List-with-Energy-Identification-Code-EIC)
were retrieved. The 18 October 2023 free-reuse list applies CC-BY-4.0 to listed
items, subject to its exceptions. Raw evidence is ignored locally; exact URLs
and hashes are in the spike provenance. There is **no owner token**, account
creation, token-bearing committed URL, authenticated API archive or assumed
licence for omitted items or excluded flow providers. Day-ahead prices and
scheduled exchanges remain research-only/PC-3/PC-5; uncertain physical flows
remain research-only/PC-5.

## Related gathering, without inheriting unverified claims

- [PR #10](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/10):
  `docs/research/dno-132kv-inventory.md`, all 14 licence areas; actual access,
  electrical completeness and file-level rights still need independent checks.
- [PR #11](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/11):
  `spikes/data-gathering/network-ie/`, reported network/IE gathering outside git
  at `~/energy-data/raw`; ETYS licence correction and ENTSO-E token blocker noted.
  SEMOpx lookback contents require inspection before claiming full DAM coverage.
- [PR #12](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/12):
  `spikes/data-gathering/elexon/`, roughly **4.2 GB** (reported **4.16 GiB /
  4,467,563,083 bytes**) compressed, stored outside git at
  `~/energy-data/raw/elexon/`. This is subsequent gathering, not this spike's
  independent full-year audit. MID rights/product labels, sparse BOA intervals,
  REMIT revisions and the report's spring numbering need the §4 semantic gates.

PRs #10 and #11 are merged and included through the main merge; #12 remains
open at this revision. No raw gathered archive is added to
PR #5; a source/manifest cross-reference is not a completeness certificate.

## Validation and remaining work

- Full local tabular/network inspections pass with `openpyxl==3.1.5`; regenerated
  statistics preserve the audited 2024 totals and physical-bus connectivity.
- Isolated cleared-only tabular execution passes without restricted datasets;
  a temporary fetch fixture proves default research-only skipping.
- Finite-value fixtures and matching/changed-hash warning fixtures pass.
- An independent star-graph reconstruction confirms physical-bus components;
  this is not an impedance or numerical power-flow validation.
- All eight ERA5 variables are in the captured official enum; bbox/volume
  arithmetic is **1,325,892,096 bytes** before encoding/metadata. No weather
  retrieval job is claimed.
- Shell/Python syntax, JSON, source fingerprints, provenance tags, raw-path
  exclusion, whitespace and absence of authored protected-document edits checked.

Outstanding: owner ENTSO-E/CDS access; independent verification of the gathered
archives and every DNO file; Q-C-12/Q-C-13/Q-M-4; D-28 numerical acceptance once
engine work is authorised; and PC-1–PC-5 clearance before commercial activity.

## Round 3 — revision-2 reviews and coordinator triage

Scope: the [revision-2 document review](claude-review-pr5-rev2-2026-11-13.md)
and [revision-2 adversarial review](gemini-adversarial-pr5-rev2-2026-11-13.md)
of `d214b73b`. Checked out PR #5, merged `origin/main`, and addressed the
coordinator's must-fix list. These responses supersede earlier script-behaviour
and licence conclusions where indicated. Script/provenance changes are in
`7340f17`; the accompanying report, questionnaire and response document the
same observed results. No new owner decisions or application code are introduced.

| Finding | Round 3 resolution |
|---|---|
| ENTSO-E licence — M-1 / ADV5-R2-1 / ADV5-3 | Corrected REPORT §4, §6 register and the ADV5-3 row above. Of the three proposed series, the 18 October 2023 list includes only physical flows (12.1.g), subject to provider exceptions including IFA/Nemo. A44 day-ahead prices (12.1.d) and A09 scheduled exchanges (12.1.f) are **research-only**, PC-3/PC-5. API access is separate from licence clearance. |
| Metadata fallback — ADV5-R2-2 | Metadata now uses `source_role: metadata`, `licence_status: public-metadata`, including OGL/CC-BY deeds in both committed and generated manifests. This is an evidence category, not a dataset licence grant; D-56 dataset tags remain unchanged. Original hashes/bytes are preserved. |
| UTC alignment — ADV5-R2-3 / m-4 | Removed the coupled annual-length assert. Demand is Europe/London settlement time (46/50 transition periods); generation is UTC (48 every day). Convert local midnight to UTC, then add elapsed half-hours. Separate calendar/uniqueness checks and a UTC-keyed join match 17,568 instants. Four transition probes pass; reversing demand CSV rows leaves the join valid. REPORT's clock-change trap states the conversion and prohibits positional joins. |
| Optional inputs / provenance — ADV5-R2-4/-5 / m-3 | Both inspectors explicitly skip absent inputs, including DUKES, ETYS and TYTFS. Generated manifests merge committed pins and prior output records before updating present files, retaining all five research-only entries in cleared-only and repeated empty runs. |
| Hash enforcement — ADV5-R2-5 | `verify_hashes.py` exits 1 on any existing-file mismatch; `--allow-drift` explicitly permits drift while reporting it. `fetch.sh` honours both outcomes and the flag. Missing files are reported/skipped, not certified as complete. An isolated actual-fetch test verifies default skipping, mismatch failure and explicit override. |
| Paths / fixtures — ADV5-R2-6, partly accepted / m-7 | `SPIKE_RAW_DIR` configures all four scripts, default `spikes/data/raw/`; external `/home/njotob/energy-data/raw` is supported with `sources.tsv` filenames. `SPIKE_OUTPUT_DIR` defaults to ignored `output/` and rejects the tracked samples directory. All eleven committed excerpts map to **open-commercial** sources; none requires removal. Every run leaves sample bytes unchanged. Original aggregate snapshots remain labelled historical, with corrected counts in REPORT; the manifest metadata classification is the only authored JSON fixture correction. `uv` is optional. |
| NGED status and area counts — ADV5-R2-8 | Circuits are filtered separately from switchgear, then by status: **907 Unchanged baseline, 53 planned Updated, 115 planned New, 63 Removed**, all with finite R/X/B/winter/summer fields. Unchanged is a case-baseline proxy, not proof of current commissioning. Per-area all-equipment totals **1,311 / 1,411 / 857 / 604** match the merged inventory; REPORT includes per-area circuit/status counts and the register no longer presents 1,138 as operational. |
| Questions — ADV5-R2-7 / m-8 | Q-C-11 is answered by D-11/D-32 (NI 275 kV included), with D-60's GB 132 kV scope retained. The unresolved date has its own stable Q-C-13; all new unanswered questions have **★** default markers. No decision-log edit is needed. |
| Text accuracy — m-2/-5/-6 | PRs #10/#11 are recorded as merged; #12 remains open at this revision. All 49 N2EXMIDP observations have zero volume; all 49 APXMIDP observations are non-zero. REPD status values differ only by case as well as whitespace, so the inspector emits case-normalised counts. Original report totals are explicitly tied to pinned hashes. |

### Round 3 validation and limits

- Full tabular and network inspections pass using `openpyxl==3.1.5` and external
  raw storage. The full network aggregate JSON matches the committed snapshot
  exactly; the physical-bus projection remains a connectivity diagnostic.
- Empty-input, cleared-only, missing-DUKES and reversed-demand runs pass. Repeated
  runs preserve absent research provenance, including a deliberately distinct
  prior output pin. Attempts to direct output into `samples/` fail before writing.
- Matching-hash, mismatching-hash and explicit-override cases pass. A temporary
  local source tests the actual fetcher, not a mock, for exit propagation and
  research-only skipping. The real fresh fetch reports **nine drifts** (seven
  legal/catalogue pages, generation mix and REPD); inspection used explicit
  `--allow-drift` and did not repin the committed manifest. The refreshed REPD
  has **14,968 rows / 3,162 Operational**, distinct from the original audit.
- All eleven retained data excerpts are open-commercial; no research-only raw
  data, network records, exchange-price samples or credentials are committed.
  Byte comparisons confirm that inspection never rewrites tracked fixtures.
- Shell/Python syntax, JSON parsing, whitespace, ignored outputs and sample
  provenance checks pass. STATUS.md and the decisions log match merged main;
  neither receives an authored edit. The task's explicit restriction overrides
  the normal end-of-session STATUS update rule.

Remaining work is unchanged: owner access for authenticated ENTSO-E/CDS jobs,
independent archive/file-level completeness and electrical validation, the open
questions above, and pre-commercial clearance. No new independent PR review or
numerical power-flow acceptance is claimed.
