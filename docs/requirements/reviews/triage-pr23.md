# PR #23 — cumulative finding triage (M0 CI checks)

- Date: 2026-11-14 UTC.
- Artifact kind: cumulative review triage and handoff, not a new review round.
- Triager role/provider: triage / OpenAI
- Reviewed head: `112de8d2b7452ecfcb45b79f91d96841add16954`
- Verdict: **must-fix** — revision 15 is paused; no bootstrap attestation or merge authorization is supplied here.
- Triaged report: `docs/requirements/reviews/gemini-2026-11-14-pr23-r14.md`

## 1. Scope, authority and evidence

[D-75](../03-decisions-log.md) ends PR #23 review rounds after the final revision 15:
the coordinator verifies the round-14 reproductions, attests the resulting exact
head and merges; later discoveries go to follow-up PRs under the merged checks.
This record does not restart review rounds or turn the paused revision into a fix.
All **12 round-14 items remain OPEN — scheduled for revision 15**, including the
PERIPHERAL items. Their implementation and future regression names are unverified.

Sources are all 28 reports linked in the ledger below, PR #23's description read
with `gh pr view 23 --json body,headRefName,headRefOid`, and commits inspected with
`git log origin/main..112de8d` on `t3/m0-cargo-workspace-ci`. Both the remote PR
head and local Git object resolve to the full reviewed SHA above. The documentation
base is main at `f42ba33`. No PR #23 content, decision, code, repository setting or
bootstrap seed is changed by this triage.

The [review brief](../REVIEW-BRIEF.md) supplies the reporting conventions. Its
PR #23 version also defines the triage header and closing-status vocabulary:
the second table column is the machine-facing status, with detailed revision/test
evidence in the last column. `open` is intentionally non-closing. The pending
checker parses one `Triaged report` per file, so the header names round 14; the
ledger and rows cover all 14 adversarial reports cumulatively, not 14 separate
machine-readable triage records. **This is not an approving independent-review
report**, and does not establish a passing all-reports review gate.

“Fixed” means the cited original reproduction/contract is corrected, not that the
entire enforcement class is bypass-proof. A later counterexample keeps its own ID
and row. Duplicate reports and carried findings are counted separately, so totals
are finding IDs, not unique root causes. Tests are unqualified function names;
their definitions were verified in each cited fixing revision's Git tree. Some
source-walker tests were retired by revision 6 and are historical evidence, not
claims that those functions still exist at the current head. Builds/tests were
not rerun for this documentation task; execution evidence is the linked reviewers'
replays and PR description, not a fresh test attestation.

Classes are copied as given. Round 3 uses the explicit retrospective classes in
the [round-4 GPT verification matrix](gpt-2026-11-13-pr23-r4.md). Rounds 1–2 have
no verified explicit class assignment: `unverified` means **class not assigned in
the source**, not an unverified finding. Severity preserves the source's
critical/high/medium/low labels; spelling classifications are not upgraded to CORE.

ADR addendum references below mean [ADR 0001](../../architecture/adr/0001-repo-layout-and-ci-gates.md)
**as present at PR #23 head `112de8d`**. Main's ADR does not yet contain those
implementation addenda. Use `git show 112de8d:docs/architecture/adr/0001-repo-layout-and-ci-gates.md`
to inspect that version; no unavailable main-branch anchor is implied. The numbered
limitations refer to that version's addenda and the linked review assessments.

### Review/revision ledger and per-round counts

Round N reviews revision N; fixes normally arrive in revision N+1. Notable
exceptions are R1/ADV23-02 (revision 2 was incomplete; trust design fixed in 3),
ADV23R10-04 (only partly fixed in 11; original footer inputs fixed in 12), F2
(logic fixed in 3; account setup still deferred), and R13-04 (carried to R14-04).

| Round | Reviewed revision head | GPT report / IDs | Gemini report / IDs | Total |
|---|---|---|---|---|
| 1 | `bebd708a574940805854886130cbf8f15907b29b` | [GPT](gpt-review-pr23-2026-11-13.md): R1–R8, C1–C3 (11) | [Gemini](gemini-adversarial-pr23-2026-11-13.md): ADV23-01..18 (18) | 29 |
| 2 | `b38db0675fa3b2eb6a05600fc6fb22d85e0dd8cd` | [GPT](gpt-2026-11-13-pr23-r2.md): F1–F9 (9) | [Gemini](gemini-2026-11-13-pr23-r2.md): ADV23R-01..09 (9) | 18 |
| 3 | `b2173e28f8f232366ffe6a27d5e787fd2adfc39c` | [GPT](gpt-2026-11-13-pr23-r3.md): R3-01..04 (4) | [Gemini](gemini-2026-11-13-pr23-r3.md): ADV23R3-01..07 (7) | 11 |
| 4 | `dc3093ea100ace554ad3a7c11a9f77927f2776a7` | [GPT](gpt-2026-11-13-pr23-r4.md): R4-01..04 (4) | [Gemini](gemini-2026-11-13-pr23-r4.md): ADV23R4-01..10 (10) | 14 |
| 5 | `b21d83de896ba09b59259fa687e427271bbc06ed` | [GPT](gpt-2026-11-13-pr23-r5.md): R5-01..04 (4) | [Gemini](gemini-2026-11-13-pr23-r5.md): ADV23R5-01..10 (10) | 14 |
| 6 | `68a583d88276cd4339c0d08cebf6f0c8367ca76a` | [GPT](gpt-2026-11-13-pr23-r6.md): R6-01..04 (4) | [Gemini](gemini-2026-11-13-pr23-r6.md): ADV23R6-01..10 (10) | 14 |
| 7 | `f986bbc48b23390ca50f4b22065313bcb08d94ea` | [GPT](gpt-2026-11-14-pr23-r7.md): R7-01..03 (3) | [Gemini](gemini-2026-11-14-pr23-r7.md): ADV23R7-01..08 (8) | 11 |
| 8 | `bcd4b2c7aa933662ad952f3656b1a1f408057fc3` | [GPT](gpt-2026-11-14-pr23-r8.md): R8-01..01 (1) | [Gemini](gemini-2026-11-14-pr23-r8.md): ADV23R8-01..08 (8) | 9 |
| 9 | `e64acc5cfb7343485e660436cfd9fb8da1a0e5dd` | [GPT](gpt-2026-11-14-pr23-r9.md): R9-01..03 (3) | [Gemini](gemini-2026-11-14-pr23-r9.md): ADV23R9-01..09 (9) | 12 |
| 10 | `88148f01fe91dc6f3f62577d3130ca1e3261fab9` | [GPT](gpt-2026-11-14-pr23-r10.md): R10-01..02 (2) | [Gemini](gemini-2026-11-14-pr23-r10.md): ADV23R10-01..06 (6) | 8 |
| 11 | `d71cf40d1fdae6eb62de28c366ce925ec3108579` | [GPT](gpt-2026-11-14-pr23-r11.md): R11-01..02 (2) | [Gemini](gemini-2026-11-14-pr23-r11.md): ADV23R11-01..06 (6) | 8 |
| 12 | `d996aac5b9d439d9f328d7f7b47a4c75ec540a91` | [GPT](gpt-2026-11-14-pr23-r12.md): R12-01..02 (2) | [Gemini](gemini-2026-11-14-pr23-r12.md): ADV23R12-01..08 (8) | 10 |
| 13 | `9db2a40a88aa9e4f1400f0417a4e188389c72653` | [GPT](gpt-2026-11-14-pr23-r13.md): R13-01..04 (4) | [Gemini](gemini-2026-11-14-pr23-r13.md): ADV23R13-01..08 (8) | 12 |
| 14 | `112de8d2b7452ecfcb45b79f91d96841add16954` | [GPT](gpt-2026-11-14-pr23-r14.md): R14-01..05 (5) | [Gemini](gemini-2026-11-14-pr23-r14.md): ADV23R14-01..07 (7) | 12 |

**Totals: 182 finding IDs across 14 rounds / 28 reports** (58 GPT, 124 Gemini): 167 fixed, 1 duplicate, 2 deferred, 12 open. F2's deferred status includes its fixed gate logic; it does not certify the missing account. R13-04's deferral is carried into open R14-04.

### Design changes that closed whole enforcement classes

- **Rust xtask, revision 2:** `c52bb3e`/`8e90945` replace the Python CI scripts
  with shared Rust checks, structured Rust/TOML/YAML parsing, trusted policy reads,
  forced-text Git diffs, full stored commit scans, real provenance/review jobs and
  line-scoped attribution exemptions. This closes the initial parser/job omissions,
  but does not by itself solve PR-controlled entry points (F1).
- **Trusted base/entry point, revision 3:** `78956df`/`b2173e2` move trusted review
  and approval execution to main-owned `pull_request_target` jobs; load current
  main policy fail-closed with only an exact-SHA bootstrap exception; protect
  workflow bodies and enforce roles/vendors, triage and tree-mode currentness.
  This closes the F1/ADV23R-08 trust-design loop, not branch-protection deployment.
- **Compiler-enforced build gate and Cargo metadata, revision 6:** `68a583d`
  withdraws the custom source-reachability walker. Cargo resolves targets, member
  globs, includes, macros, patches and build dependencies; rustc enforces the
  non-adapter unsafe forbid and build/link output supplies runtime evidence.
  R5-01..04 and ADV23R5-02..05 no longer depend on guessing expanded source paths.
- **Allowlists and compilation identity, revisions 7–8:** `f986bbc` replaces
  crate-name/solver-prefix trust with the Cargo package graph and exact adapter
  allowlist, and uses library/framework allowlists, fail-closed linker argument
  handling and feature/doctest builds. `a4d698d` resolves allowed library names to
  trusted files and separates production/dev/doctest builds. These close the
  reported probe-name, framework, response-file, shadow-library and dev-feature
  cases, while retaining the explicitly bounded limitations below.
- **Workflow/env and data allowlists, revisions 10–12:** `46e3d7e` reserves real
  check display names, pins aggregator steps and allowlists environment variables;
  `acc98b4` treats DTD markup as data instead of expanding entities and generalizes
  script-language rules. `d996aac` replaces ever-growing data-extension denylists
  with allowed tracked file types and strict text decoding. New solver/data
  suffixes are therefore data by default, not a fresh denylist round.
- **Pinned gitleaks ruleset/every text view, revision 13:** `ac70bc9` vendors
  gitleaks v8.30.1 and evaluates its rules in Rust, failing closed for unsupported
  schema/regexes or a pin mismatch. Every plausible raw/decoded secret view is
  scanned, so decoder selection cannot suppress a recognized raw token.
  `14f0579` skips the ruleset's self-matching text only while its blob is pinned.
  Revision 14 (`112de8d`) adds whole-block per-view exemptions, correct match-line
  allowlist boundaries and range history scanning. This is not universal credential
  coverage: the round-14 CORE rows remain open.
- **Line-level D-61 exemption, revisions 9–11:** `5fc4d2e`/`e64acc5` first verify
  bot block provenance without disabling CodeRabbit. `cf55dc3` in revision 10
  tracks authorship per line and treats initial/unknown text as human; `acc98b4`
  in revision 11 makes diff-budget overflow fail closed. This closes partial-bot-edit
  laundering, first-edit attribution and large-history fallback cases, preserving
  the legitimate summary exception rather than rejecting all bot output.

## 2. Finding-by-finding disposition

Each reviewer cell links to the original finding report. Test definitions are
verified at the fixing revision in the ledger; later-review verification matrices
and the PR description supply the disposition evidence. L1–L10 below are limitation
references, not additional finding IDs. Suggested revision-15 verification is a
handoff, not a claim that a named regression already exists.

| Finding ID | Disposition | Reviewer | Round | Severity | Class | One-line summary | Revision / regression evidence and qualifications |
|---|---|---|---|---|---|---|---|
| R1 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | high | unverified | PR-controlled policy can authorize weaker gates. | Fixed in revision 3; `r1_gate_graph_is_enforced`; `f1_adv23r_08_loader_uses_current_main_and_fails_closed`. Revision 2 was partial (F1/F2/F8); revision 3 corrects the trust design, not operational activation. |
| R2 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | high | unverified | Leading source-comment credits evade attribution checks. | Fixed in revision 2; `r2_adv23_01_comment_prefixes_are_detected`. Original leading-comment input fixed; trailing-comment variant is F3. |
| R3 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | high | unverified | Git attributes can hide added text from the attribution diff. | Fixed in revision 2; `r3_gitattributes_cannot_hide_added_lines`. |
| R4 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | medium | unverified | Stored commit messages are truncated at a scissors marker. | Fixed in revision 2; `r4_stored_scissors_message_is_scanned`. |
| R5 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | high | unverified | Conditional attributes and missing workspace lints weaken unsafe policy. | Fixed in revision 2; `r5_adv23_05_cfg_attr_is_rejected`; `r5_workspace_unsafe_lint_is_mandatory`. Original input fixed; compiler-enforced build gate supersedes source-only enforcement in revision 6. |
| R6 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | high | unverified | Flow-style YAML write permissions evade workflow validation. | Fixed in revision 2; `r6_adv23_15_flow_mapping_write_is_rejected`. |
| R7 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | medium | unverified | Cancellation incorrectly rules out a usable incumbent. | Fixed in revision 2; `r7_interrupted_after_incumbent_keeps_it_but_is_not_complete`; `r7_interrupted_before_incumbent_has_none`. |
| R8 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | medium | unverified | Finite gap operands can overflow into a falsely known result. | Fixed in revision 2; `r8_absolute_gap_overflow_is_unknown`; `r8_ratio_overflow_is_unknown`. |
| C1 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | low | unverified | Reference-style Markdown links are not checked. | Fixed in revision 2; `c1_adv23_18_reference_links_are_checked`. |
| C2 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | low | unverified | Python bytecode is tracked instead of ignored. | Fixed in revision 2; `adv23_11_bytecode_is_rejected`. |
| C3 | fixed | [GPT](gpt-review-pr23-2026-11-13.md) | 1 | medium | unverified | Contributor wording implies branch protection is already active. | Fixed in revision 2. CONTRIBUTING wording corrected; no named regression verified. Actual protection activation remains a deferred owner action, not a completed setting change. |
| ADV23-01 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | critical | unverified | Source comments and file headers evade attribution checks. | Fixed in revision 2; `r2_adv23_01_comment_prefixes_are_detected`. Original inputs fixed; F3 separately covers trailing comments. |
| ADV23-02 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | critical | unverified | PR-controlled scripts and deny configuration judge their own changes. | Fixed in revision 3; `f1_adv23r_08_loader_uses_current_main_and_fails_closed`. Revision 2's trusted-base rewrite was incomplete (F1); revision 3 supplies the main-owned entry point. |
| ADV23-03 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | critical | unverified | Multiline Rust lint attributes evade unsafe checking. | Fixed in revision 2; `adv23_03_multiline_attribute_is_rejected`. Superseded by compiler-enforced build gate in revision 6. |
| ADV23-04 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | critical | unverified | CI omits mandatory secrets, data and provenance checks. | Fixed in revision 2; `adv23_04_manifest_tags_are_required`; `adv23_04_data_files_and_secrets`. Later data/credential counterexamples have their own IDs; this closes the omitted-job input only. |
| ADV23-05 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | Conditional lint attributes permit unsafe opt-outs. | Fixed in revision 2; `r5_adv23_05_cfg_attr_is_rejected`. Superseded by compiler-enforced build gate in revision 6. |
| ADV23-06 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | TOML lint subtables override the intended unsafe policy. | Fixed in revision 2; `adv23_06_crate_lint_overrides_are_rejected`. |
| ADV23-07 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | Invisible characters and homoglyphs evade attribution matching. | Fixed in revision 2; `adv23_07_unicode_evasions_are_detected`. |
| ADV23-08 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | A scissors marker hides committed attribution trailers. | Fixed in revision 2; `r4_adv23_08_scissors_does_not_truncate`. |
| ADV23-09 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | Diff context is omitted from diagnostic line counting. | Fixed in revision 2; `adv23_09_line_numbers_match_new_file`. |
| ADV23-10 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | The independent-review gate passes without review evidence. | Fixed in revision 2; `adv23_10_review_gate_requires_evidence`. Evidence requirement added; role/vendor/currentness refinements are F6 and revision 3. |
| ADV23-11 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | high | unverified | Missing ignore/policy rules permit tracked Python bytecode. | Fixed in revision 2; `adv23_11_bytecode_is_rejected`. |
| ADV23-12 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Build scripts and native FFI bypass runtime dependency bans. | Fixed in revision 2; `adv23_12_native_runtime_links_are_rejected`. Original literal input fixed; source walking superseded by build-output enforcement in revision 6 and linker allowlists in revision 7; static-link bound remains L3. |
| ADV23-13 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Private workspace crates escape licence validation. | Fixed in revision 2; `adv23_13_crate_licence_must_inherit`. Both reports verify that cargo-deny rejects a private GPL-only crate after the change. |
| ADV23-14 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Unescaped diagnostics permit workflow-command injection. | Fixed in revision 2; `adv23_14_annotations_escape_untrusted_text`. |
| ADV23-15 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Inline YAML permission maps bypass write-token detection. | Fixed in revision 2; `r6_adv23_15_flow_mapping_write_is_rejected`. |
| ADV23-16 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Unsafe event/input expressions bypass the workflow filter. | Fixed in revision 2; `adv23_16_untrusted_expressions_in_run_are_rejected`. Original contexts fixed; env-expression indirection is F5. |
| ADV23-17 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Whole-file attribution exemptions exceed the intended scope. | Fixed in revision 2; `adv23_17_exemptions_are_line_scoped`. This concerns attribution exemptions, not the later secret-block exemptions. |
| ADV23-18 | fixed | [Gemini](gemini-adversarial-pr23-2026-11-13.md) | 1 | medium | unverified | Reference links and phantom requirement allocations evade doc gates. | Fixed in revision 2; `c1_adv23_18_reference_links_are_checked`; `adv23_18_phantom_allocation_is_rejected`. |
| F1 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | PR-owned entry points can replace trusted policy jobs. | Fixed in revision 3; `f1_adv23r_08_loader_uses_current_main_and_fails_closed`; `f1_trusted_workflow_rules`. Main-owned trusted entry point and protected workflow bodies; activation remains an operational prerequisite. |
| F2 | deferred | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | Shared PR-author/approver identity makes owner approval impossible. | Fixed in revision 3; `f2_owner_authored_policy_change_cannot_pass`; `f2_owner_authored_policy_pr_explains_why_it_cannot_pass`. Logic fixed in revision 3 under D-74; separate machine-account provisioning and PR authorship remain deferred follow-up. Temporary owner-account authorization for this documentation PR does not satisfy the code-policy approval gate. |
| F3 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | Attribution after executable text on the same line passes. | Fixed in revision 3; `f3_trailing_comment_is_rejected_end_to_end`. |
| F4 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | Macro-parameter lint opt-outs bypass unsafe enforcement. | Fixed in revision 3; `f4_macro_parameter_opt_out_is_rejected`. Original root-forbid fix in revision 3; superseded by compiler build gate in revision 6, with f4_macro_parameter_opt_out_is_rejected_by_the_compiler. |
| F5 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | Actions env-expression interpolation restores shell injection. | Fixed in revision 3; `f5_env_expression_in_run_is_rejected`. |
| F6 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | high | unverified | Review evidence accepts incompatible roles/vendors and changed modes. | Fixed in revision 3; `f6_adv23r_07_roles_and_vendors_are_checked`; `f6_triage_is_required`; `f6_mode_change_invalidates_review`. Review-directory exception is the separate R3-03 input. |
| F7 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | medium | unverified | Custom build-script paths and conditional native links are unscanned. | Fixed in revision 3; `f7_custom_build_script_and_conditional_link_are_rejected`. Original input fixed; build-output enforcement supersedes source scanning in revision 6. |
| F8 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | medium | unverified | CODEOWNERS checking ignores later, more-specific ownership overrides. | Fixed in revision 3; `f8_codeowners_overrides_are_rejected`. Question-mark and globstar mismatches are separate R3-04/R4-02 inputs. |
| F9 | fixed | [GPT](gpt-2026-11-13-pr23-r2.md) | 2 | medium | unverified | Unregistered dataset samples pass provenance validation. | Fixed in revision 3; `f9_unregistered_sample_is_rejected`. |
| ADV23R-01 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Mathematical minus signs evade attribution trailer matching. | Fixed in revision 3; `adv23r_01_unicode_dashes_are_detected`. |
| ADV23R-02 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | A Cyrillic homoglyph in a model name evades normalization. | Fixed in revision 3; `adv23r_02_confusable_table_is_complete`. |
| ADV23R-03 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Pictograph prefixes hide footer and trailer payloads. | Fixed in revision 3; `adv23r_03_pictograph_prefixes_are_detected`. |
| ADV23R-04 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Spaced trailers and email-free Copilot identities evade matching. | Fixed in revision 3; `adv23r_04_copilot_and_spaced_trailers_are_detected`. |
| ADV23R-05 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Git author and committer identities are not scanned. | Fixed in revision 3; `adv23r_05_commit_identities_are_checked`. |
| ADV23R-06 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | critical | unverified | Extensionless Cargo configuration bypasses lint-policy checks. | Fixed in revision 3; `adv23r_06_cargo_config_variants_are_rejected`. |
| ADV23R-07 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Model-suffix comparison permits same-vendor review approval. | Fixed in revision 3; `f6_adv23r_07_roles_and_vendors_are_checked`; `adv23r_07_same_vendor_review_is_rejected_end_to_end`. |
| ADV23R-08 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Stale-base policy loading can fall back to untrusted HEAD. | Fixed in revision 3; `f1_adv23r_08_loader_uses_current_main_and_fails_closed`. |
| ADV23R-09 | fixed | [Gemini](gemini-2026-11-13-pr23-r2.md) | 2 | high | unverified | Generic API keys, manifest file arrays and data formats are missed. | Fixed in revision 3; `adv23r_09_more_key_formats_are_detected`; `adv23r_09_manifest_file_arrays_are_read`; `adv23r_09_more_data_extensions_are_gated`. Extension denylist later superseded by file-type allowlist in revision 12. |
| R3-01 | fixed | [GPT](gpt-2026-11-13-pr23-r3.md) | 3 | high | STRUCTURAL | Tool-qualified lint names falsely satisfy the root forbid. | Fixed in revision 4; `r3_01_qualified_lint_does_not_satisfy_forbid`. Source-only enforcement superseded by compiler build gate in revision 6. |
| R3-02 | fixed | [GPT](gpt-2026-11-13-pr23-r3.md) | 3 | high | STRUCTURAL | Cargo-selected targets without Rust suffixes escape source policy. | Fixed in revision 4; `r3_02_targets_without_rs_suffix_are_scanned`. Cargo metadata/build gate replaces custom scope discovery in revision 6. |
| R3-03 | fixed | [GPT](gpt-2026-11-13-pr23-r3.md) | 3 | high | STRUCTURAL | Review-directory changes falsely preserve review currentness. | Fixed in revision 4; `r3_03_review_directory_change_invalidates_review`. |
| R3-04 | fixed | [GPT](gpt-2026-11-13-pr23-r3.md) | 3 | medium | STRUCTURAL | Missing question-mark wildcard semantics falsify CODEOWNERS coverage. | Fixed in revision 4; `r3_04_question_mark_override_is_rejected`; `r3_04_codeowners_matcher_agrees_with_git`. |
| ADV23R3-01 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | ATTRIBUTION-SPELLING | Precomposed accents evade attribution normalization. | Fixed in revision 4; `adv23r3_01_precomposed_diacritics_are_detected`. |
| ADV23R3-02 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | ATTRIBUTION-SPELLING | Vendor-prefixed model names evade attribution matching. | Fixed in revision 4; `adv23r3_02_vendor_prefixed_models_are_detected`. |
| ADV23R3-03 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | ATTRIBUTION-SPELLING | PowerShell, Lua and assembly comment wrappers evade scanning. | Fixed in revision 4; `adv23r3_03_any_comment_syntax_is_detected`. |
| ADV23R3-04 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | ATTRIBUTION-SPELLING | Google-domain assistant identities are omitted from matching. | Fixed in revision 4; `adv23r3_04_vendor_domains_are_detected`. |
| ADV23R3-05 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | STRUCTURAL | Inherited workflow/job shell defaults bypass interpreter checks. | Fixed in revision 4; `adv23r3_05_inherited_shells_are_checked`. |
| ADV23R3-06 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | high | STRUCTURAL | Schema-style index-file keys fail to register dataset files. | Fixed in revision 4; `adv23r3_06_file_references_follow_the_schema_rule`. Nested arrays are the separate R4-03 input. |
| ADV23R3-07 | fixed | [Gemini](gemini-2026-11-13-pr23-r3.md) | 3 | medium | STRUCTURAL | Alternate provenance-manifest names silently escape discovery. | Fixed in revision 4; `adv23r3_07_non_canonical_manifest_is_rejected_end_to_end`. Canonical naming plus rejection, rather than accepting arbitrary manifest names; suffix loophole is R4-04. |
| R4-01 | fixed | [GPT](gpt-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Production Cargo targets inherit the exploratory-spike exemption. | Fixed in revision 5; `r4_01_production_code_under_spikes_is_checked`. Original input fixed; custom scope walker superseded by Cargo metadata/compiler gate in revision 6. |
| R4-02 | fixed | [GPT](gpt-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Globstar semantics manufacture false CODEOWNERS coverage. | Fixed in revision 5; `r4_02_globstar_cannot_manufacture_coverage`; `r3_04_codeowners_matcher_agrees_with_git`. Unsupported partial-component globstars fail closed; version-stable Git differential cases retained. |
| R4-03 | fixed | [GPT](gpt-2026-11-13-pr23-r4.md) | 4 | medium | STRUCTURAL | Nested manifest arrays silently discard restricted file records. | Fixed in revision 5; `r4_03_nested_array_manifest_end_to_end`. |
| R4-04 | fixed | [GPT](gpt-2026-11-13-pr23-r4.md) | 4 | medium | STRUCTURAL | Manifest filename suffixes allow restricted declarations to be ignored. | Fixed in revision 5; `r4_04_extensionless_manifest_end_to_end`. |
| ADV23R4-01 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Escaped native-link names evade runtime bans. | Fixed in revision 5; `adv23r4_01_escaped_link_names_are_rejected`. Source-literal checking superseded by linker/build-output enforcement in revision 6. |
| ADV23R4-02 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Build-script modules and helpers evade runtime scanning. | Fixed in revision 5; `adv23r4_02_build_script_modules_are_scanned`. Reachability walker superseded by actual Cargo builds in revision 6. |
| ADV23R4-03 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Composite actions outside the conventional directory go uninspected. | Fixed in revision 5; `adv23r4_03_action_outside_github_actions_end_to_end`. |
| ADV23R4-04 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | STRUCTURAL | Comment delimiters glued to code suppress payload segmentation. | Fixed in revision 5; `adv23r4_04_glued_delimiters_are_detected`. |
| ADV23R4-05 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Assistant-name AI suffixes evade attribution matching. | Fixed in revision 5; `adv23r4_05_ai_suffix_is_detected`. |
| ADV23R4-06 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Reasoning-model version prefixes evade attribution matching. | Fixed in revision 5; `adv23r4_06_reasoning_model_versions_are_detected`. |
| ADV23R4-07 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Agent-mode qualifiers evade attribution matching. | Fixed in revision 5; `adv23r4_07_agent_qualifier_is_detected`. |
| ADV23R4-08 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Antigravity and its vendor identity are absent from name rules. | Fixed in revision 5; `adv23r4_08_antigravity_is_detected`. |
| ADV23R4-09 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Gemini tier and thinking qualifiers evade attribution matching. | Fixed in revision 5; `adv23r4_09_tiers_are_detected`. |
| ADV23R4-10 | fixed | [Gemini](gemini-2026-11-13-pr23-r4.md) | 4 | high | ATTRIBUTION-SPELLING | Copilot workspace/chat qualifiers evade attribution matching. | Fixed in revision 5; `adv23r4_10_copilot_surfaces_are_detected`. |
| R5-01 | fixed | [GPT](gpt-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | An OUT_DIR substring suppresses computed-include checks. | Fixed in revision 6; `r5_01_out_dir_substring_include_is_built_and_checked`. Superseded by design change in revision 6: rustc, not source reachability, checks the compiled include. |
| R5-02 | fixed | [GPT](gpt-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | An aliased include macro hides a compiled helper. | Fixed in revision 6; `r5_02_renamed_include_macro_is_built_and_checked`. Superseded by design change in revision 6: actual macro expansion is compiled. |
| R5-03 | fixed | [GPT](gpt-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Custom Cargo member-glob interpretation misses real packages. | Fixed in revision 6; `r5_03_cargo_glob_members_come_from_cargo`. Superseded by design change in revision 6: Cargo metadata owns package resolution. |
| R5-04 | fixed | [GPT](gpt-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Patch resolution drops build-dependency execution context. | Fixed in revision 6; `r5_04_patched_build_dependency_is_built_and_checked`. Superseded by design change in revision 6: Cargo resolves and executes the actual graph. |
| ADV23R5-01 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Additional wrapper variables bypass workflow environment validation. | Fixed in revision 6; `adv23r5_01_toolchain_env_is_allow_listed`; `adv23r5_01_caller_environment_cannot_weaken_the_gate`. |
| ADV23R5-02 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | OUT_DIR include handling skips compiled helper files. | Fixed in revision 6; `adv23r5_02_out_dir_includes_are_built_and_checked`. Superseded by compiler design. GPT round 6 rejects the literal published path as a successful-compilation reproduction; the valid generated-file variant is covered. |
| ADV23R5-03 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Macro-generated path modules resolve to the wrong helper file. | Fixed in revision 6; `adv23r5_03_macro_generated_path_modules_are_built_and_checked`. Superseded by compiler design; the original ordinary-build unsafe input already meets a root forbid, while adapter/link variants exercise the new boundary. |
| ADV23R5-04 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Macro-substituted native-link names evade runtime scanning. | Fixed in revision 6; `adv23r5_04_generated_link_attributes_reach_the_linker_guard`. Superseded by linker/build-output enforcement. |
| ADV23R5-05 | duplicate | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | STRUCTURAL | Patching allegedly drops build-context tracking. | Fixed in revision 6; `r5_04_patched_build_dependency_is_built_and_checked`. Duplicate of R5-04, superseded by revision-6 Cargo resolution. GPT round 6 rejects the literal member-level patch/path-dependency execution claim; it is not an independently verified exploit. |
| ADV23R5-06 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | ATTRIBUTION-SPELLING | Footer verb and joiner vocabulary omits common variants. | Fixed in revision 6; `adv23r5_06_07_attribution_phrases_are_detected`. |
| ADV23R5-07 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | ATTRIBUTION-SPELLING | Help/assistance/aid prepositional variants evade matching. | Fixed in revision 6; `adv23r5_06_07_attribution_phrases_are_detected`. |
| ADV23R5-08 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | ATTRIBUTION-SPELLING | Cody/CodeWhisperer names and vendor domains are omitted. | Fixed in revision 6; `adv23r5_08_09_listed_tools_are_detected`. |
| ADV23R5-09 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | ATTRIBUTION-SPELLING | GLM and Command model families are omitted. | Fixed in revision 6; `adv23r5_08_09_listed_tools_are_detected`. |
| ADV23R5-10 | fixed | [Gemini](gemini-2026-11-13-pr23-r5.md) | 5 | high | ATTRIBUTION-SPELLING | Space-separated credit trailer keys evade matching. | Fixed in revision 6; `adv23r5_10_any_words_by_trailer_is_detected`. |
| R6-01 | fixed | [GPT](gpt-2026-11-13-pr23-r6.md) | 6 | high | STRUCTURAL | A crate named like the compiler probe bypasses wrapper policy. | Fixed in revision 7; `r6_01_crate_named_like_the_probe_is_still_guarded`. Package-graph attribution replaces crate-name exceptions. |
| R6-02 | fixed | [GPT](gpt-2026-11-13-pr23-r6.md) | 6 | high | STRUCTURAL | Response-file quoting hides actual foreign linker inputs. | Fixed in revision 7; `r6_02_linker_response_files_and_foreign_inputs_are_refused`. Response files fail closed rather than receiving a new quoting parser; accepted fail-closed bound L5. |
| R6-03 | fixed | [GPT](gpt-2026-11-13-pr23-r6.md) | 6 | high | STRUCTURAL | All-features builds omit code enabled by default-feature choices. | Fixed in revision 7; `r6_03_feature_disabled_code_is_built_and_checked`. Feature matrix added; dev-feature masking is the separate R7-02 input; remaining multi-feature bound L4. |
| R6-04 | fixed | [GPT](gpt-2026-11-13-pr23-r6.md) | 6 | medium | STRUCTURAL | Doctests do not inherit a crate-root unsafe forbid. | Fixed in revision 7; `r6_04_doctests_are_checked`. Compiler-applied doctest attribute plus executed doctest builds replaces the incorrect revision-6 claim. |
| ADV23R6-01 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | critical | STRUCTURAL | Unreviewed solver-prefixed directories receive unsafe exemptions. | Fixed in revision 7; `adv23r6_01_adapters_come_from_the_trusted_list`. Trusted exact package/directory adapter allowlist replaces prefix trust. |
| ADV23R6-02 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | STRUCTURAL | macOS framework linker arguments bypass runtime bans. | Fixed in revision 7; `adv23r6_02_framework_requests_are_refused_by_the_linker_guard`. Library/framework allowlist replaces name-only denial; no claim of a newly executed macOS run in this triage. |
| ADV23R6-03 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | STRUCTURAL | Dynamic shell expansion bypasses environment/path file-command checks. | Fixed in revision 7; `adv23r6_03_file_commands_are_unreachable_from_scripts`. Last-step/no-outputs structure and stripped file-command environment are the guarantee, not shell-text completeness; L2 remains. |
| ADV23R6-04 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | medium | STRUCTURAL | Case-sensitive raw-data path rules admit mixed-case variants. | Fixed in revision 7; `adv23r6_04_path_rules_ignore_case`. |
| ADV23R6-05 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Generic AI/LLM trailer identities are omitted. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| ADV23R6-06 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Vendor-only assistant names evade attribution matching. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| ADV23R6-07 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Pair-programming and collaboration vocabulary is omitted. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| ADV23R6-08 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Possessive assistant/help phrasing evades attribution matching. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| ADV23R6-09 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Credit trailers with a source joiner evade attribution matching. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| ADV23R6-10 | fixed | [Gemini](gemini-2026-11-13-pr23-r6.md) | 6 | high | ATTRIBUTION-SPELLING | Roo Code, Continue and Supermaven are absent from name rules. | Fixed in revision 7; `adv23r6_05_10_generic_attribution_is_detected`. |
| R7-01 | fixed | [GPT](gpt-2026-11-14-pr23-r7.md) | 7 | high | STRUCTURAL | Allowed library names resolve to untrusted linker scripts. | Fixed in revision 8; `r7_01_allow_listed_names_cannot_resolve_to_build_script_files`. Library-file resolution rejects untrusted search directories; trusted-root location bound is L2. |
| R7-02 | fixed | [GPT](gpt-2026-11-14-pr23-r7.md) | 7 | high | STRUCTURAL | Dev-feature unification hides a production-only feature configuration. | Fixed in revision 8; `r7_02_dev_dependency_features_cannot_hide_production_code`; `r7_02_workspace_feature_unification_cannot_hide_default_code`. Separate production, all-target and doctest builds; remaining multi-feature bound L4. |
| R7-03 | fixed | [GPT](gpt-2026-11-14-pr23-r7.md) | 7 | medium | ATTRIBUTION-SPELLING | Authored CodeRabbit credit is absent from attribution matching. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. Original spelling fixed. The recommendation to suppress the bot-managed summary was withdrawn in round 8 under D-61; R8-01 tracks the legitimate exception. |
| ADV23R7-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | STRUCTURAL | Unnamed jobs with case-varied IDs spoof trusted check identities. | Fixed in revision 8; `adv23r7_01_job_ids_are_compared_case_insensitively`. |
| ADV23R7-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | STRUCTURAL | Compressed dataset formats evade provenance classification. | Fixed in revision 8; `adv23r7_02_compressed_and_sniffed_data_is_gated`. Original extensions/signatures fixed; extension enumeration superseded by revision-12 file-type allowlist. |
| ADV23R7-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | STRUCTURAL | Julia, Pythonw and PyPy interpreter families evade workflow policy. | Fixed in revision 8; `adv23r7_03_interpreters_are_matched_by_family`. |
| ADV23R7-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | ATTRIBUTION-SPELLING | Prompting verbs evade attribution matching. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. |
| ADV23R7-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | ATTRIBUTION-SPELLING | Input/guidance assistance nouns evade attribution matching. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. |
| ADV23R7-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | ATTRIBUTION-SPELLING | Continue and Supermaven vendor domains are omitted. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. |
| ADV23R7-07 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | ATTRIBUTION-SPELLING | Vercel v0 and Augment assistant names are omitted. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. |
| ADV23R7-08 | fixed | [Gemini](gemini-2026-11-14-pr23-r7.md) | 7 | high | ATTRIBUTION-SPELLING | Generic coding-assistant identities evade attribution matching. | Fixed in revision 8; `r7_03_adv23r7_04_08_attribution_spellings_are_detected`. |
| R8-01 | fixed | [GPT](gpt-2026-11-14-pr23-r8.md) | 8 | high | STRUCTURAL | Attribution enforcement rejects the legitimate D-61 bot-summary block. | Fixed in revision 9; `r8_01_bot_summary_block_needs_verified_provenance`. Original exception implemented in revision 9; whole-block provenance superseded by line-level provenance in revision 10 and fail-closed diff budget in revision 11. |
| ADV23R8-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | STRUCTURAL | Reserved check identities omit the merge-gates aggregator. | Fixed in revision 9; `adv23r8_01_spoofed_merge_gates_workflow_end_to_end`. |
| ADV23R8-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | STRUCTURAL | Dynamic-loader environment variables bypass workflow validation. | Fixed in revision 9; `adv23r8_02_loader_env_is_rejected`. |
| ADV23R8-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | STRUCTURAL | Native runtime recognition omits PyPy and related families. | Fixed in revision 9; `adv23r8_03_runtime_families_are_shared`. |
| ADV23R8-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | STRUCTURAL | Network-model and CIM/RDF dataset formats escape provenance gates. | Fixed in revision 9; `adv23r8_04_network_models_are_gated`. Original formats fixed; namespace variant is R9-03, and revision 12 replaces the extension denylist. |
| ADV23R8-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | ATTRIBUTION-SPELLING | OpenHands, Bolt and Replit assistant names are omitted. | Fixed in revision 9; `adv23r8_05_08_attribution_spellings_are_detected`. |
| ADV23R8-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | ATTRIBUTION-SPELLING | Several assistant vendor domains are omitted. | Fixed in revision 9; `adv23r8_05_08_attribution_spellings_are_detected`. |
| ADV23R8-07 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | ATTRIBUTION-SPELLING | Review/fix/debug/rewrite attribution verbs are omitted. | Fixed in revision 9; `adv23r8_05_08_attribution_spellings_are_detected`. |
| ADV23R8-08 | fixed | [Gemini](gemini-2026-11-14-pr23-r8.md) | 8 | high | ATTRIBUTION-SPELLING | Supervision vocabulary and its preposition evade matching. | Fixed in revision 9; `adv23r8_05_08_attribution_spellings_are_detected`. |
| R9-01 | fixed | [GPT](gpt-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | A partial bot edit launders inherited human-authored credit. | Fixed in revision 10; `r9_01_bot_partial_edit_does_not_launder_human_lines`; `r9_01_adv23r9_04_block_lines_need_bot_authorship`. Line-level D-61 authorship replaces whole-block introduction; diff-budget variant is R10-02. |
| R9-02 | fixed | [GPT](gpt-2026-11-14-pr23-r9.md) | 9 | medium | STRUCTURAL | Protected job IDs omit their published check display names. | Fixed in revision 10; `r9_02_adv23r9_01_gate_display_names_are_protected_end_to_end`. |
| R9-03 | fixed | [GPT](gpt-2026-11-14-pr23-r9.md) | 9 | medium | STRUCTURAL | Valid namespace forms evade RDF/XML content classification. | Fixed in revision 10; `r9_03_adv23r9_05_rdf_and_solver_models_end_to_end`. Original namespace inputs fixed; entity-composition variant is R10-01. |
| ADV23R9-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | Published check names can spoof trusted gate identities. | Fixed in revision 10; `r9_02_adv23r9_01_display_names_are_protected`. |
| ADV23R9-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | Aggregator name/body validation accepts trivial passing replacements. | Fixed in revision 10; `adv23r9_02_aggregator_name_and_steps_end_to_end`. |
| ADV23R9-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | Shell startup and runtime environment hooks bypass workflow policy. | Fixed in revision 10; `adv23r9_03_env_is_allow_listed_and_startup_hooks_rejected`. Environment allowlist replaces partial denylists. |
| ADV23R9-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | Initial human PR text is misattributed to the first recorded bot edit. | Fixed in revision 10; `adv23r9_04_initial_body_is_human`; `r9_01_adv23r9_04_block_lines_need_bot_authorship`. |
| ADV23R9-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | STRUCTURAL | Optimization instance and solver-model formats are unclassified. | Fixed in revision 10; `r9_03_adv23r9_05_rdf_and_solver_models_are_gated`. Original formats fixed; revision 12 replaces format enumeration with a file-type allowlist. |
| ADV23R9-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | ATTRIBUTION-SPELLING | JetBrains, Blackbox and Sweep assistant names are omitted. | Fixed in revision 10; `adv23r9_06_09_attribution_spellings_are_detected`. |
| ADV23R9-07 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | ATTRIBUTION-SPELLING | JetBrains, Blackbox and Sweep vendor-domain cases are omitted. | Fixed in revision 10; `adv23r9_06_09_attribution_spellings_are_detected`. |
| ADV23R9-08 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | ATTRIBUTION-SPELLING | Optimization/polish/completion credit verbs are omitted. | Fixed in revision 10; `adv23r9_06_09_attribution_spellings_are_detected`. |
| ADV23R9-09 | fixed | [Gemini](gemini-2026-11-14-pr23-r9.md) | 9 | high | ATTRIBUTION-SPELLING | Recommendation/coaching/prompt/instruction nouns are omitted. | Fixed in revision 10; `adv23r9_06_09_attribution_spellings_are_detected`. |
| R10-01 | fixed | [GPT](gpt-2026-11-14-pr23-r10.md) | 10 | high | CORE | Composed XML entities hide RDF namespaces from classification. | Fixed in revision 11; `r10_01_dtd_and_entity_declarations_are_data`. DTD markup is data; recursive entity expansion is no longer attempted. |
| R10-02 | fixed | [GPT](gpt-2026-11-14-pr23-r10.md) | 10 | medium | PERIPHERAL | The LCS size fallback falsely assigns unknown lines to the bot. | Fixed in revision 11; `r10_02_diff_budget_overflow_is_human`. Over-budget unknown lines fail closed as human-authored; legitimate restoration controls retained. |
| ADV23R10-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | CORE | Solution, basis and MIP-start formats evade provenance gates. | Fixed in revision 11; `adv23r10_01_solver_solutions_are_data`. Original formats fixed; revision 12 replaces extension denylist with allowlist. |
| ADV23R10-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | CORE | Julia and other non-Rust scripts escape the Python-only file rule. | Fixed in revision 11; `adv23r10_02_script_languages_are_rejected`; `adv23r10_02_julia_outside_spikes_end_to_end`. General source-language/shebang policy replaces Python-only checks. |
| ADV23R10-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | PERIPHERAL | Verbless assistance-credit clauses evade attribution matching. | Fixed in revision 11; `adv23r10_03_06_attribution_spellings_are_detected`. |
| ADV23R10-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | PERIPHERAL | Bare Blackbox/Sweep names evade footer and trailer matching. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. Revision 11 fixed trailers only; GPT round 11 disproved the claimed full closure. Remaining original authorship footers fixed in revision 12 (R11-02/ADV23R11-03). |
| ADV23R10-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | PERIPHERAL | Testing/verification/documentation/analysis credit verbs are omitted. | Fixed in revision 11; `adv23r10_03_06_attribution_spellings_are_detected`. |
| ADV23R10-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r10.md) | 10 | high | PERIPHERAL | CodeLlama and StarCoder model families are omitted. | Fixed in revision 11; `adv23r10_03_06_attribution_spellings_are_detected`. |
| R11-01 | fixed | [GPT](gpt-2026-11-14-pr23-r11.md) | 11 | high | CORE | UTF-16/32 encoding hides recognized secret signatures. | Fixed in revision 12; `r11_01_encoded_text_is_decoded_before_secret_scan`; `r11_01_encoded_secret_end_to_end`. Original encoding inputs fixed; single-view decoding superseded by every-view secret scanning in revision 13 after R12-01. |
| R11-02 | fixed | [GPT](gpt-2026-11-14-pr23-r11.md) | 11 | medium | PERIPHERAL | Original bare-tool authorship footers remain after trailer-only fixes. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. Verification/gratitude variants become R12-02; lowercase ordinary-word tradeoff remains R13-04/R14-04. |
| ADV23R11-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | CORE | BOM-prefixed extensionless scripts bypass shebang validation. | Fixed in revision 12; `adv23r11_01_bom_script_end_to_end`. |
| ADV23R11-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | CORE | Further solver, SAT, SMT and AMPL formats evade provenance rules. | Fixed in revision 12; `adv23r11_02_file_types_are_an_allowlist`; `adv23r11_02_unlisted_types_end_to_end`. Superseded by design change in revision 12: unlisted file types are data by default. |
| ADV23R11-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | PERIPHERAL | Bare Blackbox/Sweep authorship sentences remain unmatched. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. |
| ADV23R11-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | PERIPHERAL | Gratitude and courtesy attribution clauses are omitted. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. |
| ADV23R11-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | PERIPHERAL | Credit-related clauses and trailers evade matching. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. |
| ADV23R11-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r11.md) | 11 | high | PERIPHERAL | CodeGeeX and Granite model names are omitted. | Fixed in revision 12; `r11_02_adv23r11_03_06_attribution_spellings_are_detected`. |
| R12-01 | fixed | [GPT](gpt-2026-11-14-pr23-r12.md) | 12 | high | CORE | A decoder-selection prefix hides a raw, recognized token. | Fixed in revision 13; `r12_01_raw_view_is_scanned_whatever_the_encoding_guess`; `r12_01_encoding_guess_cannot_hide_raw_token_end_to_end`. Every plausible text view is scanned; no decoding guess suppresses raw bytes. |
| R12-02 | fixed | [GPT](gpt-2026-11-14-pr23-r12.md) | 12 | medium | PERIPHERAL | Bare-name verification and gratitude attribution remain unmatched. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. Original capitalized product-name inputs fixed; lowercase ordinary-word limitation remains L9/R13-04. |
| ADV23R12-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | CORE | PuTTY private-key armour and PPK paths escape secret rules. | Fixed in revision 13; `adv23r12_01_putty_keys_are_detected`; `adv23r12_01_putty_key_end_to_end`. Original non-exempt path fixed; report-header exemption substitution is the separate R13-01 input. |
| ADV23R12-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | CORE | GitLab Runner and Perplexity token formats are omitted. | Fixed in revision 13; `adv23r12_02_vendored_ruleset_covers_more_formats`; `adv23r12_02_vendored_formats_end_to_end`. Vendored, pinned gitleaks ruleset replaces incremental token-prefix enumeration. |
| ADV23R12-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | CodeGemma model names are absent from attribution rules. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. |
| ADV23R12-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | WizardCoder and Yi-Coder model names are omitted. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. |
| ADV23R12-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | Additional gratitude and praise vocabulary evades matching. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. |
| ADV23R12-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | Acknowledgment clauses and trailers are omitted. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. |
| ADV23R12-07 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | Consultation collaboration vocabulary is omitted. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. |
| ADV23R12-08 | fixed | [Gemini](gemini-2026-11-14-pr23-r12.md) | 12 | high | PERIPHERAL | Maintenance/testing verbs miss bare product-name tool credits. | Fixed in revision 13; `r12_02_adv23r12_03_08_attribution_spellings_are_detected`. Original capitalized inputs fixed; lowercase tradeoff is R13-04/R14-04. |
| R13-01 | fixed | [GPT](gpt-2026-11-14-pr23-r13.md) | 13 | high | CORE | An exempt PuTTY header admits an arbitrary replacement key body. | Fixed in revision 14; `r13_01_exemption_covers_the_whole_key_block`; `r13_01_exempt_header_does_not_admit_a_new_key_body`. Whole-block per-view blob exemptions replace line hashes. |
| R13-02 | fixed | [GPT](gpt-2026-11-14-pr23-r13.md) | 13 | high | CORE | A following line can trigger an upstream allowlist for a secret. | Fixed in revision 14; `r13_02_following_line_cannot_allowlist_a_finding`. Consumed line endings are trimmed; inline allow markers ignored; same-line upstream exclusions remain L10. |
| R13-03 | fixed | [GPT](gpt-2026-11-14-pr23-r13.md) | 13 | high | CORE | Secrets added then deleted/replaced within a PR evade checkout-only scans. | Fixed in revision 14; `r13_03_secrets_and_data_added_then_removed_in_a_range_fail`. Original token/raw-path inputs fixed; historical record association and delimited-table gaps remain R14-01/ADV23R14-04, not accepted by this closure. |
| R13-04 | deferred | [GPT](gpt-2026-11-14-pr23-r13.md) | 13 | low | PERIPHERAL | Lowercase ordinary-word tool credits remain deliberately unmatched. | Deferred follow-up, retained as R14-04. Accepted attribution limitation: ADR 0001 revision-13 addendum, Attribution paragraph (L9); no fixing revision or rejecting regression verified. |
| ADV23R13-01 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | CORE | Credentials in database URI user information are unrecognized. | Fixed in revision 14; `adv23r13_01_03_credential_formats_are_detected`. Original URI input fixed; query-string connection credentials are separate round-14 findings. |
| ADV23R13-02 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | CORE | Kubeconfig key-data and user credentials escape secret rules. | Fixed in revision 14; `adv23r13_01_03_credential_formats_are_detected`. Original inline inputs fixed; YAML block scalars are the separate R14-02 input. |
| ADV23R13-03 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | CORE | Netrc and pgpass credential records escape secret rules. | Fixed in revision 14; `adv23r13_01_03_credential_formats_are_detected`. Original records fixed; additional valid syntax remains R14-03. |
| ADV23R13-04 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | PERIPHERAL | CodeQwen model names are omitted. | Fixed in revision 14; `adv23r13_04_08_attribution_spellings_are_detected`. |
| ADV23R13-05 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | PERIPHERAL | Refact assistant and vendor names are omitted. | Fixed in revision 14; `adv23r13_04_08_attribution_spellings_are_detected`. |
| ADV23R13-06 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | PERIPHERAL | Appreciation attribution clauses are omitted. | Fixed in revision 14; `adv23r13_04_08_attribution_spellings_are_detected`. |
| ADV23R13-07 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | PERIPHERAL | Direct, preposition-free assistance/thanks trailers are omitted. | Fixed in revision 14; `adv23r13_04_08_attribution_spellings_are_detected`. |
| ADV23R13-08 | fixed | [Gemini](gemini-2026-11-14-pr23-r13.md) | 13 | high | PERIPHERAL | Discussion/conversation collaboration clauses evade matching. | Fixed in revision 14; `adv23r13_04_08_attribution_spellings_are_detected`. |
| R14-01 | open | [GPT](gpt-2026-11-14-pr23-r14.md) | 14 | high | CORE | Deleted/superseded provenance records hide restricted historical data. | OPEN — scheduled for revision 15; replay deleted-record and later-open-commercial relabeling cases. Fix and regression names unverified. |
| R14-02 | open | [GPT](gpt-2026-11-14-pr23-r14.md) | 14 | high | CORE | YAML block-scalar kubeconfig private-key data escapes detection. | OPEN — scheduled for revision 15; replay the client-key-data block-scalar fixture and benign controls. Fix and regression names unverified. |
| R14-03 | open | [GPT](gpt-2026-11-14-pr23-r14.md) | 14 | high | CORE | Valid netrc/pgpass variants evade credential-record rules. | OPEN — scheduled for revision 15; replay passwords with internal whitespace and netrc records without a login, with parser-backed controls for quoted/escaped variants. Fix and regression names unverified. |
| R14-04 | open | [GPT](gpt-2026-11-14-pr23-r14.md) | 14 | low | PERIPHERAL | The lowercase bare-tool attribution tradeoff remains. | OPEN — scheduled for revision 15; carried from R13-04/L9, not newly closed or silently deferred. Fix and regression names unverified. |
| R14-05 | open | [GPT](gpt-2026-11-14-pr23-r14.md) | 14 | low | PERIPHERAL | ADR overstates uniform local rules and omits decoder limits. | OPEN — scheduled for revision 15; narrow the no-heuristics claim and disclose optional decoding/archive limits and retained upstream heuristics. Documentation fix unverified; no test name known. |
| ADV23R14-01 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | CORE | HTTP Authorization header credentials outside curl evade scanning. | OPEN — scheduled for revision 15; replay Bearer and Basic header fixtures. Fix and regression names unverified. |
| ADV23R14-02 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | CORE | JDBC query-parameter credentials evade secret scanning. | OPEN — scheduled for revision 15; replay the JDBC connection-URL fixtures. Fix and regression names unverified. |
| ADV23R14-03 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | CORE | ODBC/ADO.NET and abbreviated password assignments evade scanning. | OPEN — scheduled for revision 15; replay connection strings and DB_PASS/DB_PWD/PASSPHRASE fixtures. Fix and regression names unverified. |
| ADV23R14-04 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | CORE | Transient CSV/TSV datasets evade historical provenance checks. | OPEN — scheduled for revision 15; replay add/delete delimited datasets and retained-head controls. Fix and regression names unverified. |
| ADV23R14-05 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | PERIPHERAL | StableCode and Stability vendor names are omitted. | OPEN — scheduled for revision 15. Fix and regression names unverified. |
| ADV23R14-06 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | PERIPHERAL | Derivation/source-credit clauses evade attribution matching. | OPEN — scheduled for revision 15. Fix and regression names unverified. |
| ADV23R14-07 | open | [Gemini](gemini-2026-11-14-pr23-r14.md) | 14 | high | PERIPHERAL | Direct collaboration/input credit trailers evade matching. | OPEN — scheduled for revision 15. Fix and regression names unverified. |

## 3. Accepted/documented M0 limitations

These are bounded limits accepted/documented in ADR 0001 and the reviews, not new
owner decisions or waivers of the open CORE findings. The
[round-14 GPT assessment](gpt-2026-11-14-pr23-r14.md) distinguishes them from the
current blockers. References below are to the pending PR's ADR version, as noted
in section 1; original limitations corrected by later addenda are not reinstated.

1. **L1 — bootstrap and platform trust.** ADR revision-3 addendum, residual risks
   1–4: until trusted gates land on main, the bootstrap relies on the coordinator's
   exact-SHA manual checklist, not PR-defined green checks. Required branch
   protection and trusted-run identity remain owner-controlled; reviews require a
   re-run after exact-head approval; compromised owner/machine credentials defeat
   that boundary. Reviewer/provider headers attest roles, not cryptographic vendor
   identity (round-14 assessment). This does not close F2's account prerequisite.
2. **L2 — no sandbox against same-user build-time code.** ADR revision-6 risk 5,
   revision-7 risk 9 and revision-9 clarification of risk 11: PR build scripts/proc
   macros can tamper with their own job, wrapper or trusted-by-location build,
   sysroot, driver and OS library directories; hosted-runner sudo enlarges that
   exposure. Read-only tokens, no secrets, separate trusted jobs, protected policy
   and exact-head review are controls, not process isolation.
3. **L3 — static native-library evidence.** ADR revision-6 risk 7: a static native
   library can be bundled into an rlib without a final linker argument; build-script
   search-path checks provide the stated detection, not proof of every archive's
   transitive contents. Do not describe the native-runtime gate as exhaustive.
4. **L4 — host and feature coverage.** ADR revision-6 risk 6, corrected by
   revisions 7–8, and revision-8 replacement of risk 8: another target's `cfg` is
   not compiled on this host. Certain combinations of multiple non-default
   features excluded by both defaults and all-features remain unbuilt. Root
   forbids still apply whenever those non-adapter crates compile. Doctests,
   default/no-default/single-feature and production/dev separation are fixes,
   not continuing exemptions. The 256-configuration bound fails closed.
5. **L5 — long linker response files fail closed.** ADR revision-7 risk 10:
   `@file` linker inputs are refused, including rustc-generated long-command
   fallbacks. A shorter link line is the supported workaround; there is no
   accepted response-file bypass.
6. **L6 — bot identity/history trust.** ADR revision-9 risk 12, refined in
   revision 10 and diff-budget handling in revision 11: D-61 trusts GitHub edit
   history and Bot actor identity for the exact marker block. A compromised bot/API
   remains outside that proof; bot summary prose never proves code authorship.
   Initial, retained-human, unknown and over-budget lines are not bot exemptions.
7. **L7 — classification is not licence truth or arbitrary-data recognition.**
   ADR revision-12 addendum, Content still decides paragraph, and the
   [round-12 assessment](gpt-2026-11-14-pr23-r12.md): declared licence provenance
   is not an external licence audit. Delimited tables in Markdown/source and
   arbitrary non-delimited data under an allowed text type are not exhaustively
   sniffed; DTD/namespace content checks also have their documented 64-KiB bound.
8. **L8 — incremental historical scan, not publication audit.** ADR revision-14
   History paragraph and round-14 assessment: only introduced objects not already
   reachable from the selected base are scanned; duplicate checkout versions may
   be skipped. The file-type allowlist and binary/undecodable classification apply
   only to the final checkout. Historical unknown file types therefore remain a
   reviewed M0 limitation and a later D-58 whole-history/all-branches audit concern.
   **This does not accept historical research-only record loss/relabeling
   (R14-01) or missed recognized delimited datasets (ADV23R14-04).**
9. **L9 — finite attribution vocabulary and ordinary-word tradeoff.** ADR
   revision-13 Attribution paragraph, R13-04 and R14-04: capitalized bare product
   names are distinguished from lowercase ordinary words to retain parameter-sweep,
   human-name and black-box prose controls. Lowercase bare-tool credits remain an
   explicit gap, not a closed D-52 guarantee. R14-04 is still open for revision 15;
   new later spellings follow D-75's follow-up route.
10. **L10 — secret-rule heuristics/decoder boundaries need accurate disclosure.**
    ADR revision-13 evaluator description and revision-14 Credential formats
    paragraph are qualified by R14-05: upstream entropy, stopword and same-line
    allowlists remain heuristic; inherited local rules are not uniformly free of
    placeholder exclusions. Optional base64/hex decoding and recursive archive
    inspection are omitted. These are observed evaluator bounds, but their ADR
    wording correction is **OPEN** in revision 15, not verified as already
    documented there. None excuses named credential-field gaps such as R14-02.

## 4. Deferred follow-ups and revision-15 handoff

- **Final paused revision (D-75):** implement/resolve all 12 round-14 rows and
  record exact-head reproduction evidence. Seven are CORE (three GPT, four Gemini)
  and five PERIPHERAL (two GPT, three Gemini); overlaps remain separately listed.
  Coordinator verification and bootstrap attestation follow revision 15; no new
  formal review round is requested here. Any decision to defer a round-14 item
  must be explicit; this record does not make that decision in advance.
- **D-74 account/approval operations (F2):** provision/configure the separate
  machine account and arrange a non-self exact-head owner approval for protected
  code-policy changes. This documentation PR uses the configured owner account
  under the owner's temporary authorization, not a claim that D-74 is activated.
- **Owner/coordinator deployment (C3, L1):** configure required `merge-gates`,
  `independent-review`, `policy-approval` and code-owner review on main; confirm
  the passing checks originate in the trusted workflow, refresh live head/base/
  metadata and evidence, and perform the one-time bootstrap checklist. Deployment,
  owner approval, attestation and merge are unverified/not performed by this record.
- **Security/project setup from PR #23's description:** enable/verify private
  vulnerability reporting; settle the unreleased project licence placeholder via
  D-17. Completion of either is unverified. Apply later native SDK, container,
  frontend/offline-reference licence inventories when those artifacts exist,
  rather than claiming an M0 inventory of nonexistent components.
- **M1/target-runner isolation (L2–L4):** validate runner isolation and applicable
  host/device/benchmark evidence before heavy B1 work on the D-65 system. No
  database, benchmark, target runner, native solver or new macOS execution was
  provisioned or attested by this documentation task.
- **Publication and ongoing policy maintenance (L7–L10):** perform D-58's
  whole-history/all-branches secret/licence audit before publication; maintain
  pinned gitleaks updates and human/prose negative controls. Later credential or
  spelling discoveries become protected follow-up PRs under D-75, not another
  PR #23 review loop. The previously deferrable R11-02/R12-02 original inputs
  were subsequently fixed, so they are not outstanding follow-ups. R13-04 is
  carried into open R14-04, and R14-05 still needs the explicit disclosure repair.

No closure, rejection or deferral beyond the evidence above is inferred from a
green test suite, a later commit title alone, or an intended revision-15 change.
