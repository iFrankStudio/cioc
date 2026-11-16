# CL-1B B45 — model-only forecast frozen before first native Rust CI

**Prepared on 2026-11-16 before the first native B45 GitHub Actions run.** See [prospective protocol](../studies/B45-paid-last-writer-provenance.md), [actual Rust world-law implementation](../../cl0/crates/world/src/lib.rs), [B45 native experiment](../../cl0/crates/world/tests/cl1b_b45_paid_last_writer.rs) and [independent source-parsed CIL interpreter](../analysis/b45_paid_last_writer_scan.mjs).

B44's complete-world historical equivalence arises because the current world stores only mutable values, not their write origins. B45's optional latest-writer metadata changes the *information physics* by storing one additional unit per current cell; it does not expose private event history or automatically tell a CIL organism what output is correct. A researcher-authored, charged CIL guard reads an integer origin tag and compares it with its own PID.

The CIL source-parsed forecast (each arm 8 worlds, first64 train and next64 unseen opportunities, initial 128 credits) is:

| Arm | Training true/wrong | Unseen true/wrong | CIL charged | New native code nodes | Halted | Origin sensor ops | False success first verdicts |
|---|---:|---:|---:|---:|---:|---:|---:|
| GuardTrackedSpoof | 24/0 | **96/8** | 1024 | 8 | 0 | 16 | 0 |
| GuardTrackedGenuine | 24/14 | **78/26** | 1024 | 6 | 0 | 8 | 0 |
| GuardTrackedEcho | 30/0 | **102/2** | 1024 | 8 | 0 | 10 | 0 |
| GuardUntrackedSpoof | 0/0 | **0/0** | **344** | 8 | **8** | 16 | 0 |
| OriginalTrackedSpoof | 16/48 | **0/104** | 1024 | 0 | 0 | 0 | 6 |
| OriginalUntrackedSpoof | 16/48 | **0/104** | 1024 | 0 | 0 | 0 | 6 |
| StaticTracked | 128/0 | **128/0** | 1024 | 0 | 0 | 0 | 0 |

**These are predicted values, not observed Rust results.** The full frozen vector of all seven arms is included in the Rust B45 native test, and the independent model will independently assert them when CI runs. The guard remains policy authored by the investigator. The apparent performance improvement comes from the newly granted source information plus authored branch logic, not autonomous discovery. Echo-induced repair is an intervention-identity false positive. Unknown provenance causes fail-closed halts, not productive robustness. The static 4-op copier remains better. B3 and B4 are **NOT MET**, irrespective of native pass/fail.

Additional counterexample predicted: researcher temporarily routes an existing native Write under PID1 to produce the expected value but leaves the program's future source wrong; its latest writer tag becomes PID1, which is genuine identity but **not evidence that the program will continue to work**. Reconstructing B44's paired histories under opt-in last-writer metadata makes their current serialized worlds differ, and a paid CIL read reports `1` (true native writer) versus `0` (external overwriter). Both histories are laboratory-forced; neither is endogenous code discovery.
