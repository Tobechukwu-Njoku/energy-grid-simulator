---
name: add-dataset
description: Add or update an external dataset under the provenance rules (D-56) — licence classification, manifest, download script, no raw research-only data in git. Use for any data acquisition task.
---

# Add a dataset

1. Classify the licence: `open-commercial` or `research-only`. ODbL/OSM or any share-alike
   data is never used (D-56). If unclear, stop and add a questionnaire item.
2. Bulk gathering is Gemini donkey work (D-53 routing); the result must be checked by GPT or
   Claude before merge.
3. Write a download script (Rust, D-73; Python only under `spikes/`). Never commit raw
   `research-only` files; `open-commercial` samples only if registered.
4. Register it in the dataset's `manifest.toml` (the only accepted manifest name): source URL,
   licence, classification, retrieval date, SHA-256 of every file or a `sha256_index_file`.
   Every committed data file must be referenced by a manifest.
5. Record known gaps (missing fields, periods, timezones) in the manifest notes and, if they
   affect requirements, in `05-risks.md`.
6. Run `cargo xtask provenance` before pushing; it rejects unregistered data, research-only
   files and secrets. Never commit tokens (ENTSO-E, Copernicus, DNO portals) — use
   `request_secret` and environment variables.
7. Commit `data: <dataset>` and open a PR; run a `review-round`.
