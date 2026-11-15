# Task Status Board

_Last updated: 2026-11-15_

Current task status board for the energy grid simulator project across active threads, models, and owner inputs. Statuses are verified against the repository and GitHub PR states.

## Tasks

| ID | Task | Owner / Model | Status | PR or Link | Blocker | Last Updated |
|---|---|---|---|---|---|---|
| T-1 | M0 CI merge gates & Cargo workspace: revision 15 fixes | Claude Opus 5.5 (fixes) / Coordinator (verification & merge) | blocked | [#23](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/23) (`t3/m0-cargo-workspace-ci`) | Revision 15 is PAUSED by the owner. After unpause: address open round-14 findings, verify against reviewer reproductions, commit bootstrap seed, and merge per D-75. | 2026-11-15 |
| T-2 | Cumulative triage record of PR #23 review findings | GPT 6.1 Sol | in review | [#53](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/53) (`t3/pr23-review-triage-record`) | Waiting on owner decision: merge now, or hold for Claude Opus review (D-54). Merge conflict against main to resolve. | 2026-11-15 |
| T-3 | M1 open-data acquisition plan (43 public-scope entries) | GPT 6.1 Sol | in review | [#60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60) (`docs/m1-acquisition-plan`) | Under D-54 needs Claude review; Claude work is paused by the owner. | 2026-11-15 |
| T-4 | Candidate sources for the 9 M1 data deficits | Gemini Flash 3.8 | revising | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (`docs/m1-deficit-sources`) | Revising under T-8 against 16 findings from PR #61; GPT re-check to follow. | 2026-11-15 |
| T-5 | M1 acceptance test spec (34 tests, 10 requirements covered, 8 flagged items) | GPT 6.1 Sol | in review | [#59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59) (`docs/m1-acceptance-tests`) | Under D-54 needs Claude review; Claude work is paused by the owner. | 2026-11-15 |
| T-6 | Follow-up issues for deferred PR #23 findings | GPT 6.1 Sol / Coordinator | todo | — | Blocked on completion and merge of T-1 (PR #23). | 2026-11-15 |
| T-7 | GPT fact-check of PR #58 | GPT 6.1 Sol | done | [#61](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/61) (merged) | Reject verdict with 16 findings; review report merged. | 2026-11-15 |
| T-8 | Revise PR #58 against 16 review findings | Gemini (Homelab) | running | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (`docs/m1-deficit-sources`) | — | 2026-11-15 |

## Waiting on the owner

### Active Task Decisions
- **T-1 (PR #23)**: Owner unpause needed for Claude Opus revision 15 to resolve the 12 open round-14 findings, allow coordinator verification against reviewer reproductions, commit the one-time bootstrap seed, and merge under D-75.
- **T-2 (PR #53)**: Owner decision needed whether to merge the cumulative triage record now or hold for Claude Opus cross-provider review per D-54 (with merge conflict resolution against main).
- **T-3 (PR #60)**: Requires Claude Opus cross-provider review under D-54, but Claude work is currently paused by the owner.
- **T-5 (PR #59)**: Requires Claude Opus cross-provider review under D-54, but Claude work is currently paused by the owner.

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
