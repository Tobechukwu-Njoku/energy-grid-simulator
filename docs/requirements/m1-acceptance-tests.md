# M1 acceptance-test specification

Status: specification for implementation; **no tests executed and no M1 pass claimed**.
Date: 2026-11-18. Source snapshot: main `79a2d2e` (merged into this branch).
Stable test IDs are append-only: do not renumber or reuse a retired AT-M1 ID.

## Authority and exit boundary

The M1 row in [the roadmap](06-roadmap.md) and the staged allocation in
[the requirements](04-requirements.md#staged-p1-delivery-and-acceptance) allocate
**FR-1, FR-2, FR-3, FR-4, FR-5, FR-19, FR-21 inputs, NFR-1/NFR-2 foundations,
and NFR-12.AC1/AC2**: ten requirement IDs. Decisions are authoritative in
[the decisions log](03-decisions-log.md); D-72 accepts the baseline despite older
“proposed/not accepted” wording retained in the requirements. Accepted ADRs
[0002](../architecture/adr/0002-network-and-scenario-data-model.md),
[0001](../architecture/adr/0001-repo-layout-and-ci-gates.md) and
[0005](../architecture/adr/0005-execution-topology.md) refine the contracts.
The [M1 data inventory](../research/m1-data-inventory.md) identifies candidate
sources and deficits; [PR #58 deficit candidates](../research/m1-deficit-sources.md) supply additional evidence/proposals, not owner decisions. A source listing or cached download is not ingest acceptance.

ASSUMPTION: M1 proves ingest, network representation and runner activation, with
runtime obligations staged as in the roadmap. FR-4's actual client/UI round trip,
FR-5's optimised non-stationary schedules, NFR-1's solver reruns and NFR-2's
MILP/price certificates remain later evidence, not waived criteria (Q-R-1).
No whole-requirement completion is inferred from foundation tests. Full FR-2
release evidence remains M5; synthetic/reduced success cannot satisfy D-32/D-60.

Every case records level and **Light CI: Yes/No**. “Yes” means small CPU-only
parsing, data transformation or arithmetic using an isolated temporary filesystem:
no simulator dispatch, solver, database, heavy build, full-network load or benchmark
runs in the coding/light container (D-65). Implementation is Rust under D-73;
this document supplies no code. “No” cases require the separate confirmed D-65
target; label them integration/e2e unless measuring a benchmark. M1 has no new
throughput target, and no case below is a benchmark-on-D-65-target. Applicable B1
benchmarks under NFR-4/NFR-5 remain separate gates on that target.

A case passes only when **every** stated assertion and negative control passes.
An expected rejection means a non-success result with the affected entity/field,
interval and reason where applicable, no accepted release/run, and no silent
substitution or committed partial output. Missing prerequisites are **BLOCKED**,
not PASS or a successful skip. Keep per-variant outcomes. Record test ID, source
revision, fixture hashes/rights, implementation SHA, environment, expected/actual
values, residuals and diagnostics in the evidence bundle; redact credentials.

## Fixtures and numeric oracles

Committed data fixtures, both synthetic and rights-verified real-source extracts,
live only under `spikes/data/samples/*`, with registered `open-commercial`
provenance in the sample manifest. Tests read these bounded samples there;
crate-local fixture payload directories are not allowed by
`xtask/policy/repository.toml`. Larger inputs stay outside git on the D-65 target.
Exact allow-listed metadata paths are for metadata only. Generate ephemeral
negative payloads in temporary storage; do not loosen CI to admit them.

ASSUMPTION: [Q-R-4] NI 275/110 kV cells in F-NET and AT-M1-7 are synthetic
coverage probes pending confirmation of D-32's Ireland voltage denominator.
They do not silently expand FR-2.AC1 or establish an accepted FR-2.AC3 denominator.

All payloads are newly authored synthetic data or separately rights-verified
`open-commercial` snapshots. Every actual fixture dataset has a D-56 provenance
manifest; synthetic origin and derivation are explicit. Synthetic negative tests
may *simulate* research-only, unknown or prohibited licence metadata without using
any data from those sources. Such metadata is an admission-policy input, not the
licence of the synthetic test payload. No raw research-only data or restricted
derivatives may enter git, test artifacts or redistributed evidence.

| Fixture | Definition and source relationship |
|---|---|
| F-NET | Small synthetic bus/branch system: GB 400/275/132 kV, ROI 400/220/110 kV and NI 275/110 kV; two parallel circuits, transformer, generator, storage and GSP attachments; permanent UUIDs and separate source aliases. Seven external link stubs cover France (IFA/IFA2/ElecLink), Netherlands, Belgium, Norway and Denmark; three internal stubs cover Moyle, EWIC and Greenlink. This is no claim of observed topology. |
| F-TIME | Synthetic 2023 and leap-2024 UTC grids, pinned IANA tzdata 2024b (`SRC-35`; archive SHA-256 `70e754db126a8d0db3d16d6b4cb5f7ec1e04d5f261255e4558a67fe92d39e550`, canonical acquisition-plan pin), Europe/London and Europe/Dublin mappings; synthetic operating/knowledge revisions and versioned holiday/calendar tables. |
| F-SERIES | Small synthetic demand, embedded wind/solar, weather, outages, availability, fuel/carbon, FX, constraint/curtailment and proxy rows, including nulls, revisions and deliberate errors. Real-source adapter probes may use rights-verified bounded extracts from `SRC-22`, `SRC-29`, `SRC-30`, `SRC-33`, `SRC-43`, `SRC-45`, `SRC-46`; pin exact bytes, terms and checksums, never live URLs. |
| F-SCENARIO | Versioned synthetic TOML, pinned network/input/rules/calendar/transform/proxy/FX locks, execution profile, patch and schema-migration pairs; independently reviewed semantic hash vectors for ADR 0002 deterministic CBOR/SHA-256. |
| F-RIGHTS | Entirely synthetic lineage graphs, licence records, download stubs and distinctive harmless payload markers; no real restricted extracts. |
| F-TARGET | Confirmed target hardware/OS/access inventory, trusted dispatch policy and test repository, ephemeral rootless jobs, isolated scratch storage, dummy credential canaries and a benign job marker. No real keys/licences or host-escape exploit. |
| F-EVIDENCE | Rights-verified, dated inventory/commissioning/rule evidence and expected mapping tables, acquired before real-data claims. Only synthetic/open-commercial payloads are used here. Missing rights, denominators or dates block the affected evidence variant. A research-only source can remain a declared project data gap; it cannot be copied into this suite to bypass the fixture restriction. |

`SRC-*` references use the inventory's identifiers. “Research-only / unverified”
is descriptive inventory prose: an admission tag must be exactly `research-only`.
Do not assume open rights from a publisher, website footer or software licence.
Full-source or database adapter runs move to the target even when an excerpt test
can run in light CI. No online download or account credentials are needed in light CI.

Use an independent oracle, not the production converter/resolver to produce its
own expected values. IDs, counts, dates, masks, tags, selections and hashes compare
exactly. For numerical checks, use the baseline NFR-2 limits:

- Power/limit tolerance: `epsP(S) = 0.001 + 0.000001 × S` MW.
- Energy recurrence: `epsE(S, dt) = dt × epsP(S)` MWh, with `dt` in hours.
  Check every interval separately; accumulated-ledger tolerance is the sum of
  its interval energy tolerances, and missing intervals never disappear in a sum.
- Currency totals: `epsC(Cref) = 0.01 + 0.000001 × abs(Cref)` in the stated
  currency. Do not compare GBP directly with EUR. No price/solver tolerance is
  exercised merely by checking a data ledger.

ASSUMPTION: For these small data/arithmetic tests, local scale `S` is the maximum
absolute MW term in the checked balance/recurrence or the applicable branch rating,
never a whole-grid total. Non-aligned events/durations require an explicit,
versioned rounding/resampling policy or rejection. These operational definitions
and accumulated/currency-ledger tolerances require Q-R-3 disposition; they do not amend NFR-2 or
approve an unspecified model rounding rule.

ASSUMPTION: The synthetic reconstruction FX policy uses GBP per EUR, effective at
00:00 UTC, carrying the last published valid rate across non-working days; the
as-of variant may select only a rate published by its knowledge cut. This is a
test policy, not a decision establishing real ECB timing or a project-wide FX
convention (Q-R-2; relates to Q-N-1/Q-N-2).

## Network and inventory tests

### AT-M1-1 — Typed entities, aliases, units and connectivity

- **Covers:** FR-1.AC1; D-6, D-11, D-12, D-18, D-24, D-32, D-44, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-NET with independently enumerated entity/alias tables,
  known unit bases and a valid transformer between different voltage levels.
- **Steps:** Ingest; reorder source rows and rename a station; reingest; resolve
  generator/GSP/storage/interconnector endpoints and market ownership.
- **Pass/fail:** Exact expected entity counts, units, endpoints and GB/SEM membership;
  NI and ROI remain distinguishable geographically within SEM. Renames/reordering
  preserve UUIDs; parallel circuits remain distinct; all demand has an explained
  attachment. Foreign boundaries remain external, not additional modelled markets.

### AT-M1-2 — Invalid graph and ambiguous mapping rejection

- **Covers:** FR-1.AC1; D-11, D-18, D-32, D-44, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Independent F-NET variants: dangling endpoint, duplicate
  entity UUID, ambiguous BMU/GSP alias, unexplained disconnected demand, incompatible
  voltage connection without a transformer, zero/unknown rating and unlabelled unit base.
- **Steps:** Ingest each invalid variant; repeat an island variant with an explicit
  diagnostic island declaration and a missing-parameter variant marked as a draft.
- **Pass/fail:** Every invalid variant is rejected for accepted-use admission.
  Declared islands/missing parameters can be retained in a diagnostic draft with
  exact gap IDs, but cannot gain a complete/ready label or an unlimited rating.
  A clear diagnostic preserves the original source row/alias and reason.
  Separately admit a diagnostic proxy variant with a 100 MW post-fault rating:
  `ASSUMPTION: [Q-P-01]` continuous estimate `0.88 × 100 = 88 MW`, inferred
  method/source and gap recorded. It must never count as a verified continuous
  rating or satisfy baseline rating acceptance (see AT-M1-7/29).

### AT-M1-3 — Effective asset and market-rule transitions

- **Covers:** FR-1.AC2/AC5; D-11, D-18, D-24, D-26, D-32, D-44, D-72.
- **Level:** integration. **Light CI:** Yes (bounded evidence only).
- **Preconditions/fixtures:** F-NET/F-TIME synthetic transitions; F-EVIDENCE must
  supply independent effective instants for Brexit regimes, Viking and Greenlink.
  No date is inferred from the first flow observation or a current reference list.
- **Steps:** Resolve at the last admissible interval before, at, and first interval
  after each commissioning, retirement and rule
  transitions using `[from,to)`; request an earlier holdout with a later-only
  snapshot; repeat against each evidence-backed transition.
- **Pass/fail:** Exact eligible assets/rules at each instant; no pre-commissioning
  capacity or later rule leakage. Later-only snapshots fail earlier-year validation.
  MHHS/REMA fields may be dated metadata but cannot enable P2/P3 functionality.
  Missing transition evidence blocks its real-date variant; synthetic success
  does not verify an actual commissioning or Brexit effective date.

### AT-M1-4 — Knowledge cuts, corrections and reconstruction labels

- **Covers:** FR-1.AC2/AC4/AC5, FR-3.AC2/AC4; D-18, D-23, D-38, D-44, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-TIME: value 100 effective at T, published T−2 h;
  correction 80 effective at T, published T+1 h; independently recorded acquisition
  times; a third row with unknown publication time and conflicting-overlap variant.
- **Steps:** Select at cuts T−1 h and T+2 h; select reconstruction; try as-of with
  unknown vintage; change acquisition time alone; introduce ambiguous selections.
- **Pass/fail:** As-of values are exactly 100 then 80; no later-publication leakage.
  Unknown vintage permits a labelled reconstruction but prevents an as-of claim.
  Acquisition time is never substituted for publication time; conflicts are rejected.
  Effective, publication/revision and acquisition timestamps remain distinct.

### AT-M1-5 — Directional interconnector losses and endpoint ledgers

- **Covers:** FR-1.AC3, NFR-2.AC1/AC2/AC3 (arithmetic); D-11, D-26, D-28,
  D-32, D-38, D-40, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic 30-min GB→SEM sending 100 MW, converter
  loss 1 MW plus cable loss 2 MW, receiving 97 MW; reverse-direction sending
  80 MW, converter 1 MW plus cable 1 MW, receiving 78 MW. Explicit metering boundary
  excludes onshore branch losses; idle-flow loss is zero in this synthetic model.
- **Steps:** Convert each directional record into signed endpoint injections and
  MWh; reconcile the two markets jointly; inject negative loss, receiving>sending,
  duplicate onshore loss, and missing directional coefficient variants.
- **Pass/fail:** Forward withdrawals/receipts/loss are 50/48.5/1.5 MWh; reverse
  40/39/1 MWh. Sending equals receiving plus losses within epsP/epsE for each
  interval; joint ledger counts each loss once. Invalid variants reject or remain
  explicitly unready, never receive an invented zero/default physical loss.
  Add a separate three-term variant `P_loss = P_standby + k1*abs(P) + k2*P²`
  (Q-P-03): at zero transfer use synthetic `P_standby=2 MW` and declare GB as
  the supplying endpoint. GB withdraws 2 MW (1 MWh over 30 min), SEM receives
  zero, and the loss ledger records exactly 2 MW/1 MWh once, including in the
  joint balance. The schema preserves all three coefficients and endpoint;
  missing endpoint or duplicated standby charges reject. The earlier zero-idle
  variant remains a distinct explicit model, never a universal loss assumption.
  These arbitrary synthetic coefficients establish no real-link loss factors.

### AT-M1-6 — Daily FX and declared external-boundary policy

- **Covers:** FR-1.AC3/AC4; D-11, D-26, D-38, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SERIES with synthetic rates 0.80 and 0.90 GBP per EUR
  on adjacent UTC days; weekend gap with a prior published rate; external boundary
  flows and policy locks; Q-R-2 assumption explicitly selected.
- **Steps:** Value 10 MWh at 100 EUR/MWh on each day; process midnight and weekend
  intervals; run reconstruction and publication-cut variants; omit rate/policy,
  reverse quote units, and request a freely optimised FX field in negative variants.
- **Pass/fail:** Native EUR totals remain 1,000 per interval; GBP equivalents are
  800 and 900 within epsC. Weekend carries only the last eligible published rate,
  with its age/imputation policy recorded; no prior valid rate means rejection.
  Reconstruction declares fixed historical external flows; scenario alternatives
  declare flow/price policy and hash. Missing policy, inconsistent quote or free
  FX variable rejects; FX is exogenous and cross-market totals are not counted twice.

### AT-M1-7 — Coverage denominator and operator/voltage accounting

- **Covers:** FR-2.AC1/AC2; D-30, D-31, D-32, D-44, D-56, D-60, D-71, D-72.
- **Level:** integration. **Light CI:** Yes (synthetic inventory).
- **Preconditions/fixtures:** Synthetic dated inventory with 20 nodes and 10 circuits
  in one cell, all required GB/ROI voltage cells, NI 275/110 kV and named TO/OFTO
  ownership; all 14 DNO licence areas (12 E&W, two Scottish) have explicit tracking.
  Scottish 132 kV belongs to transmission inventory without double counting.
- **Steps:** Ingest all then remove two circuits; produce operator × voltage ×
  asset-class coverage for topology, ratings, impedance and generator/GSP mapping;
  change a denominator to unknown and add an unsupported “not applicable” cell.
- **Pass/fail:** Exact 20/20 nodes and 8/10 circuits (80%) in the altered cell;
  unknown denominator is UNKNOWN, never 100%. Missing DNO areas fail complete
  coverage. Documented zero/non-applicable cells retain evidence and are not
  silently dropped. Every inferred parameter has source, method and gap status.
  With AT-M1-2's Q-P-01 variant, report 88 MW as inferred only: the verified-rating
  coverage numerator does not increase. Synthetic NI cells follow Q-R-4.

### AT-M1-8 — Equivalents, missing assets and full-network claim gate

- **Covers:** FR-2.AC2/AC3/AC4; D-30, D-31, D-32, D-44, D-56, D-60, D-71, D-72.
- **Level:** integration. **Light CI:** Yes (claim-gate logic only).
- **Preconditions/fixtures:** F-NET/F-EVIDENCE acceptance metadata, synthetic full
  denominator including secured boundaries, reduced aggregation map and a fixture
  representing each of accepted equivalent, unaccepted equivalent and missing asset.
- **Steps:** Request full-network status for each variant; remove an acceptance
  reference, retained-constraint list or lost-capability declaration; repeat with
  unknown denominator and unresolved R-12. Check real-evidence readiness separately.
- **Pass/fail:** A full claim requires 100% of inventory nodes/circuits/published
  secured boundaries modelled or individually owner-accepted as equivalents.
  Missing assets stay missing; approving a gap is not coverage or reduced-release
  permission. Reduced fixtures expose membership, retained constraints and lost
  capabilities; absent declarations reject admission. Unknown denominator, missing
  evidence or unresolved R-12 blocks full release. Simulated approval metadata
  tests the gate only; actual equivalents require real owner decision references.

## Input and calendar tests

### AT-M1-9 — Series metadata and integrity

- **Covers:** FR-3.AC1, FR-19.AC1; D-14, D-18, D-23, D-26, D-30, D-35, D-44,
  D-55, D-56, D-57, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SERIES with units, timezone/interval convention,
  source/version, publication/revision where available, acquisition time, byte
  checksum, terms evidence, exact licence tag and missing-data mask.
- **Steps:** Ingest; tamper with one byte; independently remove each mandatory
  field; make publication unknown; supply malformed units and an invalid status tag.
- **Pass/fail:** Exact metadata and checksums preserved; tampering, required-field
  omissions and invalid units/tags reject. Unknown publication is explicit and
  restricts the claim to reconstruction; a mask entry exists for every interval.
  “research-only / unverified” is rejected as a literal enum value.

### AT-M1-10 — UTC years, leap day and both DST transitions

- **Covers:** FR-3.AC1, FR-5.AC3; D-18, D-21, D-35, D-44, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** F-TIME, independently pinned tzdata version and expected
  2023/2024 London/Dublin mapping tables; synthetic rows labelled with source period,
  local offset/fold and UTC interval. Versioned holiday tables have explicit region.
- **Steps:** Construct half-hour UTC years; round-trip through local settlement
  identifiers; process 2023-03-26, 2023-10-29, 2024-03-31 and 2024-10-27; remove a fold/offset discriminator;
  change calendar/tzdata or a holiday table version and recompute identity.
- **Pass/fail:** Exactly 17,520 and 17,568 contiguous unique UTC starts, leap day
  48 periods, both local spring days 46 and autumn days 50. Repeated autumn civil
  times remain distinct intervals with source IDs; no deduplication. Ambiguous
  timestamps reject. Use UTC offsets (+01:00 vs +00:00) and fold/UTC identity,
  never `is_dst`, to distinguish repeated hours. Assert London and Dublin map
  the same tested civil times to the same UTC offsets on both sides of each
  transition even though Dublin's negative-DST `is_dst` convention is inverted;
  an implementation that uses that flag as the offset must fail.
  Calendar/holiday/tzdata versions are pinned and affect the
  input identity. Synthetic holidays prove mapping only, not actual Irish dates.

### AT-M1-11 — Operational input classes and vintage preservation

- **Covers:** FR-3.AC2; D-18, D-23, D-26, D-30, D-44, D-55, D-56, D-57, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SERIES rows for REMIT outage notices/revisions,
  generator availability, fuel/carbon costs, constraint and curtailment outturn,
  operating restrictions and NESO-style day-ahead boundary limits.
- **Steps:** Ingest each class; resolve delivery intervals and knowledge cuts;
  withdraw/revise an outage; omit one required input class and its publication vintage.
- **Pass/fail:** Effective outage/availability and revised values match the oracle
  without destroying the earlier vintage. All input classes are accounted for;
  missing classes/unknown vintages have explicit coverage gaps. Absent data is not
  zero outage/zero cost/zero curtailment. Missing vintage blocks forecast-validation
  claims while allowing declared reconstruction. Annual ETYS fallback is flagged
  per interval and cannot masquerade as day-ahead coverage (see AT-M1-26).

### AT-M1-12 — Embedded generation, pumping and station-load boundaries

- **Covers:** FR-3.AC3, NFR-2.AC1/AC2/AC3 (arithmetic); D-18, D-26, D-28,
  D-30, D-40, D-44, D-56, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic half-hour with ND 100 MW, embedded solar
  10 MW, embedded wind 5 MW, pumping 4 MW and station load 2 MW; explicitly defined
  TSD-like total 106 MW for this fixture, no exports or additional losses. Gross
  underlying demand excludes pumping/station load; their separate ledger includes them.
- **Steps:** Build underlying/embedded/net demand and the separate pumping/station
  ledger; integrate MWh; subtract pumping from ND or embedded generation twice in
  negative variants; supply uncertain estimates and missing Irish components.
- **Pass/fail:** Gross underlying demand is 115 MW, net 100 MW (50 MWh); the
  distinct extended system demand is 106 MW (53 MWh). All balances
  satisfy epsP/epsE. Double subtraction rejects; uncertain embedded estimates are
  labelled. Missing Irish embedded series remains a gap, never inferred as zero.
  Real TSD/station-load/import/export/loss boundaries need separate sourced
  reconciliation; this synthetic equation does not settle them.

### AT-M1-13 — Nodal allocation and weather replay of embedded output

- **Covers:** FR-1.AC1, FR-3.AC3/AC4; D-14, D-18, D-26, D-32, D-35, D-44, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic gross 115 MW split over GSPs with weights
  0.6/0.4, initial total embedded output 15 MW and explicit generator/interface
  mapping; a second weather profile changes embedded total to 20 MW and changes
  a separate transmission renewable profile. Mark spatial allocation synthetic.
- **Steps:** Allocate both weather versions; reconcile nodal sums; repeat with
  weights summing to 0.9, unmapped GSP and duplicated embedded generator attachment.
- **Pass/fail:** Gross GSP values 69/46 MW; aggregate net demand changes 100→95 MW
  while gross stays 115 MW. Transmission and embedded renewable profiles both
  change and are independently identified. Negative variants reject, with no
  implicit renormalisation or generation counted twice; epsP applies per balance.

### AT-M1-14 — Missing intervals, resampling and unsupported coverage

- **Covers:** FR-3.AC1/AC4, FR-5.AC3; D-14, D-18, D-21, D-23, D-35, D-44, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic hourly 60/120 MW values, explicit
  piecewise-constant policy, missing hour and source local-time ambiguity variants.
- **Steps:** Expand to half-hours and minutes; try no policy; remove a source hour;
  test versioned interpolation with its contributing-source IDs and as-of cut.
- **Pass/fail:** Half-hour values are 60/60/120/120 MW, total 180 MWh within epsE;
  finer output is marked resampled/synthetic with original cadence and transformation
  hash. Missing rows remain masked or explicitly imputed, never silently zero-filled.
  Interpolation using a later-published contributing point cannot support an as-of
  claim. Undeclared interpolation, ambiguous time or unsupported coverage rejects.

### AT-M1-15 — Weather and cost input transformations

- **Covers:** FR-3.AC2/AC4; D-14, D-18, D-23, D-26, D-30, D-35, D-44, D-56, D-57, D-72.
- **Level:** integration. **Light CI:** Yes (small synthetic arrays).
- **Preconditions/fixtures:** Synthetic hourly capacity factors 0.25/0.5 for a
  20 MW plant; 100 MW gross demand; fuel 30 GBP/MWh-thermal, heat rate 2,
  carbon 50 GBP/t, emissions 0.2 t/MWh-thermal and VOM 5 GBP/MWh-electric.
  Versioned transformation/spatial weights and monthly fuel-source cadence metadata.
- **Steps:** Convert renewable MW and SRMC vectors; resample to half-hours;
  omit heat rate, use a currency/unit mismatch and request 2024 MÉRA coverage.
- **Pass/fail:** Renewable outputs 5/10 MW and two-hour energy 15 MWh within
  accumulated epsE; cost is exactly 85 GBP/MWh-electric for these integer inputs.
  Source cadence, units, conversion and resampling
  remain visible; projected DESNZ values are not labelled historical spot outturn.
  Missing parameters/currency mismatch reject ready status. MÉRA ends in 2019;
  no fabricated 2024 data or ERA5 acquisition claim is permitted.

## Scenario and resolution tests

### AT-M1-16 — Complete canonical TOML and field-level validation

- **Covers:** FR-4.AC1/AC2 (ingest); D-8, D-19, D-20, D-35, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SCENARIO with schema/canonicalisation version,
  geography/asset snapshot, input hashes, time range/resolution, market-rule version,
  fidelity, solver profile, rolling settings, seed and knowledge policy.
- **Steps:** Parse and resolve; omit each required direct/reference field; supply
  unsupported schema, duplicate TOML key, unresolved `latest`, altered blob checksum,
  unavailable hash and unknown/executable option variants.
- **Pass/fail:** Complete input resolves to exact immutable references and typed
  values. Every negative variant fails with the offending field/reference; no newer
  blob/default replaces an unavailable lock. Parsing permits no executable payload.
  A schema default is valid only if explicit, versioned and included in identity.

### AT-M1-17 — Semantic identity golden vectors

- **Covers:** FR-4.AC1/AC2 (semantic identity), NFR-1.AC1 (foundation);
  D-19, D-20, D-28, D-33, D-35, D-70, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SCENARIO golden vectors reviewed independently of
  the production serialiser, with exact expected canonical bytes and SHA-256 digests.
  ADR 0002 defines deterministic CBOR, UUID/UTC normalisation, exact money and
  binary64 non-money unit normalisation.
- **Steps:** Compare comments/key reorder, `0.1`/`0.10`, signed zero and equivalent
  authored units; change each semantic dependency, including calendar, FX,
  transformation, rules, network, proxy and canonicalisation versions. Test NaN,
  infinity and numeric-boundary cases using the schema's valid encodings.
- **Pass/fail:** Equivalent forms produce byte-identical canonical values and
  expected digests while original authored bytes remain archived separately.
  Every semantic dependency change produces its specified distinct identity.
  Invalid/non-finite numbers reject. Never generate expected digests with the
  serialiser under test; cross-client/platform execution remains staged evidence.

### AT-M1-18 — Patches, migrations and shared round-trip contract

- **Covers:** FR-4.AC1/AC2 (data contract), NFR-1.AC1; D-8, D-19, D-20,
  D-28, D-35, D-70, D-72.
- **Level:** integration. **Light CI:** Yes (serialisation contract only).
- **Preconditions/fixtures:** F-SCENARIO pinned base and ordered patches with expected
  prior hashes, old/new schema pair and archived original run lock; shared-schema
  request/response fixture representing eventual CLI/UI exchange.
- **Steps:** Parse→export→parse through the shared contract; reorder disjoint
  patches; supply conflicting edits/wrong base; migrate to a new schema revision;
  attempt to overwrite the archived original.
- **Pass/fail:** Round trip preserves semantic hash exactly. Patch order belongs
  to identity even where final physical values coincide. Conflict/base mismatch
  rejects. Migration produces a new revision with before/after hashes, leaving
  original bytes, lock and run reference unchanged. Actual CLI→UI→CLI interaction
  must later repeat this oracle at M3/M4; a contract fixture is not that e2e pass.

### AT-M1-19 — Interval scaling of constant energy and currency ledgers

- **Covers:** FR-5.AC1 (input/ledger foundation), NFR-2.AC1/AC2/AC3;
  D-18, D-21, D-26, D-28, D-35, D-40, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** Independent 24-hour constant 60 MW input and
  50 native currency/MWh in separately labelled GBP and EUR variants; no dispatch.
- **Steps:** Materialise 30/15/5/1-minute grids and integrate supplied values;
  test omitted resolution and an intentionally unscaled hours-to-minutes mutation.
- **Pass/fail:** Exact counts 48/96/288/1,440; default is 30 minutes. All variants
  total 1,440 MWh and 72,000 in their own currency within accumulated epsE and epsC,
  with each interval independently within its epsE. Unscaled integration fails.
  Currency is not implicitly converted. Actual solved market runs remain required
  later to complete FR-5.AC1.

### AT-M1-20 — Non-stationary physical durations and ramp units

- **Covers:** FR-5.AC2 (input foundation), NFR-2.AC2/AC3;
  D-18, D-21, D-26, D-28, D-35, D-40, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic event profile at all four resolutions:
  ramp 2 MW/min, initial output 10 MW; minimum-up/down durations 1.5/2 hours;
  known initial elapsed state, availability drop at 12:00 UTC, reserve obligation
  5 MW for 30 minutes and storage 10 MW charging for one hour at efficiency 0.9.
- **Steps:** Resolve physical inputs and interval coefficients; independently
  compute first-step ramp bounds, durations and energy; inspect availability
  change and reserve/storage records; mutate MW/min and physical hours to steps.
- **Pass/fail:** Ramp increments 60/30/10/2 MW; minimum-up counts 3/6/18/90 and
  minimum-down 4/8/24/120. Charge energy is 10 MWh, state increase 9 MWh; reserve
  duration is 0.5 h (2.5 MWh if fully deployed), never multiplied by step count
  twice. Availability drop takes effect exactly at 12:00 and initial elapsed state
  is retained. Every unscaled mutation fails the independent oracle. Non-aligned
  event/duration variants reject unless an explicit rounding policy is pinned;
  optimised starts/stops/binding-reserve schedules are M2 evidence, not this pass.

### AT-M1-21 — Source cadence and incompatible time/product requests

- **Covers:** FR-5.AC3, FR-3.AC4; D-18, D-21, D-35, D-44, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-SCENARIO/F-SERIES hourly product observations,
  half-hour source data, declared resampling and the schema's explicit supported
  product/resolution table, with an intentionally unsupported pair.
- **Steps:** Request finer resolution with/without policy; combine sources with
  incompatible delivery intervals or product definitions; request unsupported pair.
- **Pass/fail:** Permitted expansion retains product, source cadence and synthetic/
  resampled flag per output. Missing policy and unsupported pair reject explicitly;
  no invented one-minute observation, annual performance claim, or silent treatment
  of GB MID as an auction price. Supported combinations come from the versioned
  contract, not a new assumption of market-product support.

## Data rights and proxy-input tests

### AT-M1-22 — Complete provenance and unknown-rights handling

- **Covers:** FR-19.AC1, FR-3.AC1; D-17, D-30, D-31, D-32, D-56, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-RIGHTS records with source/version, acquisition date,
  checksum, terms capture/hash, attribution obligations (including explicit none),
  exact admission tag, transformations and transitive parents.
- **Steps:** Admit complete record; omit each required field; use unknown commercial
  permission, noncanonical tag, checksum mismatch, missing parent and lineage cycle.
- **Pass/fail:** Complete lineage remains traversable to every source; missing
  fields/parents, cycles or checksum mismatch reject admission. Unknown permission
  is classified research-only, never open-commercial. Only the two exact admission
  tags are accepted; evidence and licence history remain immutable/versioned.

### AT-M1-23 — Transitive commercial and share-alike admission gates

- **Covers:** FR-19.AC2/AC3; D-17, D-30, D-31, D-32, D-56, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-RIGHTS policy simulations: all-open chain; publicly
  downloadable research-only parent; apparently open child with restricted
  grandparent; ODbL/OSM/share-alike ancestor; open-source tool with unlicensed dataset;
  independently evidenced clearance linked to the canonical pre-commercial ledger.
- **Steps:** Request research and commercial admission for each chain; append a
  clearance revision; repeat without evidence and with merely renamed child tag.
- **Pass/fail:** Research admits publicly downloadable research-only simulations
  with restrictions intact; commercial rejects every uncleared restricted ancestor.
  OSM/ODbL/share-alike rejects in both modes, including hidden ancestors. Tool
  licence alone never establishes data/test-use rights. Valid clearance produces
  a new traceable eligible release; relabelling alone rejects, and historical
  restricted evidence is preserved. No actual research-only payload is required.

### AT-M1-24 — Restricted-output exclusion from git and redistribution

- **Covers:** FR-19.AC2/AC3; D-17, D-30, D-56, D-72.
- **Level:** integration. **Light CI:** Yes (tiny disposable repository).
- **Preconditions/fixtures:** F-RIGHTS harmless unique marker payloads with simulated
  research-only labels, one derived extract, allowed download script/manifest and
  synthetic open-commercial control. Use a temporary repository, not project history.
- **Steps:** Run research-mode ingest to an external cache; attempt to stage raw
  marker and derived extract or include them in export/test artifacts; retain
  scripts and non-sensitive lineage evidence. Scan the git range with
  `cargo xtask provenance --base <base-sha> --head <head-sha>`; separately scan
  the resulting export package with its lineage admission checks. In the
  disposable repository, commit then delete a marker to exercise range detection.
- **Pass/fail:** Raw/derived restricted simulations stay outside tracked/public
  outputs; attempted inclusion blocks publication. Allowed scripts/manifests carry
  no restricted rows or secrets. Open-commercial control is admitted with required
  attribution at a registered `spikes/data/samples/*` path; the same control
  outside the allow-list is rejected. Export's lineage checks agree with git admission; missing external
  cache is a declared unavailable input, not a fabricated downloadable fixture.

### AT-M1-25 — Typed proxy definitions, cut-sets and missing-rule status

- **Covers:** FR-21.AC1 (inputs), FR-2.AC2; D-26, D-32, D-40, D-44,
  D-55, D-59, D-71, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic boundary/flowgate, regional minimum-unit,
  inertia-floor and Irish SNSP/minimum-conventional-unit definitions with source,
  dates, units, applicability, inference/missing status and explicit memberships.
  Boundary cut-set has signed circuit coefficients +1/−1.
- **Steps:** Ingest; independently evaluate the cut-set at supplied branch flows
  +70/+20 MW; reverse one branch's stored orientation with corresponding coefficient
  change; omit a circuit/membership/unit; offer GIS line or SNSP outturn as a rule.
- **Pass/fail:** Boundary expression is 50 MW in both orientation encodings within
  epsP. Every group refers to known typed entities and records applicability and
  inference status. Missing cut-set/membership is a declared readiness gap; GIS
  geometry is not an electrical cut-set and observed SNSP is not a ceiling rule.
  Unsourced numeric floors/unit counts never become verified operational limits.
  Enforcement/commitment/sensitivity/slack assertions in AC2–AC4 remain M2.

### AT-M1-26 — Interval-vintage proxy selection and fallback coverage

- **Covers:** FR-21.AC1, FR-3.AC2 (inputs); D-18, D-26, D-40, D-44,
  D-55, D-59, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Four synthetic half-hours: day-ahead limits 100/80 MW
  with publication vintages, one annual ETYS fallback 120 MW and one missing limit;
  optional later correction; autumn local timestamps with distinct fold/source IDs.
- **Steps:** Select reconstruction and as-of vintages; count source/fallback/missing
  coverage; cross the autumn fold; remove the declared fallback policy.
- **Pass/fail:** Exact selected values/statuses and coverage: 2/4 day-ahead, 1/4
  fallback, 1/4 missing; readiness does not call this 100% day-ahead coverage.
  Future correction is excluded before its publication cut. Repeated local hours
  retain both UTC intervals. Fallback is named/versioned with interval mask;
  undeclared substitution rejects. Missing limit is not infinity, zero or nameplate
  capacity, and annual capabilities do not establish outage-season accuracy.

## Identity and numerical foundations

### AT-M1-27 — Input and execution identities remain distinct

- **Covers:** NFR-1.AC1/AC3 (foundation), FR-4.AC1; D-19, D-20, D-28,
  D-33, D-35, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic complete manifest: canonical scenario,
  input/model hashes, engine commit, dependency locks, solver/build versions,
  parameters, seed, threads, platform, pricing method and timestamp/knowledge policy.
- **Steps:** Create two run/attempt IDs from pinned inputs; change one execution
  field at a time (including solver, threads, platform or limit); separately change
  one input hash; omit each mandatory manifest field.
- **Pass/fail:** Same input hash for identical inputs, distinct operational run IDs;
  execution changes produce distinct execution identities. Changing an input also
  changes input identity and its referenced execution manifest. All mandatory
  fields are present; missing fields reject ready status. Solver entries are
  declared configuration only: no solver reproducibility claim is made at M1.

### AT-M1-28 — Deterministic ingest reruns without mutable dependencies

- **Covers:** NFR-1.AC1/AC2/AC3 (ingest foundation), NFR-2.AC1/AC2/AC3;
  D-19, D-20, D-28, D-33, D-35, D-40, D-70, D-72.
- **Level:** integration. **Light CI:** Yes.
- **Preconditions/fixtures:** F-NET/F-SERIES/F-SCENARIO pinned releases and
  deterministic transform configuration; clean temporary output directories.
- **Steps:** Ingest twice in the same recorded environment; independently compare
  selected entities, metadata, masks, ledger values and content hashes; make the
  external catalogue alias point to another release and rerun the original lock.
- **Pass/fail:** Original locked inputs and output semantic identities remain equal;
  values satisfy epsP/epsE/epsC and exact fields match. Mutable alias changes do not
  affect a pinned rerun; unavailable original bytes produce BLOCKED/rejection,
  never substitution. This is not an NFR-1 CPU MILP rerun or bit-identical schedule
  promise; that evidence requires deterministic limits/threads/seeds or completed
  certificates at M2/M3.

### AT-M1-29 — Original-unit conversion and ledger reconciliation

- **Covers:** NFR-2.AC1/AC2/AC3 (foundations), FR-1.AC1/AC3, FR-3.AC3;
  D-18, D-26, D-28, D-32, D-33, D-38, D-40, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic 100 MVA base: 0.5 pu active power and
  10% impedance on that base; independently equivalent MW/pu representations;
  original-unit sending/receiving/demand values from AT-M1-5/AT-M1-12.
  Add 17.424 ohms at 132 kV, 100 MVA: independent `Z_base=132²/100=174.24 ohms`,
  hence `Z_pu=17.424/174.24=0.1`; retain original ohms/kV/MVA. Compare with
  independently correctly rounded 0.1 pu within a documented binary64 rounding
  bound (at most two ULP), and reject missing kV/MVA or a factor-of-100 base error.
  Preserve AT-M1-2's inferred 88 MW rating and Q-P-01 method through conversion;
  neither unit conversion nor rounding may relabel it as verified.
- **Steps:** Convert and restore units; recompute endpoint/energy ledgers independently
  in MW/MWh; inject a factor-of-100 percentage error, wrong base and rounded output.
- **Pass/fail:** Expected 50 MW and the percentage conversion's independently
  correctly rounded canonical binary64 value of 0.1 pu compare exactly; the
  ohms conversion uses the two-ULP bound above. Original units/base are retained. Physical residuals
  satisfy epsP/epsE per row, not only aggregate cancellation. Every erroneous-base/
  percentage variant fails; unknown bases reject. Test evidence retains unrounded
  numeric values, never derives acceptance from rounded presentation strings.

### AT-M1-30 — Tolerance boundaries and independent negative controls

- **Covers:** NFR-2.AC1/AC2/AC3 (foundations); D-26, D-28, D-33, D-40, D-72.
- **Level:** unit. **Light CI:** Yes.
- **Preconditions/fixtures:** Synthetic residual checks at local scale 100 MW;
  30-minute interval; exact arithmetic oracle and currency reference 1,000 GBP.
  Q-R-3 scale/aggregation assumption is recorded. Boundary comparison uses
  `abs(residual) <= threshold` in binary64. Construct the 1.00× residual by copying
  the exact binary64 threshold passed to the production comparator, not by
  subtracting MW totals. Independently verify threshold computation against the
  exact decimal oracle with its correctly rounded binary64 result. Use decimal
  oracle values for 0.99×/1.01×, then round once to binary64; test both signs.
- **Steps:** Evaluate residuals at 0.99, 1.00 and 1.01 times each tolerance; check a
  second row with the opposite sign; evaluate a short-interval energy case with
  dt=1/60 h and a multi-interval ledger including an intentionally missing interval.
- **Pass/fail:** epsP=0.0011 MW, half-hour epsE=0.00055 MWh and epsC=0.011 GBP;
  0.99× and copied-binary64-threshold residuals pass; 1.01× fails for each. Opposite errors cannot cancel row failures.
  Minute tolerance scales by 1/60, not 0.5. Missing interval prevents complete
  ledger acceptance. Record raw residual, local scale, dt and threshold. No MILP
  integrality/objective certificate or price-method pass is implied (AC4–AC6 later).

## Target-runner activation tests

### AT-M1-31 — Exact-SHA admission and untrusted dispatch denial

- **Covers:** NFR-12.AC1/AC2; D-17, D-22, D-28, D-43, D-53, D-54, D-65, D-72.
- **Pending policy reference:** PR #75 proposes the D-77 review-evidence exception.
  It is not in the decisions log or gate policy at merged main `79a2d2e`.
  Do not assume that exception is active; test the accepted gate policy and add
  the review-evidence-only variant only after that policy is accepted and merged.
- **Level:** integration. **Light CI:** No — actual D-65 admission boundary required.
- **Preconditions/fixtures:** F-TARGET with coordinator-approved exact test head/base
  and trusted policy, independent review/adversarial triage evidence, audit log and
  benign job-start marker. Test control plane is authorised separately from PR code.
- **Steps:** Try unapproved SHA, fork dispatch, changed head/base after review,
  stale/spoofed review evidence, PR-edited admission policy, missing required runner
  and pull_request_target PR-code execution path. Then submit the approved exact SHA.
- **Pass/fail:** Every unauthorised path rejects before executing job code: zero
  job-start markers and no target allocation, with explicit audit reason. Changing
  PR policy cannot weaken trusted-base admission. Required missing runner blocks,
  without coding-container or optional-GPU substitution. Only approved exact SHA
  starts the benign job; workflow/configuration inspection confirms no
  pull_request_target job executes PR code. Local policy mocks cannot pass this case.

### AT-M1-32 — Ephemeral rootless execution and resource isolation

- **Covers:** NFR-12.AC1/AC2; D-17, D-22, D-28, D-43, D-65, D-72.
- **Level:** integration. **Light CI:** No — actual D-65 execution configuration required.
- **Preconditions/fixtures:** F-TARGET benign approved job, rootless runtime/UID-map
  configuration, isolated workspace and host scratch paths outside its allowed mounts.
- **Steps:** Inspect host account, user namespace, effective capabilities, mounts
  and runtime flags; try Docker-socket bind mount, privileged mode and a forbidden
  scratch-path mount via the normal job admission interface; start a second job.
- **Pass/fail:** Host runner is unprivileged; job cannot acquire host-root privileges
  (container UID 0 is acceptable only when mapped unprivileged/rootless). No host
  Docker socket, privileged container or forbidden mount is admitted. Job workspaces
  are isolated; a second job cannot read the first's private marker. Container and
  scratch state are removed after completion/cancellation. No escape exploit or
  real secret access is needed; inspection evidence identifies the exact config.

### AT-M1-33 — Credential canaries and artifact/manifest exclusion

- **Covers:** NFR-12.AC1/AC2, NFR-1.AC1 (safe provenance);
  D-17, D-19, D-22, D-28, D-43, D-65, D-72.
- **Level:** integration. **Light CI:** No — actual D-65 credential boundary required.
- **Preconditions/fixtures:** F-TARGET dummy canaries at representative host-only
  licence/SSH/runner-credential locations and a harmless token-shaped marker supplied
  to a redaction-only fixture. Actual runner token remains host-only and unread.
- **Steps:** From approved job, attempt ordinary reads/enumeration of canary paths
  and inspect environment, mounts and allowed outputs; exercise artifact/manifest
  sanitisation on the harmless marker and inspect logs/manifests/packages.
- **Pass/fail:** Every host canary read is denied or path absent; no host licence,
  SSH/provider credential or runner token is exposed by environment/mounts. Limited
  runner credential stays inaccessible to jobs. Zero canary/marker occurrences in
  published logs, manifests or artifacts; required non-sensitive provenance remains.
  Accidental marker disclosure fails and blocks activation; evidence records the
  denial without printing credential-like payloads. No real secrets are test inputs.

### AT-M1-34 — First-heavy-job interlock and activation record

- **Covers:** NFR-12.AC1/AC2; D-17, D-22, D-28, D-43, D-65, D-72.
- **Level:** e2e. **Light CI:** No — actual D-65 workflow/runner boundary required.
- **Preconditions/fixtures:** F-TARGET activation record bound to target/configuration
  identity and trusted policy; AT-M1-31–33 evidence; benign marker standing in for
  heavy-job launch. No build/database/benchmark payload is needed to prove admission.
- **Steps:** Attempt B1 heavy dispatch before evidence exists, with each security
  case failed/missing, and after target isolation configuration changes. Then attach
  passing evidence for the exact target/configuration and dispatch the benign probe.
- **Pass/fail:** All missing/failed/stale-evidence paths keep heavy B1 disabled,
  with zero probe starts. Only fully matched passing evidence permits target probe
  start. Record first permitted launch after security evidence timestamps; the
  coding container is never a fallback. No GPU adapter is enabled by CPU evidence;
  enabling one requires actual device/toolkit/isolation verification and its gate.
  Actual heavy work, when subsequently authorised, runs only on the qualified target.

## Requirement coverage matrix

“Covered” means specified tests, not executed/passed or whole-requirement sign-off.
The matrix accounts for all ten M1 requirement IDs and every AC, distinguishing
M1 input/foundation evidence from later behaviour. AT references below mean AT-M1.

| Requirement | M1 tests | AC coverage and later evidence |
|---|---|---|
| FR-1 | AT-M1-1–6, AT-M1-13, AT-M1-29 | AC1: 1/2/13/29; AC2/AC5: 3/4; AC3: 5/6/29 arithmetic; AC4: 4/6 policies. Solved market endpoint/economic balances additionally M2. Real dates/losses blocked by gaps 4/6. |
| FR-2 | AT-M1-7/8/25 | AC1/AC2: 7 plus proxy memberships in 25; AC3/AC4: 8 claim gating and reduced disclosure. Dated real full-inventory release gate is M5; small fixtures cannot satisfy it. Gap 4. |
| FR-3 | AT-M1-4, AT-M1-9–15, AT-M1-21/22/26/29 | AC1: 9/10/14/22; AC2: 4/11/15/26; AC3: 12/13/29; AC4: 4/13/14/15/21. Real inputs still require gap-5 closure; time-vintage gaps cannot be labelled forecast validated. |
| FR-4 | AT-M1-16–18/27 | AC1: 16/17/18/27; AC2: 16 invalid fields, 17 semantic vectors, 18 migration/round-trip contract. Actual CLI/UI round trip at M3/M4; gap 1. |
| FR-5 | AT-M1-10/14/19–21 | AC1: 19 input-ledger foundation; AC2: 20 physical input scaling; AC3: 10/14/21 labels and rejection. Optimised constant/non-stationary cases at M2; gaps 1/3. |
| FR-19 | AT-M1-9/22–24 | AC1: 9/22; AC2: 23/24 research/commercial distinction and exclusion; AC3: 23/24 prohibited ancestors/tool-vs-data rights. Real clearance cannot be simulated for production sign-off. |
| FR-21 inputs | AT-M1-25/26 | AC1 input inventory, signed memberships, effective/knowledge dates and fallback masks. AC2–AC4 enforcement, binding commitment, slack/dual/repair outcomes at M2b (UC interaction M2a); gap 7. |
| NFR-1 foundations | AT-M1-17/18/27/28/33 | AC1 manifest/identity and safe metadata; AC2 ingest-repeatability only; AC3 records numerical rather than bit-identical solver claims. Actual deterministic CPU solver reruns/certificates at M2/M3; gap 1. |
| NFR-2 foundations | AT-M1-5/12/19/20/28–30 | AC1/AC2/AC3 independent power/energy/currency checks in original units. AC1 integrality/optimisation objective and AC4–AC6 MILP/reference/price certificates at M2a–c; gaps 1/3. |
| NFR-12.AC1/AC2 | AT-M1-31–34 | Exact-SHA denials, real rootless isolation, canary credential protection and pre-B1 interlock. Target unavailable until gap 8 closes. AC3 application exposure/access remains M3, with applicable private-access policy retained. |

Other P1 requirements remain at their allocated milestones: FR-6–18, FR-20
and NFR-3–11. In particular,
FR-7/FR-8 network solves, FR-9 storage optimisation, FR-10 pricing, FR-12 historical
accuracy and NFR-4/NFR-5 performance are not silently claimed by ingest arithmetic.
M0 policy/licence/language gates continue to apply to M1 changes; this is not a
replacement specification for NFR-3/8/10 or a relaxation of D-28 reviews.

## Flagged items, assumptions and exit reporting

Eight flagged items follow. **Two** are not fully testable as written without a
more precise policy/oracle (gaps 2 and 3); one is a milestone sequencing clarification
(gap 1), and five are evidence/environment blockers (gaps 4–8). Data scarcity does
not itself make a requirement logically untestable or authorise a scope reduction.

| Gap | Requirement / issue | Classification and disposition |
|---|---|---|
| GAP-M1-1 | FR-4.AC2, FR-5.AC1/AC2, NFR-1.AC2, NFR-2.AC1/AC4–AC6 need components delivered after M1 | Sequencing. ASSUMPTION: use the staged input/foundation boundary above; contract tests do not complete later behaviour. Q-R-1 asks to confirm exact exit allocation. Track deferred evidence explicitly in the M1 report. |
| GAP-M1-2 | FR-1.AC4 daily UTC FX does not fix quote direction, fixing/publication time, holiday/weekend/leading-gap or as-of carry policy | **Underspecified oracle / untestable in full as written.** Declaration can be checked; correctness of an undeclared effective policy cannot. ASSUMPTION: AT-M1-6 uses a declared synthetic policy only; Q-R-2 requests a versioned real policy. No future ECB fixing may leak into as-of selection. |
| GAP-M1-3 | FR-5.AC1/AC2 duration rounding and NFR-2.AC1/AC2 “local MW scale”/multi-interval tolerance are not operationally defined | **Underspecified oracle / untestable at all boundaries as written.** ASSUMPTION: use the explicit local-scale, per-interval, accumulation and non-aligned-event rules above for draft tests only. Q-R-3 requests disposition; no test may use whole-network scale to hide local errors or silently round physical durations. |
| GAP-M1-4 | FR-1.AC1/AC2/AC5; FR-2.AC1–AC4 dated commissioned inventory, denominators, ratings, BMU/GSP mappings and accepted equivalents | Evidence blocked. PR #58 Candidate 1.1 supplies ETYS 2024 Appendix B as a historical candidate; 1.3 proposes November 2024 LTDS. These need commissioned-asset verification. Remaining gaps include missing DNO denominators, Irish zero ratings and crosswalk deficits. Exercise the Q-P-01 inferred-rating path (AT-M1-2/7) and 132 kV ohms-to-pu path (AT-M1-29); obtain dated evidence and per-equivalent owner acceptance under D-71. AT-M1-3/7/8 synthetic passes prove only logic; full inventory acceptance stays M5. No new reduced-scope decision assumed (Q-N-3 already answered). |
| GAP-M1-5 | FR-3 inputs: Irish embedded series, GSP allocation/boundaries, weather, costs, operational vintages, calendar and historic rules | Evidence blocked. Inventory records no ERA5 bytes, MÉRA ending 2019, missing SRMC parameters, unpinned rules/calendars and incomplete Irish decomposition. PR #58 Candidate 3.1 identifies Sheffield Solar PV_Live; 3.3 supplies research-only ESB/SEAI endpoints, Q-P-02 interpolation remains proposed. Candidate 6.2 / Q-P-05 cover heat-rate/VOM proxies; Q-P-08 covers reconstruction interpolation versus as-of step-holding with imputation masks. Pin rights-verified fixtures, transforms and known vintages; declare missing inputs. Raw research-only snapshots cannot become fixtures here. Acquired SEMO prices and a current code hub do not resolve these other deficits. |
| GAP-M1-6 | FR-1.AC3 actual directional physical losses and metering conventions | Evidence blocked. All ten links' directional coefficients are MISSING; TLM candidate is unacquired and settlement multipliers cannot silently replace physical loss functions. Q-P-03 proposes standby/linear/quadratic parameters; AT-M1-5 tests non-zero standby attribution separately. Acquire per-link/date evidence; synthetic AT-M1-5 coefficients are not defaults. D-71 acquisition/provenance obligation remains, without inventing percentages. |
| GAP-M1-7 | FR-21.AC1 / FR-3.AC2 electrical cut-sets, regional unit groups, inertia/SNSP rules and time-varying vintages | Evidence blocked. GIS boundaries provide no signed electrical cut-set, SNSP outturn supplies no ceiling. PR #58 Candidate 2.1 cites the [EirGrid/SONI 2024 report](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf), §§3.3/5.1: 75% SNSP and 23,000 MW·s inertia throughout 2024; enduring minimum 3 NI + 4 ROI units from 7 April 2024. This is research-only evidence, not a suite fixture. For AT-M1-25/26 real-date variants, record the external report hash/citation, map memberships and test selection immediately before/at/after the evidenced effective instant; the exact intra-day instant and pre-April trial schedule remain UNVERIFIED. Only independently authored synthetic analogues run in this suite; real evidence claims remain blocked until permitted evidence and date precision are available. Acquire operational rule evidence and map memberships, with exact fallback coverage; unsupported rows remain gaps under D-55/D-71. |
| GAP-M1-8 | NFR-12.AC1/AC2 actual separate target identity/access/configuration | Environment blocked. Q-N-7/Q-H-7 target details remain pending; D-48 sizing is not target identity. AT-M1-31–34 cannot be satisfied by mocks, policy prose or a coding-container run. Keep heavy B1 disabled until target provision and denial/isolation evidence pass (D-65). |

Do not invent owner answers to close these gaps. Questions are recorded in
[the questionnaire, section R](02-questionnaire.md#r-m1-acceptance-oracles-and-staged-evidence).
Existing decisions/questions already cover data acquisition (D-71/Q-N-3), target
provisioning (D-65/Q-N-7) and pre-commercial clearance (D-56); do not reopen them.
The acceptance report must list 34 case outcomes/variants, the ten requirement
coverage rows with partial/deferred status, all eight gap dispositions, immutable
fixture/evidence references, and any failed or blocked tests. None are currently
asserted passed. M1 readiness requires its own test execution and resolved
applicable evidence/policy blockers; it is not a full-network validated-MVP claim.
