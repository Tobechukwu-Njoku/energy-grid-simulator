# Prior Art — reusable tools, libraries & datasets

_Last updated: 2026-11-12. Verify versions/licences before adopting (ADR required)._

## Simulation / modelling frameworks

| Tool | Lang | Licence | What it does | Fit |
|---|---|---|---|---|
| **PyPSA** (+ PyPSA-Eur, PyPSA-USA) | Python | MIT | Capacity expansion, LOPF/dispatch, linear power flow, sector coupling; uses linopy | Strong for planning/dispatch; large community |
| **pandapower** | Python | BSD-3 | AC power flow, OPF, short-circuit, state estimation; distribution focus | Strong for steady-state AC |
| **PowerModels.jl** / **PowerModelsDistribution.jl** | Julia | BSD-3 | Many OPF formulations (AC/DC/SOC/QC), research-grade | Best-in-class OPF, very fast with JuMP |
| **Sienna** (PowerSimulations.jl, PowerSystems.jl, PowerSimulationsDynamics.jl) | Julia | BSD-3 | NREL production cost + dynamics in one ecosystem | Strong for UC/ED + dynamics |
| **GridLAB-D** | C++ | BSD | Agent-based distribution simulation, household loads, markets | Distribution/DER detail |
| **OpenDSS** | C++/Pascal | BSD | EPRI distribution simulator, time-series QSTS | Distribution standard |
| **ANDES** | Python | GPL-3 | Power system dynamics (transient stability) | Dynamics; GPL caution |
| **HELICS** | C++ | BSD-3 | Co-simulation framework (T&D, markets, comms) | Glue between engines |
| **Grid2Op** | Python | MPL-2 | RL environment for grid operation | If RL/AI agents wanted |
| **MATPOWER** | MATLAB/Octave | BSD-3 | Classic PF/OPF; reference case formats | Test cases & format |
| **GenX**, **Calliope**, **oemof**, **TIMES** | Julia/Python | various | Capacity expansion / energy system models | Alternatives for long-term |
| **NVIDIA cuOpt** | CUDA | Apache-2.0 | GPU LP/MILP/routing solver | GPU acceleration if NVIDIA HW |

Commercial references (not reusable, but benchmarks for features): PLEXOS, PSS/E, PowerWorld,
DIgSILENT PowerFactory, PSCAD, Aurora.

## Solvers

Open: **HiGHS** (LP/MILP, MIT), **Ipopt** (NLP, EPL), **SCIP** (MILP, Apache-2.0),
**Clarabel** (conic), **cuDSS/cuPDLP** (GPU). Commercial: **Gurobi**, CPLEX, Xpress, Mosek.

## Data sources

- **Network topology:** OpenStreetMap power lines (via PyPSA-Eur / osm-power), ENTSO-E grid map,
  TAMU ACTIVSg synthetic grids (2k–82k buses, US-realistic), IEEE test cases, **pglib-opf**
  (benchmark library), Global Energy Monitor plant trackers, EIA-860 (US generators).
- **Operations/market:** ENTSO-E Transparency Platform, EIA-930 (US hourly), ISO APIs
  (ERCOT, CAISO OASIS, PJM Data Miner, MISO, NYISO), Open Power System Data.
- **Weather/renewables:** ERA5 (Copernicus), NREL NSRDB (solar), NREL WIND Toolkit, atlite
  (converts weather → wind/solar capacity factors), renewables.ninja, CMIP6 climate projections.
- **Costs/tech:** NREL Annual Technology Baseline (ATB), IEA, Lazard LCOE.

## Infrastructure building blocks

PostgreSQL 18 · PostGIS · TimescaleDB · Redis / Valkey · Apache Arrow/Parquet · DuckDB ·
ClickHouse · Ray / Dask / Temporal (distributed sweeps & workflows) · NATS / Kafka ·
MapLibre GL / deck.gl · OpenTelemetry / Prometheus / Grafana.
