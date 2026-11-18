
# CL-1B B56-F1d1a — correlated paid-receipt sensor: conditional cost frontier and semantic falsifier

**Status: EXACT-SOURCE NATIVE AND INDEPENDENT FULL CIL REPLAY VERIFIED; actor invention / policy choice NOT TESTED.** B56-F1d0 proved the original raw-feedback EVERY monitor less expensive than FOURTH for all16 four-bit schedules. F1d1a tests whether source-faithful B53 paid nonce/echo/commit checking changes that price ordering and whether it rejects stale receipts without guaranteeing truth.

## Preregistration and evidence

- [F1d1a frozen manifest](../studies/B56-F1d1a-native-correlated-receipt-price-microcalibration-manifest-v1.md) was committed before Rust test implementation at `db1b6e009906c84b012f0839152e9e6d55dc9858`; SHA-256 `cc68abc29e3e6bb3ef543e1b203df204793ef38ce36f3372f54f35e71e43c66b`.
- [FULL successful Actions run 44244193410](https://github.com/iFrankStudio/cioc/actions/runs/44244193410), exact tested source `8e75d91fc25f6b2b06e7d4f36047907b69ff2439`: **62/62 named Rust CL-1B native integration targets PASS, 51/51 independent observers PASS, B31/B34 ABI PASS, zero failed counters, original evidence published**. [Exact summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e75d91fc25f6b2b06e7d4f36047907b69ff2439-44244193410-1/summary.txt); [source-pinned original CIL history](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e75d91fc25f6b2b06e7d4f36047907b69ff2439-44244193410-1/b56_f1d1a_native_correlated_receipt_costs.jsonl); [independent original whole-event replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e75d91fc25f6b2b06e7d4f36047907b69ff2439-44244193410-1/b56_f1d1a_native_correlated_receipt_replay.json); [result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e75d91fc25f6b2b06e7d4f36047907b69ff2439-44244193410-1/result.txt).
- [Rust original VM test](../../cl0/crates/world/tests/cl1b_b56_f1d0_monitor_path_calibration.rs) adds a separately named F1d1a test alongside unchanged F1d0. [Independent JavaScript CIL interpreter](../analysis/b56_f1d1a_native_correlated_receipt_replay.mjs) parses the actual current Rust source graph, reexecutes **every native paid step**, and compares PC, event, registers, credits, graph, external writes, memory, storage, and final states. **First divergence null.**
- Original [scoped run 44243831061](https://github.com/iFrankStudio/cioc/actions/runs/44243831061) passed both native Rust tests but overall FAILED, correctly, because already-required F1b/F1c native traces were not produced under the selected-test scope. No claim of a successful entire scoped run was made. Full run above establishes green results; this audit's workflow fix requires all three dependent F1b/F1c/F1d0 native producers for scoped observer runs.

## Fixed real native test and resource accounting

128 isolated deterministic PID1 worlds = 2 researcher-preassembled monitor routes (EVERY/FOURTH) ×4 packet modes (FRESH, REPLAY_OLD, TORN_COMMIT, FORGED_CURRENT) ×16 exhaustive four-output truth masks. Each world uses **500 real native paid compute credits**, exactly four service Writes to cell11, 168 code slots, and the unchanged strict **16 data cells /180 code nodes /300 total storage units**. An actual B53 seven-instruction nonce generator runs before each Write; external bit20, echo27 and commit28 are written only AFTER that paid Write. No actor-generated code or autonomous policy switching occurs.

B53 check path (original nodes106–123) uses paid Read/Sub/Branch to compare current nonce26 to echo27 and commit28 before paid Read20, and rejects stale or torn receipt with a paid marker Write29. The preassembled F1c periodic monitoring counter was remapped from colliding key28 to key40, and common service continuation moved from colliding PC110 to PC70; these *researcher-authored changes* are disclosed, not hidden native mutations. Neither correlation nor commit checks authenticate evaluator semantics or close B54 time-of-check/time-of-use races.

Native totals independently replayed: **512 service Writes, 512 paid nonce generation Writes, 1,536 external feedback/echo/commit writes, 10,840 native paid CIL steps, 160 genuine feedback-bit Reads, 160 paid packet-rejection markers, 512 complete service segments; no CIL disagreement**. No uncharged request-serial setup or free compute replenishment.

## Actual price (16 worlds per policy and packet mode)

Costs exclude the **32 shared genuinely paid nonce-generator-plus-service-Write operations per world** only (the 4,096 shared native steps are INCLUDED in the 10,840 overall total).

| Response packets | EVERY monitor CIL steps | FOURTH monitor CIL steps | Paid feedback Reads E/F | Rejection markers E/F |
|---|---:|---:|---:|---:|
| FRESH: true bits, current echo+commit | **1,056** | **840** | 64 / 16 | 0 / 0 |
| REPLAY_OLD: success bit, old echo+commit | **576** | **720** | 0 / 0 | 64 / 16 |
| TORN_COMMIT: success bit, current echo, old commit | **960** | **816** | 0 / 0 | 64 / 16 |
| FORGED_CURRENT: forged success, current echo+commit | **960** | **816** | 64 / 16 | 0 / 0 |
| **Total** | **3,552** | **3,192** | 128 / 32 | 128 / 32 |

Price sum = **6,744 charged monitor instructions + 4,096 paid common instructions = 10,840**. Source-faithful B53 sensor cost for a coherent sampled reply is15 instructions; old-echo early rejection is9, torn-commit rejection is15, all under original capacity limits. In the four-success fresh case, monitoring costs **EVERY60 vs FOURTH51**, with4 vs1 actual bit observations. Under replay-old, costs **EVERY36 vs FOURTH45**, with4 vs1 invalid-packet detections and zero trusted-bit Reads. Price ranking is thus **conditional**: not one route globally dominant.

Critical information falsifier: FORGED_CURRENT keeps echo and commit equal to the actor-paid serial while lying about success. **40 actually sampled false-success bits pass** (32 EVERY +8 FOURTH across truth masks). They are NOT counted as correct service outputs; this native checker never proves external semantic truth.

## Scientific boundary and next gate

**ACCEPT only:** genuine source-priced sensor tradeoff, resource feasibility, stale/torn detection, current-serial forgery counterexample, full native/JS replay. **DO NOT claim:** native-paid self-generation of this new policy, comparative life-time utility, endogenous selection, source novelty, correct-output superiority, authenticated provenance, full B56-F1d1/B56-F1/B56-F1c, strict N56-A/B/C, B3/B4, or cyberlife.

**Next:** perform exact source/data-cell liveness and reuse budgeting against F1c's 162-slot founder before integrating 7 nonce+15 sensor CIL operations within180 nodes and16 live cells. Any required code reclamation/patching must be performed by the same PID1 with real instruction costs; no researcher-side memory teleportation. Only then preregister and test source-identical paid actor construction/selection with both trial orders, blinded/forged packet controls, monitor-edit-only prohibition, heldout shifts and strong static baselines. Fail closed if this capacity gate is not feasible.
