# Triage — Gemini adversarial review (2026-11-12)

Triaged by the coordinator per D-53. Gemini findings are claims; each is accepted, partly accepted or rejected.

| ID | Verdict | Reason / action |
|---|---|---|
| ADV-1 hardware vs "aim high" | Accept (already R-14) | Owner hardware is fixed (D-48); CPU-first. Performance NFRs must be stated against D-48, not abstract "high end" |
| ADV-2 review deadlock | Accept — **resolved by D-54** (Claude reviews GPT-authored docs) |
| ADV-3 full network vs no-share-alike | Partly accept | Claim "no permissive source exists" conflicts with PR #1 finding (NESO ETYS under NESO Open Data Licence); under GPT verification. Ireland side likely valid ("All rights reserved", PR #2). Resolved by data spike + verifications (R-12) |
| ADV-4 132 kV split | Accept | In England/Wales 132 kV is DNO-owned; in Scotland it is transmission. D-32 "all 132 kV in GB" crosses into distribution. Requirements must define the MVP boundary (e.g. Scottish 132 kV + E&W 132 kV only where in ETYS) |
| ADV-5 BM validation vs PyPSA | Accept | PyPSA has no BM. Validate UC/dispatch vs PyPSA; validate BM against historic BM outturn (Elexon/NESO constraint costs) with explicit metrics and tolerances. Feeds 04-requirements |
| ADV-6 Rust MILP ecosystem | Accept (already R-9) | Covered by ADR 0003 in progress; owner declined an optimisation spike (D-31) — revisit if ADR 0003 shows high risk |
| ADV-7 BM without voltage/stability | Partly accept — **resolved by D-55** (DC + proxy voltage/stability constraints in MVP) |
| ADV-8 agent collusion / trivial tests | Accept | Add to ADR 0001: reference-data tests owned outside the authoring PR, mutation testing on core, provenance of test fixtures |
| ADV-9 I-SEM misrepresentation | Accept | I-SEM (ex-ante, balancing, imbalance pricing) and GB↔SEM coupling post-Brexit need explicit requirements |
| ADV-10 2021–2026 market changes | Accept | Brexit decoupling, MHHS, REMA outcome (zonal rejected 2025), new interconnectors (Viking, Greenlink) must be reflected in the data model and backtest years |
| ADV-11 TimescaleDB / Redis licences | Accept (pending PR #2 verification) | Decision on Redis vs Valkey goes to owner once verified |
| ADV-12 backtest needs REMIT outages | Accept | Backtest inputs must include REMIT unavailability and curtailment data; add to requirements and data spike scope |
