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

1. Only allow-listed triggers (`pull_request`, `push`); never `workflow_run`.
   `pull_request_target` is allowed only in `trusted-gates.yml`, listed as trusted in
   `xtask/policy/gates.toml`; its jobs may not set a checkout `ref` or `repository`,
   use a local action or a cache. PR code runs with a read-only token and no secrets.
2. An explicit top-level `permissions:` block; every workflow and job permission is `read`
   or `none`, in block or flow (`{contents: write}`) form. Any write scope needs a
   separately reviewed exception in this document.
3. Every third-party action is pinned to a full 40-character commit SHA; the tag goes in a
   comment. Updating a pin is a reviewed policy change.
4. `actions/checkout` always sets `persist-credentials: false`.
5. A `run:` script contains only allow-listed `${{ }}` expressions (`runner.temp`,
   `runner.os`). Every event value reaches a script as a shell variable set in `env:`
   (`"$TITLE"`), never as `${{ env.TITLE }}`, which pastes the value into the script
   text (F5). Scripts never run Python (D-73). A `shell:` (on a step, or inherited from
   `defaults.run.shell` of a job or the workflow) is one of `bash`, `sh`, `pwsh`,
   `powershell` or `cmd`; any other value, including an `interpreter {0}` template, is
   rejected (ADV23R3-05). No YAML aliases. No `env:` may set a `CARGO_*`, `RUSTC*`,
   `RUSTDOC*`, `RUSTUP_*` or `*RUSTFLAGS` variable (flags, wrappers, linkers, lint
   caps) unless `allowed_env` in the trusted `gates.toml` lists it (ADV23R5-01).
   A `run:` script may name a file-command file (`$GITHUB_ENV`, `$GITHUB_PATH`,
   `$GITHUB_OUTPUT`, `$GITHUB_STATE`, `$GITHUB_STEP_SUMMARY`, compared without case
   or quoting) only on a line `allowed_file_command_lines` lists (none today), and
   may not reach variables indirectly (`${!x}`, `eval`, `printenv`, environment
   listings, namerefs, `ENVIRON`, `process.env`) (ADV23R6-03). That lint is not
   the guarantee: see 5a. Cargo config files may hold only `[alias]`.
5a. **Compiler-enforced unsafe scope.** The `policy` job also runs
   `cargo xtask build-gate` from `main`'s tree: it builds the PR workspace with
   `main`'s xtask as rustc wrapper, adding `-F unsafe_code` to every local crate
   except the reviewed adapters listed (package and directory) in `main`'s
   `repository.toml`, and a linker guard that accepts only allow-listed libraries
   (ADR 0001 addenda, revisions 6 and 7). It runs PR build scripts, proc macros and
   doctests, so it only ever runs in this unprivileged job, never in the trusted
   gates, and it must be the job's last step, without an `id` and in a job without
   `outputs`: whatever PR code writes to a file-command file reaches no later step,
   and a later step can only fail the job (ADV23R6-03).
6. No `secrets.*` other than `GITHUB_TOKEN`, no `secrets: inherit`; only the hosted
   labels `ubuntu-24.04` and `macos-15` before M1; no `container:` or `services:`.
7. **Trusted gates.** `independent-review` and `policy-approval` run from `main`
   (`.github/workflows/trusted-gates.yml`, `pull_request_target`). They build `main`'s
   xtask in a checkout of `main`, fetch the PR head only as Git data, and never check
   out, build or run PR code. Both compare against the current tip of `main`.
8. **Trusted policy for PR jobs.** Every policy job loads the current tip of `main`
   (`.github/actions/trusted-policy/load.sh`), not the event's base, and runs that
   tree's `xtask` with its policy files (`xtask/policy/`,
   `.githooks/attribution-patterns.regex`) against the PR checkout; cargo-deny reads
   `main`'s `deny.toml`. If `main` has no xtask policy the job **fails closed**. The one
   exception is the bootstrap seed: the exact head SHA the coordinator attests on
   `main` in `docs/process/ci-bootstrap.toml` as `seed_sha = "<40 hex>"`.
9. **Policy changes.** `.github/`, `.githooks/`, `.cargo/`, `xtask/`, `deny.toml`,
   `Cargo.toml`, `rust-toolchain.toml` and `docs/security/` are owned by the policy owner
   in `.github/CODEOWNERS` (checked with GitHub's last-match rule). The trusted
   `policy-approval` gate fails until the owner's latest review is APPROVED on the exact
   head, then needs a re-run. Agents open PRs from the machine account named
   `agent_account` in `xtask/policy/repository.toml` (D-74); a policy PR opened by the
   owner cannot pass because GitHub forbids self-approval. Because workflow and action
   files are protected, a PR cannot change what its jobs run without that approval,
   and a PR job reusing a trusted gate name fails the gate. Branch protection should
   require `merge-gates`, `independent-review` and `policy-approval`; the owner
   configures that. Residual risks are listed in ADR 0001's 2026-11-13 addendum.

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
