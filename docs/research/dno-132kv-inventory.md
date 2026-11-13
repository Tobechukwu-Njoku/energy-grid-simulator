# GB DNO 132 kV Network Data Inventory

Verified against live publisher sources on **2026-11-13**. Supports D-60; provenance
classification follows D-56. This is also the **provenance manifest** for the datasets
listed here. See the [verification report](../requirements/reviews/verify-pr10-2026-11-13.md).

## Evidence limits

- All **14 GB DNO licence areas**, across six operator groups, are covered.
- An open licence on tabular LTDS data does **not** automatically apply to CIM.
  UKPN and SPEN CIM publications have separate shared licences and approval processes.
- **UNVERIFIED** means the fact could not be established. Public metadata, advertised
  scope and downloaded records are distinguished; a catalogue/schema is not a payload.
- NGED tabular CSVs and both SSEN workbooks were downloaded without authentication.
  UKPN, SPEN and Northern Powergrid LTDS record requests returned structured
  `403 ForbiddenAccess`. No account was created or authenticated session used;
  post-login entitlements and row-level completeness are **UNVERIFIED**.
- The previous assertion of **complete** open-commercial electrical data for 11
  areas is withdrawn. Schema/licence availability does not establish completeness.
- The Scottish 132 kV transmission boundary is supported by section 64 of the
  [Electricity Act 1989][ACT]. DNO references to transmission interfaces do not
  constitute an inventory of transmission-owned circuits, transformers or sites.

## Fourteen licence areas

IDs resolve to exact dataset names, URLs, formats, licences and tags in the manifest.
“Substations” means identifiable sites/nodes associated with 132 kV equipment,
**not** a complete geospatial or ownership-verified register. Coverage below concerns
tabular/site sources, not uninspected CIM payloads. Counts include switches/links
where present and are not counts of unique physical lines, transformers or sites.

| Licence area | Dataset IDs | Login/access | 132 kV circuits | 132 kV transformers | 132 kV substations |
|---|---|---|---|---|---|
| Eastern Power Networks (EPN) | U1–U5 | U1–U4 gated; U5 additionally requires a shared-data request | **UNVERIFIED**: U1 voltage schema public, rows gated | **UNVERIFIED**: U2/U3 winding-voltage schemas only | **UNVERIFIED**: U4 site/voltage schema only |
| London Power Networks (LPN) | U1–U5 | Same as EPN | **UNVERIFIED**: area-specific 132 kV rows not inspected | **UNVERIFIED**: U2/U3 rows gated | **UNVERIFIED**: U4 rows gated |
| South Eastern Power Networks (SPN) | U1–U5 | Same as EPN | **UNVERIFIED**: U1 rows gated | **UNVERIFIED**: U2/U3 rows gated | **UNVERIFIED**: U4 rows gated |
| NGED East Midlands | N1, N2 | N1 CSV downloads returned 200 without login; N2 download access **UNVERIFIED** | Confirmed: 1,311 Table 1 rows operating at 132 kV | Confirmed: 223 two-winding and 2 three-winding rows with a 132 kV winding | Confirmed: 1,063 Attachment 8 node names contain `132`; not unique sites |
| NGED West Midlands | N1, N2 | Same as East Midlands | Confirmed: 1,411 Table 1 rows at 132 kV | Confirmed: 199 two-winding and 63 three-winding rows with a 132 kV winding | Confirmed: 1,164 Attachment 8 node names contain `132` |
| NGED South Wales | N1, N2 | Same as East Midlands | Confirmed: 857 Table 1 rows at 132 kV | Confirmed: 131 two-winding and 2 three-winding rows with a 132 kV winding | Confirmed: 732 Attachment 8 node names contain `132` |
| NGED South West | N1, N2 | Same as East Midlands | Confirmed: 604 Table 1 rows at 132 kV | Confirmed: 102 two-winding and 2 three-winding rows with a 132 kV winding | Confirmed: 517 Attachment 8 node names contain `132` |
| Southern Electric Power Distribution (SEPD) | S1 | May 2026 XLSX downloaded without login | Confirmed: 435 Table 1 rows at 132 kV | Confirmed: 218 two-winding rows with a 132 kV winding; none in three-winding table | Confirmed: named GSP groups/nodes in Tables 1/2, e.g. AMERSHAM/LOUDWATER; no coordinates verified |
| Scottish Hydro Electric Power Distribution (SHEPD) | S2 | May 2026 XLSX downloaded without login | Confirmed absent from Table 1: 2,139 rows at 33 kV, 56 at 11 kV, none at 132 kV | Confirmed absent from inspected Table 2 voltage cells | **UNVERIFIED** as a 132 kV asset inventory; GSP names alone do not prove asset voltage/ownership |
| SP Manweb (SPM) | P1–P4 | P1–P3 gated; P4 explicitly requires an access request | **UNVERIFIED**: P1 voltage schema public, rows gated | P2 title explicitly advertises **132 to 33kV**; actual rows **UNVERIFIED** | **UNVERIFIED**: P3 has nominal-voltage schema, rows gated |
| SP Distribution (SPD) | P5–P8 | P5–P7 gated; P8 explicitly requires an access request | **UNVERIFIED** at row level; 132 kV normally transmission, not a DNO circuit inventory | **UNVERIFIED**: do not infer absence of interface transformers from circuit voltages | **UNVERIFIED**: P7 abbreviations do not establish a 132 kV asset inventory |
| Northern Powergrid (Northeast) | G1–G4 | Catalogue requires login; record requests gated | **UNVERIFIED**: EHV circuit schema/description, not inspected area-specific rows | **UNVERIFIED**: G2 primary/secondary voltage schemas only | **UNVERIFIED**: G3 advertises EHV sites but has no voltage/licence field |
| Northern Powergrid (Yorkshire) | G1–G4 | Same as Northeast | **UNVERIFIED**: G1 rows gated | **UNVERIFIED**: G2 rows gated | **UNVERIFIED**: G3 needs a validated area/voltage join |
| Electricity North West (ENWL) | E1 | Secure-area login explicitly required | Introduction explicitly covers 132 kV network/circuit data; actual rows **UNVERIFIED** | Advertised within 132 kV-to-primary-busbar scope; actual rows **UNVERIFIED** | 132 kV/EHV network and substation plans advertised; individual assets **UNVERIFIED** |

## Provenance manifest (D-56)

Tags are per dataset, not per operator; a package tag covers its listed resources.
**research-only** is a commercial-clearance flag, **not permission to download,
derive, combine or redistribute**. Approval and permitted-use clearance come first.
No restricted raw data is included in this repository. Share-alike/ODbL/OSM data is
excluded; no basemap was extracted or used. If terms cannot be read, share-alike
status is **UNVERIFIED** and use is on hold.

### UK Power Networks — EPN, LPN, SPN

| ID | Exact publisher title and URL | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| U1 | [Long Term Development Statement (LTDS) Table 1 Circuit data][U1] | Export discovery advertises CSV, JSON, XLSX, Parquet; API schema public, records gated | [Creative Commons Attribution 4.0 International][CC] (`CC BY 4.0` in metadata) | **open-commercial** / no |
| U2 | [Long Term Development Statement (LTDS) Table 2a Transformer Data - 2W][U2] | Tabular API schema confirmed; actual export files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| U3 | [Long Term Development Statement (LTDS) Table 2b Transformer Data - 3W][U3] | Tabular API schema confirmed; actual export files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| U4 | [UK Power Networks Grid and Primary Sites][U4] | API schema includes `sitevoltage`, `licencearea`; actual exports **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| U5 | [Long Term Development Statement - Common Information Model (CIM)][U5] | CIM EQ/SC/SCR/SYSCAP profiles described; actual file/container formats **UNVERIFIED** behind approval | [UK Power Networks Shared Data Licence - Common Information Model][UC], name/link from metadata; licence body **UNVERIFIED**, redirects to Microsoft sign-in | **research-only** / **UNVERIFIED; do not use** |

U1 describes the May 29, 2026 publication, data modified June 29, 2026. Its area field
is **`licencearea`**, not the previously claimed `dno`; accepted values/132 kV counts
are **UNVERIFIED**. U2/U3 also expose `licencearea` and winding voltages. U1 schema
confirms percent-labelled R/X/B, ampere-labelled seasonal ratings and length in km;
the 100 MVA base is **UNVERIFIED** in the inspected U1 metadata.

The old CIM identifier `long-term-development-statement-ltds-common-information-model-cim`
returns **404** as a page and catalogue lookup. The current identifier is
**`ukpn-ltds-cim`** (U5), metadata modified October 19, 2026, not a newly verified
edition date. Its description explicitly requires registration, login **and** the
[CIM access request form][UR]. Its public metadata flag `data_visible: true` does
not establish access to the SharePoint model deliverables.

### National Grid Electricity Distribution — all four areas

| ID | Exact publisher title and URL | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| N1 | [LTDS Tabular Model][N1] | Downloaded CSVs: Table 1 Circuit, Table 2a Two-winding Transformer, Table 2b Three-winding Transformer, Attachment 8 Node; other CSV/PDF resources listed | Catalogue label **WPD Open Data Licence**, ID `wpd-open-data`; page licence link resolves to current [NGED Open Data Licence][NG] | **open-commercial** / no |
| N2 | [LTDS Common Information Model][N2] | Catalogue lists `emids.zip`, `wmids.zip`, `swales.zip`, `swest.zip`, publication-notes PDF and substation-numbers CSV; payloads **UNVERIFIED** | Same catalogue label/link relationship: **WPD Open Data Licence** → [NGED Open Data Licence][NG] | **open-commercial** / no |

The NGED licence permits commercial exploitation in clause 3.3 and requires
**Supported by NGED Open Data** attribution in clause 4.1. It is based on the Open
Government Licence framework, not an identical OGL v3.0 text. The dataset-page
licence URL `https://www.nationalgrid.co.uk/open-data-licence` redirects to [NG].

N1 Table 1 contains `Licence`, `Operating Voltage (kV)`, `R1`, `X1`, `B1`, seasonal
ratings and `Circuit Length (km)`. At 132 kV, **3,045 of 4,183 rows** have a blank/`-`
in at least one of R1/X1/B1/Summer/Winter. These include switches/links, so this is
not a defect rate for physical lines. CSV headers do not label impedance/rating
units: earlier claims of 100 MVA-base percentages and **ampere** ratings remain
**UNVERIFIED**. A standard LTDS form is not file-specific unit evidence.

Filters are resource-specific: Table 1/2b use `WMIDS`, but two-winding Table 2a uses
**`WMID`**. Attachment 8 is a node-name register, not a geospatial substation database;
names containing `132` support coverage, not counts of unique sites.

Table 1, Tables 2a/2b and Attachment 8 resource timestamps are **December 3, 2025**.
N1 package metadata was modified **July 3, 2026**, not July 28; mid-year updates
affect other resources. N2 metadata was modified **May 29, 2026**. Package metadata
timestamps must not be substituted for electrical-file edition dates.

### SSEN Distribution — SEPD and SHEPD

| ID | Exact publisher title and URL | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| S1 | [SEPD Long Term Development Statement][S1] | May 2026 Tables 1–7 XLSX downloaded/parsed; PDF statements/diagrams and ZIP CIM cases listed; ZIP contents **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] (`CC-BY-4.0` in catalogue) | **open-commercial** / no |
| S2 | [SHEPD Long Term Development Statement][S2] | May 2026 Tables 1–7 XLSX downloaded/parsed; PDF statements/diagrams and ZIP CIM cases listed; ZIP contents **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |

Stable download URLs: [SEPD May 2026 tables][SX1], [SHEPD May 2026 tables][SX2]. Both
returned files without login, redirecting to short-lived signed storage URLs; do
not preserve expiring signed URLs as permanent download sources.

SEPD Table 1 headers explicitly give R/X/B in **% on 100 MVA base**, ratings in
**MVA**, length in **km**. Two-winding Table 2 includes AMERSHAM group / LOUDWATER
132/33 kV entries; the three-winding table has no 132 kV winding entries. SHEPD
Table 1 has **33 kV and 11 kV**, not only 33 kV. Circuit/transformer voltage cells
were inspected: searching `sharedStrings.xml` alone cannot establish absence of a
numeric voltage, because XLSX numeric cells are stored separately.

### SP Energy Networks — SP Manweb and SP Distribution

| ID | Exact publisher title and URL | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| P1 | [SPM LTDS Appendix 1 Circuit Data (Table 1)][P1] | CSV/JSON/XLSX/Parquet advertised by export discovery; records gated | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P2 | [SPM LTDS Appendix 2 Transformer Data (Table 2) 132 to 33kV][P2] | Tabular API schema confirmed; files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P3 | [SPM LTDS Appendix 7 Substation Abbreviation Codes (Table 7)][P3] | API schema includes `nominal_voltage_kv`; files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P4 | [SPM LTDS CIM][P4] | CIM profiles grouped in ZIP cases described; files **UNVERIFIED** | **ENA Shared Data Licence**, hosted at [SP Energy Networks Shared Data Licence][PS]; dataset-specific Licence Details **UNVERIFIED** | **research-only** / base terms not share-alike; **use on hold** |
| P5 | [SPD LTDS Appendix 1 Circuit Data (Table 1)][P5] | Tabular API schema confirmed; files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P6 | [SPD LTDS Appendix 2 Transformer Data (Table 2)][P6] | Tabular API schema confirmed; files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P7 | [SPD LTDS Appendix 7 Substation Abbreviation Codes (Table 7)][P7] | Tabular API schema confirmed; files **UNVERIFIED** | [Creative Commons Attribution 4.0 International][CC] | **open-commercial** / no |
| P8 | [SPD LTDS CIM][P8] | CIM profiles grouped in ZIP cases described; files **UNVERIFIED** | **ENA Shared Data Licence** at [the shared-licence page][PS]; dataset-specific Licence Details **UNVERIFIED** | **research-only** / base terms not share-alike; **use on hold** |

P1 labels R/X/B as percentages on 100 MVA, ratings in MVA and length in km. P5
instead labels R/X/B in **pu on 100 MVA**; percentages and per-unit are not
interchangeable. P1/P5 metadata was modified November 28, 2025. Their actual
voltage coverage is **UNVERIFIED**: both record endpoints returned 403. The main-site
LTDS landing page returned HTML **Access Denied** here, not proof of a login gate.

P4/P8 descriptions explicitly require an [access request][PR] and shared terms;
their catalogue `license` fields are null. The [portal terms][PT] link the document
headed **ENA Shared Data Licence**. Clauses 1.2/2.1 make Licence Details/Permitted
Uses material to permission. Unless expressly permitted, clauses 3.1.4 and
3.1.6–3.1.8 restrict derivation, combination, redistribution and commercial use.
Obtain actual Licence Details and approval **before research download/use**; the
base agreement is not permission to load CIM files into this simulator.

### Northern Powergrid — Northeast and Yorkshire

| ID | Exact publisher title and URL | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| G1 | [LTDS Appendix 3 - Circuit Data][G1] | CSV/JSON/XLSX/Parquet advertised by export discovery; records gated | [Northern Powergrid Open Data Licence v1.0][NP] | **open-commercial** / no |
| G2 | [LTDS Appendix 4 - Transformer Data][G2] | Tabular API schema confirmed; files **UNVERIFIED** | [Northern Powergrid Open Data Licence v1.0][NP] | **open-commercial** / no |
| G3 | [LTDS EHV Sites][G3] | API schema has point/shape/site names, no voltage/licence field; files **UNVERIFIED** | [Northern Powergrid Open Data Licence v1.0][NP] | **open-commercial** / no |
| G4 | [LTDS Stage 2 CIM Files][G4] | API schema has existing/future model-file fields; actual XML/container formats **UNVERIFIED** | [Northern Powergrid Open Data Licence v1.0][NP] | **open-commercial** / no |

G1/G2 metadata was modified November 29, 2025. G1 exposes **`license`** and
`operating_voltage_kv`; G2 adds primary/secondary winding voltages. Actual area values
and 132 kV rows are **UNVERIFIED**. G3 metadata is dated August 8, 2023; do not treat
it as a 2025/2026 register. G4 uses `licence_area`, so filters must be dataset-specific.
The licence permits commercial exploitation in clause 3.3; clause 4.1 requires
prominent **Supported by Northern Powergrid Open Data** attribution, with a licence
link where possible. Login is required; post-login model-file entitlements remain
**UNVERIFIED**.

### Electricity North West

| ID | Publication and URLs | Format evidence | Exact licence name and link | Tag / share-alike |
|---|---|---|---|---|
| E1 | [Long term development statement][E1], [Introduction to LTDS][EI], [secure-area login][EL] | Introduction describes tabular data, plans and steady-state CIM; actual XLSX/CSV/XML/PDF files **UNVERIFIED** | **ELECTRICITY NORTH WEST SHARED DATA LICENCE**, hosted at [SP ENW Shared Licence][ES] | **research-only** / no |

The landing page identifies the latest revision as **November 2025** and requires
secure-area access. The introduction links the LTDS to the shared licence and
confirms 132 kV-to-primary-busbar scope, circuit/transformer data, diagrams and CIM
models. Actual file formats, units and individual records are **UNVERIFIED**.
Clause 1.1 limits Permitted Use to internal business use, excluding use for other
persons' benefit; clause 3.1(b) restricts modification/publication/distribution
without prior written consent. A **research-only** tag does not waive those terms.

## Outstanding checks and clearance

1. Obtain authorised UKPN/SPEN/Northern Powergrid row access and ENWL files; check
   area values, 132 kV circuits/windings/sites and completeness.
2. Obtain UKPN CIM licence text/approval and SPEN CIM Licence Details/Permitted Uses.
   No use while share-alike or permitted-use clearance remains unresolved.
3. Verify CIM payload formats, access, asset coverage and per-file notices before
   ingestion, including ostensibly open NGED/SSEN/Northern Powergrid packages.
4. Establish units from dataset-specific evidence wherever marked **UNVERIFIED**.
5. Clear, replace or remove every **research-only** source before commercial use,
   under existing [PC-5/PC-7](../requirements/TODO-pre-commercial.md). This screening
   is not legal advice or a new owner decision.

[ACT]: https://www.legislation.gov.uk/ukpga/1989/29/section/64
[CC]: https://creativecommons.org/licenses/by/4.0/
[NG]: https://commercial.nationalgrid.co.uk/open-data-licence
[NP]: https://northernpowergrid.opendatasoft.com/p/opendatalicence/
[UC]: https://ukpowernetworks.sharepoint.com/:b:/s/OpenDataPortalLibrary/IQBOc_U7vx1GQqIOIMlkBAzhAW7sqW9vVvcm1twzb7lIy_Y?e=BG45N3
[PS]: https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-shared-data-licence/
[ES]: https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/
[U1]: https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/
[U2]: https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-2a-transformer-2w/
[U3]: https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-2b-transformer-data-3w/
[U4]: https://ukpowernetworks.opendatasoft.com/explore/assets/grid-and-primary-sites/
[U5]: https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/
[UR]: https://ukpowernetworks.opendatasoft.com/explore/forms/cim-access-request-form/
[N1]: https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model
[N2]: https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model
[S1]: https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement
[S2]: https://data.ssen.co.uk/@ssen-distribution/shepd_long_term_development_statement
[SX1]: https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx
[SX2]: https://data-api.ssen.co.uk/dataset/17d2aee2-5248-45d7-a4ce-d09bdfa77b3c/resource/2bd2c3f3-6692-4e0c-8a0e-5232be6ad682/download/scottish-hydro-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx
[P1]: https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/
[P2]: https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-2-transformer-data-table-2/
[P3]: https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-7-substation-abbreviation-codes-table-7/
[P4]: https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/
[P5]: https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-appendix-1-circuit-data-table-1/
[P6]: https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-appendix-2-transformer-data-table-2/
[P7]: https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-appendix-7-substation-abbreviation-codes-table-7/
[P8]: https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-cim-eq-profile/
[PR]: https://spenergynetworks.opendatasoft.com/explore/forms/data-request/
[PT]: https://spenergynetworks.opendatasoft.com/terms/terms-and-conditions/
[G1]: https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/
[G2]: https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-4/information/
[G3]: https://northernpowergrid.opendatasoft.com/explore/dataset/ltds_ehv_sites/information/
[G4]: https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/
[E1]: https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/
[EI]: https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/introduction-to-ltds/
[EL]: https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/
