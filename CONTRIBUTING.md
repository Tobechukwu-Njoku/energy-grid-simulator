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
- **No `unsafe`** outside the reviewed solver adapters listed (package name and
  directory) under `[[adapter]]` in `xtask/policy/repository.toml`; there, allow it per
  item with a `// SAFETY:` comment. A new adapter is a protected policy change.
- **Licences:** new dependencies must pass `deny.toml` as it is on the base branch.
  Allow-list or exception changes need independent review and the policy owner's
  approval (NFR-8, `.github/CODEOWNERS`).
- **No Python or Julia** in the production dependency graph (NFR-10).
- **Workflows:** SHA-pinned actions, read-only token, no `pull_request_target` outside the
  trusted gates, and event values reach scripts as `"$VAR"`, never `${{ env.VAR }}`
  ([CI security policy](docs/security/ci-runner-policy.md)).
- **Data:** never commit raw `research-only` data; commit download scripts only (D-56).
  Every provenance manifest tags each dataset `open-commercial` or `research-only`.
  A provenance manifest is always named `manifest.toml` (or is a `sources.tsv` table);
  any other file that declares a `licence_status` or is named like a manifest is rejected,
  because it would not be read (ADV23R3-07). A manifest key names files when its last
  `_`/`-` word is `file`, `files`, `filename`, `path`, `paths` or `name` (for example
  `sha256_index_file`, `relative_path`, `local_name`), at any depth; any value shaped like
  a file name is also registered (ADV23R3-06).
- **Secrets:** `cargo xtask provenance` evaluates the [gitleaks](https://github.com/gitleaks/gitleaks)
  default ruleset (MIT), vendored unchanged in `xtask/policy/gitleaks.toml`, plus local rules
  in the same schema in `xtask/policy/secret-rules.toml`. Every file is scanned as raw bytes
  and as UTF-8, UTF-16 and UTF-32 text in both byte orders and every alignment; a finding in
  any view fails. In CI it also scans every file version any commit of the pull request
  (or push) introduced, so a secret added and later removed still fails; locally, pass
  `provenance --base origin/main --head HEAD`. Inline `gitleaks:allow` markers are ignored.
  A synthetic example quoted in a review needs a `[[secret_exemption]]` in
  `xtask/policy/repository.toml` naming the whole detected block (its lines, and for a
  private key everything through the key's end marker) by `git hash-object` of its text.
- **Policy paths** (`.github/`, `.githooks/`, `.cargo/`, `xtask/`, `deny.toml`,
  `Cargo.toml`, `rust-toolchain.toml`, `docs/security/`) need the policy owner's APPROVED
  review on the exact head. Agents push and open pull requests from the machine account
  in `xtask/policy/repository.toml` (`agent_account`, D-74); the owner cannot approve a
  pull request they opened, so an owner-opened policy change cannot pass.
- **Unsafe code:** every crate root of a package that is not a reviewed adapter (and
  every build script) starts with `#![forbid(unsafe_code)]`: the bare Rust lint, not a
  tool-qualified one such as `clippy::unsafe_code` (R3-01). Every library root also
  carries `#![doc(test(attr(forbid(unsafe_code))))]`, because each doctest is a separate
  crate (R6-04). Every file Cargo selects as a target or build script is checked
  whatever its file name (R3-02).
- **Native libraries:** the build gate's linker accepts only the libraries and
  frameworks listed in `xtask/policy/gates.toml` (`link_libraries`, `link_frameworks`)
  plus Rust's own artifacts; a new native dependency is a protected policy change.

Checks to require on `main` (branch protection is configured by the owner; it is not
on yet): `merge-gates` (all automated gates, listing the ones not yet applicable),
`independent-review` and `policy-approval` (both in `trusted-gates.yml`, which runs from
`main` and never runs pull-request code). The last two fail until the review evidence,
triage or owner approval exists for the current head; re-run them after it lands.

## Updating the gitleaks ruleset

New secret formats come from upstream gitleaks, not from local prefixes. To update (a
protected policy change that needs the owner's approval):

1. Pick a gitleaks release tag at least two weeks old and resolve it to a commit:
   `gh api repos/gitleaks/gitleaks/commits/<tag> --jq .sha`.
2. Replace `xtask/policy/gitleaks.toml` with `config/gitleaks.toml` from that commit,
   byte for byte, and `xtask/policy/gitleaks-LICENSE.txt` with its `LICENSE`.
3. In `xtask/policy/secret-rules.toml` set `[vendor]` `version`, `commit`, `source` and
   `blob` (`git hash-object xtask/policy/gitleaks.toml`). A copy that differs from the
   pinned blob fails the check.
4. Run `cargo nextest run -p xtask` and `cargo xtask provenance`. A rule whose Go regex
   does not compile in Rust, or a schema key xtask does not evaluate (for example
   `[[rules.required]]` or `[extend]`), fails closed: extend `xtask/src/secrets.rs` (and
   its Go-to-Rust `translate`) with a test, never drop the rule. Triage new findings:
   fix real secrets; add a narrow `[[allowlists]]` with `targetRules` for a false-positive
   class, or an exact-line exemption for a quoted synthetic example.
5. Remove a local rule from `secret-rules.toml` only when an upstream rule detects
   the same tokens, including the low-entropy synthetic tokens in the xtask tests.
