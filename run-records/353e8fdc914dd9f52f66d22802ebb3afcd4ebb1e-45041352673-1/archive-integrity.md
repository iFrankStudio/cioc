# C10 original-native artifact integrity record (2026-11-22)

This is a **verified index of the original downloadable GitHub Actions ZIP**, not a permanent copy of the 274 raw JSONL traces. The latter remain in the time-limited artifact and must not be described as evidence-branch archived.

- Repository: `iFrankStudio/cioc`; tested source SHA: `353e8fdc914dd9f52f66d22802ebb3afcd4ebb1e`
- Successful focused Actions run: https://github.com/iFrankStudio/cioc/actions/runs/45041352673 (attempt 1)
- Original ZIP artifact: https://github.com/iFrankStudio/cioc/actions/runs/45041352673/artifacts/14761872568 (expiration reported as 2027-02-20)
- ZIP size: **8,868,234 bytes**; ZIP SHA-256: `9b468cd8ce3c9ddc8c07ebe092decf297a7a9e77e65c7ebfe62d0f53e4f0d276`
- Downloaded and locally opened ZIP, checked ZIP CRC with `ZipFile.testzip()`: **no corrupt member**; **278 entries**, including **274 original-native JSONL world traces** and four summary/provenance/log files.
- `cl1b-c10-original-provenance.txt`: SHA-256 `2ba5c7b0aaa026771b417c83989c2c85886a79c6bb147a48fff277b9485cdcdc`; declares tested SHA and run ID above.
- `cl1b_g2_state_rollback_c10_native_274_worlds.json`: SHA-256 `feeafdfe9c805c7bd7079c5854cf21cacb8089b6534b20764b45795d37236129`.
- `cl1b_g2_state_rollback_c10_independent_replay.json`: SHA-256 `76d67596752490687b67c6ecc882213da08a44ae32176499a6cff45dffad180e`.
- `cl1b-c10-original-native-rollback.log`: SHA-256 `f8b745f2cd3d54179c3c2f2b3737cbba5c97bfb98db31346a4e347be7c4c6a4e`.

The extracted independent-replay summary reports **274 worlds**, **887,330 paid CIL instructions**, `first_divergence: null`, **21** physical parent checkpoints, **228** paid restore attempts, **19** source-effective restorations, **10** subsequent source readbacks, and a **1,840**-instruction same-initial-source suppression difference. These are *reported output values cross-checked against the archived ZIP*, **not a new independent execution** of the Rust kernel or Python replayer.

**Evidence scope and remaining gap:** The ZIP checksum and extracted summaries are now permanently indexed here, but this text does **not** preserve raw CIL events. Download and preserve the full artifact (or all 274 JSONL members with an independently verified per-file manifest) before expiry. Focused C10 success is not proof of a latest-HEAD full historical regression, autonomous ROI learning, B3, or B4.
