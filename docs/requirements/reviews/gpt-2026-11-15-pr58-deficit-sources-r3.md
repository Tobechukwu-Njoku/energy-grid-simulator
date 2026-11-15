# PR #58 final re-check — M1 deficit sources, round 3

Review date: **2026-11-15 UTC**. Reviewed [PR #58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) at exact head **`739bb70d0e1bb772ceae49f3afe99353a054b10c`**, against `main` **`9374d86e162f576c8425df8bac1e19068e959d7a`** and the [round-2 findings](gpt-2026-11-15-pr58-deficit-sources-r2.md). Source-document and questionnaire line numbers below refer to the reviewed head.

**Verdict: accept-with-corrections.** The substantive rights, historical-applicability and loss-proxy repairs are satisfactory for a candidate-source document. The remaining work is small: repair questionnaire navigation, register the price-resampling proposal, reconcile STATUS history, and apply the three still-current inline-comment corrections below. Each correction needs fewer than ten changed or inserted content lines. Previously accepted findings remain accepted; no new acquisition, modelling approval, deficit closure or commercial clearance is implied.

Scope: the three changed documents, publisher HTML/catalogue metadata, a public operating-procedure document, GitHub comments/review threads, questionnaire IDs and repository-relative links. **No datasets were downloaded.** The ETYS workbook was identified in publisher-page metadata, without fetching its contents. Public reference-document inspection is not dataset acquisition. PR #58 and its review threads were not modified.

## R2-01–R2-07 disposition

| Finding | Result | Evidence / remaining correction |
|---|---|---|
| R2-01 — missing assumption questions | **Partial** | Questionnaire lines 324–361 now contain seven real open entries, Q-P-01–Q-P-07, with options and recommended defaults. However, the new `#q-p-01`–`#q-p-07` links have no corresponding heading or explicit anchor, and the additional retained price-resampling assumption is still absent from the questionnaire. C-01/C-02. |
| R2-02 — publisher/licence links | **Resolved at candidate level** | All four replacement pages returned HTTP 200 and were read; details below. MIDAS's delayed coverage is now explicit (source lines 376–383); the exact 2024 station-variable panel remains pending, rather than acquired or established. |
| R2-03 — rights lineage | **Resolved** | Source lines 161/175 and summary row 3 retain ESB-derived `research-only`/PC-3 lineage. Candidate 7.3 and row 7 retain mixed/unverified Met Éireann terms. Candidate 8.1 and row 8 keep WRC content unverified/research-only, separate from GOV.UK and IANA. Conservative pending classifications satisfy the requested correction without inventing reuse grants. |
| R2-04 — price resampling/as-of policy | **Partial** | Lines 342–348 distinguish reconstruction from as-of selection, preserve publication cadence and imputation masks, and step-hold published prices; line 340 explicitly leaves EEX EUA acquisition unresolved. The modelling proposal is only recorded in the research document, not registered as an open questionnaire entry under AGENTS.md rule 1. C-02 completes this registration and makes auction publication timing explicit. |
| R2-05 — unpinned SEMOpx history | **Resolved** | Candidate 5.2, line 273, recommendation line 287 and summary row 5 now consistently describe a current reference with historical 2024 applicability pending. This accepts the historical caveat; the separate incorrect *current* IDC value is C-05. |
| R2-06 — HVDC formula/description | **Resolved** | Lines 232–237 and row 4 consistently describe a constant fractional exploratory proxy with omitted standby losses, disclose actual quadratic cable/fixed converter effects, and say no applicable CIGRE source has been identified. |
| R2-07 — repository reference paths | **Resolved paths** | The corrected ADR and earlier-review paths resolve under `docs/`. The R2 report is present on main and will resolve after main is incorporated; its absence from the stale PR branch is not another path defect. The blanket all-seven-resolved sentence at line 564 remains premature until C-01/C-02; C-03 qualifies it. |

## Repaired publisher pages read

| Reference | Observed evidence on 2026-11-15 |
|---|---|
| [NESO ETYS archive](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices) | HTTP 200. The page explicitly links **“ETYS24 Appendix B - System technical data”** to `/document/351916/download`, matching the revised workbook link. No workbook download or fleet-completeness verification. |
| [CEDA MIDAS Open collection](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/) | HTTP 200, real collection description rather than “no records”. It states Open Government Licence and explains that releases run through the previous complete year, explicitly giving v202407 → end-2023. It also discusses v202507. The revision's release-coverage caveat is supported; no station panel was fetched. |
| [ERA5 catalogue API](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels) | HTTP 200 JSON, `"license": "CC-BY-4.0"`, with a licence relation to `https://spdx.org/licenses/CC-BY-4.0`. This replaces the obsolete licence PDF. Account, credentials and interactive acceptance remain separate prerequisites in the document. |
| [ENTSO-E CGMES library](https://www.entsoe.eu/data/cim/cim-for-grid-models-exchange/) | HTTP 200; page title **“Common Grid Model Exchange Standard (CGMES) Library”** and explicit v2.4/v3.0 material. The revision correctly treats this as a design reference, without claiming acquired/cleared model instances. |

Also read [Met Éireann's open-data policy](https://www.met.ie/about-us/specialised-services/open-data) (HTTP 200: most datasets CC BY 4.0, some custom terms), the [WRC disclaimer](https://www.workplacerelations.ie/en/disclaimer/) (HTTP 200; no affirmative resource-specific commercial grant established), and the [SEMOpx intraday page](https://www.semopx.com/markets/intraday-market) (HTTP 200; 17:30/08:00/14:00 auction closures). These support the revised caveats. A missing grant is not a claim that holiday facts themselves are proprietary.

## Questionnaire format and identity checks

Section P follows the existing task-list format: unchecked `[ ]`, bold stable ID/title, explanatory text, options and a starred recommendation. The introductory text correctly keeps the proposals unapproved. The seven new declarations are unique and do not collide with any Section P IDs on main (there are none). Leading zeroes do not violate the stated `Q-<section>-<n>` rule; retain these published IDs.

The pre-existing duplicate Q-C-11 is already present on main and is not introduced here; no renumbering or unrelated cleanup is requested. Source-to-questionnaire file paths work, but bold task-list labels do not create GitHub fragment targets. Rendering Section P through GitHub's Markdown API confirms the absence of `q-p-01`–`q-p-07` anchors. C-01 fixes navigation without changing the established format.

## STATUS merge check

**Yes, the STATUS edit conflicts with main.** GitHub reports `mergeable: CONFLICTING`, `mergeStateStatus: DIRTY`. A non-mutating `git merge-tree --write-tree origin/main 739bb70` independently reports a single conflicted path, `docs/requirements/STATUS.md`. Both branches insert different PR #58 entries immediately below the repository metadata: main's historical R2 review and the PR's revision summary. Neither the questionnaire nor research document conflicts in that merge. C-03 preserves the review history and makes the revision claim accurate.

## Open review comments

Ran `gh pr view 58 --comments` and queried all review threads through GitHub GraphQL (`first:100`, `hasNextPage:false`). **The current state differs from the “one open thread” premise:** there are **24 total threads, 17 unresolved: 3 current and 14 outdated**. Outdated is a location flag, not proof of resolution. The general comment says automatic reviews are paused and the latest reviewed commit is `67a9d3d`, so it does not attest to `739bb70`. No request to resume or resolve anything was posted.

| Current unresolved thread | Assessment |
|---|---|
| [132 kV recommendation, line 50](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58#discussion_r4576443133) | Says NGED/SSEN plus ETYS leave other England/Wales DNO areas uncovered and asks to mark the baseline partial or add substantiated sources. **Needs the small qualification C-04.** Candidate 1.1 itself excludes England/Wales 132 kV, and Candidate 1.3 lists six DNO groups while the recommendation selects only two. Its existing general incomplete-coverage caveat should also accompany the actionable recommendation. |
| [IDC closure, line 272](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58#discussion_r4576466494) | Says the gate is **one hour**, not 30 minutes, before delivery. **Needs C-05.** Independently read the [V9.0 operating procedures](https://www.semopx.com/sites/semopx/files/2025-04/SEMOpx%20Operating%20Procedures%20V9.0.docx), linked by the live publisher page: “the Order Book will close in respect of a Trading Period one hour before the start of the relevant Trading Period”. This verifies the current reference, not its historical 2024 applicability. |
| [Input hash, line 514](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58#discussion_r4576466500) | Asks to exclude `[execution]` from canonical scenario TOML used for `input_identity_hash`. **Needs C-06.** ADR 0002's identity row requires separate input/execution hashes and its TOML example includes `[execution].profile_manifest`; hashing the whole TOML would change input identity when only execution settings change. |

The older threads are recorded here to correct the count, not used to reopen previously accepted R1/R2 dispositions. The three current concerns above are bounded documentation fixes, not grounds to repeat the wider acquisition review.

## Precise corrections

### C-01 — Restore the seven question fragment targets (R2-01)

In questionnaire Section P, insert the corresponding anchor on a separate line immediately before each existing task-list item; seven inserted lines total, no ID changes:

```html
<a id="q-p-01"></a>
<a id="q-p-02"></a>
<a id="q-p-03"></a>
<a id="q-p-04"></a>
<a id="q-p-05"></a>
<a id="q-p-06"></a>
<a id="q-p-07"></a>
```

### C-02 — Register price resampling (R2-01/R2-04)

Append these five content lines to Section P (Q-P-08 is unused at the reviewed head and main):

```markdown
<a id="q-p-08"></a>
- [ ] **Q-P-08 Fuel/carbon price resampling (M1 Deficit 6).**
  Monthly averages and auction observations require an imputation policy; see research Section 6. ASSUMPTION: resampled values are proxies, not observed spot prices.
  Options: (a) two-sided interpolation in reconstruction only, release-time step-holding for as-of · (b) release-time step-holding in both modes · (c) defer until daily observations are acquired.
  ★ (a), retaining original cadence and observed/imputed masks; every as-of value must have a verified publication timestamp no later than the knowledge cut, including auction-day observations.
```

Replace source line 342 with `- **ASSUMPTION:** [Q-P-08](../requirements/02-questionnaire.md#q-p-08) Resampling, cadence, and as-of policy (proposed).` At line 348, add that auction values become eligible only from their verified publication timestamp, subject to the same knowledge cut. Seven changed/inserted content lines total; do not amend the decisions log.

### C-03 — Reconcile STATUS and qualify the completion claim

When incorporating main into PR #58, retain main's complete R2 review entry and the earlier review history; insert the revision entry as a separate paragraph above them. Replace the revision summary's “to resolve all seven findings” with “to address R2-01–R2-07; questionnaire registration/navigation corrections remain pending”. Replace source line 564 with “This revision addresses R2-01–R2-07; questionnaire registration and navigation remain pending until the final re-check corrections are applied.” Remove the conflict markers. Update those pending statements when C-01/C-02 are actually applied; do not retroactively change the earlier exact-head verdicts.

### C-04 — Mark recommended 132 kV coverage partial

Append to source line 50: “**Coverage is partial:** NGED and SSEN do not cover all 14 DNO licence areas, and ETYS excludes England/Wales 132 kV circuits; the remaining DNO-area circuit baseline is incomplete pending dated LTDS acquisition and verification.” One changed line; no new source claim required.

### C-05 — Correct the current IDC gate

Replace source line 272 with: `  - *Intraday Continuous (IDC):* The current [SEMOpx Operating Procedures V9.0](https://www.semopx.com/sites/semopx/files/2025-04/SEMOpx%20Operating%20Procedures%20V9.0.docx) close the Order Book one hour before the start of each Trading Period; historical 2024 applicability remains UNVERIFIED pending dated pinning.` One changed line; retain the historical caveat at line 273.

### C-06 — Keep execution settings out of input identity

In source line 514, replace “canonicalized scenario TOML” with “canonicalized scenario TOML excluding the `[execution]` table”. Keep the entity parameters, time-series arrays and referenced dataset manifest hashes, and retain the separate execution hash definition. One changed line.

## Handoff

Apply C-01–C-06 on PR #58's branch and reconcile main before merging. This review PR contains only this report and a STATUS entry; it does not modify the reviewed proposal, post comments or resolve threads. No application tests or dataset completeness claims are appropriate for this documentation-only re-check.
