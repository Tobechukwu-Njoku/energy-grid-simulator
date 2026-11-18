# Independent review of PR #52 — M1 data-source inventory

- Date: 2026-11-14 UTC.
- Artifact kind: research/data-source verification under D-51.
- Author role/provider: inventory research / Google.
- Reviewer role/provider: independent source verification / OpenAI.
- Reviewed head: `e7e992e3f213a8d8645bb5e58ea1d7592c55a908`.
- Compared base and evidence main: `f42ba33848cee8c484f69a48b87afe285dc01f50`.
- Reviewed file: `docs/research/m1-data-inventory.md`; line numbers below refer
  exclusively to that pinned head, not a later revision.
- Authority: D-51, D-56, D-60, D-70, D-72; FR-1–5/19/21,
  NFR-1/2 and the M1 row of the roadmap.
- Verdict: **must-fix**. Correct the unsafe clearance labels, wrong dataset
  claims and unsupported completion assertions before relying on this inventory.
- Mutation boundary: this report and STATUS only; the inventory and decisions
  log are unchanged.

## Method, evidence and limits

Read [AGENTS.md](../../../AGENTS.md), the [review brief](../REVIEW-BRIEF.md),
[decisions](../03-decisions-log.md), [roadmap](../06-roadmap.md),
[requirements](../04-requirements.md), accepted
[ADR 0002](../../architecture/adr/0002-network-and-scenario-data-model.md),
and the existing research notes, spike reports and manifests. In this tree
the gathering directory is `spikes/data-gathering/`; there is no separate
root `data-gathering/`.

- Made unauthenticated GET requests, following redirects, to **all 77 distinct
  external Markdown-link URLs** in the inventory. Appendix A records each one.
  Results: 69 HTTP 200, three HTTP 403 and five HTTP 404. Six of those 200s are
  CKAN login pages; two SEMO 200s contain an empty error envelope rather than
  the claimed data. HTTP success is not dataset verification.
- Checked all **15 distinct repository-relative link targets** against the
  reviewed Git tree: all exist. A valid target does not establish that its
  contents support the associated claim.
- Independently matched sizes and SHA-256 for all **30 network/Irish manifest
  files** and **both SEMO price archives** in the existing external raw cache.
  This establishes the identity of those cached captures, not perpetual live
  availability or an open licence.
- Inspected live CKAN/Opendatasoft metadata, Elexon's OpenAPI and dated payloads,
  the cached Elexon system-price payload, the full cached interconnector panel,
  and the prior independently checked Elexon/SEMO coverage reports.
  Did **not** rehash/recount all 4.16 GiB / 166.9 million Elexon records.
- No accounts, authenticated exports, paid services, heavy builds, databases or
  target-runner jobs were used. Gated payloads, unpublished rules, unavailable
  permissions and unsourced numeric assumptions remain **UNVERIFIED**.
  Research classification is not legal advice or a publisher permission grant.

Repository evidence:
[network manifest](../../../spikes/data-gathering/network-ie/manifest.toml),
[network coverage](../../../spikes/data-gathering/network-ie/COVERAGE.md),
[Elexon manifest](../../../spikes/data-gathering/elexon/manifest.toml),
[Elexon coverage](../../../spikes/data-gathering/elexon/COVERAGE.md),
[SEMO manifest](../../../spikes/data-gathering/semo-2024/manifest.toml),
[SEMO coverage](../../../spikes/data-gathering/semo-2024/COVERAGE.md),
[spike report](../../../spikes/data/REPORT.md),
[source/download catalogue](../../../spikes/data/sources.tsv),
[GB notes](../../research/data-sources-gb.md),
[Irish/weather notes](../../research/data-sources-ie-weather.md), and
[DNO inventory](../../research/dno-132kv-inventory.md).

## Findings and corrections

1. **[high] Unverified commercial permission is labelled cleared.**
   Concerns inventory §§2.2, 4.1, 5.1, 6.1–6.3 and 6.5
   (lines 332–334, 464–465, 519–520, 563–564, 575–576, 587–600,
   636–637 and 724–725); FR-19.AC1.

   A public/statutory document is not itself a commercial reuse licence.
   Unsupported `open-commercial` labels include BSC text, operator loss
   specifications, Citizens Information holiday text, the unverified BoE
   series and the combined CfD/RO source. The Irish holiday entry borrows a
   data.gov.ie CC BY label without identifying any corresponding dataset.
   The repository has no root LICENSE at the reviewed head, so “Project open
   source repository licence” does not identify an existing grant either.
   [BoE terms](https://www.bankofengland.co.uk/legal) expressly exclude selected
   third-party FX series; the inventory acknowledges that the selected series
   has not been cleared but nevertheless tags it open.
   [Met Éireann's copyright notice](https://www.met.ie/about-us/copyright-notice)
   restricts general website reuse; a specific open-data grant must be linked
   to the particular MÉRA/station resource, not inferred from another product.

   **Correction:** retain `research-only` for externally sourced material
   whose applicable commercial grant is unknown. Record exact dataset-level
   grant/evidence and obligations before upgrading. Separate internally authored
   specifications/arithmetic from acquired datasets; do not invent a project
   licence. For Met Éireann, check the specific open-data product rather than
   claiming either that all its data are open or that all are restricted.
   Citizens Information, LCCC/Ofgem and selected BoE-series clearance remain
   **UNVERIFIED** here.

2. **[high] Several source URLs do not yield the claimed dataset.**
   Concerns §§1.1, 1.5, 2.1–2.2, 3.1, 4.3 and 6.2–6.4; Appendix A.

   Six NESO `/dataset/...` links redirect to login, despite public package
   APIs/resources being available. The demand slug
   `historic-demand-data-2024` is not the verified package identifier.
   The operability report, I-SEM code and UK ETS result URLs return 404.
   The undated Elexon system-prices route returns 404, and the Elexon host root
   is not a dataset endpoint. The daily SEMO register URL and reports-directory
   URL return HTTP 200 with `{"errorMessage":"","code":0}`, not XML/a
   browsable archive. The named historical daily register remains available
   in the hash-verified local capture, but was **not** retrievable from its
   cited live URL during this review.

   **Correction:** cite public
   [CKAN package_show](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data)
   and exact resource URLs from the committed manifests. The actual demand
   resource is
   [demanddata_2024.csv](https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv).
   Use
   [dated system prices](https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/2024-01-01)
   only for their actual price fields. Replace dead rule/carbon links with
   verified, version-specific documents or mark their locations unresolved.
   Do not substitute a guessed URL. Preserve the distinction between a
   verified cached capture and a currently successful download.

3. **[high] FUELHH does not supply the claimed embedded solar/wind decomposition.**
   Concerns §3.2, lines 404–414 and §5 line 778; FR-3.AC3/AC4.

   The
   [dated FUELHH response](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream?settlementDateFrom=2024-01-01&settlementDateTo=2024-01-01)
   contains fuel/interconnector categories, not separate embedded solar and
   embedded wind estimates. There is no SOLAR category. The committed annual
   coverage lists BIOMASS, CCGT, COAL, interconnectors, NPSHYD, NUCLEAR, OCGT,
   OIL, OTHER, PS and WIND, not the asserted embedded components. A WIND
   aggregate cannot establish a separate embedded-wind series.

   **Correction:** source GB embedded estimates from the explicit NESO
   historic-demand columns, with their methodology/uncertainty; identify the
   Irish equivalent separately. ECRs provide capacities, not interval output.
   Declare the net/underlying-demand equation, pumping, losses, interconnector
   treatment and GSP allocation, and test that embedded generation is not
   subtracted twice. Replace “100% downloaded” with component-level coverage;
   the complete underlying/GSP decomposition has not been established.

4. **[high] Transmission-loss-multiplier acquisition is falsely reported complete.**
   Concerns §4.3 lines 484–495 and §5 line 782; FR-1.AC3, FR-19.

   The cited URL returns 404. The correct dated system-prices endpoint has
   prices, NIV and adjustment volumes, not a delivering/offtaking BMU TLM
   panel. The cached `system_prices/2024-01-01.json` confirms this.
   None of the 12 gathered streams is identified as the claimed TLM dataset.

   **Correction:** remove the “included within settlement streams” and
   “downloaded for 2024” assertions. Identify and verify an actual TLM source,
   its zonal/BMU/production-consumption scope, effective dates and cadence.
   [Elexon's OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json)
   does expose a `transmissionLossMultiplier` in the settlement **stack**
   response, at `/balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}`;
   that is a candidate to investigate, not proof of complete TLM coverage
   or an already acquired dataset. Settlement allocation multipliers are not
   themselves measured physical network-loss outturns.

5. **[high] Directional physical loss models have not been sourced per link.**
   Concerns §4.1 lines 458–469 and §5 line 780; FR-1.AC3, NFR-2.

   Three operator home/operations pages do not establish dated, directional
   parameters for all ten links. “Typically 2.0%–4.5%” has no cited parameter
   table or units/validity basis. Neither the cited spike audit nor Irish
   source notes supplies the claimed complete loss-parameter set.
   Public regulatory filing is not a licence name.

   **Correction:** add a per-interconnector ledger for source/version,
   effective dates, direction, units, converter/cable losses, sending/receiving
   conventions and confidence. Mark absent coefficients **missing**, not
   “parameters documented”. Label any engineering approximation explicitly
   and retain its derivation/rights. Preserve power balance and avoid counting
   a link's losses again as national/branch losses.

6. **[high] Published proxy rules are confused with observations and have no
   dated parameter inventory.**
   Concerns §2.1 lines 301–324 and §5 lines 774–775; FR-21.AC1.

   SNSP observations do not identify a ceiling or a minimum-unit rule.
   The quoted GB national inertia level “~140 GVA·s, declining towards 100”
   has no supporting document/effective date and is not a safe fixed 2024
   input. The operability URL is dead. Neither a downloaded inertia-rule
   release nor a table of applicable regional unit groups is present.
   The real acquired NESO day-ahead file contains limits/forecast flows for
   constraint groups, not all the electrical branch cut-sets needed to apply
   those rows to the model.

   **Correction:** inventory each boundary's signed branch membership and
   time-varying limit/vintage; each inertia region/unit contribution; and each
   minimum-unit/SNSP rule with document, effective interval, units and missing
   status. Verify numeric thresholds from the applicable operator publication
   rather than using the narrative approximations. Mark the inertia,
   regional-unit, Irish minimum-unit and cut-set inputs **UNVERIFIED/missing**
   until that evidence exists. This does not require implementing M2 now.

7. **[high] A dated full-network/asset input is not established by later
   forecast snapshots or a portal count.**
   Concerns §1 lines 10/22, §§1.1–1.5 and §5 lines 768–773;
   FR-1.AC1/AC2/AC5, FR-2.AC2/AC3.

   ETYS 2025/26, DNO 2025/26 editions, November 2026 TEC and the current BMU
   register do not establish the 2024 commissioned topology/fleet.
   TYTFS WP/SV2024 are forecast study cases, not operating-history evidence.
   The inventory acknowledges some crosswalk work but still presents zero
   unsourced M1 inputs. Portal totals include lower voltages/equipment; they
   are not denominators for commissioned 132-kV circuits.

   **Correction:** distinguish current and historical model releases,
   effective dates and reconstruction/as-of information vintages. Track
   bus/branch/transformer/generator/GSP mappings and per-operator/voltage/asset
   numerator/denominator; unknown denominators remain incomplete.
   The gathered DNO areas are NGED's four plus SSEN SEPD, not all areas.
   There are **12 England/Wales DNO licence areas**, not “14 E&W areas”:
   the other two are Scottish distribution, while Scottish 132 kV is
   transmission. All 14 areas still need a coverage disposition under D-60.
   Also distinguish seven GB-to-external-system links from the three
   GB-to-Ireland links internal to the two-model geography; calling all ten
   “external” obscures the intended endpoint balances.
   Source the commissioning dates; 7,102 Greenlink records are not proof of
   7,102 commissioning tests. Their count/sparsity is verified, their causal
   interpretation is **UNVERIFIED**.

8. **[med] The Irish 2024 price source is the wrong lookback file and the
   holdout summary hides known gaps.**
   Concerns §6.4 lines 676–690 and §5 line 787; FR-3.AC4, FR-19.

   The linked `lookback_mkt.xlsx` contains 2018–2020 data, not 2024.
   The two 2024-capable pinned archives are correctly identified in the
   separate SEMO manifest:
   [SEMOpx October 2026 archive](https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip)
   and
   [SEMO imbalance archive](https://www.sem-o.com/documents/general-publications/Imbalance_Price_YTD_2020-2024.zip).
   Both live downloads resolve and match the cached captures.
   The report's “100%” status omits **61 missing 30-minute intervals** and
   **333 missing five-minute intervals in January–November**, in addition
   to **8,928 December five-minute intervals**. IDC interval identities are
   not recoverable from its aggregate sheet alone. IDA2/IDA3 cover only their
   auction windows, not every half-hour of the year.

   **Correction:** cite the actual archives and preserve those measured
   limitations. Say acquisition of the pinned files is complete, not all
   Irish holdout observations. Preserve the existing €108.91/MWh UTC-year
   DAM statistic and its limited agreement at published precision; do not
   confuse delivery-calendar and UTC-year selections.

9. **[med] “Already in repo / downloaded / verified” is repeatedly stronger
   than the linked evidence.**
   Concerns §5 lines 775–784 and §§5.1, 6.1–6.2.

   No complete proxy-rule release, directional-loss table, TLM capture,
   compiled holiday calendar or canonical scenario schema/checksum registry
   is present in the reviewed tree. The cited SEMO coverage does not discuss
   the Irish public-holiday schedule; the spike report does not verify the
   GOV.UK holiday logic. Requirements identify obligations; they are not
   acquired datasets or tested ingest implementations. ADR 0002 explicitly
   calls its TOML example illustrative, not an implemented schema or real
   dataset. Several fuel/subsidy “outlined/evaluated” claims point to notes
   that do not contain the asserted source evaluation.

   **Correction:** use separate evidence states: source identified, metadata
   inspected, raw acquired/hash-verified, content audited, parameters
   extracted and ingest validated. Link the exact record/artifact for each.
   The verified network/SEMO captures and genuine open samples may retain
   their measured status; unsupported “downloaded/verified” statements must
   be downgraded, not filled in by inference.

10. **[high] Fuel/carbon/subsidy coverage claims do not match the cited products.**
    Concerns §6.2 lines 569–603 and §5 line 785; FR-7.AC2 and
    ADR 0002's immutable cost inputs.

    The UK ETS URL returns 404 and identifies no EU ETS results source.
    UK auction observations would not establish a daily UK/EU price vector.
    DESNZ fossil-fuel assumptions are scenario assumptions, not a daily 2024
    NBP/TTF/ARA market history. The World Bank commodity page offers monthly
    and annual products; it does not establish the claimed combined daily
    series. LCCC's blocked portal cannot verify either its licence or an
    Ofgem RO buyout/ROC series. Blanket OGL does not automatically clear
    third-party exchange/auction data linked by government.

    **Correction:** split UKA/EUA, fuel products and CfD/RO into individually
    identified releases, original owners, currencies/units, frequencies and
    licence evidence. Add actual Ofgem RO and LCCC CfD references. Any
    monthly-to-daily expansion must remain labelled resampled; auction
    clearing prices must not silently become daily spot prices. Track heat
    rates, emissions factors, variable O&M and subsidy eligibility/mapping
    as unsourced/assumed where appropriate. Do not claim the resulting
    complete SRMC vector already exists.

11. **[med] Rule documents are neither version-pinned nor sufficient to prove
    all claimed market stages.**
    Concerns §2.2 lines 326–360; FR-1.AC2, FR-4, FR-21.

    The BSC link reaches a maintained code hub; Section Q is
    [Balancing Services Activities](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/section-q-balancing-mechanism-activities/),
    not the complete source of cashout/NIV/system-price rules.
    The I-SEM code URL is dead, and Part B alone does not identify the
    SEMOpx DAM/IDA/IDC auction rules claimed in the entry.
    JAO's terms-of-use page establishes reuse conditions, not a specific
    border's capacity-allocation rules or historic coupling regime.

    **Correction:** identify the dated BSC settlement/pricing provisions
    (including Section T as applicable), SEMO/SEMOpx trading procedures and
    border-specific allocation/market-coupling rules actually used. Record
    versions/effective dates and unresolved access. A current legal-code hub
    cannot serve as an immutable 2024 rules manifest.

12. **[high] “Deduplicating” the local fall-back hour would discard valid
    physical intervals.**
    Concerns §2.1 line 287 and §5.2 line 538; FR-3.AC1/AC4, NFR-2.

    The NESO note instructs deduplicating local duplicate hours into UTC.
    Repeated local times at the autumn transition can represent different
    real intervals and must survive. Canonical UTC alone cannot recover an
    offset/fold or settle ambiguous source rows. The Irish workbook has a
    `GMT Offset`; its four repeated local timestamps correspond to distinct
    UTC quarter-hours, as the prior network audit already explains.

    **Correction:** retain source interval identifiers, offset/fold and
    revisions, then map to UTC. Distinguish true revision duplicates from
    repeated civil times and separate constraint groups. If NESO's fields
    do not uniquely disambiguate a row, flag it rather than dropping or
    arbitrarily assigning it. Test 46/50 settlement periods, interval
    start/end conventions, leap day and conserved MWh.

13. **[med] Calendar sourcing and versioning are still incomplete.**
    Concerns §§5.1–5.3, lines 501–552; FR-3/4/5, NFR-1.

    The GOV.UK JSON resolves and presently includes 2024 for all three UK
    jurisdictions (observed range 2019–2028). The Irish webpage returns 403;
    this is **unverified access**, not evidence that it requires an account.
    It is not a machine-readable CC BY dataset just because data.gov.ie
    licenses other data. IANA's transition dates and the 17,568 UTC
    half-hours for 2024 are correct, but the inventory has not pinned a
    tzdata release or a holiday capture.

    **Correction:** separate civil bank/public holidays from market trading
    and settlement calendars. Record actual UK/Irish holiday evidence,
    exceptional holidays, tzdata version and country/market-specific clock
    conventions in the immutable input release. The ISO interval arithmetic
    is an internal derivation, not an acquired dataset/licence. Retain the
    correct DST/leap counts without calling all calendar work complete.

14. **[med] The supposedly exhaustive credentials matrix omits two listed
    gated sources and overstates approval outcomes.**
    Concerns §3 lines 732–746 and §§1.2, 6.3.

    CEDA/MIDAS account access and Renewables.ninja account/API-token access
    are required in their own entries but absent from the matrix.
    Independent UKPN/SPEN/NPg record probes return 403 while catalogue
    metadata is public; UKPN/SPEN tabular CC BY and NPg's custom Open licence
    are verified. Login gating does not turn those open licences into
    `research-only`, contrary to the generic §3.3 classification.
    UKPN's form redirects to login; SPEN's cited form redirects to the asset
    catalogue, not a verified request form.

    **Correction:** add optional CEDA/Ninja prerequisites or explicitly
    exclude those sources from the planned path. Separate account creation,
    row access, CIM approval and rights clearance; a login is not a guarantee
    of approval/export permission. Verify the current CDS-generated API
    configuration rather than prescribing an unverified legacy UID/key
    format. No authenticated CDS/ENTSO-E/CEDA/DNO/Ninja download was attempted
    here. Keep D-65 runner provisioning separate from source credentials and
    retain NFR-12.AC1/AC2 isolation/access-denial tests before any heavy job.
    Do not request or store credentials in git.

15. **[med] Scenario identity is an architectural design, not an existing
    ingest-ready schema/registry.**
    Concerns §6.1 lines 558–566 and §5 line 784; FR-4, NFR-1,
    ADR 0002.

    The conceptual specification is real, but there is no implemented
    canonical schema/checksum registry at this head. The entry's field list
    omits explicit knowledge mode, resolved release locks/patch identities,
    network fidelity, calendar/transform versions, cost/FX/proxy releases and
    provenance/rights lineage. Input and execution identities must not be
    flattened into an informal checksum of a TOML file.

    **Correction:** inventory these as internally produced M1 artifacts,
    not downloaded datasets. Follow ADR 0002's scenario/input/execution
    distinction, semantic canonicalisation, stable IDs, exact immutable
    references and effective/publication/acquisition times. List the missing
    schema, locks, migration/round-trip/hash vectors and NFR-2 fixtures.
    This review does not require implementing those artifacts in a research
    inventory PR.

16. **[low] Weather product format and cadence need narrower descriptions.**
    Concerns §6.3 lines 606–640.

    ERA5's currently attached CC BY 4.0 is independently confirmed in the
    [dataset catalogue API](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels).
    The request template and zero downloaded weather bytes are correctly
    disclosed. The MÉRA page confirms 1981–August 2019, but its sample
    download is explicitly GRIB 1, not evidence for the entry's blanket
    NetCDF/CSV description. Station observations are a different product.
    A generic “monthly updates” statement also fails to distinguish ERA5T
    near-real-time updates from consolidated ERA5.

    **Correction:** record format, cadence, publication vintage, licence and
    access per weather product. Retain the correct grid/bounding-box budget
    as an estimate, not an observed download size. MÉRA is not a 2024
    reanalysis substitute; station-data availability does not extend its
    reanalysis period.

17. **[med] Stable clearance IDs and procurement facts are misrepresented.**
    Concerns §1.4 lines 205–207 and §4 lines 750–758.

    The inventory renames **PC-5** as a DNO-specific item, but
    [the actual ledger](../TODO-pre-commercial.md) defines PC-5 as review of
    **every research-only dataset**. PC-3 is SEMO/SEMOpx, not an ENTSO-E
    clearance item. The “€1,040+/month” sentence generalises a particular
    EPEX API product to exchange power services. The GB research notes
    already distinguish that product from Nord Pool's package-specific fees.

    **Correction:** keep the ledger's stable IDs/meaning and reference
    PC-5 for unlisted research-only sources; create no owner decision by
    renaming an ID. Identify exact products/terms if discussing procurement,
    or simply say commercial price-feed/redistribution rights are unverified
    and may be paid. Do not present a universal quote or a commitment to
    purchase; D-57 does not make MID an auction price.

18. **[med] Summary statistics and the “zero unsourced inputs” conclusion are
    not auditable.**
    Concerns §1 lines 17–22 and §5 lines 764–790.

    There are **53 candidate-source blocks**, including duplicates,
    internally produced artifacts and combined multi-owner products. No
    stable-source register explains the claimed 48 unique datasets,
    25/23 licence split or 16 publishers. Those figures might use another
    grouping, but cannot be independently reconciled as written. In
    particular, a mixed licence entry is not one uniformly classified
    dataset. “Known source” currently includes generic portals and
    requirements documents even where the necessary field-level source
    has not been identified.

    **Correction:** assign stable source/resource IDs and count deduplicated
    resources with explicit counting rules and per-resource licence tags.
    Replace “0” with the explicit unsourced/unverified field-level backlog
    from findings 3–7, 10–15. Distinguish identified candidates from complete,
    correctly licensed, dated inputs. An honest incomplete inventory can
    support planning; the present readiness claims cannot.

## D-56 licence disposition

These checks cover the source families, not a new blanket grant over portals.

| Source family | Review disposition |
| --- | --- |
| ETYS publications; TYTFS network; Irish system workbook; SEMO/SEMOpx captures | Conservative `research-only` is appropriate while express file-level open designation remains unverified. Existing 30/2-file manifest identities match. |
| NESO portal boundary GIS, TEC, embedded register, constraint limits/flows and costs | Dataset metadata identifies NESO Open licence (not OGL); existing `open-commercial` tags supported. Use public metadata/resources rather than login redirects. |
| NGED tables/CIM; SSEN SEPD workbook/EQ | Existing open-commercial classifications supported by dataset-associated NGED Open / CC BY 4.0. Preserve third-party exclusions, attribution and modifications notices. |
| UKPN/SPEN tabular LTDS; Northern Powergrid LTDS | Metadata supports CC BY 4.0 / NPg Open licence respectively; observed row access requires login. Payload completeness and gated CIM grants remain unverified. |
| ENWL shared model; UKPN/SPEN restricted CIM | Retain research-only/hold pending actual permitted uses and file coverage. Do not assume research-only overrides restrictions or access approval. |
| NIE Networks | Public catalogue metadata for its 20 listed datasets identifies CC BY 4.0. OSNI-derived information has an OGL notice. No branch-impedance model completeness established. |
| ESB Networks website | Research-only classification is appropriately conservative; no complete machine-readable model established. |
| Elexon data | BMRS licence supports commercial reuse subject to exclusions/attribution; do not infer rights to third-party material outside its grant. MID is not a day-ahead auction price. BSC legal text needs separate grant evidence. |
| ENTSO-E | Conditional A11 classification, including IFA/Nemo exceptions, is supported by the official reuse list. A09 does not appear on that list. Do not generalise the grant to every owner/dataset; no token-authorised retrieval tested. |
| I-SEM legal text; JAO content | Conservative research-only tags retained. Correct the rule-document and effective-version references. |
| GOV.UK holiday data; IANA rules | OGL / public-domain basis supported. Pin captures/releases and keep internal arithmetic separate. |
| Citizens Information holidays; operator technical losses; LCCC/Ofgem; selected BoE FX; unscoped commodity/auction material; generic Met Éireann products | Open-commercial designation not established for every claimed resource: keep research-only/unverified until scoped evidence is supplied. |
| ERA5; MIDAS Open | Current catalogue CC BY 4.0 / OGL respectively are supported. Download/account and transformation work remains. |
| Renewables.ninja | Publisher's [licensing page](https://www.renewables.ninja/about) confirms **CC BY-NC 4.0**, not BY-NC-SA. Research-only is correct; commercial use needs separate permission. |
| ECB | Reuse conditions support open-commercial with source/accuracy and sold-document disclosure conditions. Preserve quote direction and fill/vintage policy. |
| Internal scenario/calendar specifications | No acquired-data licence should be invented; retain lineage for every external ancestor and clarify actual repository licensing separately. |

**Share-alike/OSM check:** no directly selected source in this inventory was
verified to carry ODbL, OSM or another share-alike licence. This is not a
certification of every dataset/basemap behind a generic portal or every future
derivative. In particular, the existing GB notes already identify OSM/ODbL
coordinates and a share-alike reduced network among PyPSA-GB inputs: those
must **not** be used as a gap-filling source. A permissive tool licence or
CC-labelled export cannot clear a prohibited ancestor (FR-19.AC3).

## M1 input coverage disposition

| Required M1 input | What is actually established | Still missing/unverified before an M1 completeness claim |
| --- | --- | --- |
| Dated topology/assets | Hash-pinned ETYS/TYTFS and partial DNO/fleet snapshots exist | Historical commissioning/revisions; all-area/operator denominators; bus/asset/GSP/BMU crosswalks; parameter/unit/rating reconciliation |
| Network rules/proxies | Real NESO limit files and Irish SNSP observations exist | Effective rule releases, cut-sets, regional memberships, numeric inertia/minimum-unit/SNSP constraints and uncertainty/vintage |
| Underlying/embedded demand | GB/Irish national demand captures and some embedded estimates exist | Correct decomposition, GSP allocation, Irish embedded components, weather-driven output, missing/estimated-field provenance |
| Losses | Resistance-containing network captures exist | Ten directional link loss models, field coverage/units, actual TLM source, physical-versus-settlement loss reconciliation |
| Calendar/resolution | Correct leap/DST counts; UK holiday source currently includes 2024 | Irish/statutory and market calendars, pinned timezone/capture versions, fold/revision and energy-conservation fixtures |
| Scenario identity | Accepted architecture and spike manifests exist | Implemented schema and immutable locks, input/execution/transform identities, lineage, round-trip/hash and numerical fixtures |
| Related cost/weather/FX inputs | Some candidates and a weather request template exist | Dated/scoped commodity/carbon/subsidy data; acquired ERA5/profiles; complete versioned FX and conversion policy |
| Runner prerequisite | D-65 requirement is identified | Separate target provisioning and NFR-12.AC1/AC2 evidence before heavy M1 work; not established by a credentials row |

## Appendix A — complete external URL sweep

Each distinct external Markdown-link URL in the pinned file appears once below,
in first-occurrence order; repeated occurrences share the result. These are
GET results from 2026-11-14 after redirects, not promises of future availability.
“Verified page” means the named page/terms/catalogue exists, **not** that every
claim attached to it or every downstream data file was verified.

The three 403s are **UNVERIFIED** at those exact links. ENTSO-E terms/list were
independently recovered via its
[official article API](https://transparencyplatform.zendesk.com/api/v2/help_center/en-us/articles/40921911218961.json)
and [reuse-list PDF](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729);
the list includes physical flows, excludes IFA/Nemo, and does not list A09.
Do not infer an account requirement from a bot/CDN denial.

| ID / URL | HTTP | Content/access result |
| --- | --- | --- |
| [U01](https://www.neso.energy/document/383936/download) | 200 | XLSX; ETYS Appendix B, matching the named product. |
| [U02](https://www.neso.energy/terms-and-conditions) | 200 | Verified general terms page; not a dataset-specific open grant. |
| [U03](https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72) | 200 | CKAN login HTML after redirect; not boundary GIS data. Use package_show/resource. |
| [U04](https://www.neso.energy/data-portal/neso-open-licence) | 200 | Verified NESO Open licence page; association must be per dataset. |
| [U05](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) | 200 | Verified NGED LTDS tabular catalogue. |
| [U06](https://www.nationalgrid.co.uk/open-data-licence) | 200 | Verified NGED Open licence, December 2020 version. |
| [U07](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) | 200 | Verified NGED CIM catalogue; nested ZIP/profile structure matters. |
| [U08](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement) | 200 | Verified SEPD catalogue page; not proof of downloaded SHEPD data. |
| [U09](https://creativecommons.org/licenses/by/4.0/) | 200 | Verified CC BY 4.0 deed; a deed alone does not associate a dataset. |
| [U10](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/) | 200 | Verified UKPN Table 1 catalogue; metadata CC BY 4.0, 5,076 records; records probe 403. |
| [U11](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/) | 200 | Verified UKPN CIM catalogue; actual permitted uses/access remain unverified. |
| [U12](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/) | 200 | Verified SPM Table 1 catalogue; metadata CC BY 4.0, 1,798 records; records probe 403. |
| [U13](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/) | 200 | Verified SPM CIM catalogue; not a retrieved/cleared CIM payload. |
| [U14](https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-shared-data-licence/) | 200 | Verified shared-licence page; actual Licence Details/Permitted Uses still needed. |
| [U15](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/) | 200 | Verified NPg circuit catalogue; metadata NPg Open licence, 1,670 records; records probe 403. |
| [U16](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/) | 200 | Verified NPg CIM catalogue; no authenticated payload inspection. |
| [U17](https://northernpowergrid.opendatasoft.com/p/opendatalicence/) | 200 | Verified NPg Open licence page. |
| [U18](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/) | 200 | Verified secure-area login/registration page, not a model download. |
| [U19](https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/) | 200 | Verified ENWL shared-licence page; restrictive permitted-use terms. |
| [U20](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip) | 200 | ZIP; correct TYTFS study-file product. |
| [U21](https://www.eirgrid.ie/disclaimer) | 200 | Verified EirGrid legal/disclaimer page; not an express open designation. |
| [U22](https://nienetworks.opendatasoft.com/pages/home/) | 200 | Verified NIE home page; separate public catalogue metadata identifies CC BY datasets. |
| [U23](https://www.esbnetworks.ie/) | 200 | Verified ESB home page, not a machine-readable electrical model. |
| [U24](https://www.esbnetworks.ie/data-legal/copyright) | 200 | Verified ESB copyright page; no general open-commercial grant. |
| [U25](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all) | 200 | JSON reference list: ten current interconnector IDs; not dated commissioning history. |
| [U26](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) | 200 | Verified BMRS licence with attribution/exclusions. |
| [U27](https://transparency.entsoe.eu/) | 200 | Verified ENTSO-E platform landing page; API requires token. |
| [U28](https://transparencyplatform.zendesk.com/hc/en-us/articles/40921911218961) | 403 | CDN/bot-denial page; exact page UNVERIFIED here. Official API/PDF checked separately. |
| [U29](https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1) | 200 | CKAN login HTML after redirect; use public package API/pinned TEC resource. |
| [U30](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract) | 200 | Verified REPD publication page; current title says quarterly extract. |
| [U31](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/) | 200 | Verified OGL v3.0 text; verify resource association and excluded third-party rights. |
| [U32](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes) | 200 | Verified DUKES electricity publication hub. |
| [U33](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json) | 200 | JSON current BMU register; not a historical 2024 registry. |
| [U34](https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml) | 200 | XML monthly registered-capacity report; correct dataset, not prices. |
| [U35](https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml) | 200 | JSON empty error envelope, not the claimed daily-register XML. Cached capture exists. |
| [U36](https://www.smartgriddashboard.com/all/open-data-license/) | 200 | Verified EirGrid Group licence; grant is scoped to expressly designated information. |
| [U37](https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26) | 200 | CKAN login HTML after redirect; use public package API/pinned constraints resource. |
| [U38](https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121) | 200 | CKAN login HTML after redirect; use public package API/pinned 24-month resource. |
| [U39](https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx) | 200 | XLSX; correct 2024 quarter-hourly system-data workbook. |
| [U40](https://www.neso.energy/publications/operability-strategy-report) | 404 | NESO page not found; numeric proxy claims not verified by this URL. |
| [U41](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/) | 200 | Redirects to Digital BSC code hub; no pinned rule version or blanket text licence established. |
| [U42](https://www.sem-o.com/rules-and-governance/rules/trading-and-settlement-code/) | 404 | SEMO page not found; not verified Part B text. |
| [U43](https://www.jao.eu/terms-conditions) | 200 | Verified JAO use terms; not border-specific capacity-allocation rules. |
| [U44](https://api.neso.energy/dataset/historic-demand-data-2024) | 200 | CKAN login HTML; use historic-demand-data package and actual 2024 CSV resource. |
| [U45](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn) | 200 | JSON demand API; undated request defaults to recent data. Use explicit 2024 dates. |
| [U46](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream) | 200 | JSON FUELHH; undated request is recent data; not an embedded solar/wind decomposition. |
| [U47](https://www.neso.energy/document/379136/download) | 200 | XLSX; correct Appendix G nodal-demand product. |
| [U48](https://www.britned.com/) | 200 | Verified BritNed homepage; no cited directional loss-parameter release. |
| [U49](https://www.eleclink.co.uk/) | 200 | Verified ElecLink homepage; no cited directional loss-parameter release. |
| [U50](https://greenlink.ie/operations/) | 200 | Verified Greenlink operations page; not a complete ten-link loss register. |
| [U51](https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/) | 404 | Resource-not-found JSON; correct dated system-price endpoint still is not a TLM panel. |
| [U52](https://www.gov.uk/bank-holidays.json) | 200 | JSON UK bank holidays; observed dates span 2019–2028 in each jurisdiction. |
| [U53](https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/) | 403 | Access denied; Irish holiday text and alleged associated CC BY grant UNVERIFIED. |
| [U54](https://www.iana.org/time-zones) | 200 | Verified IANA time-zone release page; pin an actual release. |
| [U55](https://www.gov.uk/government/publications/uk-ets-auction-results) | 404 | GOV.UK page not found; no UK/EU auction-results dataset verified. |
| [U56](https://www.gov.uk/government/collections/fossil-fuel-price-assumptions) | 200 | Verified DESNZ assumptions collection, not daily historical market prices. |
| [U57](https://www.worldbank.org/en/research/commodity-markets) | 200 | Verified commodity page; monthly/annual products, not the asserted full daily NBP/TTF/ARA set. |
| [U58](https://www.lowcarboncontracts.uk/data-portal) | 403 | Forbidden; portal contents/licence UNVERIFIED. Ofgem RO source not identified. |
| [U59](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels) | 200 | Verified ERA5 catalogue; separate catalogue API confirms CC-BY-4.0. |
| [U60](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/) | 200 | Verified MIDAS Open collection; OGL, distinct from restricted full MIDAS. |
| [U61](https://www.met.ie/climate/available-data/mera) | 200 | Verified MÉRA page; 1981–August 2019, GRIB 1 samples; separate station-data product. |
| [U62](https://www.renewables.ninja/) | 200 | Verified Ninja site; /about separately confirms CC BY-NC 4.0. |
| [U63](https://creativecommons.org/licenses/by-nc/4.0/) | 200 | Verified CC BY-NC 4.0 deed; not a share-alike licence. |
| [U64](https://data.elexon.co.uk/) | 404 | Host root returns resource-not-found JSON; use the actual API routes/manifests. |
| [U65](https://www.semopx.com/documents/general-publications/lookback_mkt.xlsx) | 200 | XLSX exists but contains 2018–2020, not the cited 2024 holdout. |
| [U66](https://reports.sem-o.com/documents/) | 200 | JSON empty error envelope, not a browsable report archive. |
| [U67](https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f) | 200 | CKAN login HTML after redirect; use public package API/pinned cost resource. |
| [U68](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html) | 200 | Verified ECB reference-rate/timing page; version daily observations and quote direction. |
| [U69](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html) | 200 | Verified ECB reuse disclaimer; includes source/accuracy and sold-document disclosure conditions. |
| [U70](https://www.bankofengland.co.uk/boeapps/database/) | 200 | Verified BoE database entry point, not the selected series/ownership clearance. |
| [U71](https://www.bankofengland.co.uk/legal) | 200 | Verified BoE terms; selected third-party FX series excluded from OGL. |
| [U72](https://cds.climate.copernicus.eu/) | 200 | Verified CDS landing page; authenticated requests/terms acceptance not tested. |
| [U73](https://ukpowernetworks.opendatasoft.com/) | 200 | Verified UKPN home page, not authenticated access. |
| [U74](https://ukpowernetworks.opendatasoft.com/explore/forms/cim-access-request-form/) | 200 | Redirects to UKPN login; actual form/approval not inspected. |
| [U75](https://spenergynetworks.opendatasoft.com/) | 200 | Verified SPEN home page, not authenticated access. |
| [U76](https://spenergynetworks.opendatasoft.com/explore/forms/data-request/) | 200 | Redirects to asset catalogue, not a verified Data Request form. |
| [U77](https://northernpowergrid.opendatasoft.com/) | 200 | Verified NPg home page, not authenticated access. |

## Acceptance boundary

**Must-fix** means the current inventory must not be treated as rights clearance,
an acquisition checklist already satisfied, or an M1 completeness attestation.
It does not require buying data or implementing the ingest engine in PR #52.
Correct the facts, link the actual resources, conservatively label unverified
rights, and expose the missing-field/access backlog. Re-check the corrected
inventory on its new exact head before relying on it under D-51.
