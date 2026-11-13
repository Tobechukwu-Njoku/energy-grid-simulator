# Focused re-check of PR #4, round 3

_Reviewer: Claude Opus 5.5 (D-54: Claude reviews GPT-authored documents) · 2026-11-13_
_Under review: PR #4 head `52fda460` (`t3/design-review-mvp-requirements-adrs`)_

Scope: I checked only whether the round 2 must-fix items are resolved, and whether the
round 3 changes introduced regressions. The must-fix items come from:

- `claude-review-pr4-rev2-2026-11-13.md` (M-1–M-6);
- `gemini-adversarial-pr4-rev2-2026-11-13.md` (ADV4-R2-01–08, the items the coordinator
  triaged as fixes);
- the coordinator triage recorded in the "Round 3" section of `response-pr4-2026-11-13.md`
  (triage items 2–8).

This is not a full re-review. I did not edit any PR file, STATUS.md or the decisions log.

## Summary

All six document must-fix items (M-4–M-6 and triage items 2–8) are resolved. The hook
fixes for M-1–M-3 work for every case they were tested against. The PR's 121 fixtures pass,
and all 21 of Gemini's cases pass.

However, the round 3 rewrite of the hook patterns introduced two regressions against the
rev-2 hook:

- **Human co-author lines are dropped again.** A human's GitHub noreply address is
  deleted when the username merely *ends* in a model or tool token, such as
  `123+octopus@users.noreply.github.com` (`opus`) or `secretagent` (`agent`).
- **Common model and bot formats are no longer stripped.** Lines naming `GPT-4o`,
  `gpt-5-codex`, `Gemini … Flash-Lite` or the `gemini-code-assist[bot]` app, and the
  Claude Code footer written with the emoji variation selector, now pass through. The
  rev-2 hook stripped all of them.

Both regressions are small regex fixes. I prototyped them (§C.4); the prototype keeps all
121 fixtures passing.

## A. Test results

Environment: Perl 5.40.1 and git 2.47.3. Hooks were taken from `52fda460` and, for
comparison, from rev 2 at `1698c4be`.

| Suite | Round 3 hook (`52fda460`) | Rev-2 hook (`1698c4be`) |
|---|---|---|
| PR fixtures, `python3 .githooks/tests/run.py` | **PASS** (121 fixtures, plus pattern-file, Git commit/merge and encoding checks) | n/a |
| My 55 rev-2 cases (body position) | **51/55** | 28/55 |
| Gemini ADV4-R2-01 cases (21 body lines) | **21/21** | n/a |
| Gemini emoji subjects (2 single-line commits) | **2/2 kept**, exit 0 | emptied to 0 bytes (git then aborts) |
| All 18 "keep" lines from both sets used as single-line subjects | **18/18 kept** | n/a |
| All 58 "strip" lines from both sets used as subjects (should abort, leaving the message unchanged) | 54/58 abort, and no message is ever rewritten; the 4 that do not abort are the 4 misses below | n/a |
| New regression probes (25 lines, §C) | **14/25** | 25/25 |

End-to-end checks in a scratch repository with `core.hooksPath` set all behaved correctly:

- A Claude Code-style `git commit -m … -m "🤖 Generated with [Claude Code](…)" -m
  "Co-Authored-By: …"` commit is stripped to subject and body.
- A subject `Generated with Claude Code` aborts with the documented diagnostic.
- CRLF messages keep their human trailers and lose the attribution line.
- The `git commit -v` scissors section is preserved verbatim.

The four misses among my 55 cases:

| Line | Rev 2 | Round 3 |
|---|---|---|
| `Generated using Anthropic's Claude` | stripped | **kept (regression)** |
| `🤖️ Generated` (U+1F916 U+FE0F) | stripped | **kept (regression)** |
| `Co-authored-by: Devin AI <158243242+devin-ai-integration[bot]@users.noreply.github.com>` | kept | kept |
| `Co-authored-by: aider (deepseek/deepseek-chat) <noreply@aider.chat>` | kept | kept |

The last two are the real trailer formats those tools emit. The round 3 fixtures cover Devin
and aider only in synthetic forms (`Authored-by: Devin <model@example.org>`), so the S-10
"covers Devin, aider" claim holds only for those synthetic forms.

## B. Must-fix resolution

| Item | Resolved? | Evidence at `52fda460` |
|---|---|---|
| M-1 subject and human false positives | **Partly**: see R3-M1 | The subject is protected: the hook aborts with a clear message and leaves the message byte-identical. The banner and pictograph rules are anchored to whole lines. Claude Monet, Claudette, Grokowski, Shannon, `jane@google.com`, cursor prose and robot-icon subjects are all kept. The response's correction of the earlier "preserves human trailers" claim is present. **New:** human GitHub noreply addresses ending in a tool token are dropped. |
| M-2 trailer and verb coverage | **Partly**: see R3-M2 | Any `*-by:`/`*-with:` key, Unicode hyphens, quoted trailers, the new verbs, `with:`, "with the help of", GPT6/Claude3/Claude_Code and the wider model families all work. Conformance fixtures are committed. **New:** model names with a letter tail or extra suffix, `[bot]` apps other than copilot/codex/claude, and the emoji variation selector now pass through. |
| M-3 malformed input and pattern robustness | **Yes** | Strict decode uses `FB_CROAK`. ISO-8859-1 input aborts and leaves the message unchanged (tested via `run.py` and directly). Blank and `#` pattern lines are ignored, CRLF pattern files work, and empty-string, invalid-regex, invalid-UTF-8 and empty rule sets all fail closed. |
| M-4 / triage 2: D-65 | **Yes** | NFR-5 statement and AC1, NFR-6.AC1, NFR-12, ADR 0001 L199–L209, ADR 0003 L245–L250, Q-N-7, roadmap M1 and the handoff (L16, L234) now name the separate D-65 target system. D-48 is labelled a provisional sizing ASSUMPTION, and the coding container is explicitly excluded. |
| M-5 / triage 5: FR-17 stress event | **Yes** | FR-17.AC1 now uses three complete UTC days, a 72-hour mean wind capacity factor below 10% and each day's mean demand above that year's 75th percentile of daily means. Overnight troughs no longer disqualify an event, screening comes before replay, and the synthetic fallback is labelled and routed to Q-N-4. |
| M-6 / triage 6: gate scope | **Yes** | FR-12.AC4, the table preamble and Q-N-4 all state that the gates apply to the perfect-information reconstruction and that as-of metrics are report-only in P1. |
| Triage 3 / ADV4-R2-02: REMA/MHHS | **Yes** | FR-1.AC2 keeps date-effective capability; FR-1.AC5 defers MHHS settlement to P2 and REMA pricing to P3, so neither is a P1 fixture. |
| Triage 4 / ADV4-R2-04: M0 security deadlock | **Yes** | M0 requires an NFR-12 policy review only. NFR-12.AC1 runs at M1, on the D-65 target, before the first heavy B1 job, using canary credentials and no exploit. The requirements milestone table, roadmap M1 and ADR 0001 agree. |
| Triage 7 / ADV4-R2-05 and -06: thresholds and BM comparator | **Yes** | Skill ≥ 0.15. Mix gate is 5 pp absolute AND 20% relative, with a zero-observed check. There is a two-sided adequacy/over-commitment gate. BM is compared against NESO's constraint-cost breakdown only, with up/offer and down/bid scored separately. RO/CfD lost-subsidy and storage opportunity costs appear in FR-7.AC2 and ADR 0003 stage 2. |
| Triage 8 / ADV4-R2-07 and -08: counterflow and repair | **Yes** | ADR 0003 L94–L119: one signed flow per link, frozen within a single-sign breakpoint interval, with an affine loss *equality*. The stage 2 LP therefore has no opposing column and no loose loss row, so negative prices cannot create a loss sink. Repair (L43–L60) is bounded: three UC re-solves, fingerprint-based early stop, one 300 s recovery MILP and a 900 s window budget, with failure reported as failed/partial. Elastic slacks are diagnostic only. The ADR does not claim convergence. |

Rejected Gemini findings (ADV4-R2-09–11) and the partial acceptance of ADV4-R2-12: I agree with
the stated reasons and have nothing to add. They were outside the must-fix set.

Housekeeping, verified:

- The PR leaves `STATUS.md` and `03-decisions-log.md` unchanged against the merge base.
- No PR-only commit message, and not the PR body, contains attribution.
- The head merges cleanly into current `origin/main` (`7eade25`; `git merge-tree`, no conflicts).

## C. Regressions introduced in round 3

### C.1 R3-M1: human GitHub noreply addresses are dropped (false positive)

`.githooks/attribution-patterns.regex` line 2 contains this e-mail alternative:

```
[^\r\n]*<?(?:…|(?:agent|claude|…|opus|…|llama|…)(?:\[bot\])?@(?:users\.)?noreply\.(?:github\.com|google\.com)|…)
```

The leading `[^\r\n]*` lets the vendor token match the *end* of any local part. As a result,
any trailer key followed by a human's GitHub noreply address is deleted when the username
ends in a listed token:

| Line | Rev 2 | Round 3 |
|---|---|---|
| `Co-authored-by: Ann Lee <123+octopus@users.noreply.github.com>` | kept | **dropped** |
| `Co-authored-by: Sam Roe <456+secretagent@users.noreply.github.com>` | kept | **dropped** |
| `Co-authored-by: Dr Llama <789+drllama@users.noreply.github.com>` | kept | **dropped** |

This is the silent human-trailer deletion that M-1 targeted. It also contradicts the response's
M-1 statement that "human Google/GitHub-noreply addresses" are preserved. Fixture `keep-6`
(`123+jane@…`) passes only because `jane` does not end in a token.

### C.2 R3-M2: model, bot and footer variants that rev 2 stripped now pass through (false negative)

Each line below was stripped by the rev-2 hook and is kept by the round 3 hook. They become
D-52 violations in a commit, and later in the CI scan that shares this pattern file.

| Line | Cause |
|---|---|
| `Co-authored-by: GPT-4o <gpt@example.com>`, `Generated with GPT-4o` | Version grammar `v?\d+(?:[.\s_-]\d+)*` allows no letter tail (`4o`) |
| `Generated with gpt-5-codex` | `codex` is not in the suffix list; `code` matches, then `x` fails the `\s*$` anchor |
| `Generated with Gemini 2.5 Flash-Lite` | `lite` is not in the suffix list |
| `Co-authored-by: gemini-code-assist[bot] <176961590+gemini-code-assist[bot]@users.noreply.github.com>` | The `[bot]` rule lists only copilot, codex and claude. This is the real trailer that GitHub writes on squash merges after a Gemini Code Assist suggestion. |
| `🤖️ Generated with [Claude Code](https://claude.com/claude-code)` | The U+FE0F variation selector after U+1F916 is not allowed. Editors and keyboards commonly insert it. |
| `🤖️ Generated`, `Generated using Anthropic's Claude` | Same selector issue; the possessive form is not covered |

### C.3 Notes (not blocking)

- **Quadratic matching on pathological lines.** In the trailer rule, `[^\r\n]*<?[^<>\s]+@` makes
  matching quadratic on a single unbroken token after a `*-by:` key: 2 k characters take
  0.16 s, 4 k take 0.67 s and 8 k take 2.3 s. Rev 2 was flat. No realistic message is
  affected, but a future CI history scan should cap line length or use a possessive
  `[^<>\s]++`.
- **Ambiguous single-name human.** `Reviewed-by: Claude <claude.dupont@example.fr>` (a human
  known by one name) is now dropped. This is an acceptable trade-off; full names are kept.
- **Trailing punctuation.** `Co-authored-by: Claude <noreply@anthropic.com>.` is kept. This is
  minor.
- **Real Devin and aider formats** (§A) are still missed. The fix is to add `aider\.chat` to
  the e-mail domains; the generalised `[bot]` rule in §C.4 covers Devin. This is should-fix,
  because neither tool is in this team's routing.
- **Stale data block after merge.** Main now carries verified 2024 I-SEM price data
  (`ef65418`). After merging, the coordinator should revisit the "Known data block" text in
  `04-requirements.md`, Q-N-3 and the handoff. Those passages are conditional ("until …
  gathered and independently verified"), so they remain correct as written.
- **Stale disposition rows.** Rows C-14 and ADV-1 in the response still say heavy work runs
  "on D-48". The round 3 section supersedes them, so a one-line pointer would be enough.
- **Small-share mix gate.** The 20% relative-MWh gate applied to technologies with a share of
  about 0.1% (oil, OCGT) is likely to fail on noise. This is an owner matter under Q-N-4; a
  materiality floor is worth offering there.

### C.4 Prototype fix (tested)

I applied these edits to a copy of `attribution-patterns.regex`. The commit-msg script needs no
changes.

1. **Noreply local part on a token boundary.** Prefix the noreply token group with
   `(?<![\w.-])`, giving `(?<![\w.-])(?:agent|claude|…)(?:\[bot\])?@(?:users\.)?noreply\.…`.
   `175728472+Copilot@` still matches; `octopus@` and `jane.gemini@` do not.
2. **Any vendor `[bot]` app.** Replace
   `[^<>\s]*(?:copilot|codex|claude)\[bot\]@` with
   `[^<>\s]*(?:copilot|codex|claude|gemini|devin|cursor|openai|anthropic|jules)[^<>\s]*\[bot\]@`.
3. **Version letter tail.** Replace every `v?\d+(?:[.\s_-]\d+)*` with
   `v?\d+[a-z]?(?:[.\s_-]\d+[a-z]?)*`.
4. **More suffixes.** Add `codex|lite|preview|cli|assist` to the trailing-suffix
   alternations, and `codex|lite|preview` to the post-version alternation.
5. **Variation selector.** Replace every `\x{1F916}` with `\x{1F916}\x{FE0F}?`.
6. **Optional, for the note above.** Add `aider\.chat` to the e-mail domains.

Results with the prototype:

| Suite | Result |
|---|---|
| PR fixtures (`run.py`) | **PASS**, 121 fixtures plus all the extra checks |
| My 55 cases | 53/55; the remaining misses are the possessive and bare `🤖️ Generated` lines |
| Gemini cases | 21/21 |
| Probes | 23/25; the remaining misses are the two non-blocking notes above |
| Timing | Unchanged and linear on the digit and banner stress cases |

Add every line from the §C.1 and §C.2 tables to `fixtures.json` (keep and strip respectively)
so that the regressions cannot return.

## Verdict

**MUST-FIX.** The round 3 revision resolves M-3, M-4, M-5, M-6 and coordinator triage items
2–8 completely, and it resolves the substance of M-1 and M-2. Two hook regressions introduced
by the round 3 pattern rewrite remain:

1. **R3-M1:** stop the noreply e-mail rule from matching vendor tokens at the end of a human
   GitHub username (prototype edit 1). Add the three §C.1 lines as `keep` fixtures.
2. **R3-M2:** restore stripping of the model, bot and footer variants that rev 2 caught:
   `GPT-4o`, `gpt-5-codex`, `…Flash-Lite`, `gemini-code-assist[bot]` and the U+FE0F Claude Code
   footer (prototype edits 2–5). Add the §C.2 lines as `strip` fixtures.

Both items are confined to `.githooks/attribution-patterns.regex` and `fixtures.json`. No
requirement, ADR or owner decision is affected. After the fix, a fixture run plus the §C
probes is sufficient; no further document review is needed.
