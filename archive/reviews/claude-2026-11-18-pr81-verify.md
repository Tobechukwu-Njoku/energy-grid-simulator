- Author role/provider: code generation / OpenAI
- Reviewer role/provider: independent code review / Anthropic
- Reviewed head: `a2194d8528dfd0382e39824a863b201ccecd4a02`
- Verdict: **approve**

# PR #81 verification review: header-error scoping and STATUS claims (2026-11-18)

Scope: [PR #81](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/81)
at the exact head above. This checks the fixes for the two findings in
`claude-2026-11-18-pr81.md`, which reviewed head `9f4b9ec`. The head adds three
commits after that one:

- `7668158` merges main at `f54279c`. That merge brings in only the earlier review report.
- `323ae5b` fixes the P2 finding.
- `a2194d8` fixes the P3 finding.

From `9f4b9ec` to the head, `review_gates.rs` changes by 14 lines,
`tests/gates.rs` gains 23 lines and STATUS is rewritten. `02-questionnaire.md`,
`03-decisions-log.md`, `xtask/policy/` and `.github/` are unchanged. Line references
are to `xtask/src/review_gates.rs` at the reviewed head. I read the code and did not
build it.

## Fix verification

### P2: a malformed report anywhere on main blocks every ordinary PR — **fixed**

`evaluate` now computes `cites_pr` before checking for header errors (lines 360-364).
It reports the errors only when `cites_pr` is true (lines 365-370). A malformed report
that does not cite the PR is skipped as non-evidence. The citation rule is:

- the parsed `Reviewed head` is a commit of the PR, or
- the header has no head, or has errors, and the text contains a full 40-character
  hex word that is a commit of the PR.

`commits` is `rev-list main..head`, so it holds full SHAs only (line 690).

I checked that a malformed report that cites the PR still fails:

| Malformed report | Parsed `head` | `cites_pr` | Result |
|---|---|---|---|
| Any field is duplicated and `Reviewed head` is a PR commit | PR commit | true (head) | header-error finding |
| A second, valid `Reviewed head` names another commit (the last value wins) and the first names a PR commit | other commit | true (text scan, which now also runs when there are errors) | header-error finding |
| `Reviewed head: garbage` after a valid PR head | PR commit (the invalid line adds only an error) | true | header-error finding |
| Invalid head and a PR SHA written elsewhere in the text | none | true (text scan) | header-error finding |
| Malformed report for another PR that does not contain a full PR SHA | other or none | false | ignored (the fix) |

Ignoring a report cannot make the gate easier to pass. Before and after this change,
a report with header errors skips the rest of the loop (`continue`, line 369), so it
never adds approval, adversarial coverage or triage. The gate approves only on a
positive valid report, and a `request changes` report has never blocked by itself.
Dropping an uncited malformed report therefore only removes a failure that had
nothing to do with this PR. Uncited malformed triage or adversarial reports also
fail closed. The PR still lacks a valid adversarial report or a complete triage and
gets that finding instead.

The D-77 publication path (`evaluate_review_evidence_pr`) is unchanged. It still
rejects malformed headers in newly published reports, so
`pr75_rejects_malformed_sha_headers` and the publication half of the duplicate-header
test still apply.

**Regression test.**
`pr81_unrelated_malformed_main_report_does_not_block_ordinary_pr` puts a report with
two `Verdict` lines on main. That report's head is a main commit. The test then adds
a complete approving evidence set for an unrelated code PR. It asserts that the
fixture really has header errors and that `independent_review` returns no findings.
Before `323ae5b`, the malformed report would have produced a finding, so the test
pins the fix.

The cited case remains covered by
`pr75_duplicate_headers_reject_publication_and_trusted_evidence`. For each of the six
fields, with both a valid and an invalid duplicate, it calls `evaluate` with the
report's head in `commits` and requires a `duplicate` finding.

### P3: false STATUS claims — **fixed**

The paragraph that claimed the matrix alignment and the D-77 rewording were included
is replaced by one line. That line says both changes were withdrawn and that neither
is in the replacement PR. The new "PR #81 review follow-up" paragraph matches the
code:

- the merge parent of `7668158` is `f54279c`;
- the gate test count is 27, which is 26 plus the new test;
- 256 tests is 255 plus the new test.

The remaining "PR #75 replacement repairs" paragraph says the matrix was restored
and the pairing expansion was not needed. Both statements are true at this head.

## Findings

None.

## Observations (no change requested)

- **Fewer diagnostics on the trusted path.** If a malformed report cites the PR only
  by a short SHA, or by a SHA with extra hex characters attached, the text scan does
  not match it, so the trusted gate now ignores it. Before this change it produced a
  header-error finding. The result is still a failure, because no valid approval
  exists: the author sees "no approving review" instead of "invalid Reviewed head
  SHA". The publication path still rejects such a report with the specific error.
- **A cited malformed report blocks its PR permanently.** If an ordinary PR merges a
  malformed report that cites a commit of PR X, PR X cannot pass. That commit stays
  in `main..head`, and D-77 forbids editing the report. Repairing it needs an owner
  bypass. This is the intended "must still fail" behaviour. It now affects only the
  cited PR instead of every PR.
- The observations from the earlier review still apply:
  - D-77 provenance relies on owner approval of the exact head.
  - Results depend on the state of main when a check runs.
  - Existing evidence cannot be corrected.
  - New reports may have mode `100755`.

## Validation

`gh pr checks 81` at the exact head:

- Linux fmt/clippy/nextest (run `44331777319`) passed 283 of 283 tests. That
  includes `pr81_unrelated_malformed_main_report_does_not_block_ordinary_pr` and
  `pr75_duplicate_headers_reject_publication_and_trusted_evidence`.
- macOS build/nextest, documentation, attribution, licences, provenance/secrets,
  repository policy and `merge-gates` (run `44331777319`) passed. The earlier
  `merge-gates` failure came from a cancelled run of the same head.
- `independent-review` fails as expected:
  - main has no approving review of this head;
  - main has no adversarial report that cites a commit of this PR.

  It names the three reports that do not cite this head, and no header errors.
- `policy-approval` fails as expected. The owner has not approved the exact head,
  and the PR changes `xtask/src/review_gates.rs` and `xtask/tests/gates.rs`, which
  are protected paths.

I did not run a local build.
