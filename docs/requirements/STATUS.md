# STATUS

_Last updated: 2026-11-15_

**Phase:** Build (M0). Requirements engineering complete. Decisions D-1 … D-76 (D-5 superseded by D-23).
Fable review (`reviews/claude-fable-5-2-2026-11-12.md`) processed in rounds A–C.
**Code allowed?** **Yes** (D-72, 2026-11-13). Requirements baselined; ADRs 0001–0006 accepted. Current milestone: **M0** (workspace, CI, licence gate).
**Repo:** private (to go public per D-58 once preconditions are met), https://github.com/Tobechukwu-Njoku/energy-grid-simulator

**PR #23 R15V3 blocking repairs (2026-11-15 UTC):** R15V3-01 is a literal
synthetic authority, not a classifier false positive: redacted it to the existing
`user:password@host` template without changing bare-authority detection. R15V3-02
now binds Markdown delimiters to the credential's own assignment, connection
attributes or JDBC/query syntax; unrelated source backticks and escaped openers
cannot truncate a password to a placeholder. Both exact reviewer fixtures have
regressions, including the Rust marker control and escaped-opener variant; the
one/two/three-backtick placeholder controls remain accepted.
Audited every review report without report exemptions and searched credential
examples manually. Eight reports now use placeholders for API/runner tokens,
database passwords, pgpass/netrc values, encoded key data and the two verification
reproductions. Historical findings and FOLLOW-UP dispositions are preserved;
exact synthetic regression inputs remain reconstructed in tests. Redacted PuTTY
header examples retain exact whole-block hash exemptions; obsolete literal-example
exemptions are removed (no new format or path allowlist). The existing
changed-private-body rejection test uses the redacted fixture. Validation results
will be recorded after the complete gates run. No
independent-review, bootstrap-attestation or merge approval is claimed.

**PR #23 R15V2-01 completion (2026-11-15 UTC):** reviewed the complete-value
credential fix at `652f15d` against the second revision-15 verification fixture.
The previously untested commit passes all 247 tests, including its four new
regressions for multiword connection passwords, literal boundaries, whole-value
placeholders and bare authorities. No additional production-code change is needed
for the requested cases. Three more tests check complete unquoted parser values
and consumed lengths, anchored template classification, and Markdown controls
across one/two/three-backtick delimiters, punctuation and quoted/braced values.
The original finding also now covers end-of-input, LF/CRLF, semicolon and a
following connection attribute. All seven focused regressions pass. Final
validation passes: `cargo test -j 2` (250 tests, zero failures),
`cargo fmt --all --check`, `cargo xtask all`, and
`cargo xtask provenance --base origin/main --head HEAD`.
Build output stays in this worktree's `target/`; final validation uses a
disk-backed, space-free TMPDIR and two build/test workers. This is local repair
evidence, not independent review or merge approval.

**PR #23 post-merge policy repair (2026-11-15 UTC):** reproduced 22 secret-scan
findings in the round-14 report imported from main. False positives at report
lines 49, 51, 70, 72, 109, 111, 113, 140, 147 and 343 came from Markdown delimiters
being consumed as credential values, ellipses, or the bare URI authority template.
Literal examples at lines 76, 77, 80, 81, 116, 119, 154, 155, 156, 157, 160 and 165
are replaced with angle-bracket placeholders; no report content is deleted.
Parser value boundaries now respect inline-code delimiters and unquoted whitespace,
and Authorization headers cannot consume a value from the following line. Escaped
LF/CRLF suffixes in shell printf examples are removed before literal classification. URI
exceptions are scoped to the exact documentation authority and an ellipsis
password, not arbitrary literal passwords or environment references. Four new
regression tests cover placeholders, descriptions, literals and mixed lines;
all 19 secret-scanner tests pass. `cargo test -j 2` passes all 243 tests, including
repository self-policy (239 existing tests plus four new regressions). Formatting,
warning-free all-target/all-feature Clippy, xtask policy, docs, checkout and
`origin/main..HEAD` provenance, full-range attribution, and `build-gate --jobs 2`
pass. Build output stays in this worktree's `target/`; test scratch uses a
disk-backed, space-free TMPDIR rather than RAM-backed `/tmp`.

**PR #23 revision 15 implementation (2026-11-15 UTC):** round-14 CORE repairs
retain historical research-only declarations from manifests/source tables and
the range base, reject removed CSV/TSV datasets, parse YAML kubeconfig scalars and
netrc/pgpass grammars, and detect literal HTTP Authorization, JDBC, ODBC/ADO.NET
and environment-key credentials. Shared attribution patterns also cover the
requested inexpensive peripheral additions. Positive and negative regressions
are included for every requested item, including placeholder/environment controls.
Local validation: `cargo test -j 2` passes 239 tests (nine new regression tests);
the final focused replay also passes all nine. Formatting, warning-free Clippy,
policy, checkout/range provenance (current main through the revised code head),
docs, full-range attribution and `build-gate --jobs 2` pass. A generated RSA PEM
inside a YAML block scalar is rejected; the corresponding environment placeholder
passes. Build output stays under the worktree's `target/`; test scratch storage
uses a disk-backed, space-free TMPDIR because existing linker-script fixtures
assume that their scratch root has no spaces.

Actual PR event replays leave independent-review blocked by missing approving
current-head evidence, and policy-approval blocked because the existing PR was
opened by the owner (D-74 prevents self-approval). No gate was weakened. Under
D-75 the coordinator must verify the new head, resolve the bootstrap prerequisites
and attest the seed; no attestation or merge is performed here. The ambiguous
lowercase bare-tool attribution tradeoff remains a follow-up. ADR 0001 now
distinguishes parser placeholder classification, format-specific regex limits
and decoder boundaries. D-75/D-76 are copied unchanged from current main's
owner decision log so this branch's references resolve.

**PR #23 revision-13 review:** exact head `9db2a40a88aa9e4f1400f0417a4e188389c72653`
re-reviewed; verdict **must-fix**. All 224 submitted tests and the original round-12
reproductions pass. Three CORE gaps remain: an exempt PuTTY header permits a real
private body, a following-line allowlist marker suppresses an AWS-secret-shaped
assignment, and add-then-delete PR history is not scanned for secrets/data. Do not
attest this head as the one-time bootstrap seed. R13-04's lowercase attribution
coverage and documented bounded follow-ups need not block independently.
Evidence, regressions, adversarial triage and M0 assessment:
[revision-13 report](reviews/gpt-2026-11-14-pr23-r13.md). PR #23 was not modified.

**PR #23 revision 15 third verification (2026-11-15 UTC):** exact head
`6ed8bc713ef531110f92c392ac1d66a816d7f9c4`; verdict **NOT ATTEST**.
All seven original round-14 CORE reproductions still reject; all 48 original
R15V2-01 variants reject, including EOF, CRLF, semicolons and following attributes;
the three original Markdown placeholder controls and additional controls pass.
Two **BLOCKING** items remain: **R15V3-01**, the merged second-verification
report's synthetic example at line 153 triggers the new bare-authority rule,
failing repository self-policy, clean-head `xtask all` and explicit range
provenance; **R15V3-02**, an unrelated earlier backtick in Rust source is mistaken
for a Markdown opener and restores the repaired password-truncation bypass.
Observed workspace tests total 249 passed and 1 failed after separately running
the range suite omitted when the exact `cargo test -j 2` command stopped.
A bounded 18-case boundary attempt plus source-context comparisons found no
old-reject/new-accept case versus `112de8d`; the source-context case blocks as an
obvious bypass of the fixed item, while distinct alternate-terminator and
unvalidated header-syntax cases are **FOLLOW-UP** under D-75, not another review
round. Do not attest this merged head. PR #23, its policy and bootstrap records
remain unchanged. Evidence and item table:
[third verification report](reviews/gpt-2026-11-15-pr23-r15-verify-3.md).

**PR #23 revision 15 second verification (2026-11-15 UTC):** exact head
`e993dfec5c5d0ca73ac765f6fc74897f2f1d7636`; verdict **NOT ATTEST**.
All seven original CORE reproductions still reject; all three previously failing
Markdown placeholder controls now pass. The complete workspace suite passes
243 tests; clean-head `xtask all`, repository self-policy and explicit provenance
from main `4599e10` through the requested head pass. New **CORE R15V2-01**:
the placeholder repair truncates unquoted connection-password values at internal
whitespace/backticks, allowing a placeholder prefix to conceal the remaining
literal password. The previous checker rejects the same fixture; the candidate
accepts it even in a full clone's `all` gate and committed historical range.
Do not attest this head until that repair regression is fixed and verified.
Bare scheme-less authority text is accepted at both heads, a pre-existing
coverage limit rather than a new repair regression. PR #23 and bootstrap records
are unchanged. Evidence, item table and constrained validation:
[second verification report](reviews/gpt-2026-11-15-pr23-r15-verify-2.md).

**PR #23 revision-15 final verification (2026-11-15 UTC, D-75):** exact head
`60bef0d07bc41882d24be7de4c98e7d97deac302`; verdict **NOT ATTEST**.
PR #23 subsequently advanced to `786e636986a259032fd974de3dd092ecc49d7ad3`;
that newer head is not verified by this report.
The seven original round-14 CORE bypasses now reject, but Markdown-delimited
placeholder controls for ADV23R14-01/02/03 still false-positive. Clean-head
`xtask all` and explicit-range provenance reject 22 lines of the committed
adversarial round-14 report. Formatting, Clippy, dependency checks, documentation
checks and the compiler-enforced build gate pass; the workspace suite completes
with 236 passed and 3 failed (self-policy plus two temporary-path fixtures).
Temporary-path fixture failures are separately isolated with a
passing no-space control. PR #23, policy and bootstrap records remain unchanged.
Evidence: [final verification](reviews/gpt-2026-11-15-pr23-r15-verify.md).

**PR #58 M1 deficit-sources revision 2 (2026-11-15 UTC):**
Revised `docs/research/m1-deficit-sources.md` and `docs/requirements/02-questionnaire.md` to address R2-01–R2-07; round-3 corrections C-01–C-06 (GPT final re-check) applied from GPT's R2 review (`reviews/gpt-2026-11-15-pr58-deficit-sources-r2.md`). Added questions Q-P-01..Q-P-07 to questionnaire Section P; repaired all broken publisher and licence URLs (ETYS archive, MIDAS Open collection UUID with v202407 coverage caveat, ERA5 CDS API CC-BY-4.0 licence, ENTSO-E CGMES library); reconciled summary licence classes with body (rows 3, 7, 8); registered explicit price interpolation and as-of policy under D-70/ADR 0002; qualified live SEMOpx trading schedule as current reference pending historical 2024 pinning; harmonised HVDC loss proxy description as constant fractional exploratory proxy with omitted standby losses; and fixed repository relative links to ADR 0002 and earlier reviews.

**PR #58 M1 deficit-sources final re-check (2026-11-15 UTC):** exact head
`739bb70d0e1bb772ceae49f3afe99353a054b10c`; verdict **accept-with-corrections**.
Repaired publisher pages read without dataset downloads. R2-02/R2-03/R2-05/R2-06
and repository reference paths are satisfactory; R2-01/R2-04 still need question
anchors and price-resampling registration. Section P follows the existing format
and introduces no ID clashes. STATUS conflicts with main; preserve review history.
GitHub has three current unresolved threads (partial 132 kV coverage, IDC closure,
input hash exclusion), plus fourteen outdated unresolved threads. Six bounded
corrections recorded; PR #58 unchanged, no acquisition or baseline approval.
[Round-3 report](reviews/gpt-2026-11-15-pr58-deficit-sources-r3.md).

**PR #58 M1 deficit-sources round-2 re-check (2026-11-15 UTC):** exact head
`67a9d3d84091dd329ae212f8f2333e59dca4cdbb`; verdict **reject**.
All sixteen original findings mapped; six requested publisher spot-checks supported.
Seven remaining findings cover missing Q-P-01–Q-P-07 questionnaire entries,
unrepaired source links, restricted/unverified rights omitted from summary rows,
price interpolation/as-of policy, unpinned 2024 SEMOpx schedules, inconsistent
HVDC proxy wording and broken repository references. No datasets downloaded;
PR #58 unchanged. No deficit closure, modelling-default approval or commercial
clearance claimed. Evidence and suggested fixes:
[round-2 report](reviews/gpt-2026-11-15-pr58-deficit-sources-r2.md).

**PR #58 M1 deficit-sources review (2026-11-15 UTC):** exact head
`db59b5b44adec4835fdd1d82d9ba8b94ebfec861`; verdict **reject**.
Publisher pages, catalogue/API metadata and reference reports checked without
downloading datasets. Sixteen findings distinguish confirmed licence/rule facts
from unsupported proxies, historical coverage and implementation claims. The
Irish seven-unit enduring-policy date is 7 April 2024, contradicting 17 July;
0.88 rating and CIGRE loss defaults have no established provenance. TLM API
coverage, BSC version mapping, plant parameters and exact fleet figures remain
unverified or need correction. Item 9 is internal implementation, not an external
dataset. PR #58 unchanged; no deficit closure or commercial clearance claimed.
Evidence and suggested corrections:
[review report](reviews/gpt-2026-11-15-pr58-deficit-sources.md).

**PR #52 M1 inventory final re-check (2026-11-14 UTC):** exact revised head
`8af069ca0af0712f29b9645810282070b02e45a4`; verdict **accept-with-corrections**.
Six round-2 findings applied; R2-05 substantially applied but still partial:
54 external rows and the 29 open-commercial / 25 research-only split reconcile,
but the publisher mapping lists 32 groups while claiming 31 distinct publishers.
Correct that count/grouping and the all-applied summary; abbreviated TSD and
metering-response wording are non-blocking. NESO's ND/TSD definitions support
the corrected ND-plus-embedded starting identity without another pumping
subtraction. New/split candidate classes checked; no identified share-alike
source admitted. Inventory unchanged; no acquisition, commercial clearance or
M1 acceptance claimed. Evidence:
[round-3 final re-check](reviews/gpt-2026-11-14-pr52-m1-inventory-r3.md).

**PR #52 M1 inventory re-check (2026-11-14 UTC):** exact revised head
`e5ec7171e2d1b57ec9c89bee6343cbbbcd7264f2`; verdict **must-fix**.
All 18 original findings mapped: nine applied at inventory level, nine partial.
Remaining defects include unsupported UK ETS OGL clearance, a new demand equation
that double-subtracts pumping, unsourced per-link loss percentages, unsupported
evidence/access claims, inconsistent Greenlink interpretations and unreconciled
source counts. Changed public endpoints and weather/licence terms spot-checked;
corrected demand CSV and both cached SEMO archives match committed hashes.
Inventory unchanged; no M1 acceptance or heavy work claimed. Evidence:
[round-2 re-check](reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md).

**PR #52 M1 inventory review (2026-11-14 UTC):** exact head
`e7e992e3f213a8d8645bb5e58ea1d7592c55a908` independently checked under D-51;
verdict **must-fix**. All 77 external link URLs and 15 repository-relative
targets checked; 30 network/Irish captures and two SEMO archives match their
committed hashes. Unsupported commercial-clearance labels, broken/misdirected
dataset links, incorrect FUELHH/TLM claims and overstated M1 completeness must
be corrected before reliance. Gated sources and missing dated rules/losses/
decomposition remain explicit gaps. No inventory edits or M1 acceptance claimed.
Evidence and corrections: [review report](reviews/gpt-2026-11-14-pr52-m1-inventory.md).

**PR #23 revision-14 review:** exact head `112de8d2b7452ecfcb45b79f91d96841add16954`
re-reviewed; verdict **must-fix**. Original round-13 CORE and adversarial
reproductions are addressed, but three CORE gaps remain: historical research-only
records can disappear or be superseded before the range scan, YAML block-scalar
kubeconfig private keys pass, and valid pgpass/netrc record variants pass. Do not
attest this head as the one-time bootstrap seed. The retained lowercase attribution
gap and bounded ADR wording/decoder disclosures are PERIPHERAL follow-ups.
Evidence, mapped regressions, adversarial triage and M0 assessment:
[revision-14 report](reviews/gpt-2026-11-14-pr23-r14.md). PR #23 was not modified.

## Snapshot of what's decided

Research + market-forecasting simulator (short-term and seasonal, D-25) of **GB + all-island
Ireland** with interconnectors, for consultancy/research, forecast customers (battery operators,
traders, suppliers/large users) and grants (D-33, D-41).
**Guiding principle (D-44): answer the present first.** Model today's grid accurately, validate it,
then scale out. Phases (`06-roadmap.md`): **P1** today's full transmission network with
unit commitment + Balancing Mechanism redispatch and a historic backtest → **P2** distribution,
home DER, statistical behavioural demand, AC power flow and voltages → **P3** zonal pricing and
2030 scenarios → **P4** dynamic stability and SaaS hardening.
The owner's core questions are listed in D-40.
**Rust everywhere code is needed**; TypeScript web UI in a pro-analytics, dark-mode style (D-47).
PostgreSQL 18 + Redis. Linux machine: Ryzen 5900X / 128 GB, consumer NVIDIA GPU, so the **CPU
path is primary** (R-14). Pluggable solvers. Future commercial SaaS, so **no share-alike data**.
Team (`docs/process/team.md`): Claude Opus 5.5 codes, GPT 6.1 Sol architects and reviews,
Gemini Flash does checked donkey work. Strict merge gates (D-28).

## Still open (non-blocking)

- Linux distro (Q-H-7)
- Remaining unanswered questionnaire items have defaults or are P2+ concerns

## Work state (coordinator: Claude Opus 5.5) — 2026-11-14

**PR #23 revision 14 (implementation, 2026-11-14):** R13-01..03 (GPT) and
ADV23R13-01..08 (Gemini) addressed (ADR 0001 revision-14 addendum). Secret exemptions
name the whole detected block (key header through end marker) by blob hash, per view;
match locations drop consumed newlines so a following line cannot allowlist a finding;
inline gitleaks markers are ignored; `provenance` scans every file version a PR's
commits introduced (CI base: current main). Local rules add URI user-info passwords,
kubeconfig credentials, netrc/pgpass records and over-150-character assignments.
Attribution adds CodeQwen, Refact, appreciation/discussion/conversation phrases and
preposition-less credit trailers. Next: round-14 review.

**PR #23 revision 13 (implementation, 2026-11-14):** R12-01..02 (GPT) and
ADV23R12-01..08 (Gemini) addressed (ADR 0001 revision-13 addendum). The secrets scan
runs over every text view of a file (raw bytes, UTF-8, UTF-16/32 both byte orders and
alignments); any view's finding fails. Secret formats now come from the gitleaks
v8.30.1 default ruleset (MIT), vendored unchanged in `xtask/policy/gitleaks.toml`,
pinned by blob id and evaluated in Rust, plus local rules (PEM header, PuTTY, `.ppk`,
earlier prefixes); unsupported rules or keys fail closed. Update procedure in
CONTRIBUTING.md. Attribution adds CodeGemma/WizardCoder/Yi-Coder, thank-you/kudos/
shout-out/acknowledgment/consultation phrases and capitalised bare tools after review
verbs. Next: round-13 review; the bootstrap seed waits for a clean review.

**PR #23 revision 12 (implementation, 2026-11-14):** R11-01..02 (GPT) and
ADV23R11-01..06 (Gemini) addressed (ADR 0001 revision-12 addendum). Files are decoded
(UTF-8/16/32, BOM or NUL pattern) before the secrets, manifest and shebang checks; a
file that is not decodable text and not an allowed image is data. `data_extensions` is
replaced by the trusted file-type allowlist (`tracked_extensions_allowed`,
`binary_extensions_allowed`, `tracked_names_allowed`); content sniffing still applies,
plus delimited tables in `.txt`. Attribution adds bare Blackbox/Sweep/Granite after
authorship verbs, thanks/courtesy/credit phrases and trailers, CodeGeeX and IBM Granite.
Next: round-12 review; the bootstrap seed waits for a clean review of the new head.

**PR #23 revision 11 (implementation, 2026-11-14):** R10-01..02 (GPT) and
ADV23R10-01..06 (Gemini) addressed (ADR 0001 revision-11 addendum). Markup holding a
DTD or entity declaration is data unless listed in the trusted `xml_dtd_allowed`
(entities are no longer expanded). Solver solution, start, basis and infeasibility
formats are data extensions. D-73 now rejects every non-allow-listed script or source
language (and non-`sh`/`bash` shebangs) outside `spikes/` and reference validators.
The LCS diff budget fails closed (unknown lines are human). Attribution adds verbless
credit phrases, tested/verified/documented/analysed, CodeLlama, StarCoder, and bare
Blackbox/Sweep as trailer values. Next: round-11 review; the one-time bootstrap seed
waits for a clean review of the new head.

**PR #23 revision 10 (implementation, 2026-11-14):** R9-01..03 (GPT) and
ADV23R9-01..09 (Gemini) addressed. D-61 provenance is per line: a CodeRabbit block is
exempt only if every line inside its markers was added by a `coderabbitai` bot edit;
lines a bot edit only kept, lines of the original body (or of the earliest known text
when GitHub lacks the creation record) and lines of unknown origin are scanned.
Gate job ids AND published check names (`[workflow.names]` in `gates.toml`) are
reserved, case-insensitively; each gate job must publish its trusted name. The
aggregator must publish `merge-gates` and run exactly main's steps. Workflow `env:`
names are an allowlist; loader and shell/interpreter/Git start-up variables are
always rejected. RDF/XML is detected whatever its prefix, declaration, encoding or
BOM; solver instances (`.mps`, `.lp`, `.qps`, `.nl`, `.gms`, ...) are data, MPS/LP/GAMS
also by content. New attribution spellings: JetBrains AI, Blackbox AI, Sweep and
domains, optimised/polished/completed, recommendation/coaching/prompt/instruction
nouns. ADR 0001 has a revision-10 addendum. Needs exact-head GPT re-review, Gemini
adversarial re-check and triage before bootstrap attestation.

**PR #23 revision 9 (implementation, 2026-11-14):** R8-01 (GPT) and ADV23R8-01..08
(Gemini) addressed. D-61 is implemented as written, and CodeRabbit's PR-description
summary stays. Only the exact marker block is exempt, and only when the PR body's
GitHub edit history (GraphQL `userContentEdits`, read-only token) shows that the
`coderabbitai` bot introduced that block text. Spoofed markers, blocks with unverified
provenance and text outside the block are still rejected. An unreadable history fails
closed. Every check name in the trusted policy (`merge-gates`, the ci.yml gates, the
trusted gates and `required_checks`) is protected in all workflows. Workflows may not
set `LD_*`/`DYLD_*`, and the build gate removes them. The build gate and the
workflow lint share one Python/Julia runtime-family list (PyPy, GraalPy, ...).
CIM/RDF, PowerFactory and other network-model files are data, and CIM/RDF XML is
also detected by content. New attribution spellings: OpenHands, Bolt, Replit and
related domains, reviewed/fixed verbs, `under` and supervision nouns. ADR 0001 has a
revision-9 addendum: risk 11 is clarified for hosted sudo, and risk 12 covers trust in
GitHub edit history. Needs exact-head GPT re-review, Gemini adversarial re-check and
triage before bootstrap attestation.

**PR #23 revision 7 (implementation, 2026-11-14):** R6-01..04 (GPT) and ADV23R6-01..10
(Gemini) addressed; the compiler-enforced design stays. Reviewed adapters are an explicit
`[[adapter]]` list (package name AND directory) in the trusted, protected
`repository.toml`. The rustc wrapper attributes every compilation through a `cargo
metadata` configuration (manifest directory, package name, source file) and refuses what it
cannot attribute; no crate-name heuristics. The linker guard accepts only allow-listed
library/framework names (`gates.toml` `link_libraries`/`link_frameworks`) and Rust's own
artifacts, and refuses response files, scripts, plugins and driver redirection. The gate
builds default, all, no-default and each single feature, and runs `cargo test --doc`; every
non-adapter library root needs `#![doc(test(attr(forbid(unsafe_code))))]`. The build-gate
step must be its job's last step (file-command guarantee); scripts may not name or
indirectly reach file-command files. Provenance path rules ignore case. Attribution: generic
AI/LLM/vendor-AI names, collaboration joiners, possessives, `from:` trailers, Roo Code,
Continue, Supermaven. Residual risks 8–10 in the ADR 0001 revision-7 addendum. Finding→test
table in the PR body. Needs exact-head GPT re-review, Gemini adversarial re-check and triage
before bootstrap attestation.

**PR #23 revision-12 review (2026-11-14 UTC):** exact head
`d996aac5b9d439d9f328d7f7b47a4c75ec540a91` re-reviewed; verdict **must-fix**.
All eight round-11 original findings are fixed with mapped regressions; all 212
submitted tests, format, clippy, xtask all and the full build gate pass. Independent
CLI/hook replay confirms the original fixes. New **CORE** R12-01: a two-byte
encoding prefix makes the decoder hide an otherwise recognized raw ASCII token;
provenance and xtask all pass, including after committing the synthetic fixture.
The rebuilt revision-11 checker rejects that exact file. Do not attest this head
as the bootstrap seed. **PERIPHERAL** R12-02 (bare-tool verification/gratitude
footers) is a safe tracked post-merge follow-up after the CORE fix and re-review.
Reproductions, round-11 mappings/triage and M0 limitation assessment:
[revision-12 report](reviews/gpt-2026-11-14-pr23-r12.md). PR #23 was not modified.

**PR #23 revision-11 review (2026-11-14 UTC):** exact head
`d71cf40d1fdae6eb62de28c366ce925ec3108579` re-reviewed; verdict **must-fix**.
Both GPT round-10 reproductions and five Gemini findings are fixed; bare-tool
trailers are fixed but the two original generated-by footers remain a safe tracked
**PERIPHERAL** follow-up (R11-02). All 205 submitted tests, format, clippy, xtask all,
the full build gate and range attribution pass. New **CORE** R11-01: a known token
signature in ordinary UTF-16/32 text passes provenance; the UTF-8 control fails.
The BOM-bearing UTF-16 fixture also passes xtask all and remains undetected after
being committed in a disposable clone. No live credential or restricted data was
used. Do not attest this head as the one-time bootstrap seed until that defect is
fixed and the resulting head reviewed. Reproductions, round-10 mappings/triage and
the M0 limitation assessment: [revision-11 report](reviews/gpt-2026-11-14-pr23-r11.md).
PR #23 was not modified.

**PR #23 revision-10 review (2026-11-14 UTC):** exact head
`88148f01fe91dc6f3f62577d3130ca1e3261fab9` re-reviewed; verdict **must-fix**.
All twelve round-9 reproductions are addressed; all 197 submitted tests, the full
build gate, format, clippy, xtask all and range attribution pass. Fifteen original
credit inputs also fail the CLI and are stripped by the hook. A new **CORE**
provenance counterexample remains: composed internal XML entities hide a valid
RDF namespace in a node-root network graph, so both provenance and xtask all pass
unregistered data. A separate **PERIPHERAL** diff-budget fallback misattributes
kept human PR-body lines to the bot; that item can be a tracked, owner-accepted
follow-up, with the ADR guarantee corrected. Do not attest this head as the
one-time bootstrap seed until the CORE defect is fixed and the new head reviewed.
Reproductions, all round-9 mappings/triage and M0 limitation assessment:
[revision-10 report](reviews/gpt-2026-11-14-pr23-r10.md). PR #23 was not modified.

**PR #23 revision-9 review (2026-11-14 UTC):** exact head
`e64acc5cfb7343485e660436cfd9fb8da1a0e5dd` re-reviewed; verdict **must-fix**.
All 185 submitted tests and the full build gate pass. All original round-8
reproduction inputs are addressed; nineteen attribution strings were also
independently rejected by the CLI and stripped by the hook. Three new STRUCTURAL
probes remain: a partial bot edit exempts inherited owner-authored credit,
actual CI check display names are omitted from name protection, and valid RDF/XML
without an XML declaration escapes sniffing when it uses a default namespace.
Historical restoration of genuinely bot-originated text is consistent with D-61;
whole-block novelty does not prove each line's origin. Risk 11 is accurate, while
risk 12 omits that inherited-content path. Do not attest this head as the one-time
bootstrap seed. Reproductions, controls, round-8 triage and the M0 assessment:
[revision-9 report](reviews/gpt-2026-11-14-pr23-r9.md). PR #23 was not modified.

**PR #23 revision-8 review (2026-11-14 UTC):** exact head
`bcd4b2c7aa933662ad952f3656b1a1f408057fc3` re-reviewed; verdict **must-fix**.
All 176 submitted tests pass; the round-7 reproduction inputs are rejected,
including the original native linker-script fixture. One STRUCTURAL rule conflict
remains: the attribution gate rejects D-61-permitted automatic bot summaries and
has no provenance-qualified block exception. The round-7 recommendation to remove
the integration is withdrawn. Standard OS library roots are reasonable M0 location
trust under the documented job-tampering limit; hosted passwordless sudo means they
are not process isolation. Do not attest this head as the one-time bootstrap seed.
Evidence, regression matrix and required correction:
[revision-8 report](reviews/gpt-2026-11-14-pr23-r8.md). PR #23 was not modified.

**PR #23 revision-7 review (2026-11-14 UTC):** exact head
`f986bbc48b23390ca50f4b22065313bcb08d94ea` re-reviewed; verdict **must-fix**.
All 163 submitted tests pass, and all fourteen round-6 reported inputs are fixed.
Two new STRUCTURAL probes remain: an allowlisted library name resolves through
an OUT_DIR linker script to a Python-named archive, and dev-dependency feature
unification hides a production single-feature native link. A CodeRabbit credit
also escapes the shared attribution patterns (ATTRIBUTION-SPELLING). The first
two gaps are outside the ADR's stated tampering/multi-feature limitations.
Do not attest this head as the one-time bootstrap seed. Reproductions, controls,
the finding/test matrix and M0 assessment:
[revision-7 report](reviews/gpt-2026-11-14-pr23-r7.md). PR #23 was not modified.

**PR #23 revision-6 review (2026-11-14 UTC):** exact head
`68a583d88276cd4339c0d08cebf6f0c8367ca76a` re-reviewed; verdict **must-fix**.
The four GPT round-5 inputs and Gemini's actionable underlying gaps are fixed;
two literal Gemini reproduction layouts need the qualifications in the report.
Four new STRUCTURAL counterexamples remain: a real crate named `___` skips the
wrapper, response-file quoting hides a native library name, all-features builds
omit default-feature link code, and doctests do not inherit the claimed root
unsafe forbid. Do not attest this head as the one-time bootstrap seed.
Reproductions, controls, regression matrix and M0 limitation assessment:
[revision-6 report](reviews/gpt-2026-11-13-pr23-r6.md). PR #23 was not modified.

**PR #23 revision 6 (implementation, 2026-11-13):** R5-01..04 (GPT) and ADV23R5-01..10
(Gemini) addressed by a change of approach (coordinator decision): the static Rust
reachability walker is deleted. What Cargo builds now comes from `cargo metadata`
(resolved members, targets, path deps, patches, build-deps); the cheap static rules stay
(crate-root `forbid`, manifests, alias-only Cargo config, workflows). New
`cargo xtask build-gate`, a step of the unprivileged `policy` CI job, builds the PR
workspace (all targets, all features) with main's xtask as `RUSTC_WRAPPER`: `-F unsafe_code`
for every repository crate outside `crates/solver-*` (lint caps stripped), plus a linker
guard and Cargo `build-script-executed` output for the Python/Julia ban. Workflows may set
no `CARGO_*`/`RUSTC*`/`RUSTDOC*`/`RUSTUP_*`/`*RUSTFLAGS` variable unless `allowed_env` in
the trusted `gates.toml` lists it. Attribution: generic verb/joiner/"help of" footer
phrases, any `<Words> by:` trailer with a model-shaped name, and Cody/Sourcegraph,
CodeWhisperer, GLM, Command R in the single name list. Residual risks in the ADR 0001
revision-6 addendum. Finding→test table in the PR body. Needs exact-head GPT re-review,
Gemini adversarial re-check and triage before bootstrap attestation.

**PR #23 revision-5 review:** exact head `b21d83de896ba09b59259fa687e427271bbc06ed`
re-reviewed; verdict **must-fix**. All 143 submitted tests pass, and all fourteen
round-4 concrete inputs are fixed. Four new STRUCTURAL probes, each verified with
real offline Cargo compilation and a rejection control, expose a substring-based
OUT_DIR include exemption, an aliased include macro, Cargo member-glob mismatch
and lost build context through a local patch. Do not attest this head as the
one-time trusted bootstrap seed. Evidence and finding-by-finding regression
assessment: [revision-5 report](reviews/gpt-2026-11-13-pr23-r5.md).
PR #23 was not modified.

**PR #23 revision-4 review:** exact head `dc3093ea100ace554ad3a7c11a9f77927f2776a7`
re-reviewed; verdict **must-fix**. All 124 submitted tests pass, and all eleven
round-3 concrete inputs are fixed. Four new STRUCTURAL probes demonstrate a
production target escaping through `spikes/`, incorrect `**/` CODEOWNERS coverage,
nested-array provenance loss and ignored extensionless manifests. Do not attest
this head as the one-time trusted bootstrap seed. Reproductions, regression
assessments and activation limits: [revision-4 report](reviews/gpt-2026-11-13-pr23-r4.md).
PR #23 was not modified.

**PR #23 revision 4 (implementation, 2026-11-13):** R3-01..04 (GPT) and ADV23R3-01..07
(Gemini) addressed with general mechanisms and mapped regression tests (finding→test
table in the PR body): structural parse of the `forbid` lint list; every Cargo-selected
target scanned regardless of suffix; no currentness exemption for the reviews directory;
`?` wildcard with a differential test against `git check-ignore`; NFKD plus removal of
all combining marks; vendor-prefixed model names and vendor domains; generic comment
delimiter stripping; shell allow-list including inherited `defaults.run.shell`; a
schema rule for manifest file keys; one canonical manifest name. Needs exact-head GPT
re-review, Gemini adversarial re-check and triage before bootstrap attestation.

**PR #23 revision-3 review:** exact head `b2173e28f8f232366ffe6a27d5e787fd2adfc39c`
re-reviewed; verdict **must-fix**. All 111 submitted tests pass. The original
F1–F9 / ADV23R-01..09 inputs are addressed (D-74 account/approval activation
remains operational), but four independent probes expose remaining unsafe-lint,
Cargo-target, review-currentness and CODEOWNERS gaps. Do not attest this head as
the trusted policy seed. Evidence and finding-by-finding regression assessment:
[revision-3 report](reviews/gpt-2026-11-13-pr23-r3.md). PR #23 was not modified.

**PR #23 revision-2 review:** exact head `b38db0675fa3b2eb6a05600fc6fb22d85e0dd8cd`
re-reviewed; verdict **must-fix**. All 81 submitted tests pass, but nine remaining
enforcement findings require correction and exact-head re-review. Evidence and
R1–R8 / ADV23-01..18 dispositions:
[revision-2 report](reviews/gpt-2026-11-13-pr23-r2.md). PR #23 was not modified.

**PR #23 revision 3 (implementation, 2026-11-13):** F1–F9 and ADV23R-01..09 addressed with
mapped regression tests (finding→test table in the PR body). Trusted gates now run from
`main` (`trusted-gates.yml`); PR jobs fail closed until `main` holds policy. **Coordinator
action before CI can pass:** commit `docs/process/ci-bootstrap.toml` with
`seed_sha = "<final PR #23 head>"` to `main` and re-run CI; then GPT re-review, Gemini
adversarial re-check and a triage record for every adversarial finding (format in
REVIEW-BRIEF.md). The D-74 machine account (`egs-bot` placeholder) is still to be created.

**Merged to main:** GB data catalogue (PR #1, verified), IE/weather/dependency-licence catalogues (PR #2,
verified), reviews and triage for requirements, PR #4 and PR #5 (PRs #3, #6, #7, #8, #9).

**2026-11-13: workspace moved to the coding container (D-65); work resumed.** Running: Claude and Gemini reviews of the revised PRs #4 and #5, GPT re-verification of PR #12, and Gemini gathering of the 2024 Irish prices. Next: merge, clean the history and make the repo public, then GPT drafts the topology ADRs 0002/0004/0005/0006 (recommended: a modular monolith).

| Item | State | Next action |
|---|---|---|
| PR #4: MVP requirements + ADR 0001/0003 | Round 3 revision pushed (`52fda46`); reviews #13, #15 merged | Claude re-check of round 3; then owner approves accuracy thresholds/defaults; merge |
| PR #17: topology ADRs 0002/0004/0005/0006 | Round 2 revision pushed (`d99ceab`); reviews #19, #20 merged | Claude re-check; then 4 owner questions (Valkey, reconstruction label, no uploaded market code before P4, forecast priority); merge |
| PR #5 data spike | **Merged** (`0a14265`) | none |
| PRs #10, #11, #12, #18 data | **Merged** (verified by GPT; known source gaps in each COVERAGE.md) | none |
| History cleanup (D-52) + public repo (D-58) | **Done** 2026-11-13: trailers stripped from all commits, history scanned (no secrets, no raw research-only data), repo public, `main` protected (no force-push, linear history) | Add required checks once M0 CI exists |
| Owner inputs | Open | Copernicus account (ERA5), ENTSO-E token, UKPN/SPEN/NPg portal logins, target-system specs (D-65) |

Rules in force: GPT for non-code (D-51), Claude reviews GPT docs (D-54), Gemini adversarial reviews (D-53)
and significant data gathering (D-62), no AI attribution (D-52), research-phase data use (D-56).
Only the coordinator edits STATUS and the decisions log.

## Next steps (agents may do now)

1. Data spike (D-31): permissively licensed GB + Ireland full transmission network + one weather
   year; record licence per dataset; answers R-12.
2. Add GB + Ireland data sources to `prior-art.md` with licences (review #7); verify licence
   flags for cuDSS, SuiteSparse/KLU, TimescaleDB on PG18 (review #11).
3. Draft MVP requirements baseline in `04-requirements.md` (review #12), marking open items `ASSUMPTION:`.
4. ADRs 0001–0006 (review §4 step 8).
5. Flip to "code allowed" once 3 and ADRs 0001–0003 are accepted.
