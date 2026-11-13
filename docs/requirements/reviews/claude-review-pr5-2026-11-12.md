# Review of PR #5: D-31 data spike and R-12 verdict

_Reviewer: Claude Opus 5.5 (cross-provider review under D-28 / D-54) · 2026-11-12 · Author of PR: GPT 6.1 Sol._
_Scope: PR #5 head `75bf34c` (`spikes/data/**`, `.gitignore`), against `main` at `8540dd5`._

Nothing outside this file was edited. PR #5 was not merged or changed.

## Verdict

**Merge after fixes.** No blockers. The licence reasoning is careful and correct on every clause I
checked, and no restricted network records are committed. The R-12 verdict "Partially" is sound.
The fixes are text changes to `REPORT.md` §7 (fallback options). That section is what the owner will
decide from, and it currently understates one option's dependency and leaves out a licence-clean option
that fits D-55.

## What I checked and how

| Check | Method | Result |
|---|---|---|
| Licence clauses | Fetched the official pages myself (list below) and NESO/NGED CKAN metadata via API | All six match the report |
| ETYS counts | Re-downloaded Appendix B to a temp dir outside the repo and re-ran an independent count | SHA-256 `d5cacf23…5e0c` matches the manifest; 8 base sheets, 2,864 branch rows, per-sheet counts identical |
| ETYS embedded terms | Read the workbook's `Note` and `Menu` sheets | No licence, copyright or reuse text inside the workbook |
| Committed data | `git diff --stat`, `git ls-tree -l`, read every file under `samples/` | 83 KB total; only cleared-source excerpts and aggregate counts (see §3) |
| Scripts | Read `fetch.sh`, `sources.tsv`, both Python scripts | No secrets, no share-alike sources; reproducibility gaps listed below |
| D-52 | Commit message, author, PR body, PR comments, all files | No attribution trailers or footers; one model name in `REPORT.md` (finding 9) |

### Licence spot-checks (official pages, 2026-11-12)

1. **NESO Open Data Licence** ([page](https://www.neso.energy/data-portal/neso-open-licence)): grants commercial
   exploitation, and "only applies to the datasets explicitly associated with this licence". Attribution
   "Supported by National Energy SO Open Data". No share-alike. CKAN confirms `NESO Open Data Licence` on
   historic demand, historic generation mix, TEC register and ETYS boundary GIS. **Report correct.**
2. **ETYS Appendix B vs NESO general terms**: the
   [appendices page](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices)
   states no licence. A CKAN search for "ETYS appendix" returns no dataset. The workbook carries no terms. The
   [website terms](https://www.neso.energy/terms-and-conditions) say NESO owns the content and users may not
   "commercially exploit" materials except as permitted by law. **Report correct.** It also uses the right
   wording: "UNVERIFIED / not cleared", not "prohibited". The coordinator's summary "not licensed for commercial
   reuse" is slightly stronger than the evidence. The accurate statement is "no commercial grant exists; the
   default terms restrict it; permission must be asked for".
3. **EirGrid disclaimer** ([page](https://www.eirgrid.ie/disclaimer)): materials may not be copied, reproduced or
   "used for any public or commercial purposes except as expressly provided for on this website" without being
   "expressly authorised in writing by EirGrid". **Report correct.** I did not re-check the TYTFS PDF copyright page
   (40 MB). The disclaimer alone is enough to block reuse.
4. **NGED LTDS**: the CKAN API returns `rights: "WPD Open Data Licence"`, `dataclassification: "Open"`,
   `isopen: true`. The [NGED Open Data Licence](https://commercial.nationalgrid.co.uk/open-data-licence)
   (December 2020, OGL-based) cl. 1.2 applies to data sets marked "Open". Cl. 3.3 allows commercial exploitation.
   Attribution "Supported by NGED Open Data". No share-alike. **Report correct.**
5. **Elexon BMRS licence** ([page](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)):
   "exploit the BMRS Data, including commercially". BMRS Data includes Insights Solution data. The licence applies
   to non-BSC parties. Attribution must propagate through sub-licences. Third-party rights are excluded. **Report
   correct.** Treating Market Index Data as UNVERIFIED (exchange-sourced) is the right caution.
6. **ERA5**: the [CDS dataset page](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=download)
   lists the licence as CC-BY. **Report correct.**

The reasoning is sound throughout. It does not let a portal-wide licence cover unrelated publications. It separates
"no grant found" from "prohibited". It also flags the BSC-party caveat, which matters if the owner's company ever
becomes a BSC party.

## Findings

Severity: **blocker** = must not merge as is · **major** = fix before merge · **minor** = fix now or in a follow-up.

### Blockers

None.

### Major

**1. Option B is presented as standalone but depends on option A.** (`REPORT.md` §7, row B)
A reduced ~300-node network that keeps transfer factors (Ward/Kron/PTDF-preserving reduction) has to be computed
*from* a full model. B therefore needs A's rights, plus most of A's assembly and reconciliation work, before the
4–8 person-weeks of reduction and validation can start. If rights are refused, B has no source at all. The row half
says this ("conditional on cleared source rights"). The effort column still makes B look like a cheaper alternative
to A, when it is really A plus extra work.
*Fix:* restate B as "A (rights + assembly) + 4–8 pw reduction", or relabel it "hand-built ~300-node model from
cleared sources", which is a different and weaker option close to C.

**2. The fallback set is missing a licence-clean intermediate option, and it does not reference D-55.**
D-55 (on `main`, after this branch's base) already decides that the MVP is "DC network plus proxy constraints",
with limits taken from NESO publications. That moves the MVP towards a boundary-constraint model. The report goes
straight from D (two zones: GB + SEM) to B (~300 nodes). It leaves out a **boundary-zone model**: roughly 20–40 GB
zones defined by the ETYS boundaries, plus the SEM. The pieces are already licence-clean:
- ETYS boundary GIS: NESO Open (report §6 already cleared it).
- `day-ahead-constraint-flows-and-limits` and `24-months-ahead-constraint-limits` on the NESO Data Portal: CKAN
  `license_id: ESO`, the same NESO Open licence id as the cleared datasets. Per-dataset confirmation is still needed.
- Zone membership can come from the cleared TEC register, BM-unit GSP groups, REPD/DUKES coordinates and the
  boundary polygons.

This can reproduce boundary-level constraint and BM redispatch costs (D-26), fits D-55, and leads into P3
zonal-pricing work. It does not need ETYS Appendix B or EirGrid permission.
*Fix:* add it as option F, with an effort estimate (I would guess 4–6 pw) and the fidelity limits: no intra-zone
congestion, and no voltage behaviour beyond the D-55 proxies. Cite D-55 in §7 and §8.

**3. England & Wales 132 kV coverage beyond NGED is not inventoried.**
NGED covers 4 of the 14 DNO licence areas. Under Ofgem's LTDS Form of Statement, every DNO now publishes LTDS
tables and/or CIM models. For example, UKPN publishes `ukpn-ltds-cim` and LTDS Table 1 on its open-data portal.
I did **not** verify UKPN's or any other DNO's licence. Third-party mirrors list the licence as "unknown". Still,
this is the most likely route to licence-clean E&W 132 kV data, and it changes the effort and feasibility of A and E.
*Fix:* add a per-DNO row (UKPN, NPg, ENWL, SPEN Manweb, SSEN Southern) to the §6 register and the §8 coverage gate,
with licence status UNVERIFIED until checked. Note that in Scotland 132 kV is transmission and is already in ETYS.

### Minor

**4. Option A's 8–16 person-weeks looks optimistic.** It covers ETYS + up to six DNO LTDS/CIM sets + Ireland:
ingest, GSP matching, status/date reconciliation and the electrical QA gate in §8.3. It does not say whether
reference validation against D-28's numerical gate is included. Either widen the range or state explicitly
that validation is excluded.

**5. Cleared-only reproduction is not possible.** `inspect_tabular.py` reads `market-index-day.json`,
`isem-flow-20240101.xml`, `etys-2025-appendix-b.xlsx` and `tytfs-2024.*` from `raw/`. These are all inspect-only.
So the cleared NESO/NGED/Elexon audit only runs after `fetch.sh --include-uncleared`. The README documents this,
but it ties a cleared result to fetching restricted files. Split the uncleared parts into their own script, or skip
missing files.

**6. `fetch.sh` does not verify hashes.** The manifest records SHA-256 values but nothing checks them. A
`shasum -a 256 -c` step, with a warning (not a failure) when a live file has changed, would make drift visible.

**7. NGED partial path is incomplete.** LTDS Table 2b (three-winding transformers) was not downloaded. The dataset's
`DSA095 - LTDS Tabular Data Sharing Assessment.pdf` was not read, and it may list redactions or caveats. CKAN also
marks the tabular dataset as "being superseded" by the CIM. Option E should target the CIM.

**8. ERA5 request variables.** This is fine for a template. For later work: `total_precipitation` is needed for
Scottish/Irish hydro inflow, and a direct/diffuse split (e.g. `total_sky_direct_solar_radiation_at_surface`)
improves PV modelling. Note this as an ASSUMPTION in §5.

**9. D-52: model name in a non-process file.** `REPORT.md` line 9 says "parallel Gemini catalogues". This is not
an authorship attribution. However, D-52 allows model names "only in process docs (e.g. team.md, review reports)",
and a spike report is not one. Reword to "parallel catalogue work". No `Co-Authored-By`, "Generated with" or 🤖
was found in the commit, PR body, PR comments or files.

**10. Branch is behind `main`.** The base is `8b70c67`; `main` now has D-54 and D-55. There are no conflicts
(PR touches only `spikes/` and `.gitignore`), but rebase so the report can cite D-55 (finding 2).

**11. Results are not carried into the requirements docs.** The owner questions sit in `REPORT.md` §8. AGENTS.md
rule 1 requires a question in `02-questionnaire.md` for each assumption, and R-12 in `05-risks.md` still says
"data spike must answer this first". The author was told not to edit those files. A follow-up PR should
(a) add Q-IDs for the three owner questions and the fallback choice (A–F), (b) update R-12 with the "Partially"
verdict and a link, and (c) update `STATUS.md`.

**12. Small script nit.** `inspect_tabular.py` writes the `nged-circuits.csv` sample twice: generic rows first,
then 132 kV circuits that overwrite them. This is harmless; drop the first write.

## Fallback options A–E: credibility summary

| Option | Estimate credible? | Fidelity claim credible? | Comment |
|---|---|---|---|
| A. Clear and assemble | Optimistic (finding 4) | Yes | Correctly warns that planning files ≠ today's grid |
| B. Reduced ~300 node | No, presented as standalone (finding 1) | Yes | Fidelity limits stated honestly |
| C. Synthetic | Plausible | Yes | Correctly notes that topology rights are still needed. Without OSM (D-30) and without cleared schematics, the topology source is unclear, so C may not be feasible at all |
| D. GB + SEM two-zone | Yes | Yes | Honest about the D-32 scope reduction |
| E. NGED regional pilot | Yes | Yes | Use the CIM, not the superseded tabular set (finding 7) |
| *F. Boundary-zone (missing)* | — | — | See finding 2; licence-clean today and fits D-55 |

## Restricted data: confirmed not committed

- `spikes/data/` totals 83,148 bytes, every file in `samples/` is under 5 KB, and there are no binaries.
- The CSV/JSON samples come only from NESO Open (demand, generation mix, TEC), NGED Open (LTDS) and the Elexon BMRS
  licence (BM units, demand, fuel, BOD, BOA). Each is 2 JSON records or 3 CSV rows, with attribution in `README.md`.
- From ETYS Appendix B and TYTFS 2024, only aggregate counts are committed (`network-statistics.json`): sheet row
  counts, voltage histograms and graph-component sizes. No bus IDs, names, impedances or ratings.
- No Market Index Data or SEMOpx XML excerpt is committed; only row counts.
- `download-manifest.json` contains byte sizes and SHA-256 values of restricted files, not their content.
- `.gitignore` excludes `/spikes/data/raw/` and `/spikes/data/.venv/`.
