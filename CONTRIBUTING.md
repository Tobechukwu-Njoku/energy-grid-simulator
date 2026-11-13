# Contributing

Read [AGENTS.md](AGENTS.md) first. Requirements live in `docs/requirements/`, accepted
architecture decisions in `docs/architecture/adr/`. Work in small commits on a feature
branch and open a pull request against `main`.

## One-time setup

1. Enable the versioned commit-message hook (D-52). It strips AI attribution trailers
   and footers, and refuses a subject line that is attribution:

       git config core.hooksPath .githooks

   It needs Perl. `--no-verify` does not bypass CI, which rejects the same patterns.
2. Install [rustup](https://rustup.rs). The toolchain (1.99.1, with rustfmt and clippy)
   is pinned in `rust-toolchain.toml` and installs automatically on first use.
3. A C linker (`cc`) is needed to link test binaries. In a container without one, a
   user-space `zig cc` wrapper works:
   `export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/path/to/zigcc`.
4. Optional, to run the same gates as CI: `cargo install --locked cargo-nextest cargo-deny`
   (CI pins cargo-nextest 0.9.151 and cargo-deny 0.20.2).

## Build and test

    cargo build --workspace --locked
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo nextest run --workspace --locked
    cargo deny --locked check

Policy and documentation checks are a Rust `xtask` crate (D-73); the hook tests also need
Perl and Git:

    cargo xtask all                                   # policy, provenance, docs
    cargo xtask policy                                # unsafe scope, manifests, workflows, Python
    cargo xtask provenance                            # dataset tags, raw data, secrets (D-56)
    cargo xtask docs                                  # Markdown links and traceability
    cargo xtask attribution --base origin/main --head HEAD   # D-52 scan of your branch
    cargo nextest run -p xtask                        # policy tests, commit-msg hook fixtures

In CI each check runs from the base revision's `xtask` and policy files (`--policy`), so a
pull request is judged by the rules it started from. Python is allowed only in `spikes/`
and allow-listed offline reference validators; CI rejects it elsewhere.

The coding container is for light builds only. Heavy builds, databases and benchmarks run
on the separate target system (D-65).

## Rules that CI enforces

- **No AI attribution** in commits, PR titles and bodies, or added lines (D-52).
- **No `unsafe`** outside `crates/solver-*` adapters; there, allow it per item with a
  `// SAFETY:` comment.
- **Licences:** new dependencies must pass `deny.toml` as it is on the base branch.
  Allow-list or exception changes need independent review and the policy owner's
  approval (NFR-8, `.github/CODEOWNERS`).
- **No Python or Julia** in the production dependency graph (NFR-10).
- **Workflows:** SHA-pinned actions, read-only token, no `pull_request_target` outside the
  trusted gates, and event values reach scripts as `"$VAR"`, never `${{ env.VAR }}`
  ([CI security policy](docs/security/ci-runner-policy.md)).
- **Data:** never commit raw `research-only` data; commit download scripts only (D-56).
  Every provenance manifest tags each dataset `open-commercial` or `research-only`.
- **Policy paths** (`.github/`, `.githooks/`, `.cargo/`, `xtask/`, `deny.toml`,
  `Cargo.toml`, `rust-toolchain.toml`, `docs/security/`) need the policy owner's APPROVED
  review on the exact head. Agents push and open pull requests from the machine account
  in `xtask/policy/repository.toml` (`agent_account`, D-74); the owner cannot approve a
  pull request they opened, so an owner-opened policy change cannot pass.
- **Unsafe code:** every crate root outside `crates/solver-*` starts with
  `#![forbid(unsafe_code)]`.

Checks to require on `main` (branch protection is configured by the owner; it is not
on yet): `merge-gates` (all automated gates, listing the ones not yet applicable),
`independent-review` and `policy-approval` (both in `trusted-gates.yml`, which runs from
`main` and never runs pull-request code). The last two fail until the review evidence,
triage or owner approval exists for the current head; re-run them after it lands.
