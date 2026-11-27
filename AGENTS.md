# AGENTS.md

Build the energy grid simulator from the [M0 requirements](docs/requirements/04-requirements.md)
and [accepted ADRs](docs/architecture/adr/). Follow the [team and review process](docs/process/team.md).

- Use the owner's product decisions in [the decisions log](docs/requirements/03-decisions-log.md)
  as the source of truth. Prefix assumptions with `ASSUMPTION:` and add an open question to the
  [questionnaire](docs/requirements/02-questionnaire.md). Keep existing IDs stable and deprecate
  obsolete entries.
- Update [STATUS](docs/requirements/STATUS.md) when requirements work changes the project state.
  Record decisions and handoffs in the repository so other threads can resume from it.
- Prefer mature open-source tools and the highest-performance option that fits the accepted
  architecture. Use the D-65 target system for heavy builds, databases and benchmarks.
- Keep commits small and push after each meaningful step unless the task calls for local work.
  Use the machine account for PRs. See [Contributing](CONTRIBUTING.md) for local setup and CI.
