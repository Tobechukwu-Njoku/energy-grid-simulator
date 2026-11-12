# Independent design review — P1 after D-50

_Reviewer: GPT 6.1 Sol · 2026-11-12 · Status: independent review and proposals, not owner decisions._

## Scope and conclusion

Read AGENTS.md and every existing file under `docs/`, including the earlier Fable
review. This review starts from D-1–D-50, not the earlier 23-decision snapshot. A
final sync incorporated main's D-51–D-53 process updates without editing those
decisions: verification roles, no attribution in deliverable metadata, and additional
adversarial review/triage. The
remote, role split, phase order and merge-gate decisions now exist; those earlier
findings are not repeated as unresolved issues. Primary technical sources were
checked on 2026-11-12; sources below distinguish documented capabilities from
unmeasured performance. No application/optimisation spike was run: D-31 permits
only the parallel data spike before baseline acceptance.

**Conclusion:** D-44 resolves the end-state phase-order contradiction, but does not
make P1 small. A full-network GB+Ireland year of UC, redispatch and credible prices
is a substantial production-cost/market-model project. It is conditionally achievable
as staged, validated rolling-horizon work, not currently a demonstrated workstation
capability or a guaranteed six-month delivery. D-34 is “6+ months; quality first”,
not a six-month deadline. Baseline acceptance needs explicit fidelity, data and
accuracy dispositions; performance feasibility remains to be measured after code is
permitted. The companion requirements draft and Proposed ADRs do not reopen that gate.

## Findings

1. **HIGH — D-15, D-26, D-44; `06-roadmap.md` P1/P3; R-11: phase order is resolved, market scope is not.**
   D-15 still says “full replica”; D-26 requires UC + BM redispatch in P1; the roadmap
   defers “full I-SEM detail” to P3. Neither D-26 nor D-44 defines which GB auctions,
   bid/offer rules, imbalance pricing, Irish balancing rules, reserve procurement or
   non-convex cost recovery may be approximated. R-11's “constraint redispatch, not
   full bid/offer replay” is a proposed mitigation, not an owner decision. Calling
   Ireland's balancing stage “GB BM” or equating GB system imbalance price with an
   observed day-ahead price creates a false comparison: Elexon explicitly distinguishes
   imbalance prices from Market Index wholesale data [S8, S9].
   **Suggested change:** agree a P1 rule/product inventory and separate national-market
   scheduling, network redispatch and pricing outputs. A fixed-commitment pricing LP
   is a transparent proxy, not a full auction/settlement replica; MIP dual prices do
   not exist in the ordinary LP sense [S5]. Owner item OI-2 and FR-7/FR-10 block an
   unqualified market-fidelity claim. This is the remaining contradiction, not a
   recommendation to move D-26 back out of P1.

2. **HIGH — D-18, D-21, D-26, D-32, D-34, D-48; R-9/R-11: the combined MVP has no established tractability envelope.**
   The earlier review identified modelling maturity; adding integer decisions and
   a full transmission network makes annual model size and solve quality the next
   issue. **ASSUMPTION:** for sizing illustration only, use 500 committable units,
   3,000 buses and 4,000 AC branches, not an asserted GB+IE inventory. A 365-day,
   half-hour year has 17,520 intervals. Three commitment/start/stop binaries per unit
   give 26.28 million binary variables; explicit bus angles, branch flows and unit
   dispatch add 52.56, 70.08 and 8.76 million continuous variables respectively:
   **157.68 million variables before storage, reserves, redispatch or constraints**.
   A 48-hour window reduces that illustrative total to 864,000, still not proof of
   fast MILP solves. Algebraic elimination can reduce these counts; a dense network
   sensitivity formulation can instead inflate nonzeros. MILP search memory/time
   cannot be predicted from those variable counts alone.
   **Suggested change:** benchmark representative small/medium/full windows on the
   actual Ryzen, with build/solve/export time, RSS, bounds/gaps and terminal-state
   policy. Use rolling horizons only as an explicit approximation; test longer
   look-ahead bias and do not report a globally optimal annual objective. Proposed
   48-hour/24-hour windows, 0.1% gap and 48-hour annual runtime are OI-5 proposals,
   not measured facts. No new optimisation spike is authorised by this review.

3. **HIGH — D-30/D-31/D-32, R-12: a reduced-network contingency must not become an accidental owner decision.**
   Full inventory coverage is different from having enough geometry to draw a map:
   topology, electrical parameters, ratings, unit-to-bus and demand mappings and
   data rights all affect the UC/redispatch result. A reduced model may validate
   the algorithm but cannot demonstrate all-node D-32 coverage or node-specific
   constraint accuracy. “Present first” does not waive the full-transmission decision.
   **Suggested change:** require asset-level coverage and parameter provenance in
   the spike handoff. Apply conservation, solver, price and validation contracts to
   either representation; tag all results with fidelity and aggregation lineage.
   Treat reduced-fixture timings only as reduced-fixture evidence. A reduced P1
   release needs an explicit new owner decision, not an ADR implication. FR-2 and
   OI-3 preserve the full-network obligation while ADRs 0002/0004 await the spike.

4. **HIGH — D-23/D-25/D-33/D-40/D-44; Q-L-4, R-8: historical reconstruction is not a forecasting track record.**
   This extends, rather than repeats, Fable's request for a backtest. Using today's
   fleet with an old weather year tests weather sensitivity; it does not reproduce
   the old market. Using realised demand, weather, outages or later-revised data
   without an as-of information cutoff tests perfect-information reconstruction,
   not day-ahead/intraday forecasting. A single price “accuracy” number can also
   conceal wrong currency/product/interval alignment, missing data and rare spikes.
   **Suggested change:** freeze calibration/test boundaries, asset/data vintages and
   native-currency products for both markets. Publish MAE/RMSE/bias, tail/negative-price
   errors, previous-week baseline performance, generation-share/energy errors and
   coverage. Match GB BM cost definitions before scoring them. Proposed numeric
   gates are explicitly OI-4, not owner-approved success criteria; failure cannot
   be hidden by calling reference-tool agreement “accuracy”. Keep P2 live forecasting
   out of P1 while preventing unsupported marketing claims (FR-12, NFR-11).

5. **HIGH — D-16/D-20, R-9; `stack-candidates.md`: Rust-first is realistic; “pure Rust solver stack” is not the requirement.**
   `good_lp` already provides Rust LP/MILP modelling with multiple solver features;
   `highs` provides row/column model construction and integer columns [S1, S2].
   Therefore “Rust has no modelling layer” is too absolute. What remains unproven
   is these paths' full-network UC throughput, memory behaviour and capability
   consistency. HiGHS itself is native code; Gurobi/Xpress/cuOpt also have native
   APIs. D-16 and D-20 can coexist through Rust modelling and FFI; rewriting their
   solvers in Rust would contradict mature-component reuse rather than fulfil it.
   **Suggested change:** evaluate `good_lp`, direct `highs`, `lp-modeler` and a small
   sparse intermediate representation against the same generated UC model. Reject
   a home-grown JuMP/linopy replacement; build only domain-specific sparse assembly
   if measured wins justify it. Preserve integrality, status, bounds/gaps and
   capability differences across adapters. ADR 0003 sets a post-baseline benchmark
   decision point instead of declaring a winner without measurements.

6. **HIGH — D-2/D-42/D-48, R-14: distinguish GPU compatibility, memory, arithmetic and actual speed.**
   The benchmark baseline is the owned 12-core/24-thread Ryzen 5900X with 128 GB RAM,
   not the planned CPU upgrade or planned 16 GB GPU. The present cards have separate
   11 GB and 8 GB VRAM; they do not constitute one 19 GB allocation. NVIDIA lists
   Pascal below cuOpt's documented compute-capability minimum; the Quadro RTX 4000
   and planned RTX 5060 Ti belong to later supported architecture classes, subject
   to the precise driver/toolkit/package combination [S3, S4]. CUDA 13's removal is
   specifically of offline compilation/library support for older architectures,
   not a claim that every pre-existing Pascal binary stops running [S10].
   R-14 is directionally prudent but “low FP64 therefore GPU solvers will not win”
   is not a benchmark: cuOpt includes mixed-precision components and MILP methods
   [S3, S11]. CPU MILP branching, presolve, memory and PCIe transfers can dominate;
   neither a guaranteed GPU win nor a categorical loss follows from FLOPS alone.
   **Suggested change:** keep CPU feasibility primary; record device/software/VRAM,
   time all transfers and the full solve, verify identical tolerances/gaps, and
   compare only actually available hardware. Never let optional GPU support turn
   D-43's required GPU gate into a green skipped job (OI-7, NFR-5/NFR-7).

7. **MED — D-21/D-26/D-38: time resolution changes the optimisation, not just chart sampling.**
   Fable already noted synthetic minute inputs; the unaddressed issue is dimensional
   correctness of minimum up/down times, ramp limits, energy/cost weights, native
   settlement products and daily FX across DST/leap years. Upsampling a half-hour
   result is not a one-minute UC solve, and a one-minute national-price output does
   not automatically represent an observed one-minute traded product.
   **Suggested change:** test minute-based durations and MW-to-MWh conversion at
   30/15/5/1 minute resolutions with UTC storage/local settlement mappings. State
   which comparisons aggregate to the observed product. Do not silently cut D-21
   to 30 minutes or imply an annual one-minute runtime target (FR-3, FR-5, OI-5).

8. **MED — D-28; reference validation: identical inputs need identical equations, and equal schedules are not required.**
   PyPSA is a valid independent implementation reference, including UC and rolling
   horizons, but differences in terminal conditions, cost weights, losses and
   network/security formulation can invalidate a direct comparison [S6]. An AC
   pglib-opf objective is not a correct oracle for a linear DC approximation. Two
   MILP solvers can return different equally good schedules; a reused solver also
   means PyPSA agreement alone is not independent evidence of solver correctness.
   **Suggested change:** use hand-solvable invariant cases plus a pinned PyPSA
   harness with equal model semantics, objective tolerances, independently checked
   residuals and bound/gap comparisons. Use pglib only for its applicable declared
   formulation; do not import AC functionality merely to make a gate meaningful.
   Keep market backtesting as a separate gate (FR-13, NFR-2, ADR 0001).

9. **HIGH — D-22/D-28/D-43/D-49: a CI pass does not enforce cross-provider review by itself.**
   Models may share the same GitHub account, so a different username or a label is
   neither necessary nor sufficient to prove a different provider. Status checks
   also need protected required contexts and current-head binding. Self-hosted
   runners can be persistently compromised by PR code even in private repositories
   [S7]. The new failure mode is a green badge generated by the same author whose
   code it is supposedly gating.
   **Suggested change:** use a separately trusted attestation/check issuer, record
   author and reviewer providers plus reviewed SHA, invalidate after new commits,
   and protect runner/workflow changes. Use isolated self-hosted GPU execution
   after exact-SHA authorisation; never expose secrets through privileged
   `pull_request_target` execution of PR content. Missing/inconclusive checks block.
   Current STATUS reports private-plan branch protection unavailable: obtain plan
   support or an approved equivalent before claiming enforced merging [S12]. D-51
   reserves Claude for code generation while team.md retains Claude approval for
   GPT-authored PRs; clarify that review-role conflict rather than invent an exception.
   D-53's Gemini adversarial review and verified triage are additional evidence, not
   a replacement for independent approval. ADR 0001/NFR-3/NFR-12 cover the proposal.

10. **MED — D-19/D-35/D-36; Q-D-5: TOML alone does not make reruns reproducible.**
    Parallel MILP search, native solver versions, time limits and input revisions
    can change incumbents. D-36 deletes unpinned full-resolution outputs after
    90 days, but says nothing about preserving licensed input snapshots. A saved
    scenario whose URL now serves different data is not a reproducible experiment.
    **Suggested change:** distinguish semantic scenario hash, execution identity,
    numerical reproducibility and bitwise determinism. Preserve manifests and
    archive legally retainable inputs for guaranteed reruns; explicitly mark expired
    or inaccessible runs. Owner items OI-6/OI-9 are necessary before promising exact
    reproduction, not a reason to silently override the retention decision.

11. **MED — D-17/D-30; R-3 and `prior-art.md`: software and dataset rights are different closure problems.**
    D-30's no-share-alike decision concerns data. It does not automatically determine
    the treatment of every copyleft software/service licence. Conversely a permissive
    Rust wrapper or PyPSA code licence does not establish the licence of native
    binaries or generated/imported network data. Fable covered licence flags; the
    new implementation gap is that `cargo-deny` cannot inventory the entire native,
    TypeScript, container, service and dataset supply chain.
    **Suggested change:** require separate inventories and explicit reviewed approval
    dispositions for all those layers. Treat the existing categorical R-3 statement
    as requiring a deployment-specific licence review, not as a legal conclusion.
    Do not substitute Valkey for the D-3 Redis decision without owner approval or
    copy public model datasets into production solely because the tool is permissive
    (FR-19, NFR-8, OI-8). This review is not a legal opinion.

12. **MED — D-8/D-44/D-47; `STATUS.md` “non-blocking”: baseline needs a narrower interaction contract.**
    A dense map/dashboard over asynchronous jobs is achievable separately from
    in-place edits to a live multi-day UC solve. D-8 says all interactions, while
    P2 has batch sweeps and live forecasting. Without a phase-specific interpretation,
    engineers can either drop an owner requirement or build interactive reoptimisation
    before the P1 validation work. `STATUS.md` also currently treats remaining open
    items as non-blocking, but accuracy, scope and data disposition are baseline/release
    blockers in this draft.
    **Suggested change:** obtain OI-10 approval for P1 submit/monitor/cancel/inspect,
    simple deterministic CLI batches and explicit later live editing. Approve measured
    desktop/UI latency criteria rather than inventing a product decision from D-47's
    visual style. Coordinator should update STATUS and questionnaire after owner
    dispositions; this task intentionally leaves them untouched.

## Proposed evidence sequence

- **Before baseline:** receive the D-31 spike, resolve fidelity/price products and
  success metrics, and accept/revise the requirements and ADRs 0001–0003. ADR 0002
  still waits on data; this review does not write it or ADR 0004.
- **After code is allowed:** M0 CI/protection; exact small mathematical cases;
  modelling/solver comparison; representative full-network window benchmark;
  storage/API/UI; full-year reconstruction and held-out price/mix report. Each stage
  has separate feasibility, historical-accuracy and performance evidence.
- **Release stop conditions:** unresolved D-32 feasibility, no acceptable open-source
  CPU path, unsupported claims of market fidelity, failed agreed accuracy/performance
  criteria, or an unenforced D-28 gate. Escalate with evidence; do not silently reduce
  scope, switch to Python runtime, or assume better hardware.

## Open items for the owner

The canonical detailed proposals are OI-1–OI-12 in `../04-requirements.md`.
All assumptions here remain proposals; the coordinator should assign questionnaire IDs.

| Item | Assumption/question requiring disposition |
|---|---|
| OI-1 | **ASSUMPTION:** linear DC/N-0 P1 with explicit loss, reserve, storage and hydro policies; approve physical scope and asset vintages. |
| OI-2 | **ASSUMPTION:** cost-based UC, labelled redispatch/pricing proxies; approve GB/I-SEM product/rule boundaries versus D-15. |
| OI-3 | **ASSUMPTION:** reduced models are test fixtures only unless D-32 is amended; decide the spike's missing-data disposition. |
| OI-4 | **ASSUMPTION:** separate held-out year, ≥99% matched coverage and draft native-currency price/mix thresholds; approve year, products, accuracy and BM-cost criteria. |
| OI-5 | **ASSUMPTION:** 48-hour/24-hour rolling windows, ≤0.1% gap and draft Ryzen runtime/RAM targets; the 500-unit/3,000-bus/4,000-branch calculation is illustrative, not a network-size decision. Confirm measured workload, terminal/absolute-gap policy and minute-input treatment. |
| OI-6 | **ASSUMPTION:** numerical reproducibility and stated residual/objective/price tolerances, not bitwise MILP identity. |
| OI-7 | **ASSUMPTION:** CPU HiGHS is primary; optional adapters require verified actual hardware/software/licences, with no assumed GPU win. |
| OI-8 | **ASSUMPTION:** unknown licence terms block admission; approve software/service/native/data evidence and allow-lists. |
| OI-9 | **ASSUMPTION:** rerun guarantees apply only with legally archived immutable inputs; approve archival alongside D-36 result retention. |
| OI-10 | **ASSUMPTION:** P1 asynchronous analyst UI and simple CLI batches, not live in-place reoptimisation; approve comparison/latency targets. |
| OI-11 | **ASSUMPTION:** protected current-head checks plus a trusted provider attestation; resolve the current plan limitation, D-51/cross-provider role conflict, D-53 triage, bootstrap and no-regression protocol. |
| OI-12 | **ASSUMPTION:** private trusted P1 operation until an explicit authentication/exposure policy exists. |

## Primary sources checked

These substantiate technical capabilities, not the unmeasured project targets.

- [S1 — good_lp API and solver-feature matrix](https://docs.rs/good_lp/latest/good_lp/).
- [S2 — highs Rust API, row/column construction and integer columns](https://docs.rs/highs/latest/highs/).
- [S3 — NVIDIA cuOpt system requirements](https://docs.nvidia.com/cuopt/user-guide/latest/system-requirements.html).
- [S4a — NVIDIA current CUDA GPU table](https://developer.nvidia.com/cuda/gpus); [S4b — legacy GPU table](https://developer.nvidia.com/cuda/legacy-gpus).
- [S5 — Gurobi: duals for mixed-integer models and fixed-model limitations](https://support.gurobi.com/hc/en-us/articles/360034305272-How-do-I-retrieve-the-dual-Pi-values-for-a-MIP-problem).
- [S6 — PyPSA unit commitment and rolling horizons](https://docs.pypsa.org/latest/user-guide/optimization/unit-commitment/).
- [S7 — GitHub Actions security hardening, including private/self-hosted runners](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions).
- [S8 — Elexon imbalance pricing](https://www.elexon.co.uk/bsc/settlement/imbalance-pricing/).
- [S9 — Elexon Market Index Data](https://www.elexon.co.uk/bsc/glossary/market-index-data/).
- [S10 — NVIDIA CUDA 13.0 release notes, removed offline architecture support](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-toolkit-release-notes/index.html).
- [S11 — NVIDIA cuOpt MIP C API](https://docs.nvidia.com/cuopt/user-guide/latest/cuopt-c/mip/mip-c-api.html) and [precision modes](https://docs.nvidia.com/cuopt/user-guide/latest/convex-settings.html).
- [S12 — GitHub rulesets and private-repository plan availability](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets).

## Limits

No full-network data has been certified, no solver/backend was installed or timed,
no GPU or remote runner was inspected, and no licence entitlement or legal compliance
was certified. The parallel data-spike report was not available in this checkout.
Review findings are architectural inferences from the decisions and the cited primary
documentation. All changes in this PR are confined to the four requested documents;
STATUS and the decisions log are reserved for the coordinator/owner.
