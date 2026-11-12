# Great Britain Electricity Data Sources Catalogue

_Checked against publisher sources on 2026-11-12. Corrects the original PR #1 catalogue; see Verification._

## 1. Executive Summary

This is a source-discovery and ingestion-screening document, not an approved dataset inventory or a requirements baseline. D-30 requires commercially permissive, non-share-alike inputs; D-31 commissions the feasibility spike; D-32 includes **all 400/275/132 kV nodes in GB**, not just the assets legally classified as transmission. Public access, a download button, and absence of share-alike do not by themselves grant commercial reuse rights.

**The claim that a complete GB network with impedances and ratings can already be built from the listed non-share-alike sources is NOT ESTABLISHED (UNVERIFIED).** ETYS contains substantial real circuit R/X/B and rating data. However:

- ETYS 2025 Appendix B covers the listed SHE Transmission, SPT, NGET and OFTO assets, not every DNO-operated 132 kV circuit in England and Wales.
- Its Note sheet explicitly identifies the ratings as **post-fault**. It does not supply the asserted separate continuous and emergency rating sets. OFTO circuit records have one rating column, not four seasonal rating columns.
- Appendix B does not supply the asserted substation coordinates or transformer tap ranges.
- The NESO open licence applies only to explicitly associated datasets. The checked portal metadata licenses the ETYS **boundary GIS**, but does not establish a licence grant for the separately published Appendix B workbook. That workbook's commercial reuse remains **UNVERIFIED**, not assumed prohibited.
- PyPSA-GB documents an ETYS model with 2,000+ buses, but its own data-reference table includes ODbL coordinates, a GPL-3.0 reduced network and a non-commercial demand source. Its MIT code licence is not blanket clearance for its inputs.
- The Crown Estate wind-polygon layer checked here is **view/access only**, not commercially reusable open GIS. Crown Estate Scotland is a different publisher with different terms.

These findings narrow the original feasibility assertion; they do not prove that a suitably licensed model is impossible. The D-31 spike must establish rights, coverage and modelling adequacy before that assertion is accepted.

## 2. Licence Interpretation

- **Open Government Licence v3.0 (OGL v3.0)**: permits commercial reuse and adaptation with conditions, including source acknowledgement; no share-alike requirement. The exact short commercial-use clause is “exploit the Information commercially and non-commercially” [OGL]. Third-party rights and other exclusions still matter.
- **CC BY 4.0**: permits commercial reuse and adaptation, with attribution, licence information and indication of changes where applicable; no share-alike requirement [CC]. It does not remove third-party rights or turn service access into an unrestricted resale right.
- **Custom open licences**: retain their actual names and conditions. Being based on OGL or CC BY does not make the licence literally OGL or CC BY.
- **Shared licences**: are access/use contracts, not synonymous with share-alike. The ENA Shared Data Licence text published by SPEN restricts copying, adaptation, combining, redistribution and commercial use unless expressly permitted by the licence/details; third-party sharing requires approval [SHARED]. A negotiated exception is possible, not automatic.
- **UNVERIFIED**: evidence was absent, blocked, conflicting, or insufficient for the stated scope. Do not ingest into the commercial model until resolved. This is distinct from a verified restriction.

## 3. Catalogue

### 1. Elexon BMRS / Insights Solution

**Official access:** [Insights website][EWEB], [API documentation][EAPI], [machine-readable OpenAPI][ESPEC], [developer portal][EDEV], [IRIS][EIRIS]. The original `insights.elexon.co.uk` hostname did not resolve. The working website is `bmrs.elexon.co.uk`.

**Scope/resolution:** OpenAPI documents physical notifications, dynamic parameters, bids/offers, acceptances, generation, interconnector flows and system prices. Data resolution, retention and publication lag are endpoint-specific; a universal 2000s/2016 start date or 1–5-minute resolution is **UNVERIFIED**. Settlement-period data must be handled separately from instantaneous operational series and revisions.

**Access correction:** a [dated system-price request][ETEST] returned HTTP 200 without credentials. Public REST access does not universally require registration or an API key. IRIS requires its own account/client setup and uses **AMQP**, not the asserted STOMP protocol; the publisher's website describes a public archive as well.

**Licence:** **BSC Open Data Licence** [ELIC]. Exact commercial-use excerpt: “exploit the Information commercially and non-commercially”. Required attribution: “Contains BSC information licensed under the BSC Open Data Licence.” Link the licence where possible. This is a licence condition, not a claimed statutory attribution rule. Commercial reuse: **YES for covered information**; share-alike: **NO**.

**Market Index Data caution:** the licence excludes third-party rights that the provider cannot license, but does **not expressly name Market Index Data**. OpenAPI describes APX/N2EX MID price/volume series; these are not automatically the exchange day-ahead auction clearing-price series. The original categorical MID exclusion is **UNVERIFIED**; obtain dataset-specific rights confirmation rather than declaring it either open or forbidden from the generic exemption alone.

### 2. NESO Data Portal

**Official access:** [portal][NPORTAL], [API guidance][NAPI]. The old `dataportal.neso.energy` homepage returned 404. The documented API base is `api.neso.energy/api/3/action/` (append an action name and its parameters).

**Scope/resolution:** demand, generation and interconnector series, balancing-service datasets and other system data are separate products. [Historic Demand Data][DEMAND] lists annual resources from **2001 through 2026** and is licensed in the portal metadata. Its notes describe publication **21 days in arrears**, with retrospective corrections. This is not a real-time feed or proof that every NESO dataset has the same history. Other series' start dates, frequency, fields and latency require their own metadata checks.

**Licence:** **National Energy SO Open Data Licence v1.0** [NLIC]. The licence explicitly limits itself to associated datasets, is based on OGL v3.0 with amendments, contains compatibility wording, and requires “Supported by National Energy SO Open Data”. Its exact short commercial-use excerpt is “exploit the Information commercially and non-commercially”. Commercial reuse: **YES for explicitly covered data**; share-alike: **NO**. Retain licence snapshots and exclusions; do not describe use as unrestricted or apply it to every NESO publication.

**Access constraints:** guidance recommends at most one CKAN request per second and two Datastore requests per minute, and warns of blocking for overload. No-registration access does not mean unlimited throughput.

### 3. NESO Electricity Ten Year Statement (ETYS)

**Official sources:** [publication hub][ETYS], [appendices index][ETYSINDEX], [2025 Appendix B workbook][ETYSB], [boundary GIS dataset][BOUNDARY]. The checked index lists ETYS 2025 and earlier material. Use a specific edition, not an unqualified claim to describe today's network.

**Actual coverage:** Appendix B has separate substation, circuit, transformer, reactive-compensation and change tables for SHE Transmission, SPT, NGET and OFTOs, plus intra-GB HVDC. It contains Scottish 132 kV transmission and some NGET/OFTO 132 kV records. Neither those records nor 132 kV transformer secondary buses establish coverage of the full England/Wales DNO 132 kV networks.

**Parameters:** see Section 4 for inspected sheet names and limitations. Technical data are genuine; complete topology, operational busbar switching detail and all required ratings are **UNVERIFIED**.

**Licence judgement:** the boundary GIS metadata explicitly gives NESO Open Data Licence and describes general boundary names/paths. That does not license every document linked from the ETYS publication hub. A CKAN search for ETYS returned the boundary product, not an Appendix B licence record. The licence grant for the separately downloaded workbook is **UNVERIFIED**. NESO's general [website terms][NTERMS] reserve rights and restrict commercial exploitation except as otherwise permitted; public download is not sufficient permission. Seek an explicit dataset-level licence or publisher confirmation. Share-alike status for the workbook: **UNVERIFIED**, not automatically NO.

### 4. NESO Future Energy Scenarios (FES)

**Official sources:** [FES hub][FES], [FES Building Block Data][FESBLOCK]. The checked hub presents FES 2025 and points to a 2026 Ten Year Outlook; do not infer a FES 2026 dataset merely from the review date.

**Scope/resolution:** annual pathway projections toward 2050 for demand, generation and other whole-energy-system drivers; regional building blocks are a distinct dataset. They are scenarios, not operational outturns or half-hourly profiles. The original mixed pathway names are not a reliable edition-specific list; use the selected publication's labels.

**Licence:** the checked building-block portal metadata explicitly associates NESO Open Data Licence. Covered data: commercial **YES**, share-alike **NO**, subject to Section 2 and NESO attribution. Blanket clearance of every report, workbook, figure and third-party input is **UNVERIFIED**.

### 5. DESNZ Renewable Energy Planning Database (REPD)

**Official source:** [quarterly extract page][REPD]. The checked page supplies July 2026 / Q2 2026 CSV and workbook downloads.

**Coverage/resolution:** UK renewable electricity projects **over 150 kW** through planning, construction, operation and decommissioning; updated in the month after each quarter. Before 2021 the threshold was **1 MW**, so earlier sub-1 MW projects may be missing. The current CSV includes storage fields. It is not a complete register of all installed renewable/storage assets or microgeneration.

**Inspected CSV fields:** capacity, technology, storage/co-location fields, status, operator/applicant, address, country, planning identifiers and X/Y coordinates. It does **not** have the asserted universal connection-point field or explicit latitude/longitude columns. Coordinates and planning status are not verified electrical bus assignments or a guarantee of survey precision.

**Licence:** GOV.UK makes its content available under OGL v3.0 except where otherwise stated [OGL]. Commercial **YES**, share-alike **NO** for covered material; preserve dataset notices and attribution. Filter UK records to the intended GB geography.

### 6. DESNZ DUKES, Table 5.11

**Official source:** [electricity chapter and tables][DUKES]. The checked release is dated **30 July 2026**; Table 5.11 describes stations operational at the **end of May 2026**, not simply the preceding calendar year.

**Inspected scope:** the workbook's Notes say the detailed list covers stations owned or operated by **Major Power Producers**. Other stations, including many renewable sites and auto-generators, appear only in the summary. There is no blanket **greater-than-36-MW** cutoff: the detailed table includes sub-1 MW entries.

**Fields:** company/site, technology/fuels, installed capacity, connection type, country/region, postcode, OS reference, X/Y coordinates, commissioning year and DESNZ site code. The publisher explicitly says geographical/connection information is supplied by generators and has not been verified by DESNZ. Not a complete fleet-to-bus mapping. Includes Northern Ireland: filter for GB.

**Licence:** OGL v3.0 for covered GOV.UK material [OGL]; commercial **YES**, share-alike **NO**, with conditions.

### 7. Transmission-owner publications and portals

- **NGET:** the original National Grid transmission homepage was blocked by HTTP 403 in this check. Its particular asset datasets/licences remain **UNVERIFIED**. `connecteddata.nationalgrid.co.uk` is the **distribution** data portal [NGED], not evidence for an NGET transmission dataset.
- **SP Transmission / SP Energy Networks:** [portal][SPEN]. The actual custom licence is **SP Energy Networks Open Data Licence** [SPENLIC], based on CC BY 4.0, not literally OGL. It permits reuse/sharing/adaptation subject to its conditions, without a share-alike clause. Dataset applicability must still be checked; the same portal also publishes shared-licence material.
- **SSEN Transmission:** [portal][SSENT], [terms][SSENTTERMS]. Its portal terms explicitly license data under CC BY 4.0 and specify attribution details and triage forms. Commercial **YES**, share-alike **NO** for covered data. This is not proof that every claimed route, physical rating or coordinate is available or complete.

Formats, update rates, account requirements and actual circuit-parameter coverage are product-specific. A schematic, geographic route, asset register and electrically connected node/branch model are not interchangeable.

### 8. DNO portals and Long Term Development Statements (LTDS)

**Publisher entry points:** [UKPN][UKPN], [NGED][NGED], [SSEN Distribution][SSEND], [Northern Powergrid][NPG], [SP Electricity North West][ENW], [SPEN][SPEN]. The former Western Power DataPortal2 URL resolves to a login; it is not a no-registration open-feed guarantee. These are portal families, not a statement about the current number of independent corporate owners.

**Licence evidence, not a blanket portal licence:**

| Publisher/sample | Checked licence evidence | Commercial / share-alike judgement |
|---|---|---|
| UKPN | [Terms][UKPNTERMS] say to check the dataset information tab: Creative Commons, OGL or a Shared Dataset Agreement. The checked ECR sample is CC BY 4.0. | Open licensed sample: YES / NO; shared assets: permission-dependent. |
| NGED | [NGED Open Data Licence][NGEDLIC] (page identifies its latest version as December 2020), based on OGL v3.0, applies to identified datasets. | YES / NO when expressly associated; not all website content. |
| SSEN Distribution | [Substation Data][SSENDSUB] metadata: Creative Commons Attribution 4.0, covering SEPD and SHEPD. | Checked sample: YES / NO; other datasets need their own check. |
| Northern Powergrid | [Open Data Licence][NPGLIC]; metadata names Northern Powergrid Open Data Licence v1.0, including its ECR product. | YES / NO for associated data, with its own attribution conditions. |
| SP Electricity North West | [ECR Part 2][ENWECR] metadata: CC BY 4.0; records require a registered account. Generic corporate website terms are not an open-data grant. | Checked sample: YES / NO; no blanket licence for all assets. |
| SPEN | [Open licence][SPENLIC] and dataset-specific CC BY 4.0 metadata coexist with [shared terms][SHARED]. | Check the actual dataset's licence, not the portal brand. |

**Contents/resolution:** LTDS and capacity products can support distribution/GSP modelling, but a complete 132/33 kV R/X/B-and-ratings model or public CIM export across every operator is **UNVERIFIED**. Check each edition, appendix, access agreement and field schema.

**Underground assets:** the checked UKPN LV/132/66/33/HV underground-cable metadata specifies **UK Power Networks Shared Data Licence - Connections**. That supports restrictions for those products, not a universal national-security rule applying to all DNO underground routes or secondary-substation data. Do not substitute the ENA template for an operator's actual contract.

### 9. Embedded Capacity Registers (ECR)

**Official samples:** [UKPN below 1 MW][UKPNECR], [SPEN ECR][SPENECR], [Northern Powergrid ECR][NPGECR], [SP Electricity North West at least 1 MW][ENWECR].

Registers describe connected/accepted generation and storage and, in some products, flexible demand/DSR, with technology, capacity, status and network associations. UKPN's checked part 1 covers **50 kW–1 MW**; ENW's checked part 2 covers **at least 1 MW** and states monthly updates. This supports part-specific thresholds, not a complete national register of every resource. UKPN explicitly warns of under-registration and missing cells.

Commercial reuse depends on the individual register's licence: the UKPN/SPEN/ENW samples declare CC BY 4.0; Northern Powergrid uses its own named open licence. There is no single ENA-imposed CC BY/OGL licence for all registers. Universal monthly freshness and fully populated GSP/primary-substation identifiers across all portals remain **UNVERIFIED**.

### 10. Nord Pool N2EX and EPEX SPOT prices

**Official evidence:** [Nord Pool power data][NPDATA], [day-ahead products and redistribution policy][NPDA], [EEX Group's EPEX market-data shop][EPEXSHOP], [specific auction API product][EPEXAPI]. The original Nord Pool market-data URL returned 404. Direct EPEX pages were blocked by HTTP 403; their unseen terms are **UNVERIFIED**, not inferred from the block.

Commercial feed products are paid and redistribution is separately licensed. Nord Pool's checked page lists package-specific annual prices and separate redistribution fees, including starting amounts of EUR 3,500 or EUR 5,500 depending on package; the original approximate universal EUR 5,000 fee is not a reliable procurement quote. The specific EPEX multi-market auction API product is displayed at **EUR 1,040/month**, with a contract required. It is not proof that every API or commercial use has that minimum fee.

Availability, time resolution, included bidding areas, delay, back-history, internal use and external redistribution depend on the chosen product/agreement. The original blanket 24-hour delay, ten-plus-year **GB** history and exact bilateral agreement names are **UNVERIFIED**. Paid access does not automatically include SaaS redistribution. No share-alike grant is asserted: these are proprietary products requiring suitable rights.

**Market distinction:** Elexon system imbalance/cashout prices and MID are not interchangeable with exchange day-ahead auction clearing prices. Simulated dispatch prices are model outputs, not historical exchange benchmarks. Do not claim validation against day-ahead prices by substituting another market's series.

### 11. Ofgem

**Official sources:** [data portal][OFGEM], [copyright][OFGEMCOPY]. The original `/data` and `/terms-and-conditions` links returned 404. The current copyright page links OGL and excludes third-party copyrighted material. Commercial **YES**, share-alike **NO** for covered Crown material, not every embedded dataset.

Ofgem publishes retail indicators, regulatory/price-control material and related statistics at product-specific frequencies. The original quoted copyright sentence was not the page's exact text and has been replaced by this paraphrase. Do not assume that all current TNUoS/BSUoS/DUoS tariffs are a single Ofgem portal product: operational tariff sourcing needs the relevant NESO/DNO publication and its own licence check.

### 12. PyPSA-GB

**Publisher-maintained sources:** [documentation][PYPSA], [data sources/licences][PYPSADATA], [network data/known issues][PYPSANET], [repository licence][PYPSALIC]. The valid licence path uses branch `master`; the original `main/LICENSE` link returned 404.

The current documentation describes a full ETYS model with **2,000+ buses**, reduced/zonal options, historical/future scenarios and half-hourly/hourly simulation. This supports an implementation reference, not verified complete as-operated topology or full licence clearance.

The code is MIT licensed. However, the data-reference table identifies a **GPL-3.0 reduced network**, **OpenStreetMap (ODbL)** among coordinate sources, Google Maps, and **ESPENI CC-BY-NC-4.0** demand. The repository licence instead calls ESPENI CC-BY-4.0: that conflict is **UNVERIFIED** pending the original publisher's terms. Do not ingest the conflicted source or assume MIT relicenses it. The reduced-network and coordinate sources independently defeat the original blanket non-share-alike claim.

Known issues include missing/guessed coordinates, zero-impedance handling, connecting islanded buses and geographic adjustments. Independently audit the exact files, transformations and upstream grants before reusing any data. No licence conclusion here depends on the academic paper's licence, which was not established by the original code-licence link.

### 13. NESO Carbon Intensity API

**Official sources:** [service][CARBON], [API definitions][CARBONAPI], [terms][CARBONTERMS]. Data are described as CC BY 4.0, commercial **YES** / share-alike **NO** with its conditions. The terms additionally restrict resale/sublicensing of **API access**, replacement services, misleading endorsement and excessive requests. Do not equate data reuse with reselling the API.

The service provides national/regional generation-mix carbon intensity, including imports/losses, at half-hourly resolution across 14 regions. The website says history is available **after 2017-09-26**, not only from 2018. It advertises **96+ hours** ahead, while the checked API definition summary says **two days** and documents 48-hour endpoints. A uniform 96-hour API forecast for all endpoints is therefore **UNVERIFIED**; test the chosen endpoint/horizon.

This is **not a marginal-emissions dataset**. Average/consumption-mix intensity cannot by itself establish the emissions response to an incremental dispatch or storage action. The original mandatory attribution wording was not established; follow CC BY and the actual dataset/terms rather than inventing a prescribed phrase.

### 14. The Crown Estate and Crown Estate Scotland

**Different publishers, different rights:** [The Crown Estate portal][TCE], [wind-polygon item and licence metadata][TCELIC], [Crown Estate Scotland website][CES], [its linked spatial hub][CESHUB], [ScotWind offers metadata][CESLIC]. The original Scotland hub hostname returned a generic ArcGIS shell, not the identifiable publisher hub; it has been replaced with the hub linked by the publisher.

**The Crown Estate:** the checked wind-polygon item is owned by `TheCrownEstate` and carries **THE CROWN ESTATE PUBLIC DATA LICENCE (GIS) – VERSION 1.1**. Sections 4.2–4.4 limit permission to access/view; the exact short restriction is “You must not copy, use, Manipulate or reproduce the Public Data”. The definition includes combining/aggregating/adapting. This is **NOT permission to ingest, adapt or redistribute that layer commercially**, regardless of free access. No share-alike obligation is the issue: the reuse grant itself is missing. A separately granted reuse licence would be needed.

**Crown Estate Scotland:** the checked ScotWind offers item explicitly links **OGL v3.0** and requires source acknowledgement. That item is commercially reusable without share-alike, subject to OGL, but is an **offers-stage snapshot**, not a guarantee of current legal agreements. Its metadata points to the official Scotland hub and a publisher GIS contact.

Neither sample establishes comprehensive/accurate cable landfalls, offshore substation coordinates, OFTO connectivity or all rounds' current status. Those claims are **UNVERIFIED**. Lease polygons are not electrical network models.

### 15. ENA National Energy System Map / Open Energy

The two original ENA resource-library URLs were blocked by HTTP 403, so the cited map's current endpoint, schema, coverage and dataset licences remain **UNVERIFIED**. A regulator's data-best-practice policy is not itself a copyright/database licence.

[Open Energy][OPENENERGY] is operated by **Icebreaker One Ltd**, not ENA. It supports discovery/access to open and shared energy data; it does not impose one OGL/CC BY licence on everything discovered. Indexing/metadata access is not clearance of underlying network data. Commercial and share-alike judgements require each source's grant; both are **UNVERIFIED** for the originally asserted consolidated open tiers.

## 4. What ETYS Appendix B Actually Supplies

Inspected **2025 Appendix B**, linked by the official [appendices index][ETYSINDEX] as a downloadable [workbook][ETYSB]:

| Workbook part | Observed contents | Correction / limitation |
|---|---|---|
| `Note` (drawing text, not ordinary cells) | SHE Transmission modelling-platform migration; ratings are post-fault; values at least 9999 in SHE circuit/change sheets indicate bus couplers or short cables; future data non-binding. | A cells-only parser misses critical semantics. Do not ingest sentinel ratings as physical ampacity or mistake platform changes for physical changes. |
| `B-1-1a`–`B-1-1d` | Site code, site name, voltage indexes for SHE Transmission, SPT, NGET and OFTO. | Not coordinates, a full busbar/switching inventory, or a complete DNO 132 kV model. |
| `B-2-1a`–`B-2-1c` | Terminal node codes, overhead/cable lengths, type, R/X/B, Winter/Spring/Summer/Autumn MVA ratings. | Electrical values are **percent on 100 MVA**, not raw ohms or already-normalised per unit. Convert by the correct voltage/base and convention. Ratings are post-fault per Note. |
| `B-2-1d` | OFTO/wind-farm/node identifiers, lengths/type, R/X/B, one MVA rating, voltage. | No four seasonal circuit-rating columns or separate continuous-rating set in this sheet. |
| `B-3-1a`–`B-3-1d` | Transformer terminals, R/X/B and MVA rating; OFTO-specific identifiers where supplied. | No asserted tap-step/range or explicit turns-ratio columns. Terminal voltages can assist ratio derivation; that is not a published tap model. |
| `B-2-2*`, `B-3-2*`, `B-4-*` | Future circuit/transformer changes and reactive compensation. | Must distinguish existing and proposed assets. Changes run **2026/27–2033/34** in the checked workbook, not a guaranteed ten-year horizon for each table. |
| `B-5-1` | Existing/planned intra-GB HVDC terminals, types, voltage, length, seasonal ratings and reactive limits. | HVDC is not an ordinary AC R/X/B branch; additional modelling is needed. |

Appendix A is identified as **system schematics and geographic drawings**, not itself the portal boundary shapefile. The separately licensed boundary GIS gives geographic study-boundary paths, not all circuit ratings, switching states or time-varying transfer limits. The original generic allocation of generation/demand to Appendices F/G is **UNVERIFIED** for the selected edition; use each edition's actual files/schema.

**Feasibility acceptance evidence still needed (D-31/D-32):** explicit rights for each workbook/GIS/input; reconcile topology against each owner's inventory and required voltage scope; account for England/Wales 132 kV distribution; audit node joins, parallel circuits, transformers, reactive equipment and HVDC; resolve continuous versus post-fault operational limits; establish coordinates without restricted/share-alike inputs; validate an edition/date-specific model against measured flows, demand and generation. A straight-line rendering is a visualisation, not the true cable route or a validation of electrical topology.

## 5. Important Missing Inputs and Controls

- **Current operation versus planning snapshots:** outages, availability/REMIT, topology changes, security/contingency limits and binding operational constraints require separate dated inputs. FES/ETYS forecasts are not realised operations.
- **Generator-to-network joins:** BM Unit identifiers, connection/GSP mapping and connected versus contracted status need auditable crosswalks. REPD planning applications and DUKES sites are not automatically unique electrical units. Reconcile duplicates and embedded generation to avoid double counting.
- **Demand definitions/time semantics:** distinguish transmission-system demand, national demand and embedded generation. Store UTC, settlement date/period, daylight-saving 46/48/50-period days, revisions, publication lags and the model's as-of date. Scenario annual energy is not a load profile.
- **Unit commitment and AC completeness:** minimum stable output, ramps, start/stop costs, fuel/carbon inputs, transformer taps and appropriate normal/emergency limits are not all supplied by a topology workbook. Record missing fields rather than silently inventing them.
- **Geography/provenance:** UK statistics include Northern Ireland; GB is England, Scotland and Wales. Offshore assets and interconnectors may need other voltage levels. Trace coordinate/basemap provenance separately from electrical data; public map display is not permission to extract its database.
- **Weather and interconnectors:** this GB catalogue alone does not supply the D-31 weather year, all-island Ireland network or complete cross-border capacities/flows. Reconcile those companion-source inventories before any full-system feasibility conclusion.
- **Per-artifact rights register:** retain dataset/version/resource IDs, download date, checksum, exact licence/version and attribution, exclusions, commercial/derivative/redistribution rights, access terms and any written permission. Resolve third-party notices and source conflicts before commercial ingestion.

## 6. Verification

### Method, counts and verdict

Review date: **2026-11-12**. Baseline: original PR #1 head `b6a940f87771fe3d60e9e3067510480342c96a03`. Read all original lines, deduplicated URLs, followed redirects, inspected publisher text/embedded page data and official dataset metadata, downloaded ETYS and DUKES workbooks, and inspected the REPD CSV header. HTTP 200 alone was not accepted as verification of content, rights or publisher identity.

The claim ledger below defines **100 substantive claim checks**, separate from the **47 unique original URL checks**. `CONFIRMED` means evidence supports the stated scope; `CORRECTED` identifies an error or materially misleading overstatement fixed here; `UNVERIFIED` identifies an unresolved claim, not a proven falsehood. Related repeated assertions in the original summary, table and diagram are counted once. Multi-part ledger rows count as one scoped check, not one per word/field.

**Results: 30 confirmed, 48 errors/material overstatements corrected, 22 unresolved claims marked UNVERIFIED.** URL failures are reported separately and are not added to the substantive-error total.

**Verdict: merge after fixes made, as a corrected research catalogue only.** This does not accept the original feasibility conclusion, approve restricted inputs, baseline requirements, or override merge gates. Unresolved entries are explicitly quarantined; the original PR description's affirmative feasibility and blanket licence claims are superseded by this document. Do not merge on the strength of those old claims.

### Claim-by-claim corrections and reasons

| Check | Original claim tested | Result | Evidence / change and reason |
|---|---|---|---|
| V01 | Elexon BSC licence name | CONFIRMED | Section 3.1: actual licence page. |
| V02 | Covered BSC data permits commercial use without share-alike | CONFIRMED | Section 3.1: grant and conditions. |
| V03 | Elexon commercial quotation is exact; attribution is statutory | CORRECTED | Removed spliced quotation; used exact excerpt and contractual attribution. |
| V04 | Elexon public REST requires API-key registration | CORRECTED | Credential-free system-price request succeeded; do not confuse IRIS account setup. |
| V05 | IRIS streaming uses WebSocket/STOMP | CORRECTED | Publisher describes AMQP. |
| V06 | Every relevant Elexon series has the asserted history/resolution | UNVERIFIED | Retention/frequency are endpoint-specific; no universal start date established. |
| V07 | BSC licence expressly carves out all MID | UNVERIFIED | It names third-party rights, not MID; obtain product-specific confirmation. |
| V08 | Insights publishes the listed BM/operational data families | CONFIRMED | Official OpenAPI inspected; no assumption of identical resolution. |
| V09 | NESO licence is based on OGL v3.0 with amendments | CONFIRMED | Section 3.2: actual named licence. |
| V10 | NESO licence automatically covers portal and all publications | CORRECTED | Licence expressly requires dataset association. |
| V11 | Original NESO commercial quotation is verbatim | CORRECTED | Replaced combined/elliptical wording with exact short excerpt. |
| V12 | Covered NESO data is commercially reusable, non-share-alike | CONFIRMED | Conditions/attribution/exclusions retained; removed unrestricted-use language. |
| V13 | Old NESO data-portal URL is usable | CORRECTED | 404; replaced portal entry point. |
| V14 | CKAN API uses the old data-portal host | CORRECTED | Replaced with documented api.neso.energy host. |
| V15 | NESO has more than fifteen years of historical data | CONFIRMED | Demand resources span 2001–2026; do not extrapolate to every series or real-time access. |
| V16 | Every listed NESO service/operational dataset has the claimed frequency | UNVERIFIED | Broad catalogue was not resource-level evidence. |
| V17 | ETYS includes SHE Transmission, SPT, NGET and OFTO technical data | CONFIRMED | Workbook menu and separate owner tables inspected. |
| V18 | ETYS supplies every required GB 132 kV circuit/node | CORRECTED | Does not establish England/Wales DNO network coverage. |
| V19 | ETYS circuit tables contain R, X, B | CONFIRMED | Actual B-2-1 headers/data, not inferred from a hub title. |
| V20 | Diagram labels R/X/B as per unit without the source base/scale | CORRECTED | Added workbook's percent-on-100-MVA units and explicit conversion requirement. |
| V21 | ETYS circuit ratings are expressed in MVA | CONFIRMED | Inspected headers. |
| V22 | ETYS gives separate continuous and emergency ratings for all circuits | CORRECTED | Note defines post-fault ratings; no asserted continuous-rating set. |
| V23 | ETYS provides post-fault thermal ratings | CONFIRMED | Note drawing text explicitly says so; sentinel exceptions retained. |
| V24 | All ETYS circuit sheets contain four seasonal rating sets | CORRECTED | OFTO circuit sheet has one rating column. |
| V25 | Appendix B publishes transformer tap ranges/steps and ratios | CORRECTED | Claimed columns absent; terminal voltage inference is not a tap dataset. |
| V26 | Transformer tables contain impedances and ratings | CONFIRMED | B-3-1 sheets contain R/X/B and rating. |
| V27 | Appendix B provides substation coordinates | CORRECTED | Site-code/name/voltage indexes are not coordinate tables. |
| V28 | Appendix B is a complete as-operated busbar/topology inventory | UNVERIFIED | No independent completeness reconciliation or switching detail established. |
| V29 | Open licence is explicitly attached to the Appendix B workbook | UNVERIFIED | Portal grant found for boundary GIS, not the separate workbook. |
| V30 | Appendix A is the cited boundary shapefile | CORRECTED | Schematics/drawings distinguished from separate GIS dataset. |
| V31 | NESO publishes explicitly licensed ETYS boundary GIS | CONFIRMED | Named portal dataset and CKAN licence metadata inspected. |
| V32 | Original boundary references establish transfer-limit values as well as shapes | UNVERIFIED | Verified GIS names/paths; did not establish the claimed complete limits dataset. |
| V33 | Every Appendix B table gives a full ten-year projection | CORRECTED | Checked changes cover 2026/27–2033/34; edition/table-specific. |
| V34 | Generation/demand uniformly appear in Appendices F/G | UNVERIFIED | Original generic allocation not established for selected edition. |
| V35 | FES explores pathways to 2050 | CONFIRMED | Publisher hub and building blocks. |
| V36 | Original mixed pathway labels identify the selected FES edition | CORRECTED | Removed unsupported/mixed labels; require edition-specific names. |
| V37 | FES provides annual scenario projections | CONFIRMED | Not operational outturn or a half-hourly profile. |
| V38 | FES building-block metadata carries NESO open licence | CONFIRMED | Explicit dataset-level grant inspected. |
| V39 | Every FES publication/input is cleared by the portal licence | UNVERIFIED | No blanket permission inferred. |
| V40 | REPD threshold is at least 150 kW in all periods | CORRECTED | Publisher says over 150 kW; historical threshold differs. |
| V41 | REPD historic coverage has the same threshold as current data | CORRECTED | Added pre-2021 1 MW threshold and missing-small-project caveat. |
| V42 | REPD is complete and coordinates are exact | CORRECTED | Planning snapshot, not exhaustive installed-asset or survey register. |
| V43 | REPD has universal connection points and latitude/longitude fields | CORRECTED | Inspected CSV has X/Y; no claimed connection-point or lat/long columns. |
| V44 | REPD includes development statuses and storage fields | CONFIRMED | Publisher description and inspected CSV header. |
| V45 | REPD is updated quarterly | CONFIRMED | Publication page specifies following-month update. |
| V46 | Covered REPD GOV.UK material uses OGL, commercial/no share-alike | CONFIRMED | GOV.UK terms and OGL, retaining exceptions. |
| V47 | DUKES 5.11 has a greater-than-36-MW threshold | CORRECTED | Actual detailed table includes sub-1-MW plant. |
| V48 | DUKES lists every UK power station individually | CORRECTED | Notes limit individual list to Major Power Producers. |
| V49 | DUKES has capacity, fuel, operator, connection and location fields | CONFIRMED | Workbook headers checked; added publisher quality caveat. |
| V50 | Annual DUKES station snapshot only covers prior calendar year | CORRECTED | July 2026 release lists end-May-2026 stations. |
| V51 | DUKES scope is UK rather than GB-only | CONFIRMED | Country field and publisher title; filter Northern Ireland. |
| V52 | Covered DUKES GOV.UK material is OGL/commercial/non-share-alike | CONFIRMED | Retain exclusions and attribution. |
| V53 | connecteddata.nationalgrid.co.uk is NGET's transmission portal | CORRECTED | It is NGED distribution; wrong authority for the claimed evidence. |
| V54 | All TO publications/data use OGL or literal CC BY | CORRECTED | Publication access is not permission; SPEN also has a custom licence. |
| V55 | NGET source/licence/asset coverage is established by cited page | UNVERIFIED | HTTP 403; not a verified licence or dataset. |
| V56 | SSEN Transmission portal terms provide CC BY 4.0 rights | CONFIRMED | Actual terms/attribution requirements inspected. |
| V57 | SPEN's licence text matches the original OGL-style quotation | CORRECTED | Actual named custom licence is based on CC BY; removed false quotation. |
| V58 | Every DNO portal dataset is commercially reusable open data | CORRECTED | UKPN explicitly lists shared agreements alongside open licences. |
| V59 | All DNO portal/API access requires tokens/registration | CORRECTED | Public metadata accessible; authentication is product-specific. |
| V60 | Complete public CIM/electrical exports exist across all DNOs | UNVERIFIED | No per-operator schema/coverage evidence. |
| V61 | All underground/secondary data is forbidden under one ENA rule | CORRECTED | Actual operator terms and licence-details exceptions govern; no universal classification established. |
| V62 | ECRs have at-least-1-MW and some smaller-resource products | CONFIRMED | UKPN 50-kW–1-MW and ENW at-least-1-MW samples. |
| V63 | ECRs completely list every distributed/flexible resource | CORRECTED | Publisher warns of missing registration/cells; thresholds and scope vary. |
| V64 | Every ECR is complete/fresh monthly across all portal families | UNVERIFIED | Monthly statement verified for ENW sample, not a universal SLA. |
| V65 | Every ECR is CC BY 4.0 or OGL | CORRECTED | Northern Powergrid sample has its own named open licence. |
| V66 | All ECR records resolve to populated GSP/primary-substation IDs | UNVERIFIED | Schema presence is not complete join coverage. |
| V67 | Exchange feed/redistribution rights require commercial agreements | CONFIRMED | Nord Pool product/policy and EEX contract-required product. |
| V68 | Nord Pool redistribution has a typical universal EUR 5,000 fee | CORRECTED | Published starting fees differ by package; no universal quote. |
| V69 | EPEX automated access has a universal minimum above EUR 1,040/month | CORRECTED | Specific checked product is exactly EUR 1,040/month; not a universal minimum. |
| V70 | All free exchange web views are delayed by 24 hours | UNVERIFIED | Direct EPEX pages blocked; no blanket public-view delay established. |
| V71 | Every exchange GB series has ten-plus years of history | UNVERIFIED | Cross-market history is not proof for each GB product. |
| V72 | Every GB auction/feed uses the asserted hourly/half-hourly resolution | UNVERIFIED | Product/date/market-specific specifications needed. |
| V73 | Cashout or modelled prices can substitute for day-ahead benchmarks | CORRECTED | Distinguished different markets and validation targets. |
| V74 | Original exact exchange agreement names and conditions are verified | UNVERIFIED | Do not invent a contract title from a shop/product page. |
| V75 | Ofgem covered Crown material is reusable under OGL | CONFIRMED | Actual copyright page links OGL, excludes third-party material. |
| V76 | Original Ofgem copyright sentence is an exact quote | CORRECTED | Removed unsupported quotation; paraphrased actual policy. |
| V77 | Ofgem data/terms URLs cited in original are usable | CORRECTED | Both 404; replaced with live portal/copyright URLs. |
| V78 | Cited Ofgem portal provides all asserted current system tariffs | UNVERIFIED | Operational product-level tariff evidence missing. |
| V79 | PyPSA-GB code is MIT licensed | CONFIRMED | Actual repository licence inspected. |
| V80 | PyPSA-GB main/LICENSE URL resolves | CORRECTED | 404; actual licence is on master. |
| V81 | Current PyPSA-GB documentation describes 2,000+ ETYS buses | CONFIRMED | Maintainer documentation confirms it. |
| V82 | PyPSA-GB proves all network/coordinate inputs are non-share-alike | CORRECTED | Own source table lists ODbL coordinates and GPL-3.0 reduced network. |
| V83 | All PyPSA-GB data inputs have settled commercial rights | CORRECTED | ESPENI NC/open licence conflict; MIT does not clear third-party data. |
| V84 | Existing model extracts all coordinates/parameters directly from Appendix B | CORRECTED | Documented coordinate guessing and supplementary sources contradict direct extraction of every field. |
| V85 | Carbon-intensity data are CC BY 4.0 | CONFIRMED | Service, API definitions and terms. |
| V86 | Every carbon API endpoint supplies 96-hour forecasts | UNVERIFIED | Website says 96+ hours; API summary/48-hour endpoints differ. |
| V87 | Carbon-intensity history begins only in 2018 | CORRECTED | Service gives availability after 2017-09-26. |
| V88 | Carbon service has half-hourly national/14-region information | CONFIRMED | Publisher description/methodology. |
| V89 | Carbon-intensity API directly supplies marginal emissions | CORRECTED | Generation/consumption mix intensity is not marginal response. |
| V90 | Original carbon attribution phrase is prescribed by the terms | CORRECTED | Removed invented mandatory wording; apply actual CC BY requirements. |
| V91 | CC BY means unrestricted resale/replacement of carbon API access | CORRECTED | Additional service terms restrict access resale and replacement products. |
| V92 | Crown Estate GIS licence is named Open Data Licence | CORRECTED | Actual checked item names Public Data Licence (GIS), version 1.1. |
| V93 | That Crown Estate licence permits commercial adaptation/reuse | CORRECTED | Sections 4.2–4.4 only grant view/access and prohibit copying/manipulation. |
| V94 | Checked Crown Estate Scotland ScotWind item uses OGL v3.0 | CONFIRMED | Item metadata links OGL and specifies source acknowledgement. |
| V95 | Original Scotland ArcGIS hostname identifies the official hub | CORRECTED | Generic shell; replaced by identifiable hub linked from publisher website. |
| V96 | Crown GIS sources comprehensively provide landfalls/substations/OFTO wiring | UNVERIFIED | Lease polygons are not that electrical/asset dataset. |
| V97 | Catalogue establishes current operational/construction/planned offshore status | UNVERIFIED | Checked Scotland item is offers-stage and warns of updates; current completeness not established. |
| V98 | Original ENA URLs/schema/coverage are all verified | UNVERIFIED | Requests blocked with 403; no false broken-link or licence conclusion. |
| V99 | Open Energy is an ENA service | CORRECTED | Publisher identifies Icebreaker One Ltd, not ENA. |
| V100 | ENA/Open Energy open tiers uniformly license all data under OGL/CC BY | UNVERIFIED | Discovery access and policy guidance are not underlying rights grants. |

### Original URL audit

All **47 unique URLs** in the original file were requested: **36 returned HTTP 200 after redirects, 5 returned 404, 5 returned 403, and 1 failed DNS resolution**. HTTP 403 means access/content verification was blocked, not that the URL is broken. A 200 login, JavaScript shell or generic hub is not substantive verification. The rows below omit schemes for retired/unverified addresses so they are not presented as working source links. Replacements and evidence are linked in the catalogue.

All **59 current source-reference links** in this corrected document returned HTTP 200 after redirects in a final pass. Dynamic application pages were corroborated with their published metadata/specifications or embedded publisher content, not just an empty HTML shell.

| Original host/path | Observed result / disposition |
|---|---|
| `bmrs.elexon.co.uk/api-documentation` | 200; official application shell; checked OpenAPI and publisher application content. |
| `carbon-intensity.github.io/api-definitions/` | 200; official docs linked by service. |
| `carbonintensity.org.uk/` | 200; official service. |
| `commercial.nationalgrid.co.uk/long-term-development` | 200; NGED distribution, not NGET. |
| `connecteddata.nationalgrid.co.uk/` | 200; NGED distribution; corrected attribution. |
| `data.gov.uk` | 200 redirect to www host; generic catalogue is not a specific REPD licence record. |
| `data.nordpoolgroup.com/` | 200; official data-portal application. |
| `data.ssen.co.uk/` | 200; SSEN Distribution. |
| `dataportal.neso.energy/` | 404; replaced with current NESO portal. |
| `dataportal2.westernpower.co.uk/` | 200 after redirect to NGED login; not open anonymous data evidence. |
| `developer.data.elexon.co.uk` | 200 after trailing-slash redirect; official developer portal. |
| `developers.nordpoolgroup.com/` | 200 redirect to reference/welcome; official developer portal. |
| `electricitynorthwest.opendatasoft.com/` | 200 redirect to pages/homepage; identifies SP Electricity North West. |
| `github.com/andrewlyden/PyPSA-GB` | 200; maintainer repository. |
| `github.com/andrewlyden/PyPSA-GB/blob/main/LICENSE` | 404; replaced with master/LICENSE. |
| `insights.elexon.co.uk/` | DNS failure; replaced with bmrs.elexon.co.uk. |
| `northernpowergrid.opendatasoft.com/` | 200 redirect to pages/home; official portal. |
| `opendata-thecrownestate.opendata.arcgis.com/` | 200; official hub, but not a commercial reuse grant. |
| `openenergy.org.uk/` | 200; Icebreaker One service, not ENA. |
| `pypsa-gb.readthedocs.io/` | 200 redirect to en/latest; maintainer docs. |
| `spatialhub-crownestatescotland.hub.arcgis.com/` | 200 generic ArcGIS shell; publisher/content UNVERIFIED at this address; replaced. |
| `spenergynetworks.opendatasoft.com/` | 200 redirect to pages/home; official portal. |
| `spenergynetworks.opendatasoft.com/p/sp-energy-networks-open-data-licence/` | 200; actual licence extracted from embedded page content. |
| `ssentransmission.opendatasoft.com/` | 200 redirect to pages/homepage; official portal. |
| `ssentransmission.opendatasoft.com/terms/terms-and-conditions/` | 200; actual CC BY and attribution conditions. |
| `ukpowernetworks.opendatasoft.com/` | 200 redirect to pages/home; official portal. |
| `webshop.eex-group.com/` | 200; official shop, with EPEX products and contract links. |
| `www.elexon.co.uk/data/open-data-requests/bsc-open-data-licence/` | 200 redirect into /bsc/; official licence. |
| `www.energynetworks.org/industry-hub/resource-library/connections-data.html` | 403; content UNVERIFIED, not declared 404. |
| `www.energynetworks.org/industry-hub/resource-library/national-energy-system-map.html` | 403; map/terms UNVERIFIED. |
| `www.epexspot.com/` | 403; direct content UNVERIFIED; used official EEX shop evidence instead. |
| `www.epexspot.com/en/market-data` | 403; direct terms/delay UNVERIFIED. |
| `www.gov.uk/government/collections/digest-of-uk-energy-statistics-dukes` | 200; official collection. |
| `www.gov.uk/government/publications/renewable-energy-planning-database-quarterly-extract` | 200; official REPD publication/downloads. |
| `www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes` | 200; official chapter/downloads. |
| `www.nationalarchives.gov.uk/doc/open-government-licence/version/3/` | 200; official licence. |
| `www.nationalgrid.co.uk/open-data-licence` | 200 redirect to commercial.nationalgrid.co.uk; NGED custom licence. |
| `www.nationalgrid.com/electricity-transmission` | 403; direct dataset/licence content UNVERIFIED. |
| `www.neso.energy/data-portal/neso-open-licence` | 200; official licence and applicability caveat. |
| `www.neso.energy/publications/electricity-ten-year-statement-etys` | 200; official ETYS hub. |
| `www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices` | 200; official appendices index. |
| `www.neso.energy/publications/future-energy-scenarios-fes` | 200; official FES hub. |
| `www.nordpoolgroup.com/` | 200; official publisher. |
| `www.nordpoolgroup.com/trading/market-data/` | 404; replaced with current product/policy page. |
| `www.ofgem.gov.uk/data` | 404; replaced with current data portal. |
| `www.ofgem.gov.uk/energy-data-and-research/data-portal` | 200 redirect to /news-and-insight/data/data-portal. |
| `www.ofgem.gov.uk/terms-and-conditions` | 404; replaced with actual copyright page. |

### Reproducible artefacts and remaining actions

- ETYS workbook: document ID **383936**, SHA-256 `d5cacf238292e8c78af989a5ee3a0c9bd8619acc6e6e67f451d6b44c71fa5e0c` at review time. Inspect the Note sheet's drawing XML as well as cell values.
- DUKES workbook: linked **DUKES_5.11.xlsx**, SHA-256 `760286f4cb3168f04a5ee1b581dbfcb35e1d65a5ea0151bb2dd4dc12dca2c72a`. Scope/quality caveats are in Notes; current station fields in 5.11 Full list.
- REPD: linked **REPD_Publication_Q2_2026.csv**; checked actual header rather than assuming connection fields. The retrieved CSV required a legacy encoding rather than UTF-8.
- Resolve ETYS workbook rights, England/Wales 132 kV coverage, normal ratings and transformer taps in the D-31 spike. Keep commercial ingestion blocked where rights are UNVERIFIED. Re-check mutable licences and edition-specific resources before use.
- Removed the affirmative feasibility diagram, synthetic exact quotations and blanket commercial-use classifications because they conveyed evidence stronger than the official sources support. No application code or owner decision was changed.

[OGL]: https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/
[CC]: https://creativecommons.org/licenses/by/4.0/
[EWEB]: https://bmrs.elexon.co.uk/
[EAPI]: https://bmrs.elexon.co.uk/api-documentation
[ESPEC]: https://data.elexon.co.uk/swagger/v1/swagger.json
[ETEST]: https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/2026-11-11?format=json
[EDEV]: https://developer.data.elexon.co.uk/
[EIRIS]: https://bmrs.elexon.co.uk/iris
[ELIC]: https://www.elexon.co.uk/bsc/data/open-data-requests/bsc-open-data-licence/
[NPORTAL]: https://www.neso.energy/data-portal
[NAPI]: https://www.neso.energy/data-portal/api-guidance
[NLIC]: https://www.neso.energy/data-portal/neso-open-licence
[NTERMS]: https://www.neso.energy/terms-and-conditions
[DEMAND]: https://www.neso.energy/data-portal/historic-demand-data
[ETYS]: https://www.neso.energy/publications/electricity-ten-year-statement-etys
[ETYSINDEX]: https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices
[ETYSB]: https://www.neso.energy/document/383936/download
[BOUNDARY]: https://www.neso.energy/data-portal/etys-gb-transmission-system-boundaries
[FES]: https://www.neso.energy/publications/future-energy-scenarios-fes
[FESBLOCK]: https://www.neso.energy/data-portal/future-energy-scenario-fes-building-block-data
[REPD]: https://www.gov.uk/government/publications/renewable-energy-planning-database-quarterly-extract
[DUKES]: https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes
[NGED]: https://connecteddata.nationalgrid.co.uk/
[NGEDLIC]: https://commercial.nationalgrid.co.uk/open-data-licence
[SPEN]: https://spenergynetworks.opendatasoft.com/
[SPENLIC]: https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-open-data-licence/
[SHARED]: https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-shared-data-licence/
[SSENT]: https://ssentransmission.opendatasoft.com/
[SSENTTERMS]: https://ssentransmission.opendatasoft.com/terms/terms-and-conditions/
[UKPN]: https://ukpowernetworks.opendatasoft.com/
[UKPNTERMS]: https://ukpowernetworks.opendatasoft.com/terms/terms-and-conditions/
[SSEND]: https://data.ssen.co.uk/
[SSENDSUB]: https://data.ssen.co.uk/@ssen-distribution/ssen-substation-data
[NPG]: https://northernpowergrid.opendatasoft.com/
[NPGLIC]: https://northernpowergrid.opendatasoft.com/p/opendatalicence/
[ENW]: https://electricitynorthwest.opendatasoft.com/
[UKPNECR]: https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-embedded-capacity-register-1-under-1mw/information/
[SPENECR]: https://spenergynetworks.opendatasoft.com/explore/dataset/embedded-capacity-register/information/
[NPGECR]: https://northernpowergrid.opendatasoft.com/explore/dataset/ecr_manual_combine_test/information/
[ENWECR]: https://electricitynorthwest.opendatasoft.com/explore/dataset/enwl-embedded-capacity-register-2-1mw-and-above/information/
[NPDATA]: https://www.nordpoolgroup.com/en/services/power-market-data-services/
[NPDA]: https://www.nordpoolgroup.com/en/services/power-market-data-services/day-ahead-market-data/
[EPEXSHOP]: https://webshop.eex-group.com/epex-spot-public-market-data
[EPEXAPI]: https://webshop.eex-group.com/data-type/be-ch-de-dk-fi-fr-gb-nl-no-pl-se-day-ahead-and-pan-eu-intraday-api-read-only
[OFGEM]: https://www.ofgem.gov.uk/news-and-insight/data/data-portal
[OFGEMCOPY]: https://www.ofgem.gov.uk/c-ofgem-2026
[PYPSA]: https://pypsa-gb.readthedocs.io/en/latest/
[PYPSADATA]: https://pypsa-gb.readthedocs.io/en/latest/data_reference/data_sources.html
[PYPSANET]: https://pypsa-gb.readthedocs.io/en/latest/data_reference/network_data.html
[PYPSALIC]: https://github.com/andrewlyden/PyPSA-GB/blob/master/LICENSE
[CARBON]: https://carbonintensity.org.uk/
[CARBONAPI]: https://carbon-intensity.github.io/api-definitions/
[CARBONTERMS]: https://github.com/carbon-intensity/terms
[TCE]: https://opendata-thecrownestate.opendata.arcgis.com/
[TCELIC]: https://www.arcgis.com/sharing/rest/content/items/abc66eeec55946c3a6b9ab492da0e8a4?f=pjson
[CES]: https://www.crownestatescotland.com/
[CESHUB]: https://crown-estate-scotland-spatial-hub-coregis.hub.arcgis.com/
[CESLIC]: https://www.arcgis.com/sharing/rest/content/items/e26b96b461a84457a54994e754ce488c?f=pjson
[OPENENERGY]: https://openenergy.org.uk/
