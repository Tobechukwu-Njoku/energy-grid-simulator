# ADR 0002 — Canonical network, data releases and TOML scenario references

- **Status:** Proposed
- **Date:** 2026-11-13
- **Decision sources:** D-6, D-11–D-12, D-18–D-21, D-24, D-30–D-32, D-35, D-37–D-38, D-40, D-44, D-55–D-60.
- **Related draft requirements:** PR #4 FR-1–FR-5, FR-9, FR-16, FR-19, FR-21; NFR-1–NFR-2, NFR-8, NFR-11.
- **Companions:** [storage](0004-storage-topology.md), [execution](0005-execution-topology.md), [market modules](0006-market-design-plugin-model.md), [topology and source snapshot](../topology.md).

## Context and authority

The same substation can occur under different operator identifiers; a circuit can
change endpoints or ratings without becoming a different physical asset. A 2024
backtest must not inherit a later interconnector, network revision or market rule.
Distribution/DER, AC parameters and dynamics are end-state concerns (D-6/D-24/D-44),
not permission to implement them in P1. All GB 132 kV remains in scope (D-60).

ASSUMPTION (Q-O-1): Every schema, identity, temporal-selection and hashing policy
below is a proposed engineering default requiring adoption through the
[companion questionnaire](../owner-questionnaire.md). It is not a new owner decision.
The D-31 inventory still has to establish obtainable assets, parameters and rights;
this ADR cannot resolve a data gap by inventing equipment or accepting equivalents.
No application code or optimisation experiment is authorised.

## Proposed decision

Use typed domain records, permanent internal entity identities and immutable,
content-addressed releases. Store validated releases relationally and export the
same identities to columnar data. Keep a materialised release manifest plus dated
changes, rather than making every run reconstruct a global event log.

| Record | Identity and version boundary |
|---|---|
| Entity | Opaque UUID, type, owning namespace and lineage; UUID never embeds mutable name, owner, geography or row position |
| External alias | Source namespace, source ID, effective range, evidence and reviewed mapping to entity; duplicates/uncertain matches stay unresolved |
| Entity revision | Entity ID, typed attributes, effective range, source publication/revision time where known, acquisition time, immutable revision hash |
| Dataset release | Manifest hash over schema, sorted revision/blob references, calendar, transformations, licence evidence and lineage; human label is only an alias |
| Network release | Immutable inventory plus effective-dated connectivity, ratings, availability and operating proxies; references exact component releases |
| Scenario revision | Original versioned TOML and canonical semantic hash, with an immutable resolved reference lock |
| Run / attempt | Unique operational run ID and attempt ID; input identity and execution identity separately hashed; equal inputs do not collapse two experiments |

Physical substation, electrical bus, voltage level, circuit, transformer, switch,
generator, storage, demand attachment and interconnector are distinct types. Two
circuits between the same buses are separate entities. Geography and display
coordinates do not determine electrical connectivity. Distinguish physical branch
orientation from a reporting sign and distinguish a market area from an electrical
island. Preserve mappings to source, solver row/column index and output entity ID.

A rename keeps the entity ID. A replacement, split or merger receives new IDs with
explicit predecessor/successor links; a correction appends a revision. Retired IDs
are never reused. Synthetic/equivalenced assets carry their own IDs, derivations,
aggregation membership and lost capabilities; they never count as observed assets.
Scenario-created entities live in a scenario namespace. Common curated datasets
may be shared read-only; private releases and aliases carry an ownership scope.
Content hashes are integrity references, not authorisation tokens (ADR 0005).

Prefer a bus/branch P1 projection with attachment records and explicit grid-interface
mappings. Leave typed, schema-versioned extension tables for phase connections,
unbalanced feeders, reactive parameters and dynamic models. Reject an unsupported
fidelity request; do not fill missing AC/dynamic fields with plausible defaults.
This avoids both a transmission-only identity scheme and a giant untyped attribute
bag. Future CIM/CGMES import maps mRIDs as external aliases; it does not redefine
canonical IDs or assume rights to a particular CIM dataset.

## Effective topology and information available at the time

Use half-open UTC effective intervals `[from, to)`, including an explicit open end.
Commissioning, retirement, switch state, endpoints, ratings, outages, memberships,
interconnector trading regimes and security proxies are dated independently.
Topology-changing events split the solve horizon; no rolling window may apply one
factorisation through an unmodelled topology transition. Shorter-than-step events
require an explicit resampling policy or an unsupported-resolution error.

Keep **effective time**, **source publication/revision time**, and **acquisition
time** separate. Imported evidence is append-only; each knowledge revision contains
its own effective range, so a later correction need not share the original range.
For a fixed knowledge cut, the resolver must produce one unambiguous value per
entity/attribute/time, or report a conflict/gap. Reject overlapping selections,
missing endpoints, impossible voltage connections and unexplained demand islands.
Do not enforce a single non-overlap constraint across all historical corrections.

A reconstruction may use a later-corrected release, declared explicitly. An as-of
forecast backtest selects only revisions provably published by its knowledge cut
and records when that cut advances across forecast origins. A present-day download
of a revised historic table is not proof of what was available then. Unknown
publication history blocks an as-of claim; acquisition time never fabricates it.
Freeze holdout inputs before calibration, as PR #4 proposes.

Canonical delivery intervals use UTC starts and durations. Version the mapping to
Europe/London and Europe/Dublin settlement dates/periods, including DST and leap
years. Preserve original timestamp conventions and missing/imputed/resampled masks.
Daily GBP/EUR FX and rule transitions use their documented effective-time policy;
local settlement period numbers are never globally unique keys.

## Versioned TOML and reproducible resolution

The following is **illustrative proposed syntax**, with deliberately non-resolving
hash placeholders; it is neither an implemented schema nor a real dataset:

```toml
schema_version = "1"
name = "historic-weather-replay"

[data]
network_manifest = "sha256:<64 lowercase hex digits>"
input_manifest = "sha256:<64 lowercase hex digits>"
patch_manifests = []

[time]
start = "2024-01-01T00:00:00Z"
end = "2025-01-01T00:00:00Z"
step_seconds = 1800
knowledge_mode = "reconstruction"

[market]
rules_manifest = "sha256:<64 lowercase hex digits>"

[execution]
profile_manifest = "sha256:<64 lowercase hex digits>"
```

The referenced manifests contain exact weather/demand/outage/cost/FX/proxy releases,
network fidelity, ordered patches and market module versions (ADR 0006). An execution
profile pins solver/options, seeds, rolling policy and checkpoint schema (ADRs
0003/0005). No run may resolve `latest`, a branch name or an unversioned URL. Authoring
tools may resolve a human alias once, then write the exact references and lock for
review. The API reports the resolution and rejects unavailable/unauthorised blobs;
it never substitutes a newer release. The same typed schema serves UI, CLI and SDK.

Patches reference a base hash and entity IDs, carry effective dates and express
checked changes with expected prior revision hashes. Ordered patch application is
part of scenario identity; conflicting edits fail. UI edits create another TOML
revision, leaving completed runs intact. Do not allow a script or executable path
inside scenario data. User-selected input locations resolve through the authorised
catalogue rather than arbitrary API filesystem reads.

Preserve original TOML bytes for human provenance. Hash its validated semantic form
after explicit default expansion, unit conversion and reference resolution, using a
schema-specific deterministic CBOR encoding [S1] and SHA-256. Specify map order,
array ordering, UTC/UUID normalisation and exact numeric representation; decimal
coefficients are normalised without lossy floating-point parsing. Reject duplicate
keys, unknown executable options, NaN/infinity and unsupported schema versions.
Comments/key order do not change identity; changed defaults or ordered patches do.
Store the canonicalisation version, original file hash and resolved lock hash.
Use maintained TOML/CBOR/hash libraries, with cross-client golden vectors after
baseline, rather than inventing a new parser or hashing protocol.

Schema migrations are explicit, deterministic transforms producing a new revision
and before/after hashes. Existing manifests remain readable through their versioned
reader or retained compatible build. An engine/build/solver/platform change creates
a different execution identity even when scenario semantics are unchanged. Numerical
agreement follows PR #4 NFR-2; neither UUIDs nor hashes promise bitwise MILP reruns.

## Provenance, licences and retention

Each release records origin URL, source version, acquisition/publication evidence,
byte checksum, units, transformations, source aliases, rights evidence and transitive
parents. Admission tags are exactly `open-commercial` or `research-only` (D-56).
Unknown permission remains research-only; ODbL/OSM/share-alike ancestors are rejected.
Derived restricted tables are restricted too. Raw/derived restricted data stays out
of git and public artifacts; source scripts and legally publishable manifests may
be committed. Check manifests themselves for sensitive fields before publication.

Commercial execution checks all ancestors, not just the final release's label.
Later clearance appends evidence and a new admissible release; historical evidence
is not rewritten. Pin legally retainable inputs alongside results; if retention
rights prevent this, preserve evidence/scripts and disclose rerun limitations.
Detailed garbage collection belongs to ADR 0004.

## Alternatives and consequences

| Alternative | Benefit | Reason to decline as the default |
|---|---|---|
| Mutable latest-state relational model | Simple CRUD and small ingest code | Undetectable rerun drift, no robust historical commissioning/knowledge cut |
| Full event sourcing | Complete audit and flexible replay | Replay/schema-evolution complexity; deterministic materialisation still needed by solvers |
| CIM/CGMES as the entire internal model | Mature interchange vocabulary and external ecosystem | Larger scope than P1, extra mapping for scenario/market/provenance semantics; retain as a future adapter |
| Graph database as authority | Natural traversal | Adds a second transactional authority; version/rights/time joins remain essential; in-memory numerical graph still needed |
| Immutable releases plus dated typed revisions (chosen) | Stable joins, bounded loading, auditable corrections and reruns | More storage and explicit conflict resolution; semantic canonicalisation must be specified and tested |

The choice follows D-19's canonical scenarios and PR #4's dated-asset/as-of criteria,
not a measured performance claim. Reuse PostgreSQL range/index primitives [S2] and
existing columnar formats; do not import prior-art OSM datasets contrary to D-56.

Before acceptance: review D-31 mappings/coverage with no scope reduction; adopt
Q-O-1; reconcile these contracts with PR #4's final schema. After baseline:
demonstrate renamed/split assets, corrected historic topology, unavailable as-of
vintages, DST/leap calendars, patch conflicts, client round-trip hashes and licence
lineage rejection. Measure full-network load/build costs on the runtime target.

## Evidence

- S1 — RFC 8949 §4.2, deterministic CBOR: https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2
- S2 — PostgreSQL 18 ranges: https://www.postgresql.org/docs/18/rangetypes.html
- Project evidence and exact PR #4 source revisions: [topology context](../topology.md#design-context-and-evidence).

These sources establish available mechanisms and required traceability; they do not
establish obtainable network coverage, an accepted schema or successful benchmarks.
