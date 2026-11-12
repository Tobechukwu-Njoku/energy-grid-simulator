# Data Sources Catalogue: Ireland, Weather, Interconnectors & FX

_Reviewed: 2026-11-12. Source availability and licence observations are date-specific._

## Scope and admission rule

This is a research inventory, not permission to ingest every public download. **D-17** requires
commercial-SaaS compatibility; **D-30** excludes share-alike data. **D-32** requires a full
transmission model, whose feasibility remains subject to the **D-31** data spike.

**UNVERIFIED** means that the stated right, coverage, interface or specification could not be
established from the cited primary evidence. Do not admit such data into the commercial pipeline
until that specific issue is resolved. Public access, a statutory publication duty and a copyright
notice are not interchangeable with a dataset-specific reuse licence. Publication copyright and
permission to reuse underlying data must be assessed separately.

## Confirmed conclusions

- The EirGrid Group Open Data Licence permits commercial exploitation of covered information,
  with attribution and exclusions. It is not proof that every group publication is covered.
- The **2024 TYTFS PDF really does reserve rights**. No commercially permissive licence for a
  complete all-island transmission parameter dataset was established in this review.
- The current CDS catalogue identifies **both ERA5 and ERA5-Land as CC BY 4.0**. The previously
  quoted legacy Copernicus commercial-use sentence was not verbatim and is removed.
- ENTSO-E's free-reuse list really is **CC BY 4.0**, but has provider/item exclusions, including
  **IFA and Nemo Link**. It is not an open licence for the entire platform.
- Renewables.ninja is **CC BY-NC 4.0**: exclude its data from commercial use unless separately licensed.
- Elexon's licence permits commercial reuse of covered BMRS data. Its live reference API confirms
  ten interconnector identifiers, including corrections for EWIC, Moyle and Greenlink below.

## Ireland: network, operations and market data

| Source and official evidence | Contents / access actually established | Licence and commercial position |
|---|---|---|
| **EirGrid Smart Grid Dashboard** — [dashboard](https://www.smartgriddashboard.com/all/), [Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) | Public dashboard; the licence refers to specified datasets in EirGrid/SONI libraries and dashboards, and information from group subsidiaries. | Commercial reuse of covered information is allowed, with attribution and third-party-rights exclusions. The original undocumented `dashboard/getallislanddata` route and circa-2014 archive start are **UNVERIFIED**. |
| **EirGrid system data** — [system and renewable data reports](https://www.eirgrid.ie/grid/system-and-renewable-data-reports), [2026 quarter-hourly publication](https://www.eirgrid.ie/publications/system-data-quarter-hourly-2026) | Download links for operational spreadsheets, including quarter-hourly system data. SONI has its own [reports page](https://www.soni.ltd.uk/grid/system-and-renewable-data-reports). | Apply the group licence only to covered datasets; retain the precise download and its terms. A uniform circa-2011 start date, frequency and schema for every series are **UNVERIFIED**. |
| **All-Island TYTFS 2024** — [publication](https://www.eirgrid.ie/all-island-ten-year-transmission-forecast-statement-2024), [actual PDF](https://www.eirgrid.ie/sites/default/files/publications/All-Island-Ten-Year-Transmission-Forecast-Statement-2024_1.pdf) | Network models/data, planning snapshots, diagrams and parameter tables. The second physical PDF page contains the copyright notice. | **Not commercially cleared.** The publication reserves rights and requires prior written permission for reproduction/transmission. Whether a separate parameter workbook is explicitly covered by the group open-data licence is **UNVERIFIED**; do not infer a workbook's terms solely from the PDF. |
| **GCS 2023–2032** — [publication and workbook link](https://www.eirgrid.ie/generation-capacity-statement-2023-2032), [PDF](https://www.eirgrid.ie/sites/default/files/publications/19035-EirGrid-Generation-Capacity-Statement-Combined-2023-V5-Jan-2024.pdf) | A named historical generation/demand adequacy statement and linked workbook, not a claim that this is the latest adequacy publication. | The PDF's printed page 22 reserves rights, including reproduction and use outside its designated purpose without permission. Separate workbook reuse rights are **UNVERIFIED**. |
| **SEMO balancing data** — [market data](https://www.sem-o.com/market-data), [actual disclaimer](https://www.sem-o.com/disclaimer) | Public market-data reports; availability is distinct from permission to redistribute them. | The disclaimer concerns accuracy/reliance, **not** the restrictive copyright quotation previously attributed to it. The group licence names SEMO, but rights in each selected report, including third-party inputs, remain **UNVERIFIED** until its coverage is established. |
| **SEMOpx day-ahead/intraday data** — [market data](https://www.semopx.com/market-data), [document library](https://www.semopx.com/market-data/document-library), [disclaimer](https://www.semopx.com/disclaimer), [joining information](https://www.semopx.com/joining-the-market/how-to-join) | Public reports and a separate exchange-membership process. Use actual market-time-unit timestamps; do not assume all present-day DAM data is hourly. | **UNVERIFIED** dataset-specific reuse. The group licence expressly names SEMOpx, subject to scope/exclusions; the disclaimer does not establish a blanket commercial ban. The mandatory DSE agreement, universal annual data fee and approximately €3,500 tariff were not substantiated and are withdrawn. Trading membership is not a data-reuse licence. |
| **Claimed ESB Networks catalogue on data.gov.ie** — [official catalogue query](https://data.gov.ie/api/3/action/package_search?q=organization%3Aesb-networks) | The originally cited organisation page returned 404; this organisation query returned zero datasets. | **UNVERIFIED**: no evidence for the claimed ESB-published CC BY collection of substations, EV charging and public-lighting datasets. Searches can instead return other publishers' ESB-related **CC BY-SA** connection statistics, which fail D-30; see the [catalogue search](https://data.gov.ie/api/3/action/package_search?q=%22ESB%20Networks%22&rows=8). |
| **ESB Networks maps / capacity publications** — [publisher](https://www.esbnetworks.ie/), [copyright policy](https://www.esbnetworks.ie/data-legal/copyright) | Website/network publications; no complete, commercially cleared feeder-parameter model was established. | Website copyright is reserved; obtain the specific dataset's permission. Do not turn this into a claim that every ESB dataset is unusable or that no open dataset exists. D-37's synthetic-feeder fallback remains relevant. |
| **NIE Networks portal** — [portal](https://nienetworks.opendatasoft.com/pages/home/), [terms](https://nienetworks.opendatasoft.com/terms/terms-and-conditions/), [dataset metadata API](https://nienetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets?limit=20) | The 20 returned records, including substations, transformers and transmission switchgear, identify **CC BY 4.0**. The terms require checking each dataset's information tab. | Commercial reuse is allowed for appropriately licensed layers with attribution. The portal also identifies OGL-covered Ordnance Survey NI information; basemaps/third-party rights are separate. Complete feeder impedances, quarterly updates and half-hourly profiles across the catalogue are **UNVERIFIED**. |

### EirGrid/SONI licence scope and Irish transmission admission gate

The [group licence](https://www.smartgriddashboard.com/all/open-data-license/) expressly permits
“exploit the Information commercially and non-commercially” and requires the attribution
“Supported by EirGrid Group Data”. It excludes rights the provider is not authorised to license
and refers to information made available under the licence. Its reference to SEMO/SEMOpx is
important, but does not justify disregarding a file's conflicting notice.

The TYTFS 2024 PDF says “All rights reserved.” Its second page prohibits reproduction/transmission
without written permission from EirGrid and SONI. Its statutory-obligations table cites
**section 38 of the Electricity Regulation Act 1999 for Ireland and condition 33 for Northern
Ireland**, not the previously asserted Irish condition 33 / NI condition 36.

**Position for D-30/D-32:** the publication restriction is confirmed; blanket open licensing of the
underlying transmission model is not. Request written confirmation identifying the exact model,
workbook, allowed extraction, commercial derivative use and redistribution. This is not a finding
that copyright necessarily prohibits every reuse of an individual fact. Public planning filings
are **not automatically permissively licensed substitutes** either. Record provenance and rights
before constructing a commercial model from them. A complete node/branch model, zero-sequence
coverage and transformer saturation/tap data sufficient for implementation remain **UNVERIFIED**;
tables in a report are not a validated machine-readable network model.

## Interconnectors and European transparency data

| Source and official evidence | Contents / access | Licence and commercial position |
|---|---|---|
| **ENTSO-E listed open data** — [official legal/support page](https://transparencyplatform.zendesk.com/api/v2/help_center/en-us/articles/40921911218961.json), [free-reuse list, 18 October 2023](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729) | Listed load forecasts, capacities, scheduled exchanges and physical flows, subject to the exact articles/provider exceptions. See the [current distribution-channel guide](https://transparencyplatform.zendesk.com/hc/en-us/articles/17260622859412-Transparency-Platform-Help-page). | **CC BY 4.0 for listed, non-excluded data**, not every field/provider. Exclusions include IFA/Nemo Link data, specified BritNed/Ukraine items and Moldova/Turkey data. Account/API arrangements are separate from reuse permission. |
| **ENTSO-E non-listed/excluded data** — [Terms of Use](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869376401) | Day-ahead prices under **article 12.1.d are absent from the free-reuse list**; distinguish them from listed imbalance/balancing prices. | **UNVERIFIED** until the primary owner's rights are checked. Absence from the list is not proof that every owner prohibits all commercial analysis or that a paid vendor agreement is always the only possible permission. |
| **Elexon Insights / BMRS** — [portal](https://bmrs.elexon.co.uk/), [licence](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/), [OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json) | The documented `/generation/outturn/interconnectors` supplies historic **half-hourly** flows; `/reference/interconnectors/all` supplies identifiers. Both were tested under the documented API base. | Commercial reuse is permitted for covered BMRS data, excluding unlicensed third-party rights. Do not conflate half-hourly outturn, instantaneous fuel-type observations and commercial schedules or promise all three at five-minute resolution. |
| **JAO** — [actual Terms of Use](https://www.jao.eu/terms-conditions) | Auction/platform data; legitimate use includes research and the user's own internal operational, analysis and decision-making purposes. | Internal use is not categorically non-commercial. External commercial exploitation/redistribution beyond permitted uses needs clearance. This is **not** an unrestricted open-data licence. |
| **Direct operator portals** — e.g. [BritNed](https://www.britned.com/), [ElecLink](https://www.eleclink.co.uk/), [Mutual Energy](https://www.mutual-energy.com/), [Greenlink operations](https://greenlink.ie/operations/) | Operator publications and availability/REMIT links. Greenlink expressly links to Elexon for operational information. | Reuse is **UNVERIFIED per dataset/operator**. A duty to publish does not confer an open licence; an operator website's copyright does not dictate independently licensed Elexon data rights either. |

### Confirmed Elexon identifier mapping

The [live reference endpoint](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all)
returned these ten records. A historic query for 2026-11-11, settlement period 1, also succeeded.
This verifies the interface/mapping, not the completeness of every historic series.

| Link | Identifier |
|---|---|
| IFA | `INTFR` |
| IFA2 | `INTIFA2` |
| ElecLink | `INTELEC` |
| BritNed | `INTNED` |
| Nemo Link | `INTNEM` |
| North Sea Link | `INTNSL` |
| Viking Link | `INTVKL` |
| Moyle | `INTIRL` |
| East–West / EWIC | `INTEW` |
| Greenlink | `INTGRNL` |

The previous `INTMOY`/`INTGRN` identifiers were wrong; `INTIRL` was incorrectly assigned to EWIC.
Elexon's required attribution is “Contains BMRS data © Elexon Limited copyright and database right [year].”
Preserve the licence link where possible and the licence's conditions on sublicensing/attribution.
The operator's [Greenlink FAQ](https://www.greenlink.ie/faqs/) gives **500 MW nominal** and
operation in early 2025; its [summary](https://www.greenlink.ie/summary/) says completion in
February 2025. These replace the unsourced 504 MW / 2024–2025 statement. Other originally listed
nameplate capacities/exact commissioning dates are **UNVERIFIED** in this review and are not
adopted as model inputs. Available transfer capability is not nameplate capacity and can vary
by direction and interval.

## Weather and renewable-resource data

| Source and official evidence | Coverage / access | Licence and commercial position |
|---|---|---|
| **ERA5** — [dataset](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels), [metadata](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels) | Hourly, 1940–present; global reanalysis. CDS offers a 0.25° regular grid; approximately 31 km is the native atmospheric resolution, not the exact spacing of that grid. | Current catalogue: **CC BY 4.0**. Commercial use is permitted with applicable attribution/change notices. Follow the selected dataset's citation and [current CDS licence](https://cds.climate.copernicus.eu/licences/cc-by). |
| **ERA5-Land** — [dataset](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-land), [metadata](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-land) | Hourly, 1950–present; approximately 9 km native resolution, delivered on a 0.1° grid. | Current catalogue: **CC BY 4.0**. Do not assume ERA5's 100 m wind fields are present in ERA5-Land or accumulated radiation/precipitation has instantaneous-flux units. Check variable and accumulation conventions before conversion. |
| **Met Office DataHub** — [portal](https://datahub.metoffice.gov.uk/) | Product-specific forecast/observation APIs. | Selected-product commercial rights, quotas, subscription requirements and price are **UNVERIFIED**. A landing page does not verify a free or paid commercial contract. |
| **MIDAS Open** — [correct CEDA collection](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/), [example dataset/licence](https://catalogue.ceda.ac.uk/uuid/ec54d5e5288a4ebb8c7ad2a1ef6aec42/) | UK land-surface observations; collection coverage from 1853, varying by variable/version. Updated annually in delayed mode. CEDA registration is required for the checked dataset. | **UK OGL v3.0**; commercially usable with attribution/citation. Check station ownership/geographic coverage in the chosen release rather than assuming all NI stations are included. |
| **Full MIDAS archive** — see the distinction in the [Open collection record](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/) | The fuller archive is described as available for academic use; it is not the Open subset. | **Not cleared under MIDAS Open's OGL.** Commercial access needs separate permission; this is not a claim that no separately licensed commercial route can exist. |
| **Met Éireann / MÉRA** — [licensing](https://www.met.ie/about-us/specialised-services/open-data), [MÉRA description](https://www.met.ie/climate/available-data/mera) | Most datasets are CC BY 4.0; MÉRA is a 2.5 km reanalysis for **1981–August 2019**. Station counts/coverage depend on the dataset. | Commercial reuse of CC BY datasets is permitted with the publisher's attribution statements, disclaimer and modification notice. Some live forecasts have a **custom licence**; exact live-API/weather-warning display conditions are **UNVERIFIED**. |
| **Renewables.ninja** — [publisher licensing](https://www.renewables.ninja/about) | Simulated wind/solar series; coverage depends on product/weather source. | **CC BY-NC 4.0**, confirmed by the publisher. Commercial use requires separate permission; no blanket 1980–present availability is established here. |
| **PVGIS** — [JRC manual](https://joint-research-centre.ec.europa.eu/photovoltaic-geographical-information-system-pvgis/getting-started-pvgis/pvgis-user-manual_en), [service](https://re.jrc.ec.europa.eu/pvg_tools/en/) | PV estimates, radiation and TMY; source periods vary by database/release. | The manual expressly allows unrestricted use of **results**. The invented long quotation and uniform 2005–present period are removed. Separately downloading upstream source datasets still needs their own terms. |
| **Global Wind Atlas** — [portal](https://globalwindatlas.info/), [World Bank metadata](https://energydata.info/api/3/action/package_show?id=global-wind-atlas) | Long-term wind-resource layers, not a chronological weather backtest. | Metadata identifies **CC BY 4.0**. Attribute the product; separately check basemap/input rights. Exact current layer resolutions/heights are **UNVERIFIED** in this audit. |
| **Global Solar Atlas** — [portal](https://globalsolaratlas.info/), [World Bank PVOUT metadata](https://energydata.info/api/3/action/package_show?id=world-photovoltaic-power-potential-pvout-gis-data-global-solar-atlas) | Long-term solar-resource / PV-potential layers. | The checked PVOUT dataset identifies **CC BY 4.0**. Do not extrapolate to every third-party layer or quote portal-wide terms not recovered. Exact resolutions remain **UNVERIFIED**. |
| **NSRDB** — [current API documentation](https://developer.nlr.gov/docs/solar/nsrdb/), [Meteosat product](https://developer.nlr.gov/docs/solar/nsrdb/nsrdb-msg-v1-0-0-download/), [developer terms](https://developer.nlr.gov/terms/) | Current publisher: **National Laboratory of the Rockies**, formerly NREL. The Meteosat product describes 2017–2019, 4 km, 15-minute data; product-specific API access applies. | Developer terms permit use for any purpose, with non-endorsement, warranty and indemnity conditions. **Not verified as the claimed CC BY 4.0 / CC BY 3.0 US combination.** Check third-party/input terms and the selected product; a TMY extension was not established. |

### Copernicus wording and conversion alternatives

The [legacy Licence to Use Copernicus Products, revision 12](https://cds.climate.copernicus.eu/licences/licence-to-use-copernicus-products)
does not contain the former document's purported verbatim commercial-use sentence. Clause 4.2
instead grants access for “any purpose in so far as it is lawful”. That supports commercial use,
but current ERA5/ERA5-Land catalogue entries identify **CC BY 4.0**. Do not replace their current
citation obligations with the old modified-Copernicus notice alone.

Computing renewable profiles from appropriately licensed weather inputs avoids Renewables.ninja's
non-commercial data licence. This is an option, not a new architecture decision. Conversion
software, power curves, CAMS products and other inputs need separate provenance/licence checks;
the prior blanket claim that all suggested alternatives are cleared is **UNVERIFIED**.

## Exchange rates: EUR/GBP (D-38)

| Source and official evidence | Access / interpretation | Licence and commercial position |
|---|---|---|
| **ECB reference rates** — [rates/timing](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html), [reuse conditions](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html) | Normally updated around 16:00 CET on working days except TARGET closing days; quoted against EUR. Use dated observations/explicit units, not an unspecified spot-price proxy. | Reuse is permitted subject to accuracy/source acknowledgement. For information incorporated into sold documents/subscriptions, users must be told it is available free from the ECB. The source-credit-only summary omitted this condition. |
| **Bank of England database** — [database](https://www.bankofengland.co.uk/boeapps/database/), [actual legal terms](https://www.bankofengland.co.uk/legal) | Daily series include `XUDLSER`. Preserve the official description/units and check conversion direction before mixing it with ECB rates. | Database reproduction generally falls under **OGL v3.0**, **but selected third-party exchange-rate series are excluded**. The exact chosen series' clearance is **UNVERIFIED** until its notes/ownership are checked; blanket open-FX clearance is withdrawn. |

## Verification

**Date: 2026-11-12.** Every substantive claim in the original 26 catalogue rows, summaries,
category analyses and source list was reviewed. Repeated unsupported claims were removed rather
than treated as independently verified facts. Publisher pages, metadata, actual OpenAPI responses
and the TYTFS/GCS/ENTSO-E PDFs were inspected. Access limitations are not interpreted as licence bans.

### Changes made

1. Replaced nonexistent EirGrid, licence, SEMO, SEMOpx, Met Éireann and Elexon paths with actual
   publisher pages. Corrected the MIDAS UUID, which returned **no records despite HTTP 200**,
   and the nonexistent Greenlink host. The old NREL hosts failed; current NLR evidence is linked.
2. Confirmed TYTFS/GCS restrictions against the PDFs, corrected TYTFS statutory references and
   separated publication rights from **UNVERIFIED** workbook/model reuse rights.
3. Removed fabricated SEMO copyright wording and unsupported SEMOpx DSE/fee assertions; the
   group open-data licence expressly names SEMO/SEMOpx, subject to its scope and exclusions.
4. Added ENTSO-E's **IFA/Nemo Link and other exclusions**. Confirmed that day-ahead prices are
   not on the list without inventing a blanket commercial-use prohibition for every owner.
5. Corrected BMRS identifiers, EWIC/Moyle mapping and universal five-minute flow/schedule claims.
   Tested reference/half-hourly-flow APIs. Corrected Greenlink's nominal-capacity/timing description;
   other unconfirmed capacities and commissioning dates are flagged rather than adopted.
6. Changed ERA5/ERA5-Land to **current CC BY 4.0**, removed the fabricated legacy quotation and
   separated native resolution, delivered grid spacing and variable availability.
7. Confirmed Renewables.ninja's restriction, MIDAS Open's OGL/account requirement, Met Éireann's
   mixed licensing, PVGIS results use and checked atlas licences; narrowed unverified coverage claims.
8. Withdrew the unsupported ESB open-catalogue and NSRDB CC claims; qualified NIE coverage and
   basemap rights. Corrected NSRDB publisher/host and narrowed product coverage.
9. Added ECB's sold-document disclosure and the Bank of England's FX exclusion. Removed invented
   combined quotations and universal access/coverage assertions throughout.

### Outstanding admission checks

- **UNVERIFIED:** commercially permissive, complete all-island transmission parameters; conflicting
  group-licence/publication notices need documented provider clarification.
- **UNVERIFIED:** SEMO/SEMOpx report/feed rights, any commercial contract/tariff and current
  market-time-unit coverage; infer neither a DSE requirement nor unrestricted reuse.
- **UNVERIFIED:** the claimed ESB collection/full feeders, untested dashboard API/archive coverage,
  direct-operator rights and any field/resolution expressly flagged in the tables.
- **UNVERIFIED:** DataHub product rights, Met Éireann custom live-forecast conditions, exact BoE
  FX-series ownership and separately suggested renewable-conversion tools/inputs.
- IFA, Nemo Link, NSL, Viking Link and National Grid pages returned **403** during the original
  URL pass. That is an access observation, not evidence of their data licences.

**Verdict: merge after fixes made**, as a qualified research catalogue. This does **not** clear
the outstanding sources for commercial ingestion or resolve the D-31/D-32 network-data gate.
