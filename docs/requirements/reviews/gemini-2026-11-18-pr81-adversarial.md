# Adversarial Review of PR #81 (merge-result gates and strict review headers)

- Author role/provider: OpenAI
- Reviewer role/provider: Google (adversarial)
- Reviewed head: `a2194d8528dfd0382e39824a863b201ccecd4a02`
- Verdict: **request changes**

## Scope

Adversarial security review of PR #81 (`fix/review-evidence-loop-v2`, head `a2194d8528dfd0382e39824a863b201ccecd4a02`), specifically evaluating `xtask/src/review_gates.rs` for bypasses of `review-only` PR classification, `policy-approval` slips, and evidence tampering or forgery.

## Findings

### ADV81-01: STATUS.md modification in review-evidence PRs lacks immutability check
- **Classification:** Confirmed
- **Citation:** `xtask/src/review_gates.rs:520`, `xtask/src/review_gates.rs:565-577`
- **Description:** `is_review_document_path` (line 520) classifies `docs/requirements/STATUS.md` as an exempt review document path alongside reviews under `REVIEWS` (`docs/requirements/reviews/`). However, in `evaluate_review_evidence_pr` (lines 565-577), the immutability check (`git cat-file -e` and `!same_entries(repo, main, head, path)`) is strictly guarded by `if path.starts_with(REVIEWS)`. Because `STATUS.md` does not start with `docs/requirements/reviews/`, existing versions of `STATUS.md` on `main` are never checked for immutability and are skipped by line 576 (`if !path.starts_with(REVIEWS) { continue; }`). A PR modifying or replacing `STATUS.md` is thus accepted as review-only without requiring independent review, allowing arbitrary overwriting or tampering with requirements verification status on `main`.

### ADV81-02: Review evidence can be forged for arbitrary or non-existent commit SHAs
- **Classification:** Confirmed
- **Citation:** `xtask/src/review_gates.rs:599-606`
- **Description:** In `evaluate_review_evidence_pr`, `Reviewed head` is validated solely by `is_sha(reviewed_head)` (verifying 40 hexadecimal characters). The gate never verifies whether `reviewed_head` actually exists in Git object storage (e.g., via `git cat-file -e`). In addition, review-evidence PRs enforce no cryptographic signatures or linkage between the PR author and the claimed reviewer vendor. Consequently, a review-evidence PR can pre-publish approving review evidence for an arbitrary or future commit SHA. Once merged to `main`, that forged review becomes permanent immutable evidence satisfying `independent_review` for that commit without any actual review taking place.

### ADV81-03: Adversarial review reports bypass verdict check and can omit findings
- **Classification:** Confirmed
- **Citation:** `xtask/src/review_gates.rs:618-626`
- **Description:** In `evaluate_review_evidence_pr`, reports matching `path.contains("adversarial")` or `matrix.is_adversarial_role(&header)` only check `matrix.adversarial_valid(&header)` (validating reviewer role and vendor). The recognized verdict check (`APPROVING.contains(&verdict) || matches!(verdict, "request changes" | "request-changes" | "must-fix")`) is situated exclusively in the `else` block (lines 627-638) and is completely bypassed for adversarial reports. An adversarial report with no verdict or an unrecognized verdict will pass validation. Furthermore, the gate does not verify that an adversarial report contains findings or a recognized finding ID structure.

### ADV81-04: Review documents permitted executable mode (+x / 100755)
- **Classification:** Confirmed
- **Citation:** `xtask/src/review_gates.rs:557`
- **Description:** In `evaluate_review_evidence_pr`, line 557 permits entries starting with `b"100755 blob "` in addition to `b"100644 blob "`. This allows new Markdown files under `docs/requirements/reviews/` or `STATUS.md` to be committed with executable permissions (`0755`) in a review-evidence pull request, permitting executable scripts disguised as documentation to pass review-only gate evaluation.

### ADV81-05: Policy approval desynchronization when main advances after approval
- **Classification:** Speculative
- **Citation:** `xtask/src/review_gates.rs:920`, `xtask/src/review_gates.rs:895`
- **Description:** In `policy_approval`, the merge result is computed against the current tip of `main` via `merge_result(repo, main, head)`, but `evaluate_approval` checks whether the policy owner approved the exact `head` commit (`commit == head`). If the policy owner approves `head` when `main` is at commit M1, and `main` subsequently advances to M2 with changes to protected or adjacent files, `policy_approval` re-evaluates the merge against M2 but still accepts the approval granted for M1. If the combination of M2 and `head` introduces unintended policy interactions, no re-approval on the merged state is required.
