# Triage of PR #81 adversarial review

- Triager role/provider: triage / OpenAI
- Triaged report: `docs/requirements/reviews/gemini-2026-11-18-pr81-adversarial.md`
- Reviewed head: `a2194d8528dfd0382e39824a863b201ccecd4a02`

Independently checked all five findings against the cited implementation, its
evidence consumers, the trusted review matrix, D-74 and D-77, and the pre-PR
implementation. This is a read-only code review: no builds, tests or executable
probes were run. All code and policy line references below are at the reviewed
head unless explicitly labelled with the base SHA.

## Dispositions

| Finding | Disposition | Classification | Reason |
|---|---|---|---|
| ADV81-01 | rejected | INVALID | STATUS is a living progress document, not immutable review evidence; D-77 explicitly permits STATUS-only publication. |
| ADV81-02 | rejected | INVALID | Publication is not authentication or approval; evidence must first enter owner-approved main and then cite an actual PR commit with current coverage. A local object-existence check would not establish reviewer identity. |
| ADV81-03 | rejected | INVALID | The matrix requires adversarial role/vendor coverage and triage of every finding, not an approving verdict or at least one finding. Zero-finding reports are legitimate. |
| ADV81-04 | rejected | INVALID | Both modes are regular Git blobs. Executability alone introduces no execution path; existing evidence remains immutable in both content and mode. |
| ADV81-05 | deferred | VALID-FOLLOW-UP | Approval is tied to the PR head rather than the main/head pair, but that behavior predates this PR and implements D-74's exact-head requirement. |

## Finding verification

### ADV81-01 — INVALID: STATUS immutability

`xtask/src/review_gates.rs:519-523` includes STATUS in the review-document path
scope. Lines 565-577 deliberately protect existing files under
`docs/requirements/reviews/` while allowing STATUS edits. Deletion and non-regular
entries are still rejected at lines 549-563. This matches the living nature of
STATUS: freezing it would prevent the progress updates the repository requires.

D-77 (`docs/requirements/03-decisions-log.md:83`) includes STATUS in direct
publication, and the documented implementation assumption explicitly allows
STATUS-only updates (`docs/requirements/02-questionnaire.md:214-224`). That
assumption is not an additional owner decision. The actual trusted evidence
enumeration reads only the reviews directory (`xtask/src/review_gates.rs:695-703`),
not STATUS; changing a status claim cannot create an approving review. The
reported missing immutability check is therefore not a gate bypass.

### ADV81-02 — INVALID: arbitrary SHA and claimed reviewer identity

The narrow observation is correct: publication checks SHA syntax, not local
commit-object existence (`xtask/src/review_gates.rs:586-600`; the parser also
checks it at lines 112-122). The claimed consequence does not follow. For an
ordinary PR, the gate loads evidence from main, not the PR branch (lines 695-703),
and constructs the eligible commit set using `rev-list main..head` (lines 690-694).
`evaluate` requires a report to cite that set (lines 360-376); approval additionally
requires an approving verdict, allowed independent pairing and current coverage
(lines 385-393). Coverage is checked against actual Git trees (lines 507-514).
A nonexistent SHA cannot be a commit in that set and supplies no approval.

A known or precomputed commit could be cited dishonestly whether or not it is
already in local object storage. Checking existence would not prove that its
contents were reviewed or that the claimed vendor authored the report. Under the
task's owner-approved-merge trust model, dishonest evidence would first have to
be accepted onto main by the owner. D-77 specifies valid role/vendor headers and
full SHA references, not signed reviewer identity. The code does not authenticate
vendors, but neither does it claim to: the trusted-main publication boundary is
the attestation boundary. Reject the alleged untrusted forgery bypass; optional
existence or signature checks would be separate hardening, not a blocking fix.

### ADV81-03 — INVALID: adversarial verdict and finding count

`xtask/src/review_gates.rs:621-628` validates the adversarial role/vendor, whereas
lines 629-646 validate an independent review's verdict and pairing. This
distinction implements `xtask/policy/review.toml:34-45`: Gemini never supplies
approval, and triage must close every finding ID. The matrix mandates neither an
adversarial verdict vocabulary nor a nonempty finding list.

The consuming gate uses the same distinction (lines 378-383 versus 385-393).
`finding_ids` extracts the documented heading IDs (lines 287-314), and every
extracted ID must have an allowed closing disposition in current triage
(lines 414-439). Even an empty ID set still requires a matching valid current
triage record; it does not remove the independent approval requirement. Requiring
at least one finding would incorrectly reject a clean adversarial review. A
report using undocumented ID formatting could justify separate format guidance,
but the submitted finding demonstrates no violation of the current matrix.

### ADV81-04 — INVALID: executable regular-file mode

`xtask/src/review_gates.rs:557-563` accepts regular blobs in modes 100644 and
100755, rejecting symlinks and gitlinks. The executable bit alone does not run a
Markdown file. New reports are read as text with `git show` (lines 579-581), and
trusted reports are likewise loaded as text (lines 695-703). The trusted workflow
fetches PR objects as data and executes main's tooling, not PR documentation
(`.github/workflows/trusted-gates.yml:30-52,62-86`).

For existing evidence, the immutability check (lines 565-574) uses `same_entries`,
which compares mode, type and object (lines 445-466). Changing only an existing
report's executable bit is therefore rejected too. No execution consumer or
required 100644-only policy is identified. Restricting new documentation to
100644 could be a style preference, but the alleged executable-script bypass
does not follow from accepting a regular 100755 blob.

### ADV81-05 — VALID-FOLLOW-UP: pre-existing base-advance approval semantics

The observed limitation is real: `policy_approval` derives affected paths from
the current merge result (`xtask/src/review_gates.rs:919-921`), but passes only
the PR head to `evaluate_approval` (lines 947-957). That function requires the
owner's latest substantive review to approve that exact head (lines 759-767),
not a particular main/head pair. A later main advance without a head change
therefore does not itself invalidate that approval when the gate is rerun.

This is not new. At the pre-PR base
`f54279c5ed4202102f9675b959d9398ff658e996`, the same file already selected the
owner's latest substantive review and compared its commit only to the head
(lines 581-589); `policy_approval` already passed only that head (lines 773-783).
PR #81 changes path detection and workflow inspection from merge-base/head to
the actual merge result, not the approval identity rule. D-74
(`docs/requirements/03-decisions-log.md:80`) requires exact-head owner approval,
not approval of a merge-result tree.

Defer base-sensitive approval binding or mandatory reapproval after main
advances as a separate policy-hardening follow-up requiring owner direction.
Do not characterize the existing exact-head rule as a regression or require
PR #81 to change D-74 to close this triage.

## Blocking fixes

- None. All five findings are closed for this head: four rejected and one
  deferred as a pre-existing, non-blocking policy-hardening follow-up. This
  triage does not waive any other review, owner approval or merge gate.
