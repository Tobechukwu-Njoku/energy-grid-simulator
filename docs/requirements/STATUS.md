# STATUS

_Last updated: 2026-11-26 UTC_

**Phase:** Build, M0 partial; code allowed under D-72. Requirements baselined;
ADRs 0001–0006 accepted. M0 closure still needs required-check deployment and
review evidence. PostgreSQL 18 + Valkey (D-67); CPU path primary. D-65 hardware,
OS and access remain unconfirmed. No heavy builds in the coding container.

**Read on demand:** [verbatim November history](status-history/2026-11.md),
[requirements](04-requirements.md), [team](../process/team.md),
[reference](../../context/reference.md), [worktree triage](../../archive/2026-11/worktrees.md).
The archive preserves historical claims, including superseded hardware/team state.

## Current queue

Coordinator directs GPT authorship, Claude independent review, verified Gemini
data/adversarial work and at most two delegates (`D-86`, pending PR #95).
No merge gate is waived. Current-head checks/reviews must be re-read before acting.

- #95 records team direction and authorized #92 history repair; reviews pending.
- #92 authorized squash pushed at `b27c66f55f96117c1863056b9ad0aa75fc2954c8`;
  full-range CI, fresh Claude/Gemini evidence, GPT triage and owner approval remain.
- #81 needs fresh independent/adversarial evidence, triage and exact-head owner
  approval. Do not merge on green build/scanning checks alone.
- #83 conflicts; preserve immutable review evidence during repair. #87 has failing
  review/docs gates and waits on #81. Decisions `D-78`–`D-85` are reserved in #87.
- #59/#60 await #87 decisions; #60 also has four optional Claude wording fixes.
- #53/#78 need relevance refresh; #78 must drop claims superseded by #81.
- This token-efficiency PR needs independent/adversarial review via the coordinator.
  Close #57 only after its outstanding owner tasks below are preserved on main.

## Waiting on the owner

Outstanding items carried from PR #57; credentials go through secret delivery,
never into chat, repository files or logs.

- **D-74:** separate machine account is configured. Owner supplies non-self approval
  on the exact protected-policy head after evidence is ready.
- **Required checks:** configure/verify `merge-gates`, `independent-review`,
  `policy-approval` and owner/code-owner behavior; record blocked-red/passing-green
  evidence. #78's older API audit reported enforcement off; recheck current settings.
- **D-65 / Q-N-7 / Q-H-7:** confirm hardware, OS and access; authorize runner
  installation and NFR-12 isolation tests before heavy B1 work.
- **Accounts:** ENTSO-E Transparency REST API token; Copernicus CDS account,
  accepted ERA5 terms and API configuration; UKPN, SPEN, NPg and ENWL portal logins.
  Optional: CEDA account for MIDAS Open; Renewables.ninja account/token if selected.
- **Security:** enable private vulnerability reporting on GitHub.
- **Q-P-01–08:** rating proxies; micro-PV interpolation/scaling; HVDC loss model;
  clearing proxy versus unit commitment; thermal efficiency/VOM; wind shear;
  PV temperature derating; fuel/carbon resampling. Proposed answers await #87;
  use [questionnaire section P](02-questionnaire.md), never infer acceptance.

## Next steps (agents may do now)

Finish current-head review evidence and policy repair; verify required-check
rollout; then refresh the M0/M1 backlog. M1 keepers are on `salvage/2026-11/` branches;
unsafe legacy edits remain intact in the linked triage record. HANDOFF is local and coordinator-owned.

<a id="snapshot-of-whats-decided"></a>
<a id="still-open-non-blocking"></a>
<a id="work-state-coordinator-claude-opus-55--2026-11-14"></a>
Historical section anchors are retained here; their contents are in the archive.
