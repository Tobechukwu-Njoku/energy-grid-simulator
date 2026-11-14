#!/usr/bin/env bash
# Loads the trusted policy tree for pull-request and push jobs (ADR 0001, R1, F1,
# ADV23R-08). The tree is the CURRENT tip of main, fetched now, never the event's
# possibly stale base. If main has no xtask policy the job fails closed, with one
# exception: the exact head SHA the coordinator attests in main's
# docs/process/ci-bootstrap.toml (`seed_sha = "<40 hex>"`) may supply its own seed.
# Outputs: dir (trusted tree), main (main SHA used).
set -euo pipefail

event="${GITHUB_EVENT_PATH:?}"
head="$(jq -r '.pull_request.head.sha // .after // empty' "$event")"
if ! printf '%s\n' "$head" | grep -Eqx '[0-9a-f]{40}'; then
  echo "::error::trusted policy: the event head is not a full commit SHA"
  exit 1
fi

git fetch --no-tags --quiet origin "+refs/heads/main:refs/remotes/origin/main"
main="$(git rev-parse --verify --quiet 'refs/remotes/origin/main^{commit}')"

if git cat-file -e "$main:xtask/Cargo.toml" 2>/dev/null; then
  source="$main"
  echo "trusted policy: current main $main"
else
  attested="$(git show "$main:docs/process/ci-bootstrap.toml" 2>/dev/null \
    | sed -n 's/^seed_sha = "\([0-9a-f]\{40\}\)"[[:space:]]*$/\1/p' | head -n 1 || true)"
  if [ -n "$attested" ] && [ "$attested" = "$head" ]; then
    source="$head"
    echo "::warning::bootstrap: main $main has no xtask policy; running the seed $head attested in main:docs/process/ci-bootstrap.toml"
  else
    echo "::error::trusted policy: main $main has no xtask policy and head $head is not the bootstrap seed attested in main:docs/process/ci-bootstrap.toml; failing closed"
    exit 1
  fi
fi

trusted="${RUNNER_TEMP:?}/trusted-policy"
rm -rf "$trusted"
git worktree prune
git worktree add --detach --quiet "$trusted" "$source"
if [ -z "${TRUSTED_POLICY_SKIP_TOOLCHAIN:-}" ]; then
  (cd "$trusted" && rustup toolchain install && rustc --version)
fi
echo "dir=$trusted" >> "${GITHUB_OUTPUT:?}"
echo "main=$main" >> "$GITHUB_OUTPUT"
