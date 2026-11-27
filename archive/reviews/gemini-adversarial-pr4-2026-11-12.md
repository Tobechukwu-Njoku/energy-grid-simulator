# Adversarial Review — PR #4 Baseline Requirements & Architecture

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-12  
**Target:** PR #4 (`docs/requirements/04-requirements.md`, `docs/architecture/adr/0001-repo-layout-and-ci-gates.md`, `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md`)  
**Scope:** Verification against `AGENTS.md`, `docs/process/team.md`, `docs/requirements/03-decisions-log.md` (D-1–D-55), and `docs/requirements/reviews/triage-gemini-adversarial-2026-11-12.md`.  

---

## Executive Summary

This adversarial review evaluates PR #4, which proposes the Phase 1 (MVP) requirements baseline, ADR 0001 (repository layout and CI gates), and ADR 0003 (optimization modeling layer and solver trait). The earlier adversarial review (ADV-1 through ADV-12) identified high-level structural and licensing risks, which were triaged in `triage-gemini-adversarial-2026-11-12.md` and informed D-51 through D-55. This review does not repeat those triaged findings.

Instead, this review targets the specific technical proposals, equations, solver abstractions, CI enforcement mechanics, and acceptance criteria introduced in PR #4. Twelve concrete findings (ADV4-1 through ADV4-12) are presented across five attack surfaces:

1. **Gameable Acceptance Criteria:** Acceptance tests structured such that an implementation can pass trivially while being practically useless (e.g. constant-input 1-minute downscaling, definition-incompatible BM data escape hatches).
2. **Missing Numbers, Metrics, and Tolerances:** Acceptance criteria lacking required tolerances or error metrics (e.g. unserved energy / curtailment omission in market backtests, arbitrary fixed-LP price proxies).
3. **Flawed CI Gates in ADR 0001:** Gates that agents can bypass or that will cause immediate, 100% fail-closed merge deadlocks on day one prior to external infrastructure provisioning.
4. **Technical Flaws in ADR 0003:** Mathematical and computational breakdown in the solver trait (e.g. relative MIP gap division by near-zero objectives, lack of PTDF / constraint screening for a 4,000-branch MILP, FFI cancellation deadlocks, and rolling-horizon terminal storage depletion).
5. **Direct Decision Contradictions:** Explicit conflicts with owner decisions (D-55 proxy constraints omitted from FR-8, D-38 interconnector loss conservation and discrete midnight FX arbitrage).

---

## Findings Matrix

| ID | Severity | Target File & Section | Category | Summary Title |
|---|---|---|---|---|
| **ADV4-1** | HIGH | `04-requirements.md` (FR-8) | Contradicts D-55 | Omission of D-55 Proxy Voltage and Stability Boundary Constraints |
| **ADV4-2** | HIGH | `04-requirements.md` (FR-12 & OI-4) | Gameable Criteria / Tolerances | BM Backtest Escape Hatch and Missing Unserved Energy / Curtailment Tolerances |
| **ADV4-3** | HIGH | `adr/0003` (Solver Trait & Lifecycle) | Technical Flaw / Numerical | Relative MIP Gap Breakdown on Zero/Near-Zero Objectives and Negative Prices |
| **ADV4-4** | HIGH | `adr/0001` (CI Graph & Enforcement) | CI Gate Day-One Failure | Fail-Closed CI Pipeline Causes Immediate Day-One Merge Deadlock |
| **ADV4-5** | HIGH | `adr/0003` (Sparse Model & NFR-5) | Technical Flaw / Hardware | HiGHS MILP Scaling Collapse on Unscreened 4,000-Line Transmission Grid |
| **ADV4-6** | HIGH | `adr/0003` (Solver Trait & Lifecycle) | Technical Flaw / FFI Safety | Asynchronous Solver Cancellation Impasse and Unsound Cross-Thread Sharing |
| **ADV4-7** | HIGH | `04-requirements.md` (FR-6, FR-9, FR-11) | Technical Flaw / Rolling Horizon | Myopic Terminal Storage Depletion and Intertemporal Carryover Infeasibility |
| **ADV4-8** | HIGH | `04-requirements.md` (FR-1, FR-8, FR-10) | Technical Flaw / Contradicts D-38 | Lossless DC Power Balance Violation by Interconnector Losses and Midnight FX Jumps |
| **ADV4-9** | MED | `04-requirements.md` (FR-5) | Gameable Criteria | Trivialization of 1-Minute Resolution via Constant-Input Acceptance Tests |
| **ADV4-10** | MED | `04-requirements.md` (NFR-4) | Missing Metric / CI Flaw | Zero-Tolerance Benchmark Regression Creates Statistically Flaky CI |
| **ADV4-11** | MED | `04-requirements.md` (FR-10) | Missing Metric / Contradicts D-15 | Vacuous Pricing Acceptance Criteria for Non-Convex Fixed-Commitment Duals |
| **ADV4-12** | MED | `adr/0001` (CI Gates & Attribution) | CI Gate Bypass / Process | Superficial Watermark Linting and Role Assignment Bypass in PR Review |

---

## Detailed Findings

### ADV4-1: Omission of D-55 Proxy Voltage and Stability Boundary Constraints
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md` — FR-8 (Physical feasibility and shortage accounting) & OI-1
- **Attack:**  
  Decision D-55 explicitly mandates: *"MVP network physics: DC network plus proxy constraints — known voltage and stability limits enter as published constraint boundaries/limits and constraint costs (NESO publications). Full AC power flow stays in P2"*.  
  However, FR-8's normative acceptance criteria define network feasibility strictly as: *"Under the proposed linear DC, N-0 formulation (OI-1), every accepted interval satisfies nodal active-power balance and branch/interconnector limits within NFR-2... Outputs explicitly state that AC voltage, reactive power, N-1 security and dynamics are not certified by P1"*.  
  FR-8 completely omits proxy voltage and stability boundaries! In the real GB transmission system, the primary transmission bottlenecks—such as the B6 Scottish-English boundary, B4 (Harkers-Hutton/Stella West), and North-to-Midlands flow limits—are voltage- and transient-stability-limited to power transfers significantly below the aggregate physical thermal capacity of individual lines.  
  By omitting NESO published boundary limits from FR-8's acceptance criteria, an implementation can model individual line MW thermal ratings alone and ignore transfer boundaries entirely. Such an implementation will satisfy FR-8 on paper while simulating unconstrained, physically impossible bulk power transfers from Scottish wind farms to English demand centers, directly contradicting D-55 and generating invalid redispatch costs.
- **Evidence:** D-55 in `docs/requirements/03-decisions-log.md`; National Grid ESO (NESO) Electricity Ten Year Statement (ETYS) System Boundary Capabilities: https://www.nationalgrideso.com/research-and-publications/electricity-ten-year-statement-etys

---

### ADV4-2: BM Backtest Escape Hatch and Missing Unserved Energy / Curtailment Tolerances
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md` — FR-12 (Historical GB and I-SEM accuracy backtest) & OI-4
- **Attack:**  
  FR-12 provides an escape hatch that renders historical validation of the Balancing Mechanism completely toothless: *"Report GB constraint/BM action volumes and costs only against definition-compatible observations; otherwise disclose missing coverage"*.  
  Because D-26 requires Balancing Mechanism redispatch and costs in the MVP, but the simulator uses a proxy redispatch formulation rather than a tick-by-tick bid/offer stack replay (OI-2), an engineering agent whose redispatch model fails can simply assert that historical Elexon/NESO BM outturns are not "definition-compatible", check the "disclose missing coverage" box, and pass the FR-12 gate with zero validated BM accuracy!  
  Furthermore, OI-4 defines price gates (MAE ≤ 20 £/MWh, RMSE ≤ 40 £/MWh, MAE ≤ previous-week baseline) and generation mix gates (mean daily energy error ≤ 15%), but specifies **zero metric, tolerance, or ceiling on Unserved Energy (EUE) or Renewable Curtailment Error**. A broken dispatch solver could dump load (unserved energy) or curtail 25% of Scottish wind generation whenever transmission lines are congested, drastically skewing market outcomes, yet still pass the aggregate generation-share and price-error gates.
- **Evidence:** Elexon BM Reporting Guidance: https://www.elexon.co.uk/guidance-notes/bm-reporting-and-data/

---

### ADV4-3: Relative MIP Gap Breakdown on Zero/Near-Zero Objectives and Negative Prices
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md` — Sparse model and assembly contract; Solver trait and lifecycle; NFR-5 / OI-5
- **Attack:**  
  ADR 0003 defines the canonical relative MIP gap as:  
  $$\text{relative\_gap} = \frac{|\text{incumbent} - \text{bound}|}{\max(1, |\text{incumbent}|)}$$  
  and NFR-5 / OI-5 mandate a relative MIP gap threshold of $\le 0.1\%$.  
  This formula breaks down in electricity balancing and redispatch subproblems (FR-7). The redispatch stage minimizes *incremental redispatch cost* ($\sum \text{Bid} \cdot \Delta P^+ - \text{Offer} \cdot \Delta P^-$). During intervals with minimal transmission congestion, the incremental cost of redispatch is near zero (e.g. £0.50 across the network). If $\text{incumbent} = 0.50$ and $\text{best\_bound} = 0.00$, the formula computes:  
  $$\frac{|0.50 - 0.00|}{\max(1, 0.50)} = \frac{0.50}{1.0} = 0.50 = 50\% \text{ relative gap}$$  
  Even though the economic discrepancy is fifty pence (£0.50), the solver reports a 50% relative gap, failing the 0.1% threshold. The solver will be forced to exhaust the entire 300-second window timeout fruitlessly branching, causing an artificial time-limit failure.  
  Conversely, during negative-price regimes (common in GB when wind generation bids negatively to avoid forfeiting ROCs or CfD top-ups), total window costs can be negative. For a minimization problem with negative costs, dividing by $|\text{incumbent}|$ without reversing the bound comparison relationship generates inverted, negative, or undefined gap metrics.
- **Evidence:** Gurobi MIP Gap Definition: https://www.gurobi.com/documentation/current/refman/mipgap.html; HiGHS MIP Solver Option Reference: https://ergo-code.github.io/HiGHS/dev/options/definitions/

---

### ADV4-4: Fail-Closed CI Pipeline Causes Immediate Day-One Merge Deadlock
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0001-repo-layout-and-ci-gates.md` — GitHub Actions gate graph, Enforcing D-28, and Enforcement acceptance tests; NFR-3, NFR-7, NFR-12
- **Attack:**  
  ADR 0001 designs a fail-closed CI pipeline where the `merge-gates` aggregator treats any missing, skipped, cancelled, or inconclusive check as a fatal failure. Specifically:  
  1. `gpu-validation`: Requires execution on an *"Isolated self-hosted Linux NVIDIA runner (D-43)"* with a disposable VM and device passthrough; CPU fallback is strictly prohibited (*"A missing required GPU run blocks the relevant gate, not silently falling back to a CPU 'GPU pass'"*).  
  2. `cross-provider-review`: Requires a *"separately trusted GitHub App or equivalent protected attestor"* issuing cryptographically signed provider attestations.  
  3. `benchmark-regression`: Requires paired candidate runs against baseline artifacts stored on `protected main`.  
  None of this infrastructure exists today:  
  - The repository has no GitHub App provisioned.  
  - The owner's workstation (Ryzen 5900X) does not have an automated IOMMU VM device passthrough runner configured (and cuOpt does not support the installed Pascal GTX 1080 Ti card).  
  - `main` has no existing benchmark baseline artifacts.  
  Under ADR 0001's fail-closed rules, every single PR will fail the required `merge-gates` aggregator on day one. ADR 0001 includes no staged bootstrap or onboarding exception, resulting in an immediate and permanent CI merge deadlock for the autonomous agent workflow.
- **Evidence:** GitHub Actions Self-Hosted Runner Hardening: https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions; ADR 0001 "Enforcing D-28 rather than documenting it"

---

### ADV4-5: HiGHS MILP Scaling Collapse on Unscreened 4,000-Line Transmission Grid
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md` — Sparse model and assembly contract; NFR-5; OI-5
- **Attack:**  
  ADR 0003 proposes solving a 48-hour rolling window with ~864,000 variables and explicitly excludes constraint screening and network sensitivity structures from the solver abstraction (*"Do not introduce dense time-by-bus-by-branch tensors... Keep network data structures out of this ADR's solver abstraction"*).  
  For the D-32 full transmission grid (~3,000 buses, 4,000 lines), formulating branch flow equations explicitly across 96 half-hour intervals injects 384,000 continuous flow variables and 768,000 inequality constraints (forward and reverse thermal line ratings) directly into the monolithic MILP constraint matrix.  
  Feeding 768,000 inequality constraints into an open-source branch-and-cut solver (HiGHS) alongside hundreds of integer unit commitment variables will severely bloat the LP simplex tableau and branch-and-cut search tree. HiGHS does not possess industrial-grade parallel MIP scalability on consumer 12-core hardware.  
  Industrial power systems optimizers (e.g. PLEXOS, PROMOD) avoid this by utilizing dynamic constraint generation / active-set screening (adding branch constraints lazily only when line limits are violated in relaxed solves). Forbidding this capability within ADR 0003's solver boundary guarantees that HiGHS will exceed the 300-second window solve budget (NFR-5), leading to timeout failures across the 365-window annual horizon.
- **Evidence:** Stott, B., Jardim, J., & Alsaç, O. (2009). "DC Power Flow Revisited", IEEE Transactions on Power Systems: https://doi.org/10.1109/TPWRS.2009.2021235; HiGHS Issue Tracker: https://github.com/ERGO-Code/HiGHS/issues

---

### ADV4-6: Asynchronous Solver Cancellation Impasse and Unsound Cross-Thread Sharing
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md` — Solver trait and lifecycle (`cancel`, `release`)
- **Attack:**  
  ADR 0003 mandates that `cancel` must support cooperative interruption with an explicit interrupted state, while requiring: *"RAII-owned native resources and clear transfer/copy ownership; no unsafe cross-thread reuse of vendor handles"*.  
  This creates a fundamental technical impasse in Rust FFI. In native C solver libraries (HiGHS, Gurobi, Xpress), interrupting an active solve requires invoking a C function (such as `Highs_interrupt(void* highs)` or `GRBterminate(GRBmodel* model)`) from an external control/timer thread while the solver worker thread is blocked inside the execution call (`Highs_run`).  
  In Rust, invoking this C function from another thread requires passing a raw pointer across threads and calling an FFI function concurrently on an active handle. The `highs` Rust crate (`highs` / `highs-sys`), which ADR 0003 selects as the default CPU candidate, does not implement `Sync` on `HighsModel`, nor does it expose `Highs_interrupt`.  
  Because ADR 0003 forbids cross-thread handle sharing and discourages direct C-API shims, asynchronous cooperative cancellation cannot be implemented in safe Rust with the `highs` crate. Long-running solves cannot be cleanly interrupted upon timeout, violating the timeout and cancellation guarantees of FR-11 and FR-15.
- **Evidence:** `highs` Rust crate API documentation: https://docs.rs/highs/latest/highs/struct.HighsModel.html; HiGHS C API interface: https://github.com/ERGO-Code/HiGHS/blob/master/src/interfaces/highs_c_api.h

---

### ADV4-7: Myopic Terminal Storage Depletion and Intertemporal Carryover Infeasibility
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md` — FR-6 (Intertemporal unit commitment), FR-9 (Storage), FR-11 (Rolling execution); ADR 0003 Benchmark Step 5
- **Attack:**  
  In a rolling-horizon framework (48-hour lookahead, 24-hour advance), PR #4 fails to specify mechanisms for two critical intertemporal edge cases:  
  1. **Terminal Storage Depletion:** FR-9 specifies bounds and round-trip efficiencies for battery and pumped hydro storage, but omits any terminal state-of-charge (SoC) valuation or constraint. In any cost-minimizing solve, an LP/MIP solver will deplete 100% of stored energy by the end of the lookahead window (or during evening peak hours in the advance window), because post-horizon energy has zero economic value. This produces artificially depressed peak prices in the committed window while handing over an empty storage reservoir to the next window, destroying realistic multi-day storage cycling.  
  2. **Intertemporal Carryover Infeasibility:** When Window 1 commits a generator to start at hour 23 with a minimum up-time of 8 hours, Window 2 (starting at hour 25) must enforce $u(g, t) = 1$ for hours 25–30. If Window 2 experiences an unexpected demand drop or a surge in wind generation, the generator's minimum generation ($P_{\min}$) may exceed net demand or cause transmission overload. Because PR #4 provides no slack variables or penalty relaxations for carried-over unit commitment state, the solver will return a hard `infeasible` status, crashing the full-year simulation.
- **Evidence:** Marwali, M. K., & Shahidehpour, S. M. (2000). "Long-term/short-term coordination in price-based unit commitment", IEEE Transactions on Power Systems: https://doi.org/10.1109/59.852109

---

### ADV4-8: Lossless DC Power Balance Violation by Interconnector Losses and Midnight FX Jumps
- **Severity:** HIGH
- **Location:** `docs/requirements/04-requirements.md` — FR-1 (Geography/Interconnectors), FR-8 (Physical feasibility), FR-10 (Prices) vs D-38; OI-1
- **Attack:**  
  FR-1 and OI-1 specify a lossless linear DC network formulation for AC branches, but mandate explicit loss modeling for interconnectors.  
  In a standard DC power flow ($B \theta = P_{\text{inj}}$), mathematical solvability requires $\sum_i P_{\text{inj}, i} = 0$. When an interconnector operates with loss factor $\eta$, power withdrawn from the sending bus exceeds power injected at the receiving bus by $\eta F_{\text{ic}}$. In a DC network, this loss represents a net energy deficit. Unless an explicit distributed loss-allocation slack formulation is implemented, the system reference bus (slack bus) will automatically absorb the entire loss deficit. This injects fictitious energy at the reference bus, distorting voltage angles, branch power flows, and locational marginal prices across the entire transmission grid.  
  Furthermore, D-38 mandates daily FX rates between GBP and EUR. In a 48-hour solve spanning midnight, a discontinuous daily FX step change at 00:00 UTC incentivizes artificial cross-border energy arbitrage: the solver will export power at 23:30 and import it back at 00:30 simply to exploit the discrete currency rate step change, fabricating unphysical interconnector schedules.
- **Evidence:** D-38 in `docs/requirements/03-decisions-log.md`; Wood, A. J., Wollenberg, B. F., & Sheblé, G. B. (2013). *Power Generation, Operation, and Control*, 3rd ed. (Wiley), Chapter 6.

---

### ADV4-9: Trivialization of 1-Minute Resolution via Constant-Input Acceptance Tests
- **Severity:** MED
- **Location:** `docs/requirements/04-requirements.md` — FR-5 (Configurable time resolution) vs D-21; OI-5
- **Attack:**  
  D-21 mandates configurable time resolution down to 1 minute. FR-5 proposes validating 1-minute execution using:  
  *"Identical constant-input 24-hour cases at 30, 15, 5 and 1 minute conserve MWh, currency totals and minimum up/down durations within NFR-2 tolerances."*  
  Validating temporal resolution downscaling using **constant inputs** over 24 hours is a trivially gameable acceptance test. When demand and generation inputs are flat, time derivatives ($\Delta P / \Delta t$) are identically zero, generator ramp constraints are completely inactive, and integer start/stop transitions never occur.  
  A severely defective 1-minute implementation—one that fails to scale ramp limits from $\text{MW}/30\text{min}$ to $\text{MW}/\text{min}$, fails to scale minimum up/down time steps (e.g. converting an 8-hour duration from 16 steps to 480 steps), or fails under 48× more integer variables—will pass this constant-input test with 100% success. FR-5 completely fails to validate non-stationary dynamic performance at 1-minute resolution.
- **Evidence:** D-21 in `docs/requirements/03-decisions-log.md`; Elexon BSC Half-Hourly Settlement: https://www.elexon.co.uk/guidance-notes/half-hourly-settlement/

---

### ADV4-10: Zero-Tolerance Benchmark Regression Creates Statistically Flaky CI
- **Severity:** MED
- **Location:** `docs/requirements/04-requirements.md` — NFR-4 (No benchmark regression); ADR 0001 (Benchmark and GPU runner policy)
- **Attack:**  
  NFR-4 dictates that: *"Any detected slowdown or memory increase blocks; overlapping uncertainty triggers reruns or an inconclusive blocking outcome, not a pass... Statistical protocol and any noise equivalence band require OI-11 agreement; no fixed percentage regression allowance is assumed."*  
  On multi-core x86_64 Linux hardware (AMD Ryzen 5900X), background OS kernel scheduling, jemalloc memory fragmentation, timer jitter, and CPU frequency scaling introduce 1–3% execution noise across identical benchmark runs.  
  By mandating that *any* detected slowdown or memory increase blocks the PR while explicitly providing no noise equivalence band (such as a 3-sigma confidence interval or a 3% tolerance) and treating inconclusive results as failure, NFR-4 creates a mathematically guaranteed flaky CI gate. Normal benchmark variance will trigger false-positive regressions, halting routine PR merges.
- **Evidence:** Georges, A., Buytaert, D., & Eeckhout, L. (2007). "Statistically rigorous Java/native performance evaluation", ACM OOPSLA: https://doi.org/10.1145/1297027.1297033

---

### ADV4-11: Vacuous Pricing Acceptance Criteria for Non-Convex Fixed-Commitment Duals
- **Severity:** MED
- **Location:** `docs/requirements/04-requirements.md` — FR-10 (Prices, currencies and economic ledgers) vs D-15, D-26; OI-2
- **Attack:**  
  FR-10 states: *"The selected post-commitment pricing method passes a small price-setting fixture and clearly labels any missing non-convex cost recovery/uplift. Fixed-commitment LP prices are not described as MILP duals or exact auction replication."*  
  This acceptance criterion specifies no numeric tolerance, no economic criteria, and no pricing validation rule beyond "passes a small price-setting fixture".  
  In electricity economics with non-convex generator commitment (start-up costs, minimum generation limits), fixing integer variables in an LP produces shadow prices that frequently drop below marginal cost or collapse to zero when units are constrained on at $P_{\min}$. Furthermore, fixed-LP duals fail to provide make-whole payments (uplift) to cover start-up costs. Without defined economic properties (e.g. price boundedness, marginal generator cost alignment), an implementation can output arbitrary numbers from an LP dual solve, label them "proxy prices", and satisfy FR-10 on paper while providing useless pricing signals to traders and battery operators (D-41).
- **Evidence:** O'Neill, R. P., et al. (2005). "Efficient market-clearing prices in markets with non-convexities", European Journal of Operational Research: https://doi.org/10.1016/j.ejor.2003.12.011

---

### ADV4-12: Superficial Watermark Linting and Role Assignment Bypass in PR Review
- **Severity:** MED
- **Location:** `docs/architecture/adr/0001-repo-layout-and-ci-gates.md` — GitHub Actions gate graph; D-51, D-52, D-54
- **Attack:**  
  ADR 0001 introduces `attribution-policy` to enforce D-52 (no AI attribution) and `cross-provider-review` to enforce provider independence. Both checks have critical bypass flaws:  
  1. **Superficial Regex Linting:** The `attribution-policy` gate inspects commit messages and file headers for specific string patterns (e.g. `Co-Authored-By:` or `Generated with`). An agent can trivially bypass the check by omitting explicit trailers while committing unreviewed AI-hallucinated content. Conversely, valid commits discussing model assignments in review files risk false-positive rejections.  
  2. **Model Role Matrix Bypass:** D-51 limits Claude Opus strictly to code generation and GPT 6.1 Sol to non-code work, with D-54 allowing Claude to review GPT documents. However, ADR 0001's `cross-provider-review` gate checks only that $\text{author\_provider} \neq \text{reviewer\_provider}$. It does not enforce artifact-to-role mappings (e.g., verifying that Gemini is not reviewing code, or that Claude is not authoring architecture specs), allowing invalid model assignments to merge unnoticed.
- **Evidence:** D-51, D-52, D-54 in `docs/requirements/03-decisions-log.md`; ADR 0001 "Enforcing D-28 rather than documenting it"
