- Author role/provider: code generation / Anthropic
- Reviewer role/provider: adversarial review / Anthropic
- Reviewed head: `70ceba9f58fccdd7c822588b400d876edceed786`
- Verdict: **request changes**

Owner-authorised exception (2026-11-27): same-vendor review by a fresh Claude thread with no prior context, because GPT is unavailable (t3code#39101) and Gemini is retired (D-91). #99 is merged by an authorised admin merge.

# PR #99 adversarial review at 70ceba9 (2026-11-27)

Goal: find any way #99 weakens the review gates beyond what D-91 intends. Tests at the head: 11/11 pass.

### ADV99-01: same-vendor adversarial review passes when the author header is missing or names two vendors (blocking)
xtask/src/review_gates.rs:272-279. `party_vendor(header.author)` returns `None` when the Author line is missing, names no known vendor, or names two vendors (`Matrix::vendor`, line 227-230). The check `author.as_ref() != Some(&vendor)` then passes. Scenario: an Anthropic-authored PR carries `claude-...-adversarial.md` with `- Reviewer role/provider: adversarial review / Anthropic` and either no Author line or `- Author role/provider: code generation / OpenAI+Anthropic`. `adversarial_valid` returns true. The PR's author vendor then satisfies its own adversarial gate, which D-91 forbids. Fix: treat a missing or ambiguous author as invalid, and add tests for both cases.

### ADV99-02: the PR author's vendor may self-triage adversarial findings (non-blocking; matches D-91)
xtask/policy/review.toml:49 (`vendors = ["openai", "anthropic"]`), `triage_valid` line 283-287. `triage_valid` does not compare the triager with the author. Scenario: Claude writes a PR, GPT supplies the adversarial review, and Claude (the author's vendor) writes a `coordinator triage / Anthropic` record that marks every finding `rejected`. The gate passes. D-91 says only "the coordinator triages". GPT is the coordinator (D-86), so allowing Anthropic triage goes beyond the text of D-91. Recommend the owner either confirm Anthropic triage or require triager vendor != author vendor.

### ADV99-03: Google is still an accepted adversarial vendor for new reports (non-blocking)
xtask/policy/review.toml:41. D-91 retires Gemini, but a newly written `adversarial review / Google` report still satisfies the gate for any head. The intent that only existing reports count is enforced only by process. Suggested fix: allow Google only for report files whose date is before 2026-11-27, or record this as an accepted gap.

### ADV99-04: the author vendor is self-asserted in the report (non-blocking, pre-existing)
Neither the author nor the reviewer header is checked against the PR's commits or account. Under D-91 the anti-self-review rule rests entirely on the Author line, which the reviewer writes. This is the same trust model as before.

### ADV99-05: the spawn-agent rule has no decision (non-blocking)
AGENTS.md rule 11 and team.md rule 8 add a GPT prompt rule citing an external issue, not a D-number. This goes against AGENTS.md rule 1 (never invent a decision). Record it as an ASSUMPTION or as its own decision.
