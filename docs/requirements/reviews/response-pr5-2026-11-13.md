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
replace full-network coverage. Questions **Q-C-11, Q-C-12 and Q-M-4** record
unresolved snapshot/connecting-voltage, weather and fallback/effort assumptions.
Q-L-7 is marked answered from existing D-57; no decision-log edit is needed.

## Document-review dispositions

| Finding | Resolution |
|---|---|
| 1. B depends on A | Fixed §7: B requires A's access/assembly/reconciliation **plus 4–8 person-weeks**, not a standalone cheaper route. Research-only inputs retain the same commercial-clearance obligations. |
| 2. Missing boundary-zone option / D-55 | Added **F**, indicative 4–6 person-weeks, ~20–40 candidate GB zones plus SEM/HVDC. Independently checked both suggested NESO constraint datasets: `license_id: ESO`, NESO Open Data Licence. Fidelity exclusions and D-60 scope-approval caveat are explicit. |
| 3. Other DNO 132-kV sources | Added UKPN, NPg, ENWL, SP Manweb and SSEN Southern register rows and explicit §8 coverage gates. Their actual file licences/access remain UNVERIFIED here pending independent checks of PR #10. Scottish 132 kV is transmission, not missing DNO data. |
| 4. A effort understated | Widened to **16–32 person-weeks** for data assembly/reconciliation. All estimates explicitly exclude D-28 independent numerical acceptance, external access delays, procurement and engine/UI work. These are assumptions, not promises. |
| 5. Cleared-only reproduction | `inspect_tabular.py` skips optional missing MID/XML and missing manifest sources; isolated cleared-only execution passes without ETYS/Irish restricted datasets. The separate network inspector still intentionally requires its research inputs. |
| 6. Fetch lacks hash verification | Added `verify_hashes.py`, called by the fetcher. Existing pins are checked; live drift warns without failing. README tells users to inspect drift before refreshing samples/pins. |
| 7. NGED Table 2b / assessment / CIM | Downloaded and counted **69** three-winding records; read **DSA095**, approved 2 December 2025, Open recommendation with amber residual security risk, no stated row-level completeness guarantee. Added exact source URLs/hashes and targeted successor CIM in E. No new raw/binary sample is committed. |
| 8. Hydro/PV weather fields | Added precipitation and total-sky direct solar radiation at surface, labelled candidate inputs, not validated hydro/PV models. Q-C-12 records the assumption and enlarged footprint. |
| 9. Non-process attribution wording | Removed the model name from REPORT.md; no attribution footer/header/trailer is added. |
| 10. Behind main | Merged main as explicitly requested; no rebase/force push. |
| 11. Requirements handoff | Updated R-12 in the risk register and added stable questionnaire IDs/defaults for open assumptions. STATUS and the append-only decisions log are deliberately untouched by authored changes. |
| 12. Double sample write | Removed the generic NGED circuit sample write; only the filtered 132-kV circuit excerpt is emitted. |

The separate inline numeric-coverage finding is also fixed: all R1/X1/B1/Winter/
Summer values must parse as finite numbers. Missing/whitespace/text/NaN/infinity
fixtures are rejected; the regenerated observed count remains **1,138**.

## Adversarial-review dispositions

| ID | Resolution against evidence and coordinator triage |
|---|---|
| ADV5-1 | **Rejected as triaged.** A presumed-open regulatory duty is not a dataset licence grant. D-56 removes the research blocker; PC-1 still covers commercial ETYS rights. |
| ADV5-2 | **Partly accepted as triaged.** Expanded every relevant DNO coverage row and linked PR #10. Do not inherit the unsupported blanket claim that all 14 DNO areas have permissively licensed 132-kV electrical models; Scotland's ownership boundary, access gates and ENWL terms matter. |
| ADV5-3 | **Accepted, verified with narrower licence scope.** Added I-SEM A44, physical A11 and scheduled A09 inputs, correct EIC, owner-token procedure and a complete-year end bound. Official CC-BY free-reuse list includes energy prices/exchanges/physical flows but has IFA/Nemo and other provider/item exceptions. No complete 2024 series or universal interconnector grant is claimed without authenticated retrieval. |
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
licence for excluded flow providers. Uncertain flows remain research-only/PC-5.

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

All three remain open PRs at this revision. No raw gathered archive is added to
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
archives and every DNO file; Q-C-11/Q-C-12/Q-M-4; D-28 numerical acceptance once
engine work is authorised; and PC-1–PC-5 clearance before commercial activity.
