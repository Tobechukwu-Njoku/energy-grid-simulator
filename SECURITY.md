# Security policy

## Reporting a vulnerability

Please report security problems privately through GitHub's
[private vulnerability reporting](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/security/advisories/new)
(Security tab, "Report a vulnerability") rather than in a public issue. The repository
owner must enable private vulnerability reporting in the repository settings for this
route to work. Include the affected commit, the steps to reproduce and the
impact you expect. There is no service in production yet, so reports mainly concern the
repository, its CI and its dependencies.

## Supported versions

The project is pre-release. Only the `main` branch is supported.

## How CI is protected

CI on pull requests runs on GitHub-hosted runners with a read-only token, no secrets and
SHA-pinned actions. Heavy jobs will run on a separate, isolated target runner from
milestone M1. The full policy, including the M1 activation checklist, is in
[docs/security/ci-runner-policy.md](docs/security/ci-runner-policy.md).

Dependencies are checked by `cargo deny` against the licence allow-list and the RustSec
advisory database (see `deny.toml`).
