# Adversarial Review (Rev 2) — PR #4 Baseline Requirements, ADRs 0001 & 0003, and Commit Hook

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-13  
**Target:** PR #4 revised at head `1698c4be` (`origin/t3/design-review-mvp-requirements-adrs`)  
**Target Documents:**
- `docs/requirements/04-requirements.md` (lines 1–429)
- `docs/architecture/adr/0001-repo-layout-and-ci-gates.md` (lines 1–225)
- `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md` (lines 1–242)
- `.githooks/commit-msg` (lines 1–27)
- `.githooks/attribution-patterns.regex` (lines 1–3)
- `docs/requirements/02-questionnaire.md` (lines 1–238)
- `docs/requirements/06-roadmap.md` (lines 1–47)
- `docs/requirements/reviews/response-pr4-2026-11-13.md` (lines 1–159)

**Baseline Context:** `AGENTS.md`, `docs/requirements/03-decisions-log.md` (D-1 through D-65).

---

## Executive Summary

This adversarial review evaluates the revision of PR #4 at head `1698c4be`. The revised PR attempts to resolve the earlier findings (ADV-1 through ADV-12 and G-1 through G-12) by decomposing the Phase 1 problem into a four-stage optimization architecture (ADR 0003), establishing bootstrap CI gate profiles and a local attribution-stripping hook (ADR 0001, `.githooks/commit-msg`), and formalizing 33 numbered requirements with proposed numerical validation thresholds.

While the revised PR shows substantial structural discipline, an aggressive adversarial audit reveals critical flaws that compromise technical feasibility, violate owner decisions, allow gameable acceptance criteria, and fail attribution enforcement:

1. **Commit Hook & Regex Failures:** Real message testing of `.githooks/commit-msg` demonstrates catastrophic false positives (stripping legitimate engineering commits mentioning standard technical terms like database/terminal "cursors", historical authors like Claude Shannon, or emoji commit subjects) alongside critical false negatives (failing to strip trailers for unlisted AI model families such as Sonnet, Opus, DeepSeek, Qwen, Llama, Mistral, alternative trailers like `Assisted-by:`, and common creation phrases like `Made with ChatGPT`).
2. **Direct Contradictions of Decision Log:** PR #4 mandates dated configurations for REMA and retail MHHS transitions in Phase 1, directly contradicting the owner's phased roadmap in D-44. Furthermore, ADR 0001 and NFR-5 mandate heavy benchmark execution on the host environment without accounting for D-65's strict prohibition on heavy builds, databases, or benchmarks in the coding environment.
3. **Untestable Acceptance Criteria & Circular CI Gates:** Milestone M0 mandates negative security tests for Docker socket escapes, SSH credential sniffing, and fork runner isolation (NFR-12) as a blocking gate before any application code can be written, even though ADR 0001 concedes that runner infrastructure and GitHub workflows are not yet deployed.
4. **Trivially Gameable Validation Thresholds:** Proposed backtest gates allow a model with zero practical predictive skill (`skill > 0` over a naive previous-week persistence benchmark) to pass as validated. Unserved energy acceptance criteria are completely vacuous on historical transmission years with zero observed load shedding, while a 5 percentage-point absolute tolerance on generation mix permits multi-hundred-percent relative errors on low-carbon and peaker technologies.
5. **Architectural & Physical Inconsistencies:** The three-stage decomposition in ADR 0003 introduces an heuristic 3-iteration bounded repair loop that cannot guarantee convergence between integer unit commitment and continuous redispatch. Additionally, enforcing directional lossy interconnectors in a continuous LP redispatch stage creates fictitious counterflows and economic distortion under negative pricing unless binary variables are introduced, which destroys the LP formulation and invalidates shadow prices.

---

## Findings Matrix

| ID | Severity | File & Line Citation | Category | Short Title |
|---|---|---|---|---|
| **ADV4-R2-01** | CRITICAL | `.githooks/attribution-patterns.regex:1-3`, `.githooks/commit-msg:1-27` | Hook Failure / False Positives & Negatives | Commit hook strips legitimate code/commits while missing key AI trailers and banners |
| **ADV4-R2-02** | HIGH | `docs/requirements/04-requirements.md:14` | Decision Contradiction (D-44) | Mandating REMA and MHHS transitions in P1 contradicts D-44 phased delivery |
| **ADV4-R2-03** | HIGH | `docs/architecture/adr/0001:179-188`, `docs/requirements/04-requirements.md:282-286` | Decision Contradiction (D-65) | Benchmark and CI runner assumptions conflict with container environment restrictions |
| **ADV4-R2-04** | HIGH | `docs/requirements/04-requirements.md:350-357, 367`, `docs/architecture/adr/0001:212` | Untestable Acceptance / CI Deadlock | NFR-12 security penetration criteria are impossible to test in M0 bootstrap |
| **ADV4-R2-05** | HIGH | `docs/requirements/04-requirements.md:391-398` | Gameable Criteria / Minimal Thresholds | Trivially gameable skill scores, unserved energy vacuity, and distorted generation mix gates |
| **ADV4-R2-06** | HIGH | `docs/requirements/04-requirements.md:94-95, 394-395` | Untestable Criteria / Economic Inconsistency | Cost-based redispatch cannot meet historic BM outturn targets containing energy balancing and strategic markups |
| **ADV4-R2-07** | HIGH | `docs/architecture/adr/0003:32, 70-76` | Mathematical Flaw / Formulation Breakdown | Directional lossy interconnector formulation breaks continuous LP redispatch under negative prices |
| **ADV4-R2-08** | HIGH | `docs/architecture/adr/0003:42-48` | Technical Flaw / Infeasibility Loop | Bounded 3-iteration repair loop lacks mathematical convergence proof between UC and redispatch |
| **ADV4-R2-09** | MEDIUM | `docs/requirements/04-requirements.md:227-236`, `docs/architecture/adr/0003:31, 58-62` | Hidden Scope Creep (D-44, D-55) | Non-linear SNSP and RoCoF frequency stability constraints creep from Phase 4 into Phase 1 |
| **ADV4-R2-10** | MEDIUM | `docs/requirements/04-requirements.md:38` | Hidden Scope Creep (D-24, D-44) | GSP embedded wind and solar disaggregation creeps into Phase 2 DER scope |
| **ADV4-R2-11** | MEDIUM | `docs/architecture/adr/0001:136-140` | Hidden Scope Creep (D-28) | Unbounded mutation testing zero-survivor requirement exceeds D-28 merge gates |
| **ADV4-R2-12** | MEDIUM | `docs/architecture/adr/0003:33, 47-48`, `docs/requirements/04-requirements.md:122-123` | Economic Flaw / Pricing Dual Distortion | Fixing repaired out-of-merit network commitment in Stage 3 pricing LP distorts national marginal prices |

---

## Detailed Findings

### ADV4-R2-01: Commit Hook Strips Legitimate Code/Commits While Missing Key AI Trailers and Banners
- **Severity:** CRITICAL
- **Location:** `.githooks/attribution-patterns.regex:1-3`, `.githooks/commit-msg:1-27`, `docs/architecture/adr/0001-repo-layout-and-ci-gates.md:147-155`
- **Attack / Vulnerability:**
  The hook implementation and its regex list fail on both false positives (stripping valid engineering messages) and false negatives (allowing obvious attribution lines through).
  
  #### 1. Legitimate Lines Wrongly Stripped (False Positives)
  Pattern 2 in `.githooks/attribution-patterns.regex:2` states:
  ```regex
  \b(?:generated|written|authored|assisted|powered)\s+(?:with|by|using)\s+\[?(?:claude|anthropic|gpt|openai|codex|gemini|chatgpt|copilot|cursor|grok)\b
  ```
  - **Collision on standard software terms ("cursor"):** The word `cursor` is an everyday software engineering term referring to database query cursors, terminal text cursors, and UI mouse cursors. Any commit describing cursor behavior is erroneously stripped:
    - *Input:* `feat: add database pagination\n\nThis query was written using cursor pagination to avoid memory spikes.`
    - *Result:* The explanation line is completely DELETED from the commit message.
    - *Input:* `fix: TUI refresh\n\nThe status bar is powered by cursor movement hooks.`
    - *Result:* The explanation line is completely DELETED.
  - **Collision on human names ("Claude"):** A human author or cited paper author named Claude (e.g. Claude Shannon, father of information theory) is stripped:
    - *Input:* `docs: cite Shannon entropy\n\nThe formulation was authored by Claude Shannon in 1948.`
    - *Result:* Line DELETED.
    - *Input:* `Co-authored-by: Claude Shannon <cshannon@example.com>`
    - *Result:* Git trailer DELETED.
  - **Total commit abortion on emoji subjects (`\x{1F916}`):** Pattern 3 in `.githooks/attribution-patterns.regex:3` strips any line containing the robot emoji `\x{1F916}`.
    - If a developer writes a conventional single-line commit referring to bot/automation fixes:
      - *Input:* `docs: fix 🤖 emoji render bug` or `🤖 Fix CI runner status check`
      - *Result:* The entire commit message is stripped to an empty file. Git immediately aborts the commit with: `Aborting commit due to empty commit message`.

  #### 2. Attribution Lines the Hook Misses (False Negatives / Bypass)
  - **Unlisted AI model families:** Pattern 1 strictly lists specific keywords (`claude|anthropic|gpt|openai|codex|gemini|google\.com|copilot|cursor|grok|noreply\@(?:anthropic|openai)`). Major frontier and open-weight model families are completely omitted:
    - *Input:* `Co-authored-by: Sonnet 3.5 <noreply@github.com>` -> **MISSED** (retained verbatim)
    - *Input:* `Co-authored-by: Opus 5.5 <opus@example.com>` -> **MISSED**
    - *Input:* `Co-authored-by: DeepSeek-V3 <deepseek@example.com>` -> **MISSED**
    - *Input:* `Co-authored-by: Qwen 2.5 <qwen@example.com>` -> **MISSED**
    - *Input:* `Co-authored-by: Llama 3.3 <llama@example.com>` -> **MISSED**
    - *Input:* `Co-authored-by: Mistral Large <mistral@example.com>` -> **MISSED**
    - *Input:* `Co-authored-by: agent <agent@users.noreply.github.com>` -> **MISSED**
  - **Alternative git trailers:** Pattern 1 only matches `^\s*co-authored-by:\s*`. Other standard git attribution trailers are completely bypassed:
    - *Input:* `Assisted-by: Claude <claude@anthropic.com>` -> **MISSED**
    - *Input:* `Authored-by: GPT-4 <gpt@openai.com>` -> **MISSED**
    - *Input:* `Prompted-by: Claude <claude@anthropic.com>` -> **MISSED**
    - *Input:* `Pair-programmed-with: Gemini <gemini@google.com>` -> **MISSED**
  - **Common creation verbs:** Pattern 2 lists only `generated|written|authored|assisted|powered`. Common creation phrases are omitted:
    - *Input:* `Made with ChatGPT` -> **MISSED**
    - *Input:* `Built with OpenAI API` -> **MISSED**
    - *Input:* `Created with Copilot` -> **MISSED**
    - *Input:* `Crafted by Claude` -> **MISSED**
  - **Generic AI banners:**
    - *Input:* `Generated using AI` / `Assisted by LLM` -> **MISSED**

- **Recommendation:**
  1. Revise `.githooks/attribution-patterns.regex` to restrict trailer checks to git trailer format (`^[A-Za-z-]+-by:`) and expand the model family list (`sonnet`, `opus`, `haiku`, `deepseek`, `qwen`, `llama`, `mistral`, `o1`, `o3`).
  2. For `cursor`, disambiguate editor attribution (e.g. `\bcursor\s+editor\b` or `\bgenerated\s+with\s+cursor\s+\d+` or inspect header/footer placement) so that database/UI cursors are not stripped.
  3. Strip only the emoji `\x{1F916}` itself from the line rather than deleting the entire line, or qualify it so that a legitimate subject line is not wiped out.
  4. Include `made with`, `built with`, `created with`, `crafted with` in Pattern 2.

---

### ADV4-R2-02: Mandating REMA and MHHS Transitions in P1 Contradicts D-44 Phased Delivery
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md:14` (FR-1.AC2)
- **Attack / Contradiction:**
  `FR-1.AC2` explicitly mandates:
  > *"The topology and operating-rule registry supports dated configurations: historical Brexit (I-SEM explicit auctions, no single European market coupling), Market-wide Half-Hourly Settlement (MHHS) transitions, Review of Electricity Market Arrangements (REMA) changes, and interconnector commissioning dates."*
  
  This requirement directly contradicts owner decision `D-44`:
  > *"Priority principle: 'answer the present first.' Simulate the current grid very accurately, then scale out from that validated model to zonal pricing and future scenarios. Phase order: P1 current transmission (MVP) → P2 distribution first ... → P3 zonal pricing and 2030 deployment scenarios → P4 dynamic stability, SaaS hardening."*

  1. **REMA is Phase 3:** REMA is the UK Government (DESNZ) consultation on reforming the electricity market, centered primarily on introducing nodal or zonal locational marginal pricing and reforming the wholesale market. Zonal/locational pricing is explicitly assigned to **Phase 3** under D-40 and D-44. Furthermore, REMA has not concluded; no dated statutory regulations or operational rules exist today. Requiring P1 code to support hypothetical "REMA changes" is premature scope creep that cannot be tested against historical ground truth.
  2. **MHHS is Phase 2 Distribution:** Market-wide Half-Hourly Settlement (MHHS) is an ongoing GB retail market settlement reform placing distribution-connected smart meters onto half-hourly settlement schedules. Retail distribution settlement belongs in Phase 2 ("distribution first"), not in Phase 1 wholesale transmission dispatch.

- **Recommendation:**
  Remove "Market-wide Half-Hourly Settlement (MHHS) transitions, Review of Electricity Market Arrangements (REMA) changes" from `FR-1.AC2`. Restrict P1 dated configurations strictly to historical Brexit interconnector coupling changes and physical asset commissioning dates.

---

### ADV4-R2-03: Benchmark and CI Runner Assumptions Conflict with Container Environment Restrictions
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0001-repo-layout-and-ci-gates.md:179-188`, `docs/requirements/04-requirements.md:282-286` (NFR-5.AC1-AC2), `docs/requirements/03-decisions-log.md` (D-65)
- **Attack / Contradiction:**
  Decision `D-65` (ratified 2026-11-13) establishes a strict boundary for the agent work environment:
  > *"Environments: the LXC container (Debian 13, `/home/njotob/.t3/projects/Dev - GitHub - Energy`) is for coding and agent coordination only. The simulator runs on a separate target system (details to come); no heavy builds, databases or benchmarks on the coding container."*

  However, ADR 0001 (lines 179–188) and NFR-5 assume that the self-hosted runner executes heavy CPU/reference/benchmark jobs directly on the D-48 host machine (`AMD Ryzen 9 5900X, 128 GB RAM`) with ephemeral rootless containers, utilizing up to 96 GiB of RAM.
  Because the current development container is an LXC container running on Debian 13, agents and sub-agents working within this repository cannot launch nested rootless Docker containers, spin up PostgreSQL 18 / Redis databases, or run heavy benchmarks locally.
  Furthermore, ADR 0001 fails to define how CI jobs are scheduled between the agent container and the "separate target system" mandated by D-65, creating an unbridgeable operational contradiction for the B1 profile.

- **Recommendation:**
  Update ADR 0001 and NFR-5 to explicitly reference D-65. Clarify that benchmark regression gates (NFR-4, NFR-5) and database integration tests execute exclusively on the dedicated external target runner, and forbid any local harness or CI workflow from attempting to spawn nested benchmark containers inside the coding container.

---

### ADV4-R2-04: NFR-12 Security Penetration Criteria Are Impossible to Test in M0 Bootstrap
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md:350-357` (NFR-12.AC1-AC3), `docs/requirements/04-requirements.md:367` (Milestone M0), `docs/architecture/adr/0001-repo-layout-and-ci-gates.md:212`
- **Attack / Untestable Criteria:**
  Under `04-requirements.md:367`, Milestone M0 (governance/bootstrap) assigns `NFR-12` as a required gate before application code can begin.
  `NFR-12.AC1` demands:
  > *"Negative tests reject an unauthorised SHA, a fork PR on the self-hosted runner, licence/SSH-secret reads, host/container escape via a Docker socket mount and a stale review. No pull_request_target job executes PR code; no licence secrets or tokens enter result manifests."*

  This acceptance criterion cannot be satisfied or tested during Milestone M0:
  1. ADR 0001 explicitly concedes at line 212: *"No current workflow or protected-main configuration is claimed by this ADR."*
  2. The self-hosted GitHub Actions runner does not yet exist.
  3. No rootless container execution harness exists.
  4. Testing container escapes via Docker socket mounts requires an active Docker daemon and an offensive penetration harness, which is entirely outside the scope of an energy grid simulation platform and prohibited inside the LXC coding container (D-65).
  
  Mandating penetration-testing negative test suites as exit evidence for M0 creates an impossible requirement that blocks moving past the bootstrap phase.

- **Recommendation:**
  Reallocate NFR-12 operational runner verification to Milestone M3 (service/deployment) or mark it as an infrastructure policy checklist rather than an executable automated test suite in M0.

---

### ADV4-R2-05: Trivially Gameable Skill Scores, Unserved Energy Vacuity, and Distorted Generation Mix Gates
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md:391-398` (Numeric Accuracy Thresholds Table)
- **Attack / Trivially Gameable Thresholds:**
  The proposed backtest accuracy thresholds in the table at lines 388–400 contain severe mathematical and structural loopholes:

  #### 1. Price Baseline Skill Score (`skill > 0`) is Trivially Gameable (Line 392)
  The gate requires:
  $$\text{Skill} = 1 - \frac{\text{MAE}(\text{model})}{\text{MAE}(\text{persistence})} > 0$$
  - A skill score strictly greater than zero means an improvement of `0.0001%` over a naive persistence model passes the validation bar.
  - Furthermore, previous-week persistence (predicting price at time $t$ from $t - 168\text{ hours}$) is an exceptionally weak baseline for electricity prices because wind generation and temperatures vary wildly week-on-week. An economic simulator that cannot achieve a meaningful skill margin (e.g. $\ge 15\%$) is commercially and scientifically useless, yet passes this gate.

  #### 2. Unserved Energy Criterion is Vacuous (Line 398)
  Line 398 requires:
  > *"Zero false-shortage intervals on the held-out observed-adequate set at NFR-2 tolerance; if observed shortage exists, event total MWh error ≤ 10% and duration error ≤ one model interval per event"*
  - In Great Britain and Ireland, there were **zero** transmission-level rolling blackout or unserved energy events during recent historical years (e.g. 2020–2024).
  - Consequently, the conditional clause (`if observed shortage exists...`) is **never evaluated**.
  - Any trivial model that over-supplies generation or uses conservative generator availability will trivially produce zero false shortages and pass this gate with 100%, while completely failing to detect actual stress margins.

  #### 3. Technology Share 5 Percentage-Point Error Allows Massive Distortions (Line 393)
  Line 393 sets:
  > *"Annual technology-share absolute error ≤ 5 percentage points per technology"*
  - In Great Britain, small but critical flexible technologies have low single-digit annual market shares:
    - Coal outturn (2023): ~1.0%. A 5 percentage-point error allows the model to predict 6.0% (a **500% relative error**) or 0%.
    - Hydro outturn: ~1.8%. A 5 percentage-point error allows 6.8% (a **277% relative error**).
    - Biomass outturn: ~5.0%. A 5 percentage-point error allows 10.0% or 0% (complete omission).
  - Setting an absolute threshold of 5 percentage points without a relative error cap allows low-carbon and peaker technologies to be wildly distorted or wiped out entirely while passing the gate.

- **Recommendation:**
  1. Increase the minimum price skill score to $\text{Skill} \ge 0.15$ (or $\ge 0.20$) against persistence.
  2. Complement the unserved energy check with an operating reserve margin / loss-of-load expectation (LOLE) sensitivity check under synthetic de-rating.
  3. Formulate technology-share error as $\min(5\text{ percentage points}, 20\%\text{ of observed share})$ so that small technologies are protected from massive percentage distortion.

---

### ADV4-R2-06: Cost-Based Redispatch Cannot Meet Historic BM Outturn Targets Containing Energy Balancing and Strategic Markups
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md:94-95` (FR-7.AC2), `docs/requirements/04-requirements.md:141-142` (FR-12.AC2-AC3), `docs/requirements/04-requirements.md:394-395`
- **Attack / Structural Inconsistency:**
  Decision `D-59` mandates that the MVP Balancing Mechanism must be modeled as cost-based redispatch:
  > *"MVP Balancing Mechanism: cost-based redispatch (plants redispatched on estimated costs to relieve network/proxy limits), validated against historic BM costs and volumes. No replay of historic submitted bids/offers."*

  However, lines 394–395 set strict backtest accuracy gates:
  - Monthly BM/constraint cost error summed $\le 25\%$; annual absolute bias $\le 15\%$.
  - Monthly redispatch volume error summed $\le 20\%$; annual volume bias $\le 10\%$.

  This creates an unresolvable contradiction with historical data:
  1. **Energy Balancing vs. Network Constraints:** In the real GB Balancing Mechanism, NESO takes actions for two distinct purposes: system balancing (resolving network constraints) and energy balancing (rebalancing forecast errors in demand and wind/solar between gate closure and real time, frequency control, and generator trips). A deterministic, perfect-information simulation has **zero energy imbalance**. Its Stage 2 redispatch only resolves network/proxy constraints. Comparing constraint-only redispatch volumes against total historical BM volumes will inevitably exhibit massive negative volume bias ($> 10\%$).
  2. **Strategic Bidding Markups vs. Estimated Plant Costs:** Historical BM costs reflect generator strategic bidding behind constraints (e.g. Scottish wind curtailment bid pricing, CCGT offer prices exceeding £2,000/MWh to £4,000/MWh during system stress). A model calculating cost-based redispatch using short-run marginal costs (SRMC of fuel + carbon, e.g. £80–£120/MWh) will **never** match historic BM costs within 15% annual bias.
  
  Demanding that a pure SRMC cost-based redispatch model without bidding behavior match historical BM financial outturn within 15% bias sets an impossible validation bar that forces either gaming or permanent failure.

- **Recommendation:**
  Explicitly redefine the BM validation metric in `04-requirements.md:394-395` to benchmark against **NESO published constraint-specific outturn** (e.g. Monthly Balancing Services Summaries / MBSS constraint expenditure and volume data), rather than gross gross Balancing Mechanism outturn. Acknowledge that SRMC redispatch estimates the *economic cost of constraints*, not the *commercial settlement transfer*.

---

### ADV4-R2-07: Directional Lossy Interconnector Formulation Breaks Continuous LP Redispatch Under Negative Prices
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md:32, 70-76`
- **Attack / Mathematical Flaw:**
  ADR 0003 defines Stage 2 as:
  > *"Full-network DC redispatch LP with stage-1 commitment fixed... Minimise estimated incremental operating cost relative to the national schedule..."* (line 32)
  
  And specifies directional interconnector modeling:
  > *"Directional HVDC/interconnector variables distinguish sending and receiving MW with receiving = sending − non-negative losses... Use sourced piecewise loss curves or an explicit tested approximation; prevent simultaneous opposite-direction flows from manufacturing negative-price revenue or energy. A continuous relaxation is permitted only when its no-counterflow property is checked; otherwise use a supported direction formulation and declare any stage-2 integrality explicitly."* (lines 70–76)

  This formulation breaks down mathematically:
  1. **Fictitious Counterflow in Continuous LPs:** In a linear program without integer variables, modeling directional power flow $f_{u \to v} \ge 0$ and $f_{v \to u} \ge 0$ with losses ($f_{\text{rec}} = (1 - \alpha) f_{\text{send}}$) relies on the economic assumption that prices are positive and transmission is costly, ensuring at least one direction is zero. However, when local nodal prices become negative (e.g. high wind curtailment in Scotland or Ireland), the linear solver can minimize costs by simultaneously sending and receiving power on the interconnector ($f_{u \to v} > 0$ and $f_{v \to u} > 0$). This effectively "burns" surplus energy through artificial interconnector losses, creating a fictitious energy sink to avoid curtailment penalties!
  2. **Integrality Destroys LP Properties:** If binary variables are introduced to enforce non-simultaneous flow ($u_{\text{dir}} \in \{0, 1\}$), Stage 2 becomes an integer MILP. This directly contradicts line 32 ("redispatch LP"), prevents HiGHS from returning valid LP shadow prices/duals for network constraints, and causes solve times on the 4,000-bus network to explode beyond NFR-5 budgets.

- **Recommendation:**
  In Stage 2, fix the interconnector flow direction from the Stage 1 national schedule, or enforce a unidirectional flow variable per interconnector with a linearization that does not permit simultaneous bi-directional flow in continuous relaxation.

---

### ADV4-R2-08: Bounded 3-Iteration Repair Loop Lacks Mathematical Convergence Proof Between UC and Redispatch
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md:42-48`
- **Attack / Algorithm Breakdown:**
  ADR 0003 describes handling redispatch infeasibility:
  > *"Fixed commitment can make redispatch infeasible. Identify violated rows, feed a regional commitment requirement or valid feasibility cut to stage 1, and re-solve UC then redispatch. ASSUMPTION: limit to three repair iterations per window, then mark failed/partial with diagnostics; never drop the proxy or present the unsolved schedule as valid."* (lines 42–45)

  This heuristic decomposition is unstable:
  1. **Absence of Feasibility Cut Derivation:** ADR 0003 fails to define the mathematical structure of a "valid feasibility cut" from a continuous LP to an intertemporal MILP with generator minimum up/down times, ramp limits, and startup curves.
  2. **Oscillation and Non-Convergence:** In heavily constrained networks (e.g. Scottish B6 boundary under storm conditions), starting an uncommitted peaker or CCGT in the south to relieve a constraint may violate intertemporal down-times in subsequent intervals or create downstream congestion elsewhere. Heuristic cuts frequently oscillate without converging in 3 iterations.
  3. **High Annual Failure Rate:** With 365 daily advance windows per year, even a 2% window failure rate means 7 to 8 windows per year will fail to resolve, marking the entire annual backtest run as "failed/partial" under FR-11.AC2.

- **Recommendation:**
  Formulate Stage 2 with explicit, high-penalty slack variables (unserved energy and generation curtailment) on all branch and proxy constraints so that Stage 2 is **guaranteed to be feasible**. Use the slack duals to generate rigorous Benders-style integer cuts, and define fallback fast-start unit commitment logic rather than terminating after 3 iterations.

---

### ADV4-R2-09: Non-Linear SNSP and RoCoF Frequency Stability Constraints Creep from Phase 4 into Phase 1
- **Severity:** MEDIUM
- **Location:** `docs/requirements/04-requirements.md:227-236` (FR-21.AC1), `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md:31, 58-62`
- **Attack / Hidden Scope Creep:**
  `FR-21.AC1` mandates:
  > *"Sourced proxies explicitly include NESO Scottish boundary transfer limits (B6 and successors), regional minimum-generation/inertia rules, reactive power/voltage boundary limits where published as active-power constraints, and Ireland's dated System Non-Synchronous Penetration (SNSP) limits and Rate of Change of Frequency (RoCoF) constraints."*

  1. **Contradicts Phased Roadmap (D-44):** Decision `D-44` explicitly reserves dynamic stability for **Phase 4**:
     > *"Phase order: P1 current transmission (MVP) → P2 distribution first ... → P3 zonal pricing ... → P4 dynamic stability, SaaS hardening."*
  2. **Non-Linear Complexity:** SNSP (System Non-Synchronous Penetration) is a non-linear rational metric:
     $$\text{SNSP} = \frac{P_{\text{wind}} + P_{\text{solar}} + P_{\text{HVDC,import}}}{P_{\text{demand}} + P_{\text{HVDC,export}}}$$
     Enforcing SNSP and RoCoF limits in Stage 1 UC requires non-convex constraints or gross linear approximations that creep into Phase 4 frequency stability modeling.
  3. **Decision D-55 Scope:** Decision `D-55` only authorized published boundary transfer limits (such as NESO ETYS boundary transfer capabilities) to proxy AC power flow; it did not authorize pulling Phase 4 frequency dynamics and RoCoF constraints into Phase 1.

- **Recommendation:**
  Remove RoCoF and dynamic frequency limits from FR-21. Limit Irish proxy constraints in P1 to published transmission flowgate and export boundary MW limits, deferring non-linear SNSP and RoCoF constraints to Phase 4.

---

### ADV4-R2-10: GSP Embedded Wind and Solar Disaggregation Creeps into Phase 2 DER Scope
- **Severity:** MEDIUM
- **Location:** `docs/requirements/04-requirements.md:38` (FR-3.AC3)
- **Attack / Hidden Scope Creep:**
  `FR-3.AC3` mandates:
  > *"GSP demand identifies underlying demand and distribution-connected wind/solar, represented as weather-driven negative demand at the transmission interface. Fixtures reconcile this decomposition with net outturn and reject double subtraction; uncertain embedded estimates are labelled."*

  Decision `D-24` and `D-44` explicitly state that distribution networks and distributed energy resources (DER: rooftop solar, embedded generation, batteries, heat pumps) are brought forward to **Phase 2**.
  Mandating that Phase 1 ingest must disaggregate every Grid Supply Point (GSP) into true gross demand plus weather-driven embedded solar and wind time-series requires gathering distribution-level asset registries and building thousands of sub-grid weather transformation models. In Phase 1, GSP net demand time-series are directly available from NESO/Elexon outturn. Disaggregating underlying gross demand from embedded renewables is Phase 2 DER scope.

- **Recommendation:**
  Revise FR-3.AC3 to allow Phase 1 to consume net metered GSP demand directly, with weather-driven embedded renewable disaggregation optional or deferred to Phase 2.

---

### ADV4-R2-11: Unbounded Mutation Testing Zero-Survivor Requirement Exceeds D-28 Merge Gates
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0001-repo-layout-and-ci-gates.md:136-140`
- **Attack / Hidden Scope Creep:**
  ADR 0001 states:
  > *"ASSUMPTION (Q-N-11): Run cargo-mutants on changed commitment/redispatch/network/pricing/optimisation logic; zero unexplained surviving or timed-out non-equivalent mutants. Equivalent mutants require individually recorded reviewer justification; expand nightly scope instead of hiding changed-core survivors in a percentage budget."*

  This requirement introduces unmanageable friction that was never approved by the owner:
  1. `D-28` defines the required merge gates: `cargo fmt --check`, `cargo clippy -D warnings`, tests, `cargo-deny`, numerical results matching reference cases, and benchmark regression checks. It does **not** mandate `cargo-mutants`.
  2. Running mutation testing on numerical optimization engines (which compile dozens of crates and invoke heavy solvers across hundreds of test permutations) takes hours or days.
  3. Enforcing a "zero unexplained surviving or timed-out mutants" gate on every core PR will completely paralyze autonomous development.

- **Recommendation:**
  Demote `cargo-mutants` from a blocking PR merge gate to a non-blocking weekly or nightly diagnostic sweep, consistent with D-28.

---

### ADV4-R2-12: Fixing Repaired Out-of-Merit Network Commitment in Stage 3 Pricing LP Distorts National Marginal Prices
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md:33, 47-48`, `docs/requirements/04-requirements.md:122-123` (FR-10.AC2)
- **Attack / Economic Flaw:**
  ADR 0003 line 33 defines Stage 3 as a continuous national pricing LP fixing the Stage 1 UC decisions.
  However, line 47 states:
  > *"Save the revised commitment used for final schedules and re-run its pricing LP; distinguish original and repaired stages."*

  This creates an economic contradiction in price formation:
  1. If Stage 2 redispatch required turning on an expensive, out-of-merit peaker in an import-constrained local pocket (e.g. London or Devon) to satisfy transmission security, that unit was committed **solely for network reasons**.
  2. If Stage 3 fixes the *repaired* commitment in a copper-plate LP without network constraints, that out-of-merit unit is now treated as committed generation in the national merit order.
  3. Because the copper-plate LP has no network constraints, it will generate from this unit and set the national marginal price $\lambda$ to the peaker's cost, or if forced to minimum generation, its lower bound dual will distort energy prices for the entire nation.
  4. Conversely, if Stage 3 fixes the *original* Stage 1 commitment, the pricing LP does not reflect the units actually generating electricity!

- **Recommendation:**
  Clarify in ADR 0003 that Stage 3 national pricing LP strictly fixes the Stage 1 national commitment, and that network-driven commitment additions in Stage 2 are accounted for exclusively as out-of-merit redispatch costs, not energy price setters.

---

## Conclusion & Recommendation for PR #4 Revision

PR #4 at head `1698c4be` represents a substantial improvement over rev 1, notably in its four-stage solver decomposition and structured questionnaire aliases. However, it cannot be baselined in its current form due to:
1. Critical failures and false positives in `.githooks/commit-msg` that strip valid engineering text and crash commit workflows.
2. Direct contradictions with owner roadmap decisions (D-44, D-65).
3. Blocking bootstrap acceptance criteria (NFR-12) that cannot be executed in M0.
4. Gameable accuracy thresholds (skill > 0, vacuous unserved energy).
5. Formulation vulnerabilities in continuous lossy interconnector dispatch and bounded repair loops.

The PR author must revise these specific criteria and regex definitions before PR #4 can be recommended for baseline adoption.
