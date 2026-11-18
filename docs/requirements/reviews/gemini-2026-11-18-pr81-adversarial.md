# Adversarial Review of PR #81 (Review Evidence Gate & Merge Validation)

- Author role/provider: code generation / OpenAI
- Reviewer role/provider: adversarial review / Google
- Reviewed head: `9f4b9ecbc2d5d2dc87233725ba8392891fcd5534`
- Verdict: **no-findings**

## Scope

Adversarial review and robustness analysis of PR #81 (`fix/review-evidence-loop-v2`, head `9f4b9ecbc2d5d2dc87233725ba8392891fcd5534`), superseding PR #75:
- Resolving circular review gate deadlock for review-evidence PRs under decision D-77.
- Measuring PR changes against `main` using `git merge-tree --write-tree main head` rather than `git merge-base`.
- Failing closed when merge conflicts or unrelated histories occur.
- Enforcing strict header parsing with duplicate rejection across all machine-checked fields.
- Validating full 40-character lowercase hexadecimal commit SHAs without silent truncation.
- Restricting review-evidence PRs to regular markdown files under `docs/requirements/reviews/` and `docs/requirements/STATUS.md`, rejecting symlinks and gitlinks.
- Enforcing strict immutability of existing review evidence on `main`.
- Requiring exact-head owner approval for protected path modifications.

Methodology: code reading and Git inspection only; no Cargo, builds or test suites were executed.

## Adversarial Threat Modeling & On-Paper Attack Constructions

We evaluate candidate pull requests designed on paper to attempt bypassing review classification, slipping code past `policy-approval`, or modifying review evidence.

### 1. Criss-Cross Merges and Merge-Base Classification Bypass

- **Candidate Attack:** In a Git repository with multiple merge bases (criss-cross history), an attacker crafts a branch whose tree is derived from an older merge base $B_1$, merges a hardened feature branch $B_2$ with `-s ours` (effectively discarding/reverting $B_2$'s code or policy changes), and commits only a review document `docs/requirements/reviews/report.md`. Under PR #75, `git merge-base main head` arbitrarily returned $B_1$; diffing against $B_1$ showed only the review document, wrongly classifying the PR as review-only and merging a revert of $B_2$.
- **Mechanism in PR #81:**
  - `merge_result` (`xtask/src/review_gates.rs:486-493`) invokes `git merge-tree --write-tree main head`.
  - In `independent_review` (`review_gates.rs:674-679`), the effective diff is evaluated as `changed_paths(repo, &main, &merged)`.
  - In `policy_approval` (`review_gates.rs:918-919`), protected paths are evaluated against `changed_paths(repo, main, &merged)`.
  - In `still_current` (`review_gates.rs:504-513`), review freshness checks diff paths against `same_entries(repo, main, &merged, &path)`.
- **Outcome:** Git's 3-way merge recursive logic merges the merge bases into a virtual base tree containing $B_2$'s changes. Because the PR head reverted $B_2$, the merge result tree `merged` contains the reverted files. `changed_paths(main, merged)` reveals the reverted code/policy files.
  - In `evaluate_review_evidence_pr` (`review_gates.rs:528-531`), `changed.iter().all(|path| is_review_document_path(path))` evaluates to `false`. The PR is denied review-only exemption and falls through to requiring full independent review evidence on `main` for all commits (`review_gates.rs:687-709`).
  - In `policy_approval` (`review_gates.rs:918-944`), any reverted protected paths are detected and require explicit policy-owner approval on the exact head.
  - An empty diff (`changed.is_empty()`, `review_gates.rs:676-678`) occurs only if `merged` matches `main` identically, which produces zero changes to repository files upon merge.
- **Robustness:** Attack prevented. True merge results are accurately classified.

### 2. Conflicting Merges

- **Candidate Attack:** A pull request introduces changes that conflict with recent commits on `main`, attempting to trigger an unhandled condition or default-allow behavior in `merge-tree`.
- **Mechanism in PR #81:**
  - When merge conflicts exist, `git merge-tree --write-tree main head` exits with a non-zero exit status (code 1) or outputs conflict messages without generating a clean tree object (`review_gates.rs:487-492`).
  - The `git` runner (`xtask/src/util.rs:182-190`) checks `output.status.success()`, returning `Err(format!("git merge-tree ..."))`.
  - `merge_result` propagates the `Err` (`review_gates.rs:487`), causing `independent_review` (`review_gates.rs:674`) and `policy_approval` (`review_gates.rs:918`) to fail closed immediately.
  - In `still_current` (`review_gates.rs:506`), an error is caught by `still_current(...).unwrap_or(false)` (`review_gates.rs:706`), denying review currency.
- **Robustness:** Attack prevented. Conflicting merges fail closed across all gates.

### 3. Path Tricks (Directory Traversal, Submodules, Symlinks, Non-Canonical Paths)

- **Candidate Attack:** A pull request attempts to bypass review-document restrictions using:
  1. Nested subdirectories (e.g. `docs/requirements/reviews/sub/nested.md`).
  2. Directory traversal or backslashes (e.g. `docs/requirements/reviews/../file.md`, `docs/requirements/reviews\nested.md`).
  3. Symlinks (mode `120000`) or Git submodules / gitlinks (mode `160000`) targeting arbitrary files or policy configurations.
  4. Non-markdown extensions or short filenames (e.g. `docs/requirements/reviews/payload.rs`, `.md`, `STATUS.md.bak`).
- **Mechanism in PR #81:**
  - `is_review_document_path` (`review_gates.rs:517-522`) strictly specifies:
    `path == "docs/requirements/STATUS.md" || path.strip_prefix(REVIEWS).is_some_and(|name| name.len() > 3 && name.ends_with(".md") && !name.contains(['/', '\\']))`.
    Any path with nested directories, path separators, wrong extensions, or `name.len() <= 3` (e.g. `.md`) returns `false`, disqualifying the PR from review-only treatment.
  - `evaluate_review_evidence_pr` inspects each path in `merged` using `git ls-tree -z --full-tree head -- ":(literal){path}"` (`review_gates.rs:536-545`).
  - Line 548 strictly enforces: `entry.starts_with(b"100644 blob ") || entry.starts_with(b"100755 blob ")`. Symlinks (`120000 blob`) and submodules (`160000 commit`) fail this condition with the finding: `"review document must be a regular file (no symlinks or gitlinks)"` (`review_gates.rs:550-554`).
  - Deletions result in an empty entry and are rejected with `"review document cannot be deleted in a review-evidence pull request"` (`review_gates.rs:546-553`).
- **Robustness:** Attack prevented. All path trick variants and non-regular file modes are caught and rejected.

### 4. Duplicate Header Fields and Ambiguity Exploits

- **Candidate Attack:** A review report attempts to smuggle conflicting metadata or override security checks by repeating header fields:
  1. A valid `Reviewed head` followed by an unreviewed or malformed commit SHA.
  2. An approving `Verdict` preceded or followed by `request changes`.
  3. Duplicate `Author role/provider`, `Reviewer role/provider`, `Triager role/provider`, or `Triaged report` declarations.
- **Mechanism in PR #81:**
  - In `parse_header` (`review_gates.rs:105-119`), a `BTreeSet` tracks all observed machine-checked field names within the first 40 lines.
  - If `field(line, name).is_some() && !seen.insert(name)` evaluates to `true`, `header.errors.push(format!("duplicate {name} header field"))` (`review_gates.rs:117-118`).
  - In both trusted evidence evaluation (`review_gates.rs:360-363`) and incoming review-evidence PR evaluation (`review_gates.rs:571-574`), `!header.errors.is_empty()` triggers an immediate finding and halts approval consideration.
- **Robustness:** Attack prevented. Duplicate header declarations strictly invalidate the report.

### 5. Malformed SHAs and Truncation Exploits

- **Candidate Attack:** A review report supplies a malformed commit identifier (e.g. short 7-character SHA, 40-character prefix followed by trailing text, uppercase hexadecimal characters, or non-hex characters) to see if permissive parsing or `.take(40)` truncation accepts unvetted commits.
- **Mechanism in PR #81:**
  - In `parse_header` (`review_gates.rs:121-131`), enclosing backticks are stripped only when present as both prefix and suffix (`.strip_prefix('`').and_then(|v| v.strip_suffix('`'))`), and an optional trailing period is removed.
  - The candidate string is checked via `is_sha(sha)` (`xtask/src/util.rs:230-235`), which requires exact length 40 and lowercase hexadecimal bytes `b'0'..=b'9' | b'a'..=b'f'`.
  - Non-conforming values (including partial, uppercase, padded, or malformed strings) record `"invalid Reviewed head SHA"` in `header.errors`, failing both publication and evidence evaluation (`review_gates.rs:129, 360, 572, 587`).
- **Robustness:** Attack prevented. Commit SHAs must be exact 40-character lowercase hexadecimal hashes.

### 6. Evidence Immutability and Forgery

- **Candidate Attack:** A review-evidence PR attempts to overwrite or tamper with an existing review report on `main` to alter historical findings, or delete an existing report.
- **Mechanism in PR #81:**
  - In `evaluate_review_evidence_pr` (`review_gates.rs:556-565`), if `cat-file -e {main}:{path}` succeeds, the gate verifies `same_entries(repo, main, head, path)`.
  - Any change to existing review reports on `main` emits the finding `"existing review evidence on main cannot be modified"` (`review_gates.rs:561-563`).
  - Deletions are blocked by lines 546-553.
  - New reports must comply with valid verdicts (`APPROVING`, `request changes`, `must-fix`, `review_gates.rs:628-638`) and authorized vendor pairings from `xtask/policy/review.toml` (`review_gates.rs:639-646`).
- **Robustness:** Attack prevented. Historical review evidence on `main` is strictly immutable.

### 7. Slipping Code Past `policy-approval`

- **Candidate Attack:** A PR modifying protected paths attempts to slip past owner approval by:
  1. Opening from the policy owner's account.
  2. Submitting without an explicit `APPROVED` review on the exact head SHA.
  3. Introducing workflow jobs with names identical to protected trusted gates (`independent-review`, `policy-approval`, `merge-gates`).
- **Mechanism in PR #81:**
  - `policy_approval` evaluates `changed_paths(repo, main, &merged)` (`review_gates.rs:918-919`), capturing all changes in the true merge result.
  - If opened by the policy owner, lines 748-753 reject the PR, explaining that GitHub prevents self-approval and directing that policy PRs be opened by `automated-machines` (D-74).
  - For PRs opened by the agent account, lines 754-763 verify that the owner's latest non-comment review is `APPROVED` on the exact head commit SHA (`state == "APPROVED" && commit == head`).
  - All workflows in `merged` are scanned against trusted gate names (`review_gates.rs:920-942, 780-845`), blocking gate spoofing.
- **Robustness:** Attack prevented. Protected paths cannot merge without exact-head owner approval.

## Verdict

No bypasses, security gaps, or robustness flaws were identified in the review gate logic of PR #81.
Explicit finding count: **0 findings** (**no-findings**).
