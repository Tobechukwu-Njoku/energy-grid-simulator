# CI execution and secret-handling policy (NFR-12)

- **Status:** M0 policy, for review. Derived from ADR 0001 ("Runner security, throughput
  and spend") and NFR-12, both accepted under D-72. It adds no new decisions.
- **Scope at M0:** this policy and the hosted-runner configuration that enforces it.
  Nothing here provisions a runner. NFR-12.AC1/AC2 runner evidence belongs to M1.

## 1. Where code runs

| Environment | What may run there | What must never run there |
|---|---|---|
| GitHub-hosted runners (M0 onward) | fmt, clippy, nextest, cargo-deny, hook/policy tests, documentation gates, light Linux/macOS builds | Secrets other than the read-only `GITHUB_TOKEN`; solver licences; database or benchmark jobs |
| Coding container (D-65) | Editing, review, light builds and the hook/structural checks | Heavy builds, databases, benchmarks, simulator runs, CI runner software |
| D-65 target system (M1 onward) | Heavy B1 jobs: reference, mutation, benchmark, database and simulator runs | Untrusted fork code; any job before the M1 negative-test suite passes |

A heavy job whose required target runner is missing or misidentified **blocks**. It never
falls back to a hosted runner or the coding container, and a CPU run never stands in for a
GPU gate (NFR-12.AC3, NFR-7.AC4).

## 2. Hosted-runner rules (enforced now)

`cargo xtask policy` parses every workflow and local action as YAML and fails the
`policy` job if any of these breaks:

1. Only allow-listed triggers (`pull_request`, `pull_request_review`, `push`); never
   `pull_request_target` or `workflow_run`. PR code runs with a read-only token and no secrets.
2. An explicit top-level `permissions:` block; every workflow and job permission is `read`
   or `none`, in block or flow (`{contents: write}`) form. Any write scope needs a
   separately reviewed exception in this document.
3. Every third-party action is pinned to a full 40-character commit SHA; the tag goes in a
   comment. Updating a pin is a reviewed policy change.
4. `actions/checkout` always sets `persist-credentials: false`.
5. A `run:` script contains only allow-listed `${{ }}` expressions (`env.*`, `runner.*`);
   every event value reaches a script through `env:` or the event file. Scripts never
   run Python (D-73). No YAML aliases, `RUSTFLAGS` or rustc wrapper.
6. No `secrets.*` other than `GITHUB_TOKEN`, no `secrets: inherit`; only the hosted
   labels `ubuntu-24.04` and `macos-15` before M1; no `container:` or `services:`.
7. **Trusted policy.** Every policy job checks out the base revision
   (`.github/actions/trusted-policy`) and runs that revision's `xtask` with its policy
   files (`xtask/policy/`, `.githooks/attribution-patterns.regex`) against the PR
   checkout; cargo-deny reads the base `deny.toml`. A PR therefore cannot weaken the
   rules applied to itself. The trusted gate graph (`xtask/policy/gates.toml`) requires
   every gate job, unconditional, and `merge-gates` to depend on all of them.
   Bootstrap: when the base has no `xtask` yet (the M0 seed PR), the PR's own copy runs
   with a warning; that seed is reviewed independently.
8. **Policy changes.** `.github/`, `.githooks/`, `.cargo/`, `xtask/`, `deny.toml`,
   `Cargo.toml`, `rust-toolchain.toml` and `docs/security/` are owned by the policy owner
   in `.github/CODEOWNERS`. The `policy-approval` check fails until the owner approves
   the exact head. A pull-request workflow is itself defined by the PR, so these checks
   rely on branch protection requiring `merge-gates`, `independent-review` and
   `policy-approval` by name, plus code-owner review; the owner configures that.

No caches are written by M0 workflows. If caching is added, PR jobs may only read caches
written by trusted `main` runs, and caches are keyed by toolchain, platform, lockfile and flags.

## 3. Target runner design (activated at M1)

ADR 0001 ASSUMPTIONS (Q-N-7, Q-N-11, Q-N-12) apply until the owner supplies target details.

- **Dispatch:** only a trusted workflow on `main` dispatches a heavy job, naming an exact
  commit SHA that already passed the hosted gates and has current-head review evidence.
  The runner rejects a stale, unknown or unreviewed SHA. Fork PRs never reach it.
- **Account:** a dedicated unprivileged account, not in `sudo`, `docker` or similar groups.
- **Isolation:** each job runs in an ephemeral rootless container, destroyed afterwards.
  No Docker socket, SSH keys, cloud or provider credentials, or solver licence files are
  mounted. Network and filesystem access are limited to what the job declares.
- **Runner token:** the limited registration token is the only host credential and is not
  readable from job containers.
- **Licensed solvers and GPUs:** not used in standard CI (FR-18.AC4). Enabling one needs
  its own reviewed isolation, device or licence check and the applicable gate.
- **Limits:** rootless containers reduce accidental exposure. They do not defend against
  a malicious host admin, a kernel compromise or misuse of the shared owner identity.

## 4. M1 activation checklist (NFR-12.AC1/AC2)

The runner stays disabled until every item passes on the D-65 target, with recorded
commands and results. Tests use canaries, not real secrets, and no host-escape exploit.

- [ ] Dispatch of an unauthorised SHA is rejected.
- [ ] Dispatch of a stale SHA (no longer the reviewed head) is rejected.
- [ ] A fork PR cannot trigger a job on the runner.
- [ ] A job cannot read a canary licence file or canary SSH key placed on the host.
- [ ] A job cannot mount or reach the Docker/Podman socket.
- [ ] A job cannot read the runner token.
- [ ] A missing required runner blocks the job instead of falling back.
- [ ] An untrusted edit to policy files in a PR does not change the policy applied.
- [ ] Spoofed review or bot-summary metadata does not satisfy a gate.
- [ ] The rootless execution configuration is inspected and recorded.

M3 adds service and exposure checks. A failed item keeps the runner disabled.

## 5. Application exposure (NFR-12.AC3)

P1 services listen on localhost or a private VPN only, with no public endpoint. External
exposure waits for answers to Q-L-2 and Q-L-3.

## 6. Secrets and manifests

No licence secrets, tokens or credentials are committed or written into scenario, run,
dataset or provenance manifests. Owner credentials (for example ENTSO-E or Copernicus)
stay outside the repository and outside CI until a reviewed mechanism exists.
Report suspected vulnerabilities as described in [SECURITY.md](../../SECURITY.md).
