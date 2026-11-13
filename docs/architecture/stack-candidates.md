# Stack Candidates (TENTATIVE — not decisions)

_Last updated: 2026-11-12 (revised for Rust-first, D-20). Nothing here is accepted until an ADR in `adr/` is accepted by the owner._

Guiding stance (D-2): performance and capability over resource cost.

| Layer | Leading candidate | Alternatives | Depends on |
|---|---|---|---|
| System of record | **PostgreSQL 18** (D-3) + **PostGIS** (grid geometry) | — | Q-C-6 |
| Time-series results | **TimescaleDB** on PG18 (hypertables, compression) for queryable results; **Parquet on object storage + DuckDB** for bulk sweep outputs | ClickHouse | Q-H-5, Q-I-6 |
| Cache / queue / pub-sub | **Valkey** (D-67): cache and live state; clients use only the shared Redis protocol. P1 job dispatch uses PostgreSQL | Redis as a compatible alternative, with deployment terms reviewed | D-67 |
| Simulation core | **Rust (D-20)**: `faer` / KLU / NVIDIA cuDSS (FFI) for sparse linear algebra; `highs` crate, **Clarabel** (Rust-native conic), cuOpt via FFI; pluggable solver trait (D-16). PyPSA / PowerModels / pandapower only as offline validators | — | D-10, D-20 |
| Solvers | HiGHS + Ipopt default; Gurobi if allowed; cuOpt/cuDSS if NVIDIA GPUs | — | Q-I-3, Q-H-2 |
| Distributed execution | Rust workers claiming durable **PostgreSQL** jobs in P1 (D-67); Valkey for ephemeral notifications/live state | Later queue transport requires an ADR; Ray is Python-centric → disfavoured by D-20 | D-67, Q-D-3, Q-H-3 |
| API | Rust **Axum** + **tonic** (gRPC) between services; WebSocket for live sandbox streaming; `sqlx` for Postgres | GraphQL | D-20, Q-J-4 |
| Analytics | Rust **Polars** / **DataFusion** over Arrow/Parquet | DuckDB | D-20 |
| SDK / CLI | Rust crate + CLI (primary); Python bindings optional later | — | D-20 |
| Frontend | TypeScript + React, **MapLibre GL + deck.gl** (GPU map rendering), Apache ECharts / uPlot for time series | Svelte | Q-G-* |
| Data interchange | Apache Arrow / Parquet (Rust `arrow-rs`; also readable by offline Python/Julia validators) | — | — |
| Observability | OpenTelemetry → Prometheus + Grafana + Tempo/Loki | — | — |
| Packaging/Deploy | Docker Compose (workstation) → Kubernetes (cluster) | Nomad | Q-H-3 |

## Open architectural questions

1. Single engine vs plug-in multi-engine architecture (likely multi-engine, given Q-B-1 breadth).
2. D-67 selects Valkey for cache/live state and PostgreSQL for P1 dispatch; remaining storage details require an accepted ADR.
3. Where do bulk results live — DB or object storage? (Recommended: summaries in Timescale,
   full-resolution in Parquet.)
