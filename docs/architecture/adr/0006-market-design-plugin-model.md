# ADR 0006 — Versioned market modules with explicit capabilities and fidelity

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-11, D-15–D-22, D-25–D-26, D-28, D-35, D-38, D-40–D-41, D-44, D-55, D-57, D-59, D-69.
- **Related draft requirements:** PR #4 FR-1, FR-4–FR-11, FR-12–FR-13, FR-18, FR-21; NFR-1–NFR-2, NFR-8–NFR-11.
- **Companions:** [data model](0002-network-and-scenario-data-model.md), [execution](0005-execution-topology.md), [topology/source snapshot](../topology.md).

## Context and authority

D-15 requires GB and I-SEM arrangements and alternative zonal/nodal designs in the
end state. D-44 orders present-day fidelity before P3 zonal/2030 scenarios. P1 uses
UC plus cost-based redispatch (D-26/D-59), with DC plus published security proxies
(D-55), not historical bid/offer acceptance or a complete settlement replica.

GB bilateral/exchange scheduling and I-SEM ex-ante/balancing arrangements are not
identical market engines with different currency labels. Equally, a zonal scenario
must not require rewriting physical assets, storage state or solver adapters.
ADRs 0001/0003 already place dated rules in market, equations in network, solver IR
in optimisation, and UC/redispatch/pricing in distinct modules.

Contracts/composition below are proposed engineering defaults under D-22.
D-69 (Q-O-5, answers Q-J-2 for market code) permits no customer-uploaded market code
before P4; only first-party compiled modules and typed parameters are supported. The
[companion questionnaire](../owner-questionnaire.md) records this product choice.
Market parameter values and historical effective dates require sourced evidence;
this ADR invents none. PR #4's Q-N-1/Q-N-2 pricing/proxy defaults remain proposals.
No application code, early zonal implementation or relaxation of D-15 is authorised.

## Proposed decision and plug-in boundary

Start with **trusted, compiled-in Rust market modules** behind a versioned domain
interface and registry. “Plug-in” means independently selectable/qualifiable rule
implementation, not arbitrary dynamic native library loading. A new module requires
a reviewed build/release. End users change typed versioned parameters in TOML;
they cannot upload code or arbitrary solver expressions into a shared runtime
before P4. P4 must explicitly decide isolation/extension policy before any such
capability; compiled trusted modules remain the default until then.

| Contract surface | Inputs / output and restrictions |
|---|---|
| Describe | Stable module ID, semantic contract version, implementation/build digest, effective rule releases, supported geography/products/resolution/stages and fidelity inventory |
| Validate/resolve | Typed scenario, rule release, calendar, network capabilities, costs/FX/boundary evidence; returns resolved plan or field/capability errors |
| Schedule intent | Products, market memberships, commitment eligibility/reserve requirements and typed domain constraints/cost terms for commitment builder |
| Redispatch intent | Fixed-schedule baseline, cost policy and applicable proxies; typed actions/constraints for redispatch builder, no silent commitment/security relaxation |
| Pricing intent | Market/zone boundaries, fixed-UC continuous pricing method, scarcity/degeneracy/non-convex-cost policy and required dual availability |
| State and outputs | Versioned intertemporal state fragment plus typed schedules, prices, actions and economic ledgers with validity/coverage/fidelity |

The engine fixes stage order, rolling boundaries, cancellation and result acceptance.
Market modules express economic policies through narrow typed builder interfaces;
network owns electrical equations/loss balances/security checks, optimisation owns
the immutable sparse IR, solver adapters own native sessions. A module cannot open
Valkey/Redis or PostgreSQL, fetch unpinned data, mutate global topology, invoke a solver or
publish results itself. Inject immutable data, clock/cut and seeded randomness
explicitly; no ambient wall clock/network reads in rule evaluation.

Each contributed term/row has a rule/provenance ID, units, stage and applicability;
the builder validates references, integrality, supported capabilities and economic
accounting before emitting IR. Typed policies cover common products and parameters;
a genuinely new formulation may extend the versioned interface and requires new
qualification. Do not promise every market can fit an unchanged trait forever.
Reuse mature solvers/builders under ADR 0003 rather than implementing a rule-language
compiler or symbolic optimisation framework.

## GB, I-SEM and the P1 fidelity inventory

Create separate `gb` and `isem` registry identities with dated **rule releases**;
do not assign an assumed historic ruleset just from the engine's current version.
The resolved market plan composes both plus interconnector/boundary coordination.

| Feature | GB P1 interpretation | I-SEM P1 interpretation | End-state obligation / limitation |
|---|---|---|---|
| Ex-ante schedule | National copper-plate UC scheduling proxy | All-island copper-plate UC, DAM-like scheduling proxy | Actual GB self/bilateral/exchange schedules and I-SEM auction/trading detail require later explicit fidelity work |
| Network actions | Cost-based BM redispatch proxy | Cost-based balancing/constraint redispatch proxy | D-59 fixes MVP cost basis; no historical bid/offer replay claim |
| National price | Fixed-UC national LP marginal-price proxy, GBP/MWh | Fixed-UC national LP proxy, EUR/MWh | Keep separate from imbalance settlement and from observed exchange products |
| Reserves/security | Sourced aggregate reserve and commitment/flowgate proxies | Sourced reserves, SNSP/minimum-unit and applicable proxies | Product-level procurement, full AC and dynamics remain outside P1 |
| Observation reference | Free GB Market Index/imbalance series, each labelled with its actual definition | Free published I-SEM DAM/balancing series with matching definitions and rights | An observed reference is not evidence of full market replication; unavailable compatible series stays unvalidated |

Every feature entry is `implemented`, `proxy`, `deferred` or `unsupported`, with
effective dates, source evidence, reason and validation coverage. P1 module names
and exported metadata disclose proxy fidelity; a module's supported-feature list
cannot advertise the D-15 end state prematurely. Maintain the backlog toward that
end state without silently reclassifying required future features as exclusions.

Use the pre-pass → national UC → fixed-commitment network redispatch → national
fixed-UC pricing pipeline for P1. Commitment infeasibility feeds the bounded UC↔
redispatch repair loop. **Non-convergence is reported and invokes a fallback; reaching
the repair bound does not by itself fail the whole run.** This agrees with the
merged PR #4 ADR 0003 recovery contract; fallback failure remains failed/partial.

The proposed fallback is a capability-checked, time-bounded **network-aware joint
UC/redispatch solve for the affected window**, permitting commitment changes while
retaining all physical/security rows, intertemporal state and independent full-row
checks. Persist its formulation/version, trigger, repair count, original/final
commitment, bounds/gaps, cost difference and approximation/fallback flag. A qualified
feasible incumbent may continue subsequent windows from its validated state; final
pricing is rerun against that final commitment with original/repaired/fallback prices
identified separately. Fallback cost is not silently treated as ordinary scheduling.

Curtailment and physically modelled load shedding retain explicit penalties and
reporting; no arbitrary branch/security slack or disabled 132 kV constraint is added
to make a window appear feasible. Non-convergence alone is not proof of shortage.
If fallback also cannot produce a physically valid state, report that distinct
outcome and retain a partial run/coverage with diagnostics; never fabricate state,
skip an interval and claim completion. A mathematical infeasibility or backend error
is different from a heuristic iteration limit. Use merged ADR 0003’s proposed
Q-N-5 budgets: at most three UC repair re-solves, one 300-second network-aware
recovery MILP, freeze recovered commitment/segments and re-solve the network LP,
then final-commitment pricing, all within the 900-second total window budget.
Capability and full-network qualification remain post-baseline evidence.

National pricing excludes redispatch network rents; nodal/proxy duals stay labelled
diagnostics. Non-convex start/no-load costs and uncovered revenues are reported;
fixed-UC LP prices do not assert uplift or exact imbalance settlement.

## Dated composition, products, interconnectors and currency

A market-rule release includes parameter tables, code/build identity, source hashes,
effective range and knowledge evidence. Bind its hash in ADR 0002's scenario lock.
At a rule transition, select the dated regime and translate state through an explicit
compatible mapping; fail if unavailable. Version the product calendar and mapping
between model intervals and settlement/trading products. One-minute dispatch must
not be presented as an observed one-minute market product when settlement is coarser.

GB, I-SEM and external boundary modules contribute to **one coordinated solve plan**,
not independent optimisations that schedule the same interconnector capacity twice.
The coordinator owns each interconnector's shared capacity allocation, transfer
schedule, effective explicit/implicit trading regime and direction policy. Its
outputs become endpoint injections through network's loss model. External historical
flows versus alternative price/flow policies are scenario-labelled (PR #4 Q-N-2).
Do not assume GB↔SEM is a permanently implicitly coupled auction or use future links
in an earlier year. The required coupling and commissioning transition fixtures
come from PR #4 FR-1; dates must be sourced, not guessed here.

Prices and cashflows keep native GBP/EUR units. Convert the joint optimisation
objective once into its declared numeraire using the pinned daily exogenous FX
series and effective calendar (D-38). Convert pricing duals back to native currency/
MWh with objective and interval scales recorded. Do not add GBP and EUR directly,
double-count a transfer payment as production cost, manufacture an FX trade or
charge interconnector energy losses twice. Cost, revenue, balancing action and
congestion-rent ledgers remain distinct and reconcile signed prices/flows.

An output series identifies module/rule version, market or zone, product, delivery
start/duration, price method, currency/units, original/repaired stage, fidelity and
validity. Incomplete/unsupported pricing yields an unavailable field with reason,
not zero or a substituted product. Reference data retains its own product definition;
GB Market Index is not silently relabelled auction day-ahead. D-57's free-source
policy and D-56's lineage/admission rules apply to parameter/observed-price datasets.

## Zonal and nodal alternatives in P3

A `zonal` module pins a dated zone-definition release mapping canonical buses to
pricing areas, and a capacity methodology/input release. Zonal membership does not
change bus IDs or erase intra-zonal branches. Declare the chosen methodology
(for example NTC or qualified flow-based constraints), calibration and residual
redispatch treatment; no method is presumed historically real for GB.

Zonal scheduling/pricing may replace the P1 national scheduling/pricing policies,
while physical feasibility/redispatch still use the same accepted network and
intertemporal asset states. Export zonal prices, inter-zonal limits/rents and
intra-zonal redispatch separately. A nodal alternative requires network-constrained
pricing with explicit losses/reserves/commitment treatment; it cannot simply rename
P1 redispatch duals and call the result a market design.

Paired market comparisons fix network, weather, costs, boundary policy, temporal
coverage and solver tolerances unless a scenario diff explicitly changes them.
Report welfare/production cost, redispatch, curtailment, unserved energy and revenue
effects with native-currency/FX reconciliation. Zone choice, capacity assumptions
and pricing method are experimental inputs, not evidence of adopted REMA policy.
Alternative modules must declare a stage plan compatible with engine/checkpoint
contracts; a new formulation can require new builders and schema versions, but
does not require replacing storage, jobs, physical identities or solver adapters.

## Alternatives, trade-offs and evidence

| Model | Strength | Reason to decline as the default |
|---|---|---|
| Rules embedded in engine/solver branches | Quick first implementation | Couples jurisdictions, physics and vendor interfaces; fragile comparative experiments |
| Declarative TOML-only DSL | Easy inspection and constrained execution | Parameters cannot express every auction/settlement algorithm; DSL compiler becomes a new framework |
| Dynamic Rust/C shared libraries | Runtime extension and low call overhead | Unstable Rust ABI, unsafe lifecycle and native code access; poor tenant isolation |
| WASM/WASI sandbox | Portable, potentially useful for future third-party rules | Runtime/host API/resource-limit qualification and data marshalling; not a solver sandbox or automatic security proof |
| Separate market service per design | Independent implementation languages/releases | RPC/state consistency and extra deployment surface; conflicts with Rust-only runtime if it wraps validators |
| Compiled trusted Rust modules (chosen) | Type-checked domain boundaries and reproducible builds, no new runtime | Deploy to add code; trait/schema evolution needs compatibility tests; no untrusted extensions |

The choice follows D-20, ADR 0001's module split and ADR 0003's solver-neutral staged
pipeline. Operator rule documents [S1–S3] establish that rules/products need distinct
jurisdiction/date treatment; they do not validate our proxies or historical accuracy.
WASM/process extension remains an option if an owner-approved third-party extension
use case arrives; keep task/result/state schemas versioned to support that change.

Before adoption: apply D-69, review a sourced rule/fidelity inventory and the
merged PR #4 price/product definitions. After baseline: identical-input
GB/I-SEM reference fixtures, cross-border loss/FX/datetime reconciliation, unsupported
capability rejection, rule-transition/state migration, cancellation/restart and
negative/degenerate-price cases. P1 historical price/BM validation remains separate
from solver agreement. P3 adds congestion-within/across-zone and paired-market
comparisons; no zonal result or performance benchmark is claimed now.

## Primary references and evidence limits

- S1 — SEMO Trading and Settlement Code / balancing-market rules: https://www.sem-o.com/rules-and-modifications/balancing-market-modifications/market-rules
- S2 — SEMOpx ex-ante operator and market documentation entry point: https://www.semopx.com/
- S3 — Elexon Balancing and Settlement Code entry point: https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/
- Exact PR #4 pipeline/requirements evidence: [source snapshot](../topology.md#design-context-and-evidence).

SEMO/SEMOpx entry points were accessible during research on 2026-11-13; Elexon's
entry point returned HTTP 403. No specific current BSC clause or complete historic
rules archive was verified here. Pin the legally retainable source documents,
effective dates and knowledge evidence before claiming any particular rule is
implemented or a historic market arrangement has been replicated.
