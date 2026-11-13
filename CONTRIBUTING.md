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

Policy and documentation checks need only Python 3, Perl and Git:

    python3 .githooks/tests/run.py              # commit-msg hook fixtures
    python3 -m unittest discover -s ci/tests    # CI attribution check, hook parity
    python3 ci/doc_gates.py                     # Markdown links and traceability
    python3 ci/policy_checks.py                 # unsafe scope, workflow hardening

The coding container is for light builds only. Heavy builds, databases and benchmarks run
on the separate target system (D-65).

## Rules that CI enforces

- **No AI attribution** in commits, PR titles and bodies, or added lines (D-52).
- **No `unsafe`** outside `crates/solver-*` adapters; there, allow it per item with a
  `// SAFETY:` comment.
- **Licences:** new dependencies must pass `deny.toml`. Allow-list or exception changes
  need independent review (NFR-8).
- **No Python or Julia** in the production dependency graph (NFR-10).
- **Workflows:** SHA-pinned actions, read-only token, no `pull_request_target`
  ([CI security policy](docs/security/ci-runner-policy.md)).
- **Data:** never commit raw `research-only` data; commit download scripts only (D-56).

The single required check is `merge-gates`. It fails if any gate fails and lists the gates
that are not yet applicable. Independent review evidence is still checked by the coordinator
on the exact head SHA (NFR-3).
