# Task Status Board

_Last updated: 2026-11-15_

Current task status board for the energy grid simulator project across active threads, models, and owner inputs. Statuses are verified against the repository and GitHub PR states. Under D-76, GPT 6.1 Sol (medium effort) writes code in place of Claude Opus.

## Tasks

| ID | Task | Owner / Model | Status | PR or Link | Blocker | Last Updated |
|---|---|---|---|---|---|---|
| T-1 | M0 CI merge gates & Cargo workspace: revision 15 fixes | GPT 6.1 Sol (medium) / Coordinator (verification & merge) | running | [#23](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/23) (head `60bef0d`) | Revision 15 pushed (`c027ab7`), main merged in (`60bef0d`); CI running. | 2026-11-15 |
| T-2 | Cumulative triage record of PR #23 review findings | GPT 6.1 Sol | in review | [#53](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/53) (`t3/pr23-review-triage-record`) | Waiting on owner decision: merge now, or hold for Claude Opus review (D-54). Merge conflict against main to resolve. | 2026-11-15 |
| T-3 | M1 open-data acquisition plan (43 public-scope entries) | GPT 6.1 Sol | in review | [#60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60) (`docs/m1-acquisition-plan`) | Under D-54 needs Claude review; Claude work is paused by the owner. | 2026-11-15 |
| T-4 | Candidate sources for the 9 M1 data deficits | Gemini Flash 3.8 | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (merged `1734f39`) | Six round-3 corrections applied; merged. | 2026-11-15 |
| T-5 | M1 acceptance test spec (34 tests, 10 requirements covered, 8 flagged items) | GPT 6.1 Sol | in review | [#59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59) (`docs/m1-acceptance-tests`) | Under D-54 needs Claude review; Claude work is paused by the owner. | 2026-11-15 |
| T-6 | Follow-up issues for deferred PR #23 findings | GPT 6.1 Sol / Coordinator | todo | — | Blocked on completion and merge of T-1 (PR #23). | 2026-11-15 |
| T-7 | GPT fact-check of PR #58 | GPT 6.1 Sol | done | [#61](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/61) (merged) | Reject verdict with 16 findings; review report merged. | 2026-11-15 |
| T-8 | Revise PR #58 against 16 review findings | Gemini (Homelab) | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (head `67a9d3d`) | Addressed 16 review findings from PR #61. | 2026-11-15 |
| T-9 | GPT re-check of PR #58 | GPT 6.1 Sol | done | [#62](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/62) (merged) | Reject verdict with 7 findings; review report merged. | 2026-11-15 |
| T-10 | Gemini (Personal) revises PR #58 for round 2 | Gemini (Personal) | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (head `739bb70`) | Addressed 7 round-2 findings; added Section P to questionnaire. | 2026-11-15 |
| T-11 | GPT round-3 re-check of PR #58 | GPT 6.1 Sol | done | [#63](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/63) (merged) | Accept-with-corrections verdict; review report merged. | 2026-11-15 |
| T-12 | Verify PR #23 against round-14 reproductions | GPT 6.1 Sol (high) | running | Branch `review/pr23-r15-verify` | — | 2026-11-15 |

## Waiting on the owner

### Active Task Decisions
- **T-2 (PR #53)**: Owner decision needed whether to merge the cumulative triage record now or hold for Claude Opus cross-provider review per D-54 (with merge conflict resolution against main).
- **T-3 (PR #60)**: Requires Claude Opus cross-provider review under D-54, but Claude work is currently paused by the owner.
- **T-5 (PR #59)**: Requires Claude Opus cross-provider review under D-54, but Claude work is currently paused by the owner.

### Open Questionnaire Items (Section P — M1 Deficit Methodologies)
- **Q-P-01**: Post-fault to continuous thermal rating proxy (M1 Deficit 1: universal 0.88 multiplier vs asset-class proxies vs explicit continuous ratings under D-71).
- **Q-P-02**: ROI micro-PV capacity interpolation and rooftop scaling (M1 Deficit 3: linear interpolation between annual endpoints with utility profile scaling vs monthly steps vs independent model).
- **Q-P-03**: HVDC interconnector loss formulation and parameters (M1 Deficit 4: calibrated three-term power-loss model vs constant exploratory proxy vs commercial deemed factors).
- **Q-P-04**: Wholesale market clearing proxy vs unit commitment (M1 Deficit 5: baseline MILP unit commitment per D-26 / FR-7 vs exploratory continuous SRMC merit-order proxy).
- **Q-P-05**: Thermal generation efficiency and variable O&M defaults (M1 Deficit 6: author-proposed technology proxies from DESNZ projections vs calibrated plant-specific heat rates).
- **Q-P-06**: Wind speed hub-height scaling shear exponent (M1 Deficit 7: multi-level ERA5 fields vs constant power-law shear exponents alpha=0.143 onshore / 0.11 offshore).
- **Q-P-07**: Solar PV temperature derating coefficient (M1 Deficit 7: constant gamma = 0.004 / deg C default vs manufacturer-specific coefficients).
- **Q-P-08**: Fuel/carbon price resampling (M1 Deficit 6: two-sided interpolation in reconstruction vs release-time step-holding for as-of vs deferral).

### Owner To-Dos
- **Bot account creation (D-74)**: Create the GitHub machine account `egs-bot` and generate its personal access token (PAT) for automated PR pushes and opening.
- **Branch protection policy**: Decide on repository branch protection rules on GitHub (required status checks, approvals, merge strategies).
- **Target system provisioning (D-65)**: Provide target system specs, credentials, and access for the dedicated target environment (AMD Ryzen 9 5900X / Intel Core Ultra, 128 GB RAM) for heavy builds, databases, benchmarks, and GPU runner.
- **External portal accounts and API logins**: Obtain logins and API credentials for M1 data sources:
  - **ENTSO-E Transparency Platform**: Personal REST API security token (`transparency@entsoe.eu`).
  - **Copernicus Climate Data Store (CDS)**: Registered account, accept ERA5 terms, and configure API key in `~/.cdsapirc`.
  - **Distribution Network Operators (DNOs)**: Open data portal accounts for UK Power Networks (UKPN), SP Energy Networks (SPEN), Northern Powergrid (NPg), and Electricity North West Limited (ENWL).
  - **CEDA Archive**: User registration for Met Office MIDAS Open surface observations.
  - **Renewables.ninja**: User account and API token.
- **Security configuration**: Enable private vulnerability reporting on the GitHub repository.
