# Worktree triage — 2026-11-26 UTC

Scope: shared Energy workspace; paths below are relative to its root unless
marked otherwise. No worktree, branch, local edit, data or checkpoint was deleted.
No `refs/t3` checkpoint was pushed. No raw dataset was staged.

## Safety decisions

T3 activity was inspected across all 175 project threads (two list pages), plus
the live coordinator messages. Root, this checkout, the #92 execution checkout
and **all review checkouts** are retained. Active #92 independent review was
observed; idle older threads are not proof that a checkout is disposable.

The older workspace contains staged scanner rewrites, immutable-report deletions,
untracked reproductions and potentially secret-like fixtures. Bulk salvage/push
or removal would require a separate content audit. All such paths remain intact
below; there are **no discards** to approve or recover. Clean historical branch
tips often have no current upstream ref; retain their local refs/checkouts until
publication provenance is proven. A stale-looking path alone is not authority
to discard it.

Only explicit workspace-local ignore entries were added for the two new nested
checkouts and HANDOFF; no broad ignore for source documentation. Tracked files
are unaffected. Existing `t3-*` excludes remain. The committed ignore rules also
cover future `worktrees/` checkouts. The local coordinator HANDOFF content was
not edited, copied into Git or reformatted.

## Keeper publication

`docs/m1-acquisition-plan` and `docs/m1-acceptance-tests` each had three local
commits ahead of their corresponding remote snapshot. Scan their full unpublished
ranges before a fast-forward push; never force these branches. Publication does
not imply approval, docs-gate success or readiness to merge. Results follow below.

## Inventory and exact remaining paths

Snapshot taken before this thread's own docs edits. `no upstream` means no
same-named local remote-tracking ref, not proof that no remote contains the work.
All entries below are retained unless an explicit later disposition is recorded.

| Checkout | Branch / head | Unpublished count | Remaining work |
|---|---|---|---|
| `Dev - GitHub - Energy` | `docs/team-direction-2026-11-26` / `9198aced4178` | 0 | dirty; exact paths below |
| `docs-token-efficiency-2-a8ca93f2b12e2ab9` | `docs/token-efficiency-2026-11` / `d888c47f0c06` | no upstream | clean; retained |
| `fix-pr92-history-repair-9940c19feaa9aeef` | `(detached)` / `b27c66f55f96` | no upstream | clean; retained |
| `pr23-r12-inspect` | `(detached)` / `d996aac5b9d4` | no upstream | dirty; exact paths below |
| `pr23-r13-inspect` | `(detached)` / `9db2a40a88aa` | no upstream | dirty; exact paths below |
| `pr23-r14-inspect` | `(detached)` / `112de8d2b745` | no upstream | dirty; exact paths below |
| `pr23-r9-inspect` | `(detached)` / `e64acc5cfb73` | no upstream | dirty; exact paths below |
| `t3-088e279a-319c327894ac6700` | `t3/adversarial-review-pr5-spike` / `979608adb662` | no upstream | clean; retained |
| `t3-0c93eaab-cdb10bdbdacd0a60` | `t3/pr23-review-round-5` / `346ed6f5f2c6` | no upstream | dirty; exact paths below |
| `t3-19c8ced0-5d817bae28bdb04e` | `review/gpt-pr75-2026-11-18-evidence-loop` / `334a2be03935` | no upstream | clean; retained |
| `t3-1ebe7f49-d068c020340bbd4e` | `t3/semo-2024-price-data-gathering` / `494961bd88c0` | no upstream | clean; retained |
| `t3-1fbbe428-d0d57af87ca36680` | `review/pr60-gemini-adversarial` / `4fc1e7f3b343` | no upstream | clean; retained |
| `t3-210d476e-83d3a9f3928d0297` | `review/claude-pr17` / `11a25f4c7ac7` | no upstream | clean; retained |
| `t3-24e84e6b-edf59d7ca6059600` | `docs/claude-review-pr75` / `5a3036b0db05` | no upstream | clean; retained |
| `t3-27e0af67-731b3e568c96fe47` | `review/claude-pr92` / `cc819a9f8dcf` | no upstream | clean; retained |
| `t3-304a35c7-e3939a2b142de128` | `t3/review-evidence-loop-fixes` / `b39bfcf3caed` | no upstream | dirty; exact paths below |
| `t3-30eae792-259d8da708639384` | `docs/m1-acquisition-plan` / `c36ef371964b` | 3 | clean; retained |
| `t3-311c9dd3-626f718e94b682db` | `t3/pr23-review-round-6` / `5fdac339c7ea` | no upstream | dirty; exact paths below |
| `t3-4ad88010-d2cde769d5af537e` | `(detached)` / `e5ec7171e2d1` | no upstream | clean; retained |
| `t3-4c15d953-3b87b25573b4b7cd` | `t3/r12-transmission-data-spike` / `6817f4228a08` | no upstream | clean; retained |
| `t3-4d66913d-1412448fcd7466c1` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-555d01cb-5d077076a38e3197` | `docs/review-pr60-verify` / `b892ae7ea5e1` | no upstream | clean; retained |
| `t3-6c68b84a-e2bf01edcb110e3b` | `fix/xtask-policy-review-followups` / `e651c1c4d1f8` | no upstream | clean; retained |
| `t3-6d439957-71a6b81c34e287b4` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-7dd81420-2ef153ee44078d0e` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-7f37b10d-f7775d69251e235a` | `t3/verify-pr18-isem-prices` / `0a142656259c` | no upstream | clean; retained |
| `t3-7f6737c6-bd854cc6f2b32f75` | `docs/review-pr60-final` / `98169223dfbf` | no upstream | clean; retained |
| `t3-8a751807-7cdac9de93910732` | `t3/adversarial-review-pr4-rev2` / `4317a89a4a43` | no upstream | clean; retained |
| `t3-8b8c1822-87b75c41339c8999` | `docs/gemini-adversarial-review-pr81-1795016347` / `4748777e67c3` | no upstream | clean; retained |
| `t3-8b9e9e6a-9c8377277652ec5b` | `t3/design-review-mvp-requirements-adrs` / `6c6a9eb23de9` | no upstream | clean; retained |
| `t3-8e8e341a-eb19e73e018829d8` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-8ebc6e15-53edd422accf4792` | `(detached)` / `e7e992e3f213` | no upstream | clean; retained |
| `t3-9b40bc81-08f5faf60a047b26` | `docs/triage-gemini-pr81-2026-11-18` / `5ed8137982f2` | no upstream | clean; retained |
| `t3-9c679c19-a4a1955369288ed0` | `t3/recheck-pr17-r2` / `412860d4884d` | no upstream | clean; retained |
| `t3-b4a34d84-25fc67598032ac3d` | `t3/pr5-rev2-review` / `0d360850244f` | no upstream | clean; retained |
| `t3-b6ea257d-8c53e6c827afe2bb` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-bf9de70d-2dbcd554114f8f68` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-c5892f25-366c27742112755e` | `pr23-r15` / `efb9fe57c458` | no upstream | clean; retained |
| `t3-c5c88744-d99fadc9d7e113e5` | `docs/m1-acceptance-tests` / `e6fd14914fc3` | 3 | clean; retained |
| `t3-c5d4fc54-5d10302147b8bf49` | `(detached)` / `ad42c1cf9367` | no upstream | dirty; exact paths below |
| `t3-ca80ffee-c9d6965995ef54e6` | `t3/elexon-gb-2024-data-gathering` / `c105546349ae` | no upstream | clean; retained |
| `t3-cadf7a82-db00b4cae9e3c285` | `docs/claude-pr81-verify` / `a2bf317140f3` | no upstream | clean; retained |
| `t3-cb3e5e9f-9a543285dbe95594` | `fix/xtask-deferred-policy-followups` / `1b5007dc9971` | 2 | dirty; exact paths below |
| `t3-d05bbb7b-b96114d91ece77f4` | `t3/adversarial-review-pr23-m0` / `b031458b323a` | no upstream | clean; retained |
| `t3-d171b3f7-cf5bc1975b96118c` | `review/pr4-r3-recheck` / `abc2939017b3` | no upstream | clean; retained |
| `t3-d63b5fdb-3c69238e61dc06c6` | `t3/fix-pr92-review-followups` / `2af57e71e6d6` | no upstream | dirty; exact paths below |
| `t3-d8f2bf9a-bd0a3b5b5ba89442` | `(detached)` / `d99ceabbc068` | no upstream | clean; retained |
| `t3-dbeace22-12291e4802ca199e` | `review/pr23-independent-2026-11-13` / `d158aee14dab` | no upstream | clean; retained |
| `t3-ddabe8ef-e55e7fa0cb250352` | `fix/pr92-completion` / `51d43cbd8032` | no upstream | clean; retained |
| `t3-e134499b-40078be01c3c84b3` | `docs/claude-review-pr81` / `cf2a28328f7e` | no upstream | clean; retained |
| `t3-f2111f4e-98f02ed405a6e144` | `t3/design-pr4-pr17-consistency` / `9678e0c7b0a1` | no upstream | clean; retained |
| `t3-f76adbe7-271d64dc09076890` | `review/pr4-rev2-claude` / `fae1559dc638` | no upstream | clean; retained |
| `t3-fcb3ebaf-191ac8729c5a83dd` | `t3/m0-cargo-workspace-ci` / `260d5f9410ca` | no upstream | dirty; exact paths below |
| `t3-fd248ea6-1fb53fc6a3e1aae6` | `t3/design-adrs-0002-0006` / `210401cdbe62` | no upstream | clean; retained |
| `t3-fd2b0c32-7710c06367e34792` | `t3/gemini-adversarial-review-pr17` / `cda912db6dc7` | no upstream | clean; retained |
| `t3-fdda40d0-5df4f023038196d9` | `t3/review-pr-95-d86-d87` / `9198aced4178` | no upstream | clean; retained |
| `../pr23m (outside root)` | `(detached)` / `624489c09e00` | no upstream | clean; retained |

### Dev - GitHub - Energy

```text
?? docs-token-efficiency-2-a8ca93f2b12e2ab9/
?? fix-pr92-history-repair-9940c19feaa9aeef/
```

### pr23-r12-inspect

```text
?? review-all.log
?? review-attribution.log
?? review-build-gate.log
?? review-clippy.log
?? review-doc-baseline/
?? review-docs-base.log
?? review-docs-report.log
?? review-hook-message.txt
?? review-pr-body.txt
?? review-pr-history.json
?? review-pr-metadata.json
?? review-r11-build.log
?? review-r11-control.log
?? review-replay-results.json
?? review-replay.log
?? review-tests.log
?? review_replay.py
```

### pr23-r13-inspect

```text
?? review-all.log
?? review-artifacts/
?? review-attribution.log
?? review-build-gate.log
?? review-clippy.log
?? review-deny.log
?? review-tests.log
```

### pr23-r14-inspect

```text
?? libpq5_17.11-0+deb13u1_amd64.deb
?? putty-tools_0.83-3_amd64.deb
?? review-all.log
?? review-build-gate.log
?? review-clippy.log
?? review-deny.log
?? review-extra.json
?? review-extra.log
?? review-extra.py
?? review-fmt.log
?? review-history.log
?? review-libpq-download.log
?? review-libpq/
?? review-main-docs.log
?? review-putty-download.log
?? review-putty/
?? review-record-probe.json
?? review-record-probe.log
?? review-record-probe.py
?? review-replay.json
?? review-replay.log
?? review-replay.py
?? review-report-docs.log
?? review-scratch/
?? review-tests.log
?? review-yaml.rs
```

### pr23-r9-inspect

```text
?? genuine-event.json
?? hook-replay.txt
?? hosted-failures.log
?? launder-body.txt
?? launder-history.json
?? live-body.txt
?? live-event.json
?? live-history.json
?? live-replays.json
?? probe-body.txt
?? probe-history.json
?? review-build-gate.log
?? review-fixture/
?? review-tests.log
?? spell-replays.json
?? target-gate-review/
?? target-review-r9/
```

### t3-0c93eaab-cdb10bdbdacd0a60

```text
?? docs/requirements/reviews/gpt-2026-11-13-pr23-r5.md
```

### t3-304a35c7-e3939a2b142de128

```text
 M docs/requirements/03-decisions-log.md
 M docs/requirements/STATUS.md
```

### t3-311c9dd3-626f718e94b682db

```text
 M docs/requirements/STATUS.md
?? docs/requirements/reviews/gpt-2026-11-13-pr23-r6.md
```

### t3-4d66913d-1412448fcd7466c1

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/STATUS.md
M  docs/requirements/reviews/gemini-2026-11-13-pr23-r2.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
M  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify.md
M  xtask/Cargo.toml
D  xtask/policy/history-secret-exemptions.toml
M  xtask/policy/repository.toml
M  xtask/policy/secret-rules.toml
M  xtask/src/repository.rs
M  xtask/src/secrets.rs
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-6d439957-71a6b81c34e287b4

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/STATUS.md
M  docs/requirements/reviews/gemini-2026-11-13-pr23-r2.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify.md
M  xtask/Cargo.toml
D  xtask/policy/history-secret-exemptions.toml
M  xtask/policy/repository.toml
M  xtask/policy/secret-rules.toml
M  xtask/src/repository.rs
M  xtask/src/secrets.rs
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-7dd81420-2ef153ee44078d0e

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/STATUS.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
M  xtask/Cargo.toml
D  xtask/policy/history-secret-exemptions.toml
M  xtask/policy/secret-rules.toml
M  xtask/src/repository.rs
M  xtask/src/secrets.rs
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-8e8e341a-eb19e73e018829d8

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/STATUS.md
M  docs/requirements/reviews/gemini-2026-11-13-pr23-r2.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r13.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
M  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify.md
M  xtask/Cargo.toml
D  xtask/policy/history-secret-exemptions.toml
M  xtask/policy/repository.toml
M  xtask/policy/secret-rules.toml
M  xtask/src/repository.rs
M  xtask/src/secrets.rs
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-b6ea257d-8c53e6c827afe2bb

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/STATUS.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
M  xtask/Cargo.toml
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-bf9de70d-2dbcd554114f8f68

```text
M  Cargo.lock
M  Cargo.toml
M  docs/requirements/02-questionnaire.md
M  docs/requirements/STATUS.md
M  docs/requirements/reviews/gemini-2026-11-13-pr23-r2.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md
M  docs/requirements/reviews/gemini-2026-11-14-pr23-r13.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
M  docs/requirements/reviews/gpt-2026-11-14-pr23-r13.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r3.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources-r2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources-r3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources.md
D  docs/research/m1-data-inventory.md
D  docs/research/m1-deficit-sources.md
M  xtask/Cargo.toml
D  xtask/policy/history-secret-exemptions.toml
M  xtask/policy/repository.toml
M  xtask/policy/secret-rules.toml
M  xtask/src/repository.rs
M  xtask/src/secrets.rs
M  xtask/src/secrets/credentials.rs
M  xtask/tests/policy.rs
```

### t3-c5d4fc54-5d10302147b8bf49

```text
 M docs/requirements/02-questionnaire.md
 M docs/requirements/m1-acceptance-tests.md
```

### t3-cb3e5e9f-9a543285dbe95594

```text
 M .githooks/attribution-patterns.regex
 M .githooks/tests/fixtures.json
 M xtask/src/attribution.rs
 M xtask/src/secrets/credentials.rs
 M xtask/tests/hook.rs
```

### t3-d63b5fdb-3c69238e61dc06c6

```text
 M .githooks/attribution-patterns.regex
 M .githooks/tests/fixtures.json
 M docs/architecture/adr/0001-repo-layout-and-ci-gates.md
 M docs/requirements/STATUS.md
 M xtask/src/attribution.rs
 M xtask/src/secrets/credentials.rs
 M xtask/tests/hook.rs
 M xtask/tests/policy.rs
```

### t3-fcb3ebaf-191ac8729c5a83dd

```text
D  .cargo/config.toml
D  .claude/skills/add-dataset/SKILL.md
D  .claude/skills/delegate-code/SKILL.md
D  .claude/skills/record-decision/SKILL.md
D  .claude/skills/review-round/SKILL.md
M  .githooks/attribution-patterns.regex
M  .githooks/commit-msg
M  .githooks/tests/fixtures.json
A  .githooks/tests/run.py
D  .github/CODEOWNERS
D  .github/actions/trusted-policy/action.yml
D  .github/actions/trusted-policy/load.sh
M  .github/workflows/ci.yml
D  .github/workflows/trusted-gates.yml
M  .gitignore
M  CONTRIBUTING.md
M  Cargo.lock
M  Cargo.toml
A  ci/__pycache__/doc_gates.cpython-313.pyc
A  ci/attribution-exemptions.txt
A  ci/attribution-match.pl
A  ci/attribution_policy.py
A  ci/doc_gates.py
A  ci/policy_checks.py
A  ci/tests/__pycache__/test_attribution.cpython-313.pyc
A  ci/tests/test_attribution.py
M  crates/api/Cargo.toml
M  crates/api/src/lib.rs
M  crates/cli/Cargo.toml
M  crates/cli/src/main.rs
M  crates/commitment/Cargo.toml
M  crates/commitment/src/lib.rs
M  crates/domain/Cargo.toml
M  crates/domain/src/lib.rs
M  crates/engine/Cargo.toml
M  crates/engine/src/lib.rs
M  crates/ingest/Cargo.toml
M  crates/ingest/src/lib.rs
M  crates/market/Cargo.toml
M  crates/market/src/lib.rs
M  crates/network/Cargo.toml
M  crates/network/src/lib.rs
M  crates/optimisation/Cargo.toml
M  crates/optimisation/src/gap.rs
M  crates/optimisation/src/lib.rs
M  crates/optimisation/src/status.rs
M  crates/pricing/Cargo.toml
M  crates/pricing/src/lib.rs
M  crates/redispatch/Cargo.toml
M  crates/redispatch/src/lib.rs
M  crates/scenario/Cargo.toml
M  crates/scenario/src/lib.rs
M  crates/sdk/Cargo.toml
M  crates/sdk/src/lib.rs
M  crates/solver-cuopt/Cargo.toml
M  crates/solver-gurobi/Cargo.toml
M  crates/solver-highs/Cargo.toml
M  crates/solver-xpress/Cargo.toml
M  deny.toml
M  docs/architecture/adr/0001-repo-layout-and-ci-gates.md
M  docs/requirements/02-questionnaire.md
M  docs/requirements/03-decisions-log.md
M  docs/requirements/REVIEW-BRIEF.md
M  docs/requirements/STATUS.md
D  docs/requirements/reviews/gemini-2026-11-13-pr23-r2.md
D  docs/requirements/reviews/gemini-2026-11-13-pr23-r3.md
D  docs/requirements/reviews/gemini-2026-11-13-pr23-r4.md
D  docs/requirements/reviews/gemini-2026-11-13-pr23-r5.md
D  docs/requirements/reviews/gemini-2026-11-13-pr23-r6.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r10.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r11.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r13.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r7.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r8.md
D  docs/requirements/reviews/gemini-2026-11-14-pr23-r9.md
D  docs/requirements/reviews/gemini-adversarial-pr23-2026-11-13.md
D  docs/requirements/reviews/gpt-2026-11-13-pr23-r2.md
D  docs/requirements/reviews/gpt-2026-11-13-pr23-r3.md
D  docs/requirements/reviews/gpt-2026-11-13-pr23-r4.md
D  docs/requirements/reviews/gpt-2026-11-13-pr23-r5.md
D  docs/requirements/reviews/gpt-2026-11-13-pr23-r6.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r10.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r11.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r12.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r13.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r14.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r7.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r8.md
D  docs/requirements/reviews/gpt-2026-11-14-pr23-r9.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r3.md
D  docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-4.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify-5.md
D  docs/requirements/reviews/gpt-2026-11-15-pr23-r15-verify.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources-r2.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources-r3.md
D  docs/requirements/reviews/gpt-2026-11-15-pr58-deficit-sources.md
D  docs/requirements/reviews/gpt-review-pr23-2026-11-13.md
D  docs/research/m1-data-inventory.md
D  docs/research/m1-deficit-sources.md
M  docs/security/ci-runner-policy.md
D  xtask/Cargo.toml
D  xtask/policy/attribution-exemptions.toml
D  xtask/policy/gates.toml
D  xtask/policy/gitleaks-LICENSE.txt
D  xtask/policy/gitleaks.toml
D  xtask/policy/history-secret-exemptions.toml
D  xtask/policy/repository.toml
D  xtask/policy/review.toml
D  xtask/policy/secret-rules.toml
D  xtask/src/attribution.rs
D  xtask/src/build_gate.rs
D  xtask/src/docs.rs
D  xtask/src/lib.rs
D  xtask/src/main.rs
D  xtask/src/repository.rs
D  xtask/src/review_gates.rs
D  xtask/src/rust_policy.rs
D  xtask/src/secrets.rs
D  xtask/src/secrets/credentials.rs
D  xtask/src/util.rs
D  xtask/src/workflows.rs
D  xtask/tests/build_gate.rs
D  xtask/tests/common/mod.rs
D  xtask/tests/gates.rs
D  xtask/tests/hook.rs
D  xtask/tests/policy.rs
D  xtask/tests/range.rs
```
