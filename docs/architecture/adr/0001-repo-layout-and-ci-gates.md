# ADR 0001 — Repository layout and enforced CI gates

- **Status:** Proposed
- **Date:** 2026-11-12
- **Linked decisions:** D-3, D-4, D-8, D-16, D-17, D-19, D-20, D-22, D-28, D-29, D-31, D-43, D-48, D-49, D-51, D-52, D-53.
- **Related requirements:** FR-4, FR-13–FR-19; NFR-1–NFR-4, NFR-7, NFR-8, NFR-10, NFR-12 in `../../requirements/04-requirements.md`.

## Context

This is a documentation-only requirements repository. Application code is prohibited
until the coordinator's baseline gate opens; D-31 permits only the data spike. This
ADR proposes future layout and enforcement, not workspace scaffolding or workflows
in this PR. Neither this Proposed ADR nor a green documentation check changes STATUS.

D-20 calls for Rust production modelling/backend/SDK/CLI and a TypeScript frontend;
offline Python/Julia reference tools remain permitted. D-28 requires formatting,
lint, tests, licence/advisory checks, reference agreement, no benchmark regression
and independent review. D-43 requires merge-gate evidence from CI, with a self-hosted
GPU runner; a successful SSH experiment is not a substitute. D-49/team.md strengthens
independence to **cross-provider** review. The owned Linux CPU and Mac development
paths must remain usable without CUDA or commercial solver licences.

During drafting, main gained D-51–D-53. Apply the refined verification roles, prohibit
attribution/watermark metadata under D-52 and require D-53's additional adversarial
review with verified triage. The coordinator reports that branch protection is not
available on the current private-repository plan; rule-based discipline today is
not equivalent to the technical enforcement proposed below.

## Decision (proposed)

### Cargo workspace and frontend boundaries

Use one repository and one root Cargo workspace with a committed `Cargo.lock`, pinned
Rust toolchain and explicit default CPU features. Proposed directories, created only
after code is allowed:

```text
Cargo.toml                       workspace and shared dependency policies
Cargo.lock / rust-toolchain.toml reproducible Rust build inputs
deny.toml                        reviewed Rust licence/advisory configuration
crates/
  domain/                        identifiers, units, result/status contracts
  scenario/                      TOML schema, validation, canonical hashing
  optimisation/                  linear-model contract and solver trait (ADR 0003)
  solver-highs/                  default CPU adapter
  solver-gurobi/                 optional licensed adapter
  solver-xpress/                 optional licensed adapter
  solver-cuopt/                  optional GPU adapter
  engine/                        UC, redispatch, pricing, rolling orchestration
  api/                           Rust service and application composition
  sdk/                           Rust client API
  cli/                           Rust command-line application
apps/web/                        TypeScript UI; own package/lock/config files
schemas/                        shared versioned scenario/API schema artifacts
tests/
  cases/                         small licensed, hashed mathematical fixtures
  reference/                     pinned offline PyPSA/optional Julia environment
  acceptance/                    API/UI and historical validation specifications
benchmarks/                      fixture manifests and benchmark definitions
ci/                             trusted gate specifications and evidence schemas
.github/workflows/               CI entry points, added after baseline acceptance
docs/                           decisions, requirements, ADRs and validation reports
spikes/                         D-31 throwaway data exploration, not runtime code
```

The `optimisation` contract depends only on domain/numerical value types, not a
concrete backend. Adapters depend on that contract; the engine consumes it;
application composition selects adapters. No engine-to-frontend or production-to-
offline-validator dependency is permitted. UI and CLI use the same versioned scenario
schema/API; fixture round trips detect schema drift. Do not generate an SDK with a
second scenario semantics implementation.

Default workspace membership/feature selection builds the CPU path. Optional adapter
packages have explicit capability-specific CI jobs, so `--all-features` is not a
synonym for a meaningful build when SDKs/devices are absent. Native FFI unsafe code
is confined to reviewed adapters. Optional adapter packages expose contracts without
native linking by default; named features enable their real SDK implementations.
Thus `--workspace` CPU tests do not accidentally require commercial SDKs. Native
numerical libraries do not violate D-20;
Python/Julia runtime modelling would. JavaScript package management and exact crate
versions are selected/pinned during implementation, not implied owner decisions here.

Keep PostgreSQL 18 and Redis as D-3 requires, but **do not settle their storage/job
roles here**. ADR 0002 owns the data model after the spike; ADR 0004 owns storage.
Bulk market/network/weather data stay outside Git except licensed small fixtures
and immutable manifests. A manifest points to authorised artifacts and preserves
hashes; it does not make restricted data redistributable.

### GitHub Actions gate graph

Each PR targets protected `main`. The eventual workflow evaluates the current PR head
and merge candidate; all evidence binds the tested SHA, input/model hashes and workflow
revision. `merge_group` runs the equivalent graph if a merge queue is enabled.

| Required gate | Work and evidence | Execution |
|---|---|---|
| `rust-format` | `cargo fmt --all --check` | Hosted Linux |
| `rust-lint` | `cargo clippy --workspace --all-targets -- -D warnings` for default CPU, plus explicit supported feature jobs | Hosted Linux; capability jobs as needed |
| `cpu-tests` | Locked workspace tests, small solver/domain cases and CPU-only build/run | Hosted Linux + macOS |
| `web-contract-tests` | Locked TypeScript install, type/lint/build/tests and shared-schema round trips | Hosted Linux |
| `licence-advisory` | `cargo deny check`; separate reviewed native, frontend, service/container and data inventories | Hosted Linux + trusted evidence verification |
| `numerical-reference` | Independent residuals and exact cases, pinned PyPSA comparison on identical inputs with stated tolerances | CPU CI; offline validator environment separate from production |
| `gpu-validation` | GPU build/conformance/numerical parity on a supported installed device; logs actual driver/toolkit/VRAM | Isolated self-hosted Linux NVIDIA runner (D-43) |
| `benchmark-regression` | Paired protected-main/candidate performance cases on stable D-48 hardware, including the applicable GPU case | Serial self-hosted performance runner |
| `cross-provider-review` | Trusted current-head attestation of author/reviewer providers and review evidence | Independently credentialled check issuer |
| `adversarial-review-triage` | D-53 adversarial findings and independently verified accept/reject dispositions for substantive changes; cannot substitute for approval | Protected evidence verifier |
| `attribution-policy` | D-52 check of source/file headers, commits and PR metadata; process/review documents follow their explicit exception | Hosted Linux + protected metadata check |
| `merge-gates` | Verifies every required gate's conclusion, SHA and artifact identity; missing/cancelled/inconclusive is failure | Protected aggregator |

Use `cargo test --workspace --locked` for the default CPU workspace and a separately
enumerated feature/adapter matrix. Do not require unavailable commercial licences to
pass standard CI. When an optional adapter is changed, its capability tests require
real licensed/runtime evidence or remain blocking; a successful stub build is not
solver conformance. A vendor-independent trait mock tests the contract only.

Numerical gates use FR-13/NFR-2 semantics. A DC fixture is compared to an identical DC
reference, not an AC pglib objective; future AC gates can add appropriate cases. A
per-PR representative numerical/benchmark suite supplies D-28 evidence. Full-year
historical accuracy and NFR-5 throughput are release acceptance jobs and may run
scheduled/as needed; they do **not** replace the per-PR gates. Changes invalidating
the full-year baseline require new acceptance evidence before release.

Do not use path filters that make required check contexts disappear. A trusted change
classifier can emit explicit non-applicability evidence only under an accepted policy
when the capability is provably unaffected; the aggregator still runs. During the
documentation-only bootstrap, record that Cargo/application tests do not yet exist;
do not manufacture passing solver/benchmark results. **OI-11 must settle that bootstrap
policy before autonomous merges depend on it.** This PR merely opens the design for
review and does not assert these gates are already configured or satisfied.

### Enforcing D-28 rather than documenting it

Configure repository rulesets/branch protection to require PRs, all required contexts,
current-head approval/evidence and no force pushes/deletion. Configure expected check
sources where supported. Restrict the bypass list; the routine agent merger is not
an administrator/bypass actor. Dismiss stale approvals after new code and rerun the
gate graph after an update to the merge candidate. Workflow/policy changes require
independent review of the enforcement itself, and cannot authorise themselves using
new unreviewed classifier/attestor code.

If multiple models share one human GitHub identity, a PR review username alone cannot
prove provider independence. A separately trusted GitHub App or equivalent protected
attestor issues `cross-provider-review` with:

- author provider/model and reviewed head SHA;
- reviewer provider/model and independent review record;
- a verified provider inequality and review outcome;
- protected issuer identity, timestamp and artifact digest.

PR comments, labels and an author's own workflow output cannot mint this attestation.
The attestor must verify the evidence against orchestrator/provider records, not
accept free-text model names. Assistant output is verified by the designated verifier
under D-51. D-53 adversarial review is a separate non-approving input whose findings
must be triaged; it cannot authorise a merge. D-51's code-only role allocation and
team.md's retained cross-provider approval rule need OI-11 clarification for
specification-authored PRs. A failed check is not overridden by a review.

Push each meaningful documentation/implementation step in small commits (D-53).
Do not add generated-by footers, attribution trailers or badges to source, commits
or PR metadata (D-52). Review/process evidence may record provider identities where
the decision explicitly permits it; do not turn that evidence into attribution in
file headers or release artifacts.

### Benchmark and GPU runner policy

Use the **owned Ryzen 5900X / 128 GB** for performance acceptance. Record actual OS,
BIOS/power profile, cores/threads, available RAM, solver/thread settings and concurrent
services. Serialise benchmark jobs; do not run a second solver concurrently. Compare
protected-main and candidate on the same device and dataset; preserve raw repetitions,
model sizes, optimality quality, RSS and wall-clock stages. Baseline artifacts come
from protected main, not files freely rewritten by the PR. A faster but less accurate
solve cannot pass a no-regression gate. NFR-4's unresolved statistical policy remains
blocking, not an implicit 5% slowdown allowance.

SSH is for development and benchmark diagnosis; required artifacts are reproduced by
CI. The GPU runner advertises its actual device and supports only verified combinations.
The GTX 1080 Ti cannot be assumed to run cuOpt; a planned RTX 5060 Ti is not installed
capacity. The build matrix distinguishes CPU-only, supported GPU and unavailable
capabilities. Unavailable required GPU evidence is a blocking result, not a skip.

Treat PR code as untrusted even in this private repository. Authorise the exact SHA
through a trusted review/dispatch boundary before allowing GPU-host execution. Use
a disposable isolated runner VM with device passthrough, or an equivalently isolated
reimaged worker; merely marking a runner registration ephemeral does not clean its
host. Do not execute untrusted PR content with `pull_request_target` secrets, mount
host Docker sockets, expose SSH credentials, or share writable caches with protected
jobs. Keep commercial licence material scoped to authorised jobs and out of logs.
If isolation cannot be supplied on the owner workstation, block the GPU gate and
resolve provisioning; do not silently weaken D-43.

### Enforcement acceptance tests

Before enabling autonomous merge, demonstrate with deliberately failing PRs that:

1. Every D-28 gate can independently stop merge, including absent/inconclusive GPU
   and benchmark jobs and stale approvals/checks after a push.
2. Same-provider or self-asserted review fails; a protected different-provider
   attestation for the exact head passes; forged/reused evidence fails.
3. A PR modifying CI, the classifier or benchmark baselines cannot approve itself.
4. A non-authorised PR cannot start GPU-host code or read licence/attestor/SSH secrets.
5. CPU tests pass without CUDA on Linux and macOS; unsupported GPU features fail
   explicitly instead of reporting a CPU fallback as GPU validation.

## Alternatives considered

- **Multiple repos / independent release trains:** increase schema/version coordination
  before the engine is validated; defer until a demonstrated ownership/deployment need.
- **Python orchestration around Rust kernels:** conflicts with D-20 for production
  modelling; keep Python/Julia only in isolated validation environments.
- **GPU required by all crates / all-features CI:** prevents CPU-only Mac/Linux builds
  and conflates device availability with correctness. Use explicit features and jobs.
- **Hosted CI only or SSH-only acceptance:** the former lacks the D-43 owned GPU path;
  the latter fails the CI-only merge-gate decision.
- **GitHub approval or PR label as the only model review check:** does not prove provider
  independence when models share credentials; use a trusted current-SHA attestation.
- **Skip expensive gates when runners are busy:** contradicts D-28. Queue or fail closed;
  a narrower approved representative suite is different from silently skipping a gate.

## Consequences

- Rust/domain/solver boundaries stay testable without a frontend or CUDA installation.
- A single repository makes cross-language schema and numerical changes atomic.
- Self-hosted CI and trusted review attribution are genuine provisioning/security work,
  not free consequences of adding YAML. Long GPU/performance queues may limit throughput.
- Exact CI enforcement depends on the private repository's GitHub plan and permissions;
  current STATUS reports no branch protection. Upgrade or approve a technically
  equivalent mechanism before claiming enforced autonomous merging.
- Full-year validation is expensive, but representative per-PR cases cannot be used to
  claim full-network historical accuracy or throughput before acceptance measurements.
- This ADR does not create a storage schema, choose result formats, accept data sources
  or resolve market rules; those decisions remain with their requirements/ADRs.

## Open items for the owner

- **ASSUMPTION (OI-11):** use protected current-head gates and an independently credentialled
  review attestor. Resolve the current plan limitation, attestor authority, docs-only
  bootstrap, D-51 versus cross-provider review roles, D-53 triage and benchmark/noise
  protocol; there is no automatic D-28 waiver.
- **ASSUMPTION (OI-7):** the actual GPU/toolkit combination can be made available in
  isolated self-hosted CI; confirm hardware, driver/distro and licensing entitlements.
- **ASSUMPTION (OI-8):** unknown licence/advisory dispositions block until explicitly
  reviewed; approve the allow-lists for native/frontend/services as well as Rust/data.
- **ASSUMPTION (OI-12):** PR code needs trusted exact-SHA runner authorisation and P1
  remains privately operated until an application access policy is accepted.

These refer to the detailed OI register in `../../requirements/04-requirements.md`;
the coordinator records owner answers, not this ADR.

## Primary references (checked 2026-11-12)

- [GitHub rulesets, required checks and private-repository plan availability](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets).
- [GitHub Actions security hardening and self-hosted runner risks](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions).
- [cuOpt supported hardware/software](https://docs.nvidia.com/cuopt/user-guide/latest/system-requirements.html).
- [Cargo workspace reference](https://doc.rust-lang.org/cargo/reference/workspaces.html).
